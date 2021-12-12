use crate::error::{RispError, RispResult};
use crate::parser::{RispExp};

use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct RispEnv {
    data: HashMap<String, RispExp>,
}

impl RispEnv {
    pub fn def(&mut self, name: &str, exp: &RispExp) -> RispResult<RispExp> {
        self.data.insert(name.to_owned(), exp.clone());
        Ok(RispExp::Nil)
    }

    pub fn has_interned_var(&self, name: &str) -> bool {
        self.data.contains_key(name)
    }

    pub fn get(&self, name: &str) -> RispResult<&RispExp> {
        self.data
            .get(name)
            .ok_or(RispError::UnexpectedSymbol(name.to_owned()))
    }
}
