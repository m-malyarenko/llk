pub mod parser;
pub mod tree;

use std::collections::HashMap;
use std::collections::HashSet;

use crate::error::LlkError;

type LlkProduction = (char, Option<String>);
type LlkLut = HashMap<(char, String), String>;

pub struct LlkGrammar {
    term_symbols: HashSet<char>,
    nterm_symbols: HashSet<char>,
    start_symbol: char,
    lookahead: usize,
    productions: Vec<LlkProduction>,
}

impl LlkGrammar {
    const EOF: char = '\0';
    const MIN_LOOKAHEAD: usize = 1;
    const MAX_LOOKAHEAD: usize = 16;

    pub fn new(
        term_symbols: HashSet<char>,
        nterm_symbols: HashSet<char>,
        start_symbol: char,
        lookahead: usize,
        productions: Vec<LlkProduction>,
    ) -> Result<LlkGrammar, LlkError> {
        let grammar = LlkGrammar {
            term_symbols,
            nterm_symbols,
            start_symbol,
            lookahead,
            productions,
        };

        grammar_assert::assert_grammar(&grammar)?;

        Ok(grammar)
    }

    pub fn from_json(json_string: &str) -> Result<LlkGrammar, LlkError> {
        unimplemented!()
    }

    pub fn first(&self, string: &str) -> Result<HashSet<Option<String>>, LlkError> {
        grammar_assert::assert_grammar_string(self, string)?;

        let mut first_set = HashSet::new();

        if string.is_empty() {
            /* Calculate FIRST set of ε */
            first_set.insert(None);
        } else if string.len() == 1 {
            /* Calculate FIRST set of a symbol */
            let symbol = string.chars().nth(0).unwrap();

            if self.is_term(symbol) {
                /* FIRST set of terminal symbol is itself */
                first_set.insert(Some(string.to_owned()));
            } else {
                /* FIRST set of non-termonal symbol */

                /* If non-terminal symbol derives ε then add ε to its FIRST set */
                if self.derives_epsilon(symbol) {
                    first_set.insert(None);
                }

                /* Get all non-terminal non-ε derivatives */
                let derivatives: Vec<String> =
                    self.derive(symbol).drain(..).filter_map(|d| d).collect();

                /* Scan through all derivatives */
                for derivative in derivatives {
                    /* Skip symbols that have ε in their FIRST set */
                    let non_empty_suffix = if let Some(suffix) = derivative
                        .strip_prefix(|c: char| self.first(&c.to_string()).unwrap().contains(&None))
                    {
                        suffix
                    } else {
                        &derivative
                    };

                    if non_empty_suffix.is_empty() {
                        first_set.insert(None);
                    } else {
                        /* Get all k-prefixes of non-ε suffix of the production's RHS */
                        first_set.extend(
                            self.get_term_prefixes(non_empty_suffix)
                                .drain()
                                .map(|s| Some(s)),
                        )
                    }
                }
            }
        } else {
            /* Calculate FIRST set of a string */
            /* Most often used to define the FIRST set of the RHS of a production */
            if string
                .chars()
                .all(|symbol| self.first(&symbol.to_string()).unwrap().contains(&None))
            {
                first_set.insert(None);
            };

            first_set.extend(self.get_term_prefixes(string).drain().map(|s| Some(s)))
        }

        Ok(first_set)
    }

    pub fn follow(&self, nterm: char) -> Result<HashSet<String>, LlkError> {
        if !self.is_nterm(nterm) {
            return Err(LlkError::IllegalOperation(
                "FOLLOW set for not non-terminal symbol".to_string(),
            ));
        }

        fn inner<'a>(
            grammar: &'a LlkGrammar,
            nterm: char,
            visited: &mut HashSet<&'a LlkProduction>,
        ) -> HashSet<String> {
            let mut follow_set = HashSet::new();

            for production in &grammar.productions {
                if let None = production.1 {
                    continue;
                }

                let prod_nterm = production.0;
                let prod_derivative = (&production.1).as_ref().unwrap();

                if let Some(suffixes) = grammar.get_nterm_suffixes(prod_derivative, nterm) {
                    let mut suffixes_first_set: HashSet<Option<String>> = suffixes
                        .iter()
                        .flat_map(|s| grammar.first(s).unwrap())
                        .collect();

                    if prod_nterm != nterm
                        && !visited.contains(production)
                        && (prod_derivative.ends_with(nterm) || suffixes_first_set.contains(&None))
                    {
                        visited.insert(production);
                        follow_set.extend(inner(grammar, prod_nterm, visited));
                    }

                    follow_set.extend(suffixes_first_set.drain().filter_map(|s| s))
                }
            }

            follow_set
        }

