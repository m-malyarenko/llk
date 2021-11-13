use serde_json::Value as JsonValue;
use std::collections::HashSet;

use super::LlkGrammar;
use crate::error::LlkError;

const TERM_SYMBOLS_KEY: &str = "term_symbols";
const NTERM_SYMBOLS_KEY: &str = "nterm_symbols";
const START_SYMBOL_KEY: &str = "start_symbol";
const LOOKAHEAD_KEY: &str = "lookahead";
const PRODUCTIONS_KEY: &str = "productions";
const PRODUCTION_NTERM_KEY: &str = "nterm";
const PRODUCTION_DERIVATIVE_KEY: &str = "derivative";

pub(super) fn parse_grammar_json(json_string: &str) -> Result<LlkGrammar, LlkError> {
    if let Ok(json_values) = serde_json::from_str(json_string) {
        let json_values: JsonValue = json_values;

        /* Terminal symbols */
        let term_symbols = &json_values[TERM_SYMBOLS_KEY];
        if !term_symbols.is_string() {
            return Err(LlkError::GrammarFromJsonFailed(
                "invalid terminal symbols definition format".to_string(),
            ));
        }
        let term_symbols: HashSet<char> = term_symbols.as_str().unwrap().chars().collect();

        /* Non-terminal symbols */
        let nterm_symbols = &json_values[NTERM_SYMBOLS_KEY];
        if !nterm_symbols.is_string() {
            return Err(LlkError::GrammarFromJsonFailed(
                "invalid non-terminal symbols definition format".to_string(),
            ));
        }
        let nterm_symbols: HashSet<char> = nterm_symbols.as_str().unwrap().chars().collect();

        /* Start symbol */
        let start_symbol = &json_values[START_SYMBOL_KEY];
        if !start_symbol.is_string() {
            return Err(LlkError::GrammarFromJsonFailed(
                "invalid start symbol definition format".to_string(),
            ));
        }
        let start_symbol = start_symbol.as_str().unwrap();
        if start_symbol.len() != 1 {
            return Err(LlkError::GrammarFromJsonFailed(
                "invalid start symbol definition".to_string(),
            ));
        }
        let start_symbol = start_symbol.chars().nth(0).unwrap();

        /* Lookahead */
        let lookahead = &json_values[LOOKAHEAD_KEY];
        if !lookahead.is_u64() {
            return Err(LlkError::GrammarFromJsonFailed(
                "invalid lookahead definition format".to_string(),
            ));
        }
        let lookahead = lookahead.as_u64().unwrap() as usize;

        /* Productions */
        let productions = &json_values[PRODUCTIONS_KEY];
        if !productions.is_array() {
            return Err(LlkError::GrammarFromJsonFailed(
                "invalid productions definition format".to_string(),
            ));
        }
        let productions_array = productions.as_array().unwrap();
        let mut productions = Vec::new();
        for production in productions_array {
            if !production.is_object() {
                return Err(LlkError::GrammarFromJsonFailed(
                    "invalid productions definition format".to_string(),
                ));
            }
            if !production[PRODUCTION_NTERM_KEY].is_string() {
                if !production.is_object() {
                    return Err(LlkError::GrammarFromJsonFailed(
                        "invalid production non-treminal definition format".to_string(),
                    ));
                }
            }
            let production_nterm = production[PRODUCTION_NTERM_KEY].as_str().unwrap();
            if production_nterm.len() != 1 {
                return Err(LlkError::GrammarFromJsonFailed(
                    "invalid production non-terminal definition".to_string(),
                ));
            }
            let production_nterm = production_nterm.chars().nth(0).unwrap();
            let production_derivative = production.get(PRODUCTION_DERIVATIVE_KEY);
            if let None = production_derivative {
                return Err(LlkError::GrammarFromJsonFailed(
                    "invalid production derivative definition".to_string(),
                ));
            }
            let production_derivative = production_derivative.unwrap();
            if !production_derivative.is_null() || !production_derivative.is_string() {
                return Err(LlkError::GrammarFromJsonFailed(
                    "invalid production derivative definition".to_string(),
                ));
            }
            let production_derivative = if production_derivative.is_null()
                || production_derivative.as_str().unwrap().is_empty()
            {
                None
            } else {
                Some(production_derivative.as_str().unwrap().to_owned())
            };

            productions.push((production_nterm, production_derivative))
        }

        let productions = LlkGrammar::normalize_productions(productions, start_symbol);

        let grammar = LlkGrammar::new(
            term_symbols,
            nterm_symbols,
            start_symbol,
            lookahead,
            productions,
        )?;

        Ok(grammar)
    } else {
        Err(LlkError::GrammarFromJsonFailed(
            "invalid JSON string".to_string(),
        ))
    }
}
