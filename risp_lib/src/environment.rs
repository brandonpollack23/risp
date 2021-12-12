use crate::error::{RispError, RispResult};
use crate::parser::RispExp;

use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct RispEnv<'a> {
    data: HashMap<String, RispExp>,
    outer: Option<&'a RispEnv<'a>>,
}

impl<'a> RispEnv<'a> {
    pub fn with_outer<'b>(outer: &'b RispEnv) -> RispEnv<'b> {
        RispEnv {
            data: Default::default(),
            outer: Some(outer),
        }
    }

    pub fn def(&mut self, name: &str, exp: &RispExp) -> RispResult<RispExp> {
        self.data.insert(name.to_owned(), exp.clone());
        Ok(RispExp::Nil)
    }

    pub fn has_interned_var(&self, name: &str) -> bool {
        self.data.contains_key(name)
    }

    pub fn get(&self, name: &str) -> RispResult<&RispExp> {
        match self.data.get(name) {
            Some(r) => Ok(r),
            None => self
                .outer
                .map_or(Err(RispError::UnexpectedSymbol(name.to_owned())), |outer| {
                    outer.get(name)
                }),
        }
    }
}