        Ok(inner(self, nterm, &mut HashSet::new()))
    }
}

impl LlkGrammar {
    fn choise(&self, production: &LlkProduction) -> HashSet<String> {
        let prod_nterm = production.0;
        let prod_derivative = if let Some(derivative) = &production.1 {
            derivative.clone()
        } else {
            String::default()
        };

        let first_set: HashSet<String> = self
            .first(&prod_derivative)
            .unwrap()
            .drain()
            .map(|s| s.unwrap_or_default())
            .collect();
        let follow_set: HashSet<String> = self.follow(prod_nterm).unwrap();
        let choise_set: HashSet<String> = if follow_set.is_empty() {
            first_set
        } else {
            first_set
                .iter()
                .flat_map(|s| {
                    std::iter::repeat(s)
                        .zip(&follow_set)
                        .map(|(prefix, suffix)| {
                            let mut choise_string = format!("{}{}", prefix, suffix);
                            choise_string.truncate(self.lookahead);
                            choise_string
                        })
                })
                .collect()
        };

        choise_set
    }

    fn is_term(&self, symbol: char) -> bool {
        self.term_symbols.contains(&symbol) || symbol == LlkGrammar::EOF
    }

    fn is_nterm(&self, symbol: char) -> bool {
        self.nterm_symbols.contains(&symbol)
    }

    fn derives_epsilon(&self, symbol: char) -> bool {
        !self.is_term(symbol)
            && self
                .productions
                .iter()
                .any(|(nterm, derivative)| *nterm == symbol && matches!(derivative, None))
    }

    fn derive(&self, symbol: char) -> Vec<Option<String>> {
        if self.is_nterm(symbol) {
            self.productions
                .iter()
                .filter_map(|(nterm, derivative)| {
                    if *nterm == symbol {
                        Some(derivative.clone())
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            vec![Some(symbol.to_string())]
        }
    }

    fn get_term_prefixes(&self, string: &str) -> HashSet<String> {
        /* Recursive expansion procedure */
        fn inner(grammar: &LlkGrammar, string: &str, prefix_len: usize) -> Vec<String> {
            if let Some(leftmost_nterm) = string.chars().find(|&c| grammar.is_nterm(c)) {
                /* String contains at least one non-terminal symbol */
                let (prefix, suffix) = string.split_once(leftmost_nterm).unwrap();

                if prefix.len() >= prefix_len {
                    /* If prefix length is enough return it */
                    vec![format!(
                        "{term_prefix:.limit$}",
                        term_prefix = prefix,
                        limit = prefix_len
                    )]
                } else {
                    let prefix_rest_len = prefix_len - prefix.len();

                    grammar
                        .derive(leftmost_nterm)
                        .drain(..)
                        .map(|d| d.unwrap_or_default())
                        .filter(|s| !s.starts_with(leftmost_nterm))
                        .flat_map(|s| inner(grammar, &(s + suffix), prefix_rest_len))
                        .map(|s| {
                            format!(
                                "{prefix_start}{prefix_end}",
                                prefix_start = prefix,
                                prefix_end = s,
                            )
                        })
                        .collect()
                }
            } else {
                /* If string contains only terminal symbols return its k-prefix */
                vec![format!(
                    "{term_prefix:.limit$}",
                    term_prefix = string,
                    limit = prefix_len
                )]
            }
        }

        inner(self, string, self.lookahead).drain(..).collect()
    }

    fn get_nterm_suffixes<'a>(&self, string: &'a str, nterm: char) -> Option<Vec<&'a str>> {
        if string.is_empty() || !string.contains(nterm) {
            return None;
        }

        let mut suffixes = Vec::new();

        /* Find all x suffixes in wAx production derivation */
        let mut current_suffix = string;
        while let Some((_prefix, suffix)) = current_suffix.split_once(nterm) {
            if !suffix.is_empty() {
                /* If production is B => wAx push x to the list */
                suffixes.push(suffix);
            }
            current_suffix = suffix;
        }

        Some(suffixes)
    }

    fn format_production((nterm, derivative): &LlkProduction) -> String {
        let derivative = if let Some(string) = derivative {
            string.clone()
        } else {
            String::default()
        };

        format!("({} -> {})", nterm, derivative)
    }
}

mod grammar_assert {
    use super::LlkGrammar;
    use crate::error::LlkError;
    use std::collections::HashSet;

