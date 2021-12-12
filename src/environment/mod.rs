mod macros;

use crate::error::{RispError, RispResult, ILLEGAL_TYPE_FOR_ARITHMETIC_OP};
use crate::parser::{RispBuiltinFunction, RispExp, RispFunction};
use crate::{number_list_apply, number_list_subtractive_apply};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct RispEnv {
    data: HashMap<String, RispExp>,
}

impl RispEnv {
    // TODO on math functions handle overflow/cases
    pub fn plus(&self, args: &[RispExp]) -> RispResult<RispExp> {
        number_list_apply!(args, Iterator::sum)
    }

    pub fn minus(&self, args: &[RispExp]) -> RispResult<RispExp> {
        return number_list_subtractive_apply!(
            args,
            RispExp::Integer(0),
            std::ops::Sub::sub,
            Iterator::sum
        );
    }

    pub fn multiply(&self, args: &[RispExp]) -> RispResult<RispExp> {
        number_list_apply!(args, Iterator::product)
    }

    pub fn divide(&self, args: &[RispExp]) -> RispResult<RispExp> {
        if args.len() < 1 {
            return Err(RispError::ArityMismatch(RispFunction::Builtin(
                RispBuiltinFunction::Divide,
            )));
        }

        number_list_subtractive_apply!(
            args,
            RispExp::Integer(0),
            std::ops::Div::div,
            Iterator::product
        )
    }

    fn check_for_illegal_arithmetic_input(args: &[RispExp]) -> RispResult<()> {
        if args
            .iter()
            .any(|arg| !(matches!(arg, RispExp::Integer(_) | RispExp::Float(_))))
        {
            return Err(RispError::ArithmeticError(ILLEGAL_TYPE_FOR_ARITHMETIC_OP));
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