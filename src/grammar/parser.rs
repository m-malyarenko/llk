use crate::error::LlkError;
use crate::grammar::tree::LlkTree;
use crate::grammar::{LlkGrammar, LlkLut};

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

        while lookahead_start < string.len() {
            let string_rest = &target_string[lookahead_start..];

            /* Get the top of the stack */
            let top = *stack.last().unwrap();

            /*
             * If stack top matches first symbol in the rest of the string
             * pop stack top symbol and continue from next input string symbol
             */
            if self.grammar.is_term(top) && top == string_rest.chars().nth(0).unwrap() {
                stack.pop();
                lookahead_start += 1;
            } else {
                let lookahead = if string_rest.len() >= lookahead_len {
                    string_rest[..lookahead_len].to_owned()
                } else {
                    string_rest[..].to_owned()
                };

                if let Some(production_rhs) = self.lut.get(&(top, lookahead)) {
                    stack.pop();

                    /* Push production RHS to the stack */
                    stack.extend(production_rhs.chars().rev());

                    /* Update derivation tree */
                    let tree_node_stack_top = tree_node_stack.pop().unwrap();

                    for symbol in production_rhs.chars() {
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
}

impl LlkParser {
    fn create_lut(grammar: &LlkGrammar) -> LlkLut {
        let mut lut = LlkLut::new();

        for production in &grammar.productions {
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
                    .zip(std::iter::repeat(prod_derivative)),
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
    println!("LUT: {:?}", parser.lut);

    let tree = parser.parse("aaab").unwrap();
    for symbol in &tree {
        print!("{}", symbol);
    }
    println!();
}