    pub(super) fn assert_grammar(grammar: &LlkGrammar) -> Result<(), LlkError> {
        assert_symbols(grammar)?;
        assert_productions(grammar)?;
        assert_lookahead(grammar)?;
        assert_llk_conditions(grammar)?;

        Ok(())
    }

    pub(super) fn assert_grammar_string(
        grammar: &LlkGrammar,
        string: &str,
    ) -> Result<(), LlkError> {
        let unknown_symbol = string
            .chars()
            .find(|c| !grammar.is_term(*c) && !grammar.is_nterm(*c));

        if let Some(symbol) = unknown_symbol {
            Err(LlkError::UnknownSymbol(symbol))
        } else if string.contains(LlkGrammar::EOF)
            && !string.ends_with(LlkGrammar::EOF)
            && !string.chars().filter(|c| *c == LlkGrammar::EOF).count() == 1
        {
            Err(LlkError::UnknownSymbol(LlkGrammar::EOF))
        } else {
            Ok(())
        }
    }

    pub(super) fn assert_term_string(grammar: &LlkGrammar, string: &str) -> Result<(), LlkError> {
        for symbol in string.chars() {
            if !grammar.is_term(symbol) {
                return Err(LlkError::UnknownSymbol(symbol));
            }
        }

        Ok(())
    }

    fn assert_symbols(grammar: &LlkGrammar) -> Result<(), LlkError> {
        /* Check for empty terminal symbols set */
        if grammar.term_symbols.is_empty() {
            return Err(LlkError::InvalidGrammar(
                "treminal symbols set is empty".to_string(),
            ));
        }
        /* Check for emty non-terminal symbols set */
        if grammar.nterm_symbols.is_empty() {
            return Err(LlkError::InvalidGrammar(
                "non-treminal symbols set is empty".to_string(),
            ));
        }
        /* Check if start symbol is non-terminal symbol */
        if !grammar.nterm_symbols.contains(&grammar.start_symbol) {
            return Err(LlkError::InvalidGrammar(
                "start symbol is not non-terminal".to_string(),
            ));
        }
        /* Check if treminal and non-terminal symbols sets intersects */
        if !grammar.term_symbols.is_disjoint(&grammar.nterm_symbols) {
            return Err(LlkError::InvalidGrammar(
                "terminal and non-terminal symbols sets intersects".to_string(),
            ));
        }

        Ok(())
    }

    fn assert_productions(grammar: &LlkGrammar) -> Result<(), LlkError> {
        /* Check for empty grammar productions list */
        if grammar.productions.is_empty() {
            return Err(LlkError::InvalidGrammar(
                "productions list is empty".to_string(),
            ));
        }
        /* Check that all the LHS of productions are non-terminal symbols */
        let invalid_lhs_list: Vec<String> = grammar
            .productions
            .iter()
            .filter_map(|p| {
                if !grammar.is_nterm(p.0) {
                    Some(LlkGrammar::format_production(p))
                } else {
                    None
                }
            })
            .collect();
        if !invalid_lhs_list.is_empty() {
            return Err(LlkError::InvalidGrammar(format!(
                "production LHS is not a non-terminal symbol: {:?}",
                invalid_lhs_list
            )));
        }
        /* Check if all the RHS of productions contains only defined symbols */
        let invalid_rhs_list: Vec<String> = grammar
            .productions
            .iter()
            .filter_map(|p| {
                if !grammar.is_term(p.0) && !grammar.is_nterm(p.0) {
                    Some(LlkGrammar::format_production(p))
                } else {
                    None
                }
            })
            .collect();
        if !invalid_rhs_list.is_empty() {
            return Err(LlkError::InvalidGrammar(format!(
                "unknown symbol(s) in production RHS: {:?}",
                invalid_rhs_list
            )));
        }
        /* Check if grammar rules list contains at least one start symbol production */
        if grammar
            .productions
            .iter()
            .all(|(nterm, _derivative)| *nterm != grammar.start_symbol)
        {
            return Err(LlkError::InvalidGrammar(
                "no production for start symbol".to_string(),
            ));
        }
        /* Check if each non-terminal symbols has at least one derivation */
        let unused_nterms_list: Vec<char> = grammar
            .nterm_symbols
            .iter()
            .filter(|&nterm| !grammar.productions.iter().any(|(c, _s)| *c == *nterm))
            .copied()
            .collect();
        if !unused_nterms_list.is_empty() {
            return Err(LlkError::InvalidGrammar(format!(
                "unused non-terminal symbol(s): {:?}",
                unused_nterms_list
            )));
        }

        Ok(())
    }

