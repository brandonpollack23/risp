use crate::environment::RispEnv;
use crate::error::RispResult;
use crate::parser::{RispBuiltinFunction, RispExp, RispFunction};

// TODO "define" symbols into env
// TODO lambdas/function definitions

// TODO NOW fully implement
pub fn eval(exp: &RispExp, env: &mut RispEnv) -> RispResult<RispExp> {
    match exp {
        RispExp::List(forms) => eval_func(forms, env),
        _ => panic!("NOT IMPLEMENTED YET!"),
    }
}

fn eval_func(forms: &[RispExp], env: &mut RispEnv) -> RispResult<RispExp> {
    let (first, rest) = forms.split_first().unwrap();
    match first {
        RispExp::Func(f) => match f {
            RispFunction::Builtin(RispBuiltinFunction::Plus) => env.plus(rest),
            RispFunction::Builtin(RispBuiltinFunction::Minus) => env.minus(rest),
            RispFunction::Builtin(RispBuiltinFunction::Multiply) => env.multiply(rest),
            _ => panic!("NOT IMPLEMENTED YET!"),
        },
        _ => panic!("NOT IMPLEMENTED YET!"),
    }
}

#[cfg(test)]
mod tests {
    use crate::error::{RispError, ILLEGAL_TYPE_FOR_ARITHMETIC_OP};
    use pretty_assertions::assert_eq;

    use crate::eval::eval;
    use crate::parser::{RispBuiltinFunction, RispExp, RispFunction};
    use crate::RispEnv;

    #[test]
    fn plus_2_or_more() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Integer(37),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(37 + 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Integer(37),
            RispExp::Integer(42),
            RispExp::Integer(42),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(37 + 3 * 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Integer(37),
            RispExp::Integer(-42),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(37 - 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Integer(-37),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(-37 + 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Float(-37f64),
            RispExp::Integer(42),
        ]);
        assert_eq!(
            eval(&exp, &mut env).unwrap(),
            RispExp::Float(-37f64 + 42f64)
        );
    }

    #[test]
    fn plus1() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Integer(37),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(37));
    }

    #[test]
    fn plus0() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![RispExp::Func(RispFunction::Builtin(
            RispBuiltinFunction::Plus,
        ))]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(0));
    }

    #[test]
    fn plus_non_number() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Symbol("Locutus".to_string()),
            RispExp::Integer(42),
        ]);
        assert_eq!(
            eval(&exp, &mut env).unwrap_err(),
            RispError::ArithmeticError(ILLEGAL_TYPE_FOR_ARITHMETIC_OP)
        );
    }

    #[test]
    fn minus_2_or_more() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::Integer(37),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(37 - 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::Integer(37),
            RispExp::Integer(42),
            RispExp::Integer(42),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(37 - 3 * 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::Integer(37),
            RispExp::Integer(-42),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(37 - (-42)));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::Integer(-37),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(-37 - 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::Float(-37f64),
            RispExp::Integer(42),
        ]);
        assert_eq!(
            eval(&exp, &mut env).unwrap(),
            RispExp::Float(-37f64 - 42f64)
        );
    }

    #[test]
    fn minus1() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::Integer(37),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(37));
    }

    #[test]
    fn minus0() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![RispExp::Func(RispFunction::Builtin(
            RispBuiltinFunction::Minus,
        ))]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(0));
    }

    #[test]
    fn minus_non_number() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::Symbol("Locutus".to_string()),
            RispExp::Integer(42),
        ]);
        assert_eq!(
            eval(&exp, &mut env).unwrap_err(),
            RispError::ArithmeticError(ILLEGAL_TYPE_FOR_ARITHMETIC_OP)
        );
    }

    fn multiply_2_or_more() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Multiply)),
            RispExp::Integer(37),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(37 * 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Multiply)),
            RispExp::Integer(37),
            RispExp::Integer(42),
            RispExp::Integer(42),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(37 * 3 * 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Multiply)),
            RispExp::Integer(37),
            RispExp::Integer(-42),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(37 * (-42)));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Multiply)),
            RispExp::Integer(-37),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(-37 * 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Multiply)),
            RispExp::Float(-37f64),
            RispExp::Integer(42),
        ]);
        assert_eq!(
            eval(&exp, &mut env).unwrap(),
            RispExp::Float(-37f64 * 42f64)
        );
    }

    #[test]
    fn multiply1() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Multiply)),
            RispExp::Integer(37),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(37));
    }

    #[test]
    fn multiply0() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![RispExp::Func(RispFunction::Builtin(
            RispBuiltinFunction::Multiply,
        ))]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(1));
    }

    #[test]
    fn multiply_non_number() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::Symbol("Locutus".to_string()),
            RispExp::Integer(42),
        ]);
        assert_eq!(
            eval(&exp, &mut env).unwrap_err(),
            RispError::ArithmeticError(ILLEGAL_TYPE_FOR_ARITHMETIC_OP)
        );
    }

    // TODO NOW div
    // TODO NOW and
    // TODO NOW or
    // TODO NOW not
    // TODO NOW xor
}
