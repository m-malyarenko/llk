use std::collections::HashMap;

use crate::error::LlkError;
use crate::grammar::tree::LlkTree;
use crate::grammar::LlkGrammar;

type LlkLut = HashMap<(char, String), (String, usize)>;

pub struct LlkParser {
    grammar: LlkGrammar,
    lut: LlkLut,
}

impl LlkParser {
    pub fn new(grammar: LlkGrammar) -> LlkParser {
        LlkParser {
            lut: LlkParser::create_lut(&grammar),
            grammar,
        }
    }

    pub fn parse(&self, string: &str) -> Result<LlkTree, LlkError> {
        super::grammar_assert::assert_term_string(&self.grammar, string)?;

        let target_string: String = format!("{}{}", string, LlkGrammar::EOF);
        let lookahead_len = self.grammar.lookahead;
        let mut stack = Vec::new();
        let mut tree_root = LlkTree::new(self.grammar.start_symbol);
        let mut tree_node_stack = Vec::new();

        /* Init stack with the start symbol */
        stack.push(self.grammar.start_symbol);

        /* Init tree node stack with root tree */
        tree_node_stack.push(&mut tree_root as *mut LlkTree);

        let mut lookahead_start = 0;

        while lookahead_start < target_string.len() - 1 {
            let target_string_rest = &target_string[lookahead_start..];

            /* Get the top of the stack */
            let top = *stack.last().unwrap();

            /*
             * If stack top matches first symbol in the rest of the string
             * pop stack top symbol and continue from next input string symbol
             */
            if self.grammar.is_term(top) && top == target_string_rest.chars().nth(0).unwrap() {
                stack.pop();
                lookahead_start += 1;
            } else {
                let lookahead = if target_string_rest.len() >= lookahead_len {
                    target_string_rest[..lookahead_len].to_owned()
                } else {
                    target_string_rest[..].to_owned()
                };

                if let Some((rhs, id)) = self.lut.get(&(top, lookahead)) {
                    stack.pop();

                    /* Push production RHS to the stack */
                    stack.extend(rhs.chars().rev());

                    /* Update derivation tree */
                    let tree_node_stack_top = tree_node_stack.pop().unwrap();

                    unsafe {
                        (*tree_node_stack_top).set_production_id(*id);
                    }

                    for symbol in rhs.chars() {
                        if self.grammar.is_nterm(symbol) {
                            unsafe {
                                let new_node = (*tree_node_stack_top).push_node(symbol);
                                tree_node_stack.push(new_node);
                            }
                        } else if symbol != LlkGrammar::EOF {
                            unsafe {
                                (*tree_node_stack_top).push_leaf(symbol);
                            }
                        }
                    }
                } else {
                    return Err(LlkError::DerivationFailed(String::default()));
                }
            }
        }

        Ok(tree_root)
    }

    pub fn get_stat_string(&self) -> String {
        let mut output_buffer = String::new();

        output_buffer += &format!("LL({}) grammar:\n\n", self.grammar.lookahead);
        output_buffer += &format!("Terminal symbols: {:?}\n", self.grammar.term_symbols);
        output_buffer += &format!("Non-terminal symbols: {:?}\n", self.grammar.nterm_symbols);
        output_buffer += &format!("Start symbol: {:?}\n", self.grammar.start_symbol);
        output_buffer += "Productions:\n";

        for (id, production) in self.grammar.productions.iter().enumerate() {
            output_buffer += &format!("\t#{} {}\n", id, LlkGrammar::format_production(production));
        }

        output_buffer += &format!("\nLL({}) FIRST & FOLLOW:\n\n", self.grammar.lookahead);

        for nterm in &self.grammar.nterm_symbols {
            let first_set: std::collections::HashSet<String> = self
                .grammar
                .first(&nterm.to_string())
                .unwrap()
                .drain()
                .map(|s| {
                    if let Some(string) = s {
                        string.replace(LlkGrammar::EOF, "")
                    } else {
                        String::default()
                    }
                })
                .collect();
            let follow_set: std::collections::HashSet<String> = self
                .grammar
                .follow(*nterm)
                .unwrap()
                .iter()
                .map(|s| s.replace(LlkGrammar::EOF, ""))
                .collect();

            output_buffer += &format!("FIRST({}): {:?}\n", nterm, first_set);
            output_buffer += &format!("FOLLOW({}): {:?}\n\n", nterm, follow_set);
        }

        output_buffer += &format!(
            "LL({}) LUT:\n\
        N - non-treminal symbol\n\
        LA - lookahead string\n\n\
        +{:-^3}+{:-^la_len$}+{:-^10}\n",
            self.grammar.lookahead,
            "N",
            "LA",
            "RULE",
            la_len = self.grammar.lookahead + 2
        );

        for ((nterm, lookahead), (prod, id)) in &self.lut {
            let lookahead = lookahead.replace(LlkGrammar::EOF, "\u{25A9}");
            output_buffer += &format!(
                "|{:^3}|{:^la_len$}| #{} {:<8}\n",
                nterm,
                lookahead,
                id,
                prod,
                la_len = self.grammar.lookahead + 2
            )
        }

        output_buffer
    }
}

impl LlkParser {
    fn create_lut(grammar: &LlkGrammar) -> LlkLut {
        let mut lut = LlkLut::new();

        for (id, production) in grammar.productions.iter().enumerate() {
            let prod_nterm = production.0;
            let prod_derivative = if let Some(derivative) = &production.1 {
                derivative.clone()
            } else {
                String::default()
            };

            let choise_set = grammar.choise(production);

            lut.extend(
                std::iter::repeat(prod_nterm)
                    .zip(choise_set)
                    .zip(std::iter::repeat((prod_derivative, id))),
            );
        }

        lut
    }
}

#[test]
fn create_lut_test() {
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

    println!("Here comes the LUT: {:?}", LlkParser::create_lut(&grammar));
}

#[test]
fn parsing_test() {
    let term_symbols = vec!['a', 'b'].drain(..).collect();
    let nterm_symbols = vec!['S', 'A'].drain(..).collect();
    let start_symbol = 'S';
    let lookahead = 2;
    let productions = vec![
        ('S', Some("Ab".to_string())),
        ('A', Some("aA".to_string())),
        ('A', Some("a".to_string())),
    ];

    let grammar = LlkGrammar::new(
        term_symbols,
        nterm_symbols,
        start_symbol,
        lookahead,
        productions,
    )
    .unwrap();

    let parser = LlkParser::new(grammar);
    println!("{}", parser.get_stat_string());

    let tree = parser.parse("aaab").unwrap();
    for (symbol, production_id) in tree.lrn() {
        print!("{:?}", (symbol, production_id));
    }
    println!();
}