    fn assert_lookahead(grammar: &LlkGrammar) -> Result<(), LlkError> {
        /* Check is lookahead is in valid range */
        if grammar.lookahead < LlkGrammar::MIN_LOOKAHEAD
            || grammar.lookahead > LlkGrammar::MAX_LOOKAHEAD
        {
            return Err(LlkError::InvalidGrammar(format!(
                "illegal lookahead value: {}, must be in [{min},{max}]",
                grammar.lookahead,
                min = LlkGrammar::MIN_LOOKAHEAD,
                max = LlkGrammar::MAX_LOOKAHEAD,
            )));
        }

        Ok(())
    }

    fn assert_llk_conditions(grammar: &LlkGrammar) -> Result<(), LlkError> {
        /* Check for self left-recursion */
        let self_lr_productions: Vec<String> = grammar
            .productions
            .iter()
            .filter_map(|p| {
                let nterm = p.0;
                if let Some(derivative) = &p.1 {
                    if derivative.starts_with(nterm) {
                        Some(LlkGrammar::format_production(p))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        if !self_lr_productions.is_empty() {
            return Err(LlkError::InvalidGrammar(format!(
                "self left-recursion in production(s): {:?}",
                self_lr_productions
            )));
        }
        /* Check for cross left-recursion */
        let cross_lr_productions: Vec<String> = grammar
            .productions
            .iter()
            .filter_map(|p| {
                let nterm = p.0;
                let derivations: Vec<String> =
                    grammar.derive(nterm).drain(..).filter_map(|d| d).collect();
                let derived_nterm_prefixes: Vec<char> = derivations
                    .iter()
                    .filter_map(|s| {
                        if let Some(c) = s.chars().nth(0) {
                            if grammar.is_nterm(c) {
                                Some(c)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                if derived_nterm_prefixes.iter().any(|c| {
                    grammar
                        .derive(*c)
                        .drain(..)
                        .filter_map(|d| d)
                        .any(|s| s.starts_with(nterm))
                }) {
                    Some(LlkGrammar::format_production(p))
                } else {
                    None
                }
            })
            .collect();
        if !cross_lr_productions.is_empty() {
            return Err(LlkError::InvalidGrammar(format!(
                "cross left-recursion in production(s): {:?}",
                self_lr_productions
            )));
        }
        /* Check if grammar has unreachable or unresolved non-terminal symbols */
        let reachable_nterms = get_reachable_nterms(grammar);
        let resolvable_nterms = get_resolvable_nterms(grammar);
        let difference = &grammar.nterm_symbols - &(&reachable_nterms & &resolvable_nterms);
        if !difference.is_empty() {
            return Err(LlkError::InvalidGrammar(format!(
                "unreachable or unresolvable non-terminal symbols: {symbols:?}",
                symbols = difference
            )));
        }

        /* Check is grammar is LL(k) */
        for (i, production_a) in grammar.productions.iter().enumerate() {
            let choise_a = grammar.choise(production_a);

            for production_b in grammar.productions.iter().skip(i + 1) {
                if production_a.0 == production_b.0 && !choise_a.is_disjoint(&grammar.choise(production_b)) {
                    return Err(LlkError::InvalidGrammar(format!(
                        "grammar rules do not define LL({k}) grammar:\n\
                            \tproduction collision: {prod_a} and {prod_b}\n\
                            \tproduction choise can not be infered with lookahead 1",
                        k = grammar.lookahead,
                        prod_a = LlkGrammar::format_production(production_a),
                        prod_b = LlkGrammar::format_production(production_b),
                    )));
                }
            }
        }

        Ok(())
    }

    fn get_reachable_nterms(grammar: &LlkGrammar) -> HashSet<char> {
        let mut cur_reachable_set = HashSet::with_capacity(grammar.nterm_symbols.len() + 1);
        let mut next_reachable_set = HashSet::with_capacity(grammar.nterm_symbols.len() + 1);
        cur_reachable_set.insert(grammar.start_symbol);

        while next_reachable_set.len() != cur_reachable_set.len() {
            /*
             * Inspect new non-terminal symbols
             * and extend next set with derivative non-terminal symbols
             *
             * M_[0] = {S}
             * M_[k+1] = M_[k] UNION {B | EXIST A IN M_[k]: A => aBb}
             */
            let derivatives: HashSet<String> = (&cur_reachable_set - &next_reachable_set)
                .iter()
                .flat_map(|c| grammar.derive(*c))
                .collect::<HashSet<Option<String>>>()
                .drain()
                .filter_map(|d| d)
                .collect();

            next_reachable_set.extend(
                derivatives
                    .iter()
                    .flat_map(|s| s.chars().filter(|c| grammar.is_nterm(*c))),
            );
            next_reachable_set.extend(&cur_reachable_set);

            /* Swap sets */
            let tmp_set = cur_reachable_set;
            cur_reachable_set = next_reachable_set;
            next_reachable_set = tmp_set;
        }

        cur_reachable_set
    }

    fn get_resolvable_nterms(grammar: &LlkGrammar) -> HashSet<char> {
        let mut cur_resolvable_set = HashSet::with_capacity(grammar.nterm_symbols.len() + 1);
        let mut next_resolvable_set = HashSet::with_capacity(grammar.nterm_symbols.len() + 1);

        /* Include all the one-step resolvable non-terminal symbols into the initial set */
        cur_resolvable_set.extend(grammar.productions.iter().filter_map(|(c, s)| {
            if s.is_some()
                && s.as_ref()
                    .unwrap()
                    .chars()
                    .all(|x| grammar.term_symbols.contains(&x))
            {
                Some(c)
            } else {
                None
            }
        }));

        while next_resolvable_set.len() != cur_resolvable_set.len() {
            /*
             * Inspect non-terminal symbols that derives string with non-terminals in the current set
             * and extend next set with them
             *
             * V_[k+1] = V_[k] UNION {B | (EXIST a IN V*): B =>* a}
             */
            next_resolvable_set.extend(grammar.productions.iter().filter_map(|(c, s)| {
                if s.is_some()
                    && s.as_ref().unwrap().chars().all(|x| {
                        grammar.term_symbols.contains(&x) || cur_resolvable_set.contains(&x)
                    })
                {
                    Some(c)
                } else {
                    None
                }
            }));

            /* Swap sets */
            let tmp_set = cur_resolvable_set;
            cur_resolvable_set = next_resolvable_set;
            next_resolvable_set = tmp_set;
        }

        cur_resolvable_set
    }
}

#[test]
fn first_set_test() {
    use std::iter::FromIterator;

    let grammar = LlkGrammar {
        term_symbols: vec!['a', 'b', '$'].drain(..).collect(),
        nterm_symbols: vec!['S', 'A'].drain(..).collect(),
        start_symbol: 'S',
        lookahead: 3,
        productions: vec![
            ('S', Some("Ab$".to_string())),
            ('A', Some("aA".to_string())),
            ('A', Some("a".to_string())),
        ],
    };

    assert_eq!(
        grammar.first("S").unwrap(),
        HashSet::from_iter(vec![
            Some("ab$".to_string()),
            Some("aab".to_string()),
            Some("aaa".to_string())
        ])
    );
    assert_eq!(
        grammar.first("A").unwrap(),
        HashSet::from_iter(vec![
            Some("a".to_string()),
            Some("aa".to_string()),
            Some("aaa".to_string())
        ])
    );
    assert_eq!(
        grammar.first("a").unwrap(),
        HashSet::from_iter(vec![Some("a".to_string())])
    );
    assert_eq!(
        grammar.first("b").unwrap(),
        HashSet::from_iter(vec![Some("b".to_string())])
    );
    assert_eq!(
        grammar.first("$").unwrap(),
        HashSet::from_iter(vec![Some("$".to_string())])
    );
    assert_eq!(
        grammar.first("Ab$").unwrap(),
        HashSet::from_iter(vec![
            Some("ab$".to_string()),
            Some("aab".to_string()),
            Some("aaa".to_string())
        ])
    );
    assert_eq!(
        grammar.first("aA").unwrap(),
        HashSet::from_iter(vec![Some("aaa".to_string()), Some("aa".to_string())])
    );
}

#[test]
fn follow_set_test() {
    use std::iter::FromIterator;

    let grammar = LlkGrammar {
        term_symbols: vec!['a', 'b', '$'].drain(..).collect(),
        nterm_symbols: vec!['S', 'A'].drain(..).collect(),
        start_symbol: 'S',
        lookahead: 3,
        productions: vec![
            ('S', Some("Ab$".to_string())),
            ('A', Some("aA".to_string())),
            ('A', Some("a".to_string())),
        ],
    };

    assert_eq!(grammar.follow('S').unwrap(), HashSet::from_iter(vec![]));
    assert_eq!(
        grammar.follow('A').unwrap(),
        HashSet::from_iter(vec!["b$".to_string()])
    );
    assert!(matches!(
        grammar.follow('a'),
        Err(LlkError::IllegalOperation(_))
    ));
}
