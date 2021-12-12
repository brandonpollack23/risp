use crate::error::{ILLEGAL_TYPE_FOR_ARITHMETIC_OP, RispError, RispResult};
use crate::parser::{RispBuiltinFunction, RispExp, RispFunction};
use crate::{number_list_apply, number_list_subtractive_apply};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct RispEnv {
    pub data: HashMap<String, RispExp>,
}

impl RispEnv {
    pub fn def(&mut self, name: &String, exp: &RispExp) -> RispResult<RispExp> {
        self.data.insert(name.clone(), exp.clone());
        Ok(RispExp::Nil)
    }

    fn check_arity_is_two_or_less(rest: &[RispExp], error: RispError) -> RispResult<()> {
        if rest.len() > 2 {
            return Err(error);
        }
        Ok(())
    }

    fn truthiness(b: &RispExp) -> bool {
        match b {
            RispExp::Nil => false,
            RispExp::Bool(false) => false,
            _ => true,
        }
    }
    fn check_for_illegal_arithmetic_input(args: &[RispExp]) -> RispResult<()> {
        if args
            .iter()
            .any(|arg| !(matches!(arg, RispExp::Integer(_) | RispExp::Float(_))))
        {
            return Err(RispError::TypeError(ILLEGAL_TYPE_FOR_ARITHMETIC_OP));
        }

        Ok(())
    }

    fn exp_to_float(arg: &RispExp) -> f64 {
        match arg {
            RispExp::Integer(i) => f64::from(*i),
            RispExp::Float(f) => *f,
            _ => panic!(),
        }
    }

    fn exp_to_int(arg: &RispExp) -> i32 {
        match arg {
            RispExp::Integer(i) => *i,
            _ => panic!(),
        }
    }
}
