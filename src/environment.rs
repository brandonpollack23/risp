use crate::error::{RispError, RispResult, ILLEGAL_TYPE_FOR_ARITHMETIC_OP};
use crate::parser::RispExp;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct RispEnv {
    data: HashMap<String, RispExp>,
}

impl RispEnv {
    // TODO on math functions handle overflow/cases
    pub fn plus(&self, args: &[RispExp]) -> RispResult<RispExp> {
        Self::check_for_illegal_arithmetic_input(args)?;

        return if args.iter().any(|arg| matches!(arg, RispExp::Float(_))) {
            Ok(RispExp::Float(
                args.iter().map(|arg| Self::exp_to_float(arg)).sum(),
            ))
        } else {
            Ok(RispExp::Integer(
                args.iter().map(Self::exp_to_int).sum::<i32>(),
            ))
        };
    }

    pub fn minus(&self, args: &[RispExp]) -> RispResult<RispExp> {
        Self::check_for_illegal_arithmetic_input(args)?;

        let first = args.first();
        if first.is_none() {
            return Ok(RispExp::Integer(0));
        }

        let rest = &args[1..];
        return if args.iter().any(|arg| matches!(arg, RispExp::Float(_))) {
            let first = Self::exp_to_float(first.unwrap());
            let sub = rest.iter().map(Self::exp_to_float).sum::<f64>();
            Ok(RispExp::Float(first - sub))
        } else {
            let first = Self::exp_to_int(first.unwrap());
            let sub = rest.iter().map(Self::exp_to_int).sum::<i32>();
            Ok(RispExp::Integer(first - sub))
        };
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

    pub fn multiply(&self, p0: &[RispExp]) -> RispResult<RispExp> {
        todo!()
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
