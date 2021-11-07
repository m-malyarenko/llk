use std::collections::HashMap;
use std::collections::HashSet;

use crate::error::LlkError;

pub struct LlkGrammar {
    pub term_symbols: HashSet<char>,
    pub nterm_symbols: HashSet<char>,
    pub start_symbol: char,
    pub lookahead: usize,
    pub productions: HashMap<char, Vec<Option<String>>>,
}

pub(super) const EOF: char = '\u{0003}';

impl LlkGrammar {
    pub fn new(
        term_symbols: HashSet<char>,
        nterm_symbols: HashSet<char>,
        start_symbol: char,
        lookahead: usize,
        productions: Vec<(char, Option<String>)>,
    ) -> Result<LlkGrammar, LlkError> {
        unimplemented!()
    }

    pub fn from_json(json_string: &str) -> Result<LlkGrammar, LlkError> {
        unimplemented!()
    }

    pub fn first(&self, string: &str) -> HashSet<Option<String>> {
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
                        .strip_prefix(|c: char| self.first(&c.to_string()).contains(&None))
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
                .all(|symbol| self.first(&symbol.to_string()).contains(&None))
            {
                first_set.insert(None);
            };

            first_set.extend(self.get_term_prefixes(string).drain().map(|s| Some(s)))
        }

        first_set
    }

    pub fn follow(&self, nterm: char) -> HashSet<String> {
        unimplemented!()
    }
}

impl LlkGrammar {
    fn is_term(&self, symbol: char) -> bool {
        self.term_symbols.contains(&symbol) || symbol == EOF
    }

    fn is_nterm(&self, symbol: char) -> bool {
        self.nterm_symbols.contains(&symbol) || self.start_symbol == symbol
    }

    fn derives_epsilon(&self, symbol: char) -> bool {
        !self.is_term(symbol) && self.productions[&symbol].contains(&None)
    }

    fn derive(&self, symbol: char) -> Vec<Option<String>> {
        if self.is_nterm(symbol) {
            self.productions[&symbol].clone()
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
                        .filter(|s| !s.starts_with(leftmost_nterm)) // TODO Сомнительное рещение для предотвращения беск. рекурсии
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
}

mod grammar_assert {
    use super::LlkGrammar;
    use crate::error::LlkError;
    use std::collections::HashSet;

    fn assert_grammar(grammar: &LlkGrammar) -> Result<(), LlkError> {
        unimplemented!()
    }

    fn assert_symbols(grammar: &LlkGrammar) -> Result<(), LlkError> {
        unimplemented!()
    }

    fn assert_rules(grammar: &LlkGrammar) -> Result<(), LlkError> {
        unimplemented!()
    }

    fn assert_lookahead(grammar: &LlkGrammar) -> Result<(), LlkError> {
        unimplemented!()
    }

    fn get_reachable_nterms(grammar: &LlkGrammar) -> HashSet<char> {
        unimplemented!()
    }

    fn get_resolvable_nterms(grammar: &LlkGrammar) -> HashSet<char> {
        unimplemented!()
    }
}

#[test]
fn first_set_test() {
    use std::iter::FromIterator;

    let mut grammar = LlkGrammar {
        term_symbols: vec!['a', 'b', '$'].drain(..).collect(),
        nterm_symbols: vec!['A'].drain(..).collect(),
        start_symbol: 'S',
        lookahead: 3,
        productions: HashMap::new(),
    };

    grammar
        .productions
        .insert('S', vec![Some("Ab$".to_string())]);
    grammar
        .productions
        .insert('A', vec![Some("aA".to_string()), Some("a".to_string())]);

    assert_eq!(
        grammar.first("S"),
        HashSet::from_iter(vec![
            Some("ab$".to_string()),
            Some("aab".to_string()),
            Some("aaa".to_string())
        ])
    );
    assert_eq!(
        grammar.first("A"),
        HashSet::from_iter(vec![
            Some("a".to_string()),
            Some("aa".to_string()),
            Some("aaa".to_string())
        ])
    );
    assert_eq!(
        grammar.first("a"),
        HashSet::from_iter(vec![Some("a".to_string())])
    );
    assert_eq!(
        grammar.first("b"),
        HashSet::from_iter(vec![Some("b".to_string())])
    );
    assert_eq!(
        grammar.first("$"),
        HashSet::from_iter(vec![Some("$".to_string())])
    );
    assert_eq!(
        grammar.first("Ab$"),
        HashSet::from_iter(vec![
            Some("ab$".to_string()),
            Some("aab".to_string()),
            Some("aaa".to_string())
        ])
    );
    assert_eq!(
        grammar.first("aA"),
        HashSet::from_iter(vec![Some("aaa".to_string()), Some("aa".to_string())])
    );
}
