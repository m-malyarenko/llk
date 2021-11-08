use crate::error::LlkError;
use crate::grammar::tree::LlkTree;
use crate::grammar::{LlkGrammar, LlkLut};

pub struct LlkParser {
    grammar: LlkGrammar,
    lut: LlkLut,
}

impl LlkParser {
    pub fn new(grammar: LlkGrammar) -> LlkParser {
        let lut = grammar.create_lut();
        LlkParser { grammar, lut: lut }
    }

    pub fn parse(&self, string: &str) -> Result<LlkTree, LlkError> {
        for c in string.chars() {
            if !self.grammar.is_term(c) {
                return Err(LlkError::UnknownSymbol(c));
            }
        }

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
