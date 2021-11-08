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

        /* Init stack with the start symbol */
        stack.push(self.grammar.start_symbol);

        for start_symbol_idx in 0..string.len() {
            let rest_target_string = &target_string[start_symbol_idx..];
            let current_term = rest_target_string.chars().nth(0).unwrap();

            /* Get the top of the stack */
            let top = *stack.last().unwrap();

            /* If stack top is terminal symbol */
            if self.grammar.is_term(top) && top == current_term {
                stack.pop();
                continue;
            }

            /* If stack top is non-terminal symbol */
            let lookahead = if rest_target_string.len() >= lookahead_len {
                rest_target_string[..lookahead_len].to_owned()
            } else {
                rest_target_string[..].to_owned()
            };

            if let Some(production_rhs) = self.lut.get(&(top, lookahead)) {
                stack.pop();
                stack.extend(production_rhs.chars().rev());
            } else {
                return Err(LlkError::DerivationFailed(String::default())); // FIXME Исправить сообщение ошибки
            }
        }

        unimplemented!()
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
