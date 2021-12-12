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
            RispFunction::Builtin(RispBuiltinFunction::Divide) => env.divide(rest),
            RispFunction::Builtin(RispBuiltinFunction::And) => env.boolean_and(rest),
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
            RispError::TypeError(ILLEGAL_TYPE_FOR_ARITHMETIC_OP)
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
            RispError::TypeError(ILLEGAL_TYPE_FOR_ARITHMETIC_OP)
        );
    }

    #[test]
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
        assert_eq!(
            eval(&exp, &mut env).unwrap(),
            RispExp::Integer(37 * 42 * 42 * 42)
        );

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
            RispError::TypeError(ILLEGAL_TYPE_FOR_ARITHMETIC_OP)
        );
    }

    #[test]
    fn divide_2_or_more() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Divide)),
            RispExp::Integer(100),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(100 / 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Divide)),
            RispExp::Integer(1000),
            RispExp::Integer(10),
            RispExp::Integer(10),
            RispExp::Integer(5),
        ]);
        assert_eq!(
            eval(&exp, &mut env).unwrap(),
            RispExp::Integer(1000 / 10 / 10 / 5)
        );

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Divide)),
            RispExp::Integer(100),
            RispExp::Integer(-50),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(100 / -50));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Divide)),
            RispExp::Integer(-100),
            RispExp::Integer(50),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(-100 / 50));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Divide)),
            RispExp::Float(-1000f64),
            RispExp::Integer(500),
        ]);
        assert_eq!(
            eval(&exp, &mut env).unwrap(),
            RispExp::Float(-1000f64 / 500f64)
        );
    }

    #[test]
    fn divide1() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Divide)),
            RispExp::Integer(37),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Integer(37));
    }

    #[test]
    fn divide0() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![RispExp::Func(RispFunction::Builtin(
            RispBuiltinFunction::Divide,
        ))]);
        assert_eq!(
            eval(&exp, &mut env).unwrap_err(),
            RispError::ArityMismatch(RispFunction::Builtin(RispBuiltinFunction::Divide))
        );
    }

    #[test]
    fn divide_non_number() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Divide)),
            RispExp::Symbol("Locutus".to_string()),
            RispExp::Integer(42),
        ]);
        assert_eq!(
            eval(&exp, &mut env).unwrap_err(),
            RispError::TypeError(ILLEGAL_TYPE_FOR_ARITHMETIC_OP)
        );
    }

    #[test]
    fn and_2_or_more() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Bool(true),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Bool(false),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Bool(false));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Bool(true),
            RispExp::Bool(false),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Bool(false));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Bool(true),
            RispExp::Bool(true),
            RispExp::Bool(true),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Bool(true));
    }

    #[test]
    fn and1() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Bool(false),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Bool(false));
    }

    #[test]
    fn and0() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![RispExp::Func(RispFunction::Builtin(
            RispBuiltinFunction::And,
        ))]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Bool(true));
    }

    #[test]
    fn and_non_bool() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Symbol("Locutus".to_string()),
            RispExp::Integer(42),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Nil,
            RispExp::Integer(42),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Bool(false));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Integer(0),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Integer(42),
            RispExp::Bool(false),
        ]);
        assert_eq!(eval(&exp, &mut env).unwrap(), RispExp::Bool(false));
    }

    // TODO NOW and
    // TODO NOW or
    // TODO NOW not
    // TODO NOW xor
}
