use crate::environment::RispEnv;
use crate::error::{RispError, RispResult};
use crate::parser::{RispBuiltinFunction, RispExp, RispFunction};

pub fn eval(exp: RispExp, env: &mut RispEnv) -> RispResult<RispExp> {
    match exp {
        RispExp::List(forms) => eval_list_as_func(forms, env),
        RispExp::Symbol(s) if env.data.contains_key(&s) => {
            Ok(env.data.get(&s).map(|x| x.clone()).unwrap())
        }
        _ => Ok(exp),
    }
}

fn eval_list_as_func(mut forms: Vec<RispExp>, env: &mut RispEnv) -> RispResult<RispExp> {
    if forms.len() < 1 {
        return Ok(RispExp::List(vec![]));
    }

    let evaluated = forms
        .drain(..)
        .map(|x| eval(x, env))
        .collect::<RispResult<Vec<RispExp>>>()?;
    let (first, rest) = evaluated.split_first().unwrap();
    match first {
        RispExp::Func(f) => match f {
            RispFunction::Builtin(RispBuiltinFunction::Plus) => plus(&rest),
            RispFunction::Builtin(RispBuiltinFunction::Minus) => minus(&rest),
            RispFunction::Builtin(RispBuiltinFunction::Multiply) => multiply(&rest),
            RispFunction::Builtin(RispBuiltinFunction::Divide) => divide(&rest),

            RispFunction::Builtin(RispBuiltinFunction::And) => boolean_and(&rest),
            RispFunction::Builtin(RispBuiltinFunction::Xor) => boolean_xor(&rest),
            RispFunction::Builtin(RispBuiltinFunction::Or) => boolean_or(&rest),
            RispFunction::Builtin(RispBuiltinFunction::Not) => boolean_not(&rest),

            RispFunction::Builtin(RispBuiltinFunction::LT) => op_lt(&rest),
            RispFunction::Builtin(RispBuiltinFunction::LTE) => op_lte(&rest),
            RispFunction::Builtin(RispBuiltinFunction::GT) => op_gt(&rest),
            RispFunction::Builtin(RispBuiltinFunction::GTE) => op_gte(&rest),
            RispFunction::Builtin(RispBuiltinFunction::EQ) => op_eq(&rest),

            RispFunction::Builtin(RispBuiltinFunction::Def) => {
                if rest.len() != 2 {
                    return Err(RispError::ArityMismatch(f.clone()));
                }
                if let RispExp::Symbol(name) = rest.get(0).unwrap() {
                    let expr = rest.get(1).unwrap();
                    return env.def(name, expr);
                }
                return Err(RispError::MalformedDefExpression);
            }

            RispFunction::Builtin(RispBuiltinFunction::If) => {
                if rest.len() != 3 {
                    return Err(RispError::ArityMismatch(f.clone()));
                }
                if let RispExp::Bool(b) = rest.get(0).unwrap() {
                    let true_branch = rest.get(1).unwrap();
                    let else_branch = rest.get(2).unwrap();
                    return Ok(if *b {
                        true_branch.clone()
                    } else {
                        else_branch.clone()
                    });
                }
                return Err(RispError::MalformedDefExpression);
            }

            RispFunction::Function(f) => f(&rest),
        },
        _ => Err(RispError::FirstFormMustBeFunction(first.clone())),
    }
}

// TODO on math functions handle overflow/cases
pub fn plus(args: &[RispExp]) -> RispResult<RispExp> {
    number_list_apply!(args, Iterator::sum)
}

pub fn minus(args: &[RispExp]) -> RispResult<RispExp> {
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

/// Boolean and all args.  The only false values are nil and false.
pub fn boolean_and(&self, args: &[RispExp]) -> RispResult<RispExp> {
    Ok(RispExp::Bool(
        args.iter()
            .map(Self::truthiness)
            .reduce(|x, y| x && y)
            .unwrap_or(true),
    ))
}

pub fn boolean_or(&self, args: &[RispExp]) -> RispResult<RispExp> {
    Ok(RispExp::Bool(
        args.iter()
            .map(Self::truthiness)
            .reduce(|x, y| x || y)
            .unwrap_or(false),
    ))
}

pub fn boolean_xor(&self, args: &[RispExp]) -> RispResult<RispExp> {
    Ok(RispExp::Bool(
        args.iter()
            .map(Self::truthiness)
            .reduce(|x, y| x ^ y)
            .unwrap_or(false),
    ))
}

pub fn boolean_not(&self, args: &[RispExp]) -> RispResult<RispExp> {
    if args.len() != 1 {
        return Err(RispError::ArityMismatch(RispFunction::Builtin(
            RispBuiltinFunction::Not,
        )));
    }
    Ok(RispExp::Bool(!Self::truthiness(args.first().unwrap())))
}

pub fn op_lt(&self, rest: &[RispExp]) -> RispResult<RispExp> {
    Self::check_arity_is_two_or_less(
        rest,
        RispError::ArityMismatch(RispFunction::Builtin(RispBuiltinFunction::LT)),
    )?;
    if rest.len() == 1 {
        return Ok(RispExp::Bool(true));
    }
    match (rest.get(0).unwrap(), rest.get(1).unwrap()) {
        (a, b) => Ok(RispExp::Bool(a < b)),
    }
}

pub fn op_lte(&self, rest: &[RispExp]) -> RispResult<RispExp> {
    Self::check_arity_is_two_or_less(
        rest,
        RispError::ArityMismatch(RispFunction::Builtin(RispBuiltinFunction::LTE)),
    )?;
    if rest.len() == 1 {
        return Ok(RispExp::Bool(true));
    }
    match (rest.get(0).unwrap(), rest.get(1).unwrap()) {
        (a, b) => Ok(RispExp::Bool(a <= b)),
    }
}

pub fn op_gt(&self, rest: &[RispExp]) -> RispResult<RispExp> {
    Self::check_arity_is_two_or_less(
        rest,
        RispError::ArityMismatch(RispFunction::Builtin(RispBuiltinFunction::GT)),
    )?;
    if rest.len() == 1 {
        return Ok(RispExp::Bool(true));
    }
    match (rest.get(0).unwrap(), rest.get(1).unwrap()) {
        (a, b) => Ok(RispExp::Bool(a > b)),
    }
}

pub fn op_gte(&self, rest: &[RispExp]) -> RispResult<RispExp> {
    Self::check_arity_is_two_or_less(
        rest,
        RispError::ArityMismatch(RispFunction::Builtin(RispBuiltinFunction::GTE)),
    )?;
    if rest.len() == 1 {
        return Ok(RispExp::Bool(true));
    }
    match (rest.get(0).unwrap(), rest.get(1).unwrap()) {
        (a, b) => Ok(RispExp::Bool(a >= b)),
    }
}

pub fn op_eq(&self, rest: &[RispExp]) -> RispResult<RispExp> {
    Self::check_arity_is_two_or_less(
        rest,
        RispError::ArityMismatch(RispFunction::Builtin(RispBuiltinFunction::EQ)),
    )?;
    if rest.len() == 1 {
        return Ok(RispExp::Bool(true));
    }
    match (rest.get(0).unwrap(), rest.get(1).unwrap()) {
        (a, b) => Ok(RispExp::Bool(a == b)),
    }
}


#[cfg(test)]
mod tests {
    use crate::error::{RispError, ILLEGAL_TYPE_FOR_ARITHMETIC_OP};
    use pretty_assertions::assert_eq;

    use crate::environment::RispEnv;
    use crate::eval::eval;
    use crate::parser::{RispBuiltinFunction, RispExp, RispFunction};

    #[test]
    fn plus_2_or_more() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Integer(37),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(37 + 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Integer(37),
            RispExp::Integer(42),
            RispExp::Integer(42),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(37 + 3 * 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Integer(37),
            RispExp::Integer(-42),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(37 - 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Integer(-37),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(-37 + 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Float(-37f64),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Float(-37f64 + 42f64));
    }

    #[test]
    fn plus1() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Integer(37),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(37));
    }

    #[test]
    fn plus0() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![RispExp::Func(RispFunction::Builtin(
            RispBuiltinFunction::Plus,
        ))]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(0));
    }

    #[test]
    fn plus_non_number() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::String("Locutus".to_owned()),
            RispExp::Integer(42),
        ]);
        assert_eq!(
            eval(exp, &mut env).unwrap_err(),
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
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(37 - 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::Integer(37),
            RispExp::Integer(42),
            RispExp::Integer(42),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(37 - 3 * 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::Integer(37),
            RispExp::Integer(-42),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(37 - (-42)));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::Integer(-37),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(-37 - 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::Float(-37f64),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Float(-37f64 - 42f64));
    }

    #[test]
    fn minus1() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::Integer(37),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(37));
    }

    #[test]
    fn minus0() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![RispExp::Func(RispFunction::Builtin(
            RispBuiltinFunction::Minus,
        ))]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(0));
    }

    #[test]
    fn minus_non_number() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::String("Locutus".to_owned()),
            RispExp::Integer(42),
        ]);
        assert_eq!(
            eval(exp, &mut env).unwrap_err(),
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
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(37 * 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Multiply)),
            RispExp::Integer(37),
            RispExp::Integer(42),
            RispExp::Integer(42),
            RispExp::Integer(42),
        ]);
        assert_eq!(
            eval(exp, &mut env).unwrap(),
            RispExp::Integer(37 * 42 * 42 * 42)
        );

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Multiply)),
            RispExp::Integer(37),
            RispExp::Integer(-42),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(37 * (-42)));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Multiply)),
            RispExp::Integer(-37),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(-37 * 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Multiply)),
            RispExp::Float(-37f64),
            RispExp::Integer(42),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Float(-37f64 * 42f64));
    }

    #[test]
    fn multiply1() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Multiply)),
            RispExp::Integer(37),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(37));
    }

    #[test]
    fn multiply0() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![RispExp::Func(RispFunction::Builtin(
            RispBuiltinFunction::Multiply,
        ))]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(1));
    }

    #[test]
    fn multiply_non_number() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Minus)),
            RispExp::String("Locutus".to_owned()),
            RispExp::Integer(42),
        ]);
        assert_eq!(
            eval(exp, &mut env).unwrap_err(),
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
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(100 / 42));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Divide)),
            RispExp::Integer(1000),
            RispExp::Integer(10),
            RispExp::Integer(10),
            RispExp::Integer(5),
        ]);
        assert_eq!(
            eval(exp, &mut env).unwrap(),
            RispExp::Integer(1000 / 10 / 10 / 5)
        );

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Divide)),
            RispExp::Integer(100),
            RispExp::Integer(-50),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(100 / -50));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Divide)),
            RispExp::Integer(-100),
            RispExp::Integer(50),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(-100 / 50));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Divide)),
            RispExp::Float(-1000f64),
            RispExp::Integer(500),
        ]);
        assert_eq!(
            eval(exp, &mut env).unwrap(),
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
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(37));
    }

    #[test]
    fn divide0() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![RispExp::Func(RispFunction::Builtin(
            RispBuiltinFunction::Divide,
        ))]);
        assert_eq!(
            eval(exp, &mut env).unwrap_err(),
            RispError::ArityMismatch(RispFunction::Builtin(RispBuiltinFunction::Divide))
        );
    }

    #[test]
    fn divide_non_number() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Divide)),
            RispExp::String("Locutus".to_owned()),
            RispExp::Integer(42),
        ]);
        assert_eq!(
            eval(exp, &mut env).unwrap_err(),
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
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Bool(false),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Bool(true),
            RispExp::Bool(false),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Bool(true),
            RispExp::Bool(true),
            RispExp::Bool(true),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));
    }

    #[test]
    fn and1() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Bool(false),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));
    }

    #[test]
    fn and0() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![RispExp::Func(RispFunction::Builtin(
            RispBuiltinFunction::And,
        ))]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));
    }

    #[test]
    fn and_non_bool() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::String("Locutus".to_owned()),
            RispExp::Integer(42),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Nil,
            RispExp::Integer(42),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Integer(0),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::And)),
            RispExp::Integer(42),
            RispExp::Bool(false),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));
    }

    #[test]
    fn or_2_or_more() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Or)),
            RispExp::Bool(true),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Or)),
            RispExp::Bool(false),
            RispExp::Bool(false),
            RispExp::Bool(false),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Or)),
            RispExp::Bool(true),
            RispExp::Bool(false),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Or)),
            RispExp::Bool(true),
            RispExp::Bool(true),
            RispExp::Bool(true),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));
    }

    #[test]
    fn or1() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Or)),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Or)),
            RispExp::Bool(false),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));
    }

    #[test]
    fn or0() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![RispExp::Func(RispFunction::Builtin(
            RispBuiltinFunction::Or,
        ))]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));
    }

    #[test]
    fn or_non_bool() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Or)),
            RispExp::String("Locutus".to_owned()),
            RispExp::Integer(42),
            RispExp::Bool(false),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Or)),
            RispExp::Nil,
            RispExp::Bool(false),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Or)),
            RispExp::Integer(0),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Or)),
            RispExp::Integer(42),
            RispExp::Bool(false),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));
    }

    #[test]
    fn not_2_or_more() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Not)),
            RispExp::Bool(true),
            RispExp::Bool(true),
        ]);
        assert_eq!(
            eval(exp, &mut env).unwrap_err(),
            RispError::ArityMismatch(RispFunction::Builtin(RispBuiltinFunction::Not))
        );

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Not)),
            RispExp::Bool(false),
            RispExp::Bool(false),
            RispExp::Bool(false),
        ]);
        assert_eq!(
            eval(exp, &mut env).unwrap_err(),
            RispError::ArityMismatch(RispFunction::Builtin(RispBuiltinFunction::Not))
        );
    }

    #[test]
    fn not1() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Not)),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Not)),
            RispExp::Bool(false),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));
    }

    #[test]
    fn not0() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![RispExp::Func(RispFunction::Builtin(
            RispBuiltinFunction::Not,
        ))]);
        assert_eq!(
            eval(exp, &mut env).unwrap_err(),
            RispError::ArityMismatch(RispFunction::Builtin(RispBuiltinFunction::Not))
        );
    }

    #[test]
    fn not_non_bool() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Not)),
            RispExp::String("Locutus".to_owned()),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Not)),
            RispExp::Nil,
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Not)),
            RispExp::Integer(0),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));
    }

    #[test]
    fn xor_2_or_more() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Xor)),
            RispExp::Bool(true),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Xor)),
            RispExp::Bool(false),
            RispExp::Bool(false),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Xor)),
            RispExp::Bool(true),
            RispExp::Bool(false),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Xor)),
            RispExp::Bool(true),
            RispExp::Bool(true),
            RispExp::Bool(true),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));
    }

    #[test]
    fn xor1() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Xor)),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Xor)),
            RispExp::Bool(false),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));
    }

    #[test]
    fn xor0() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![RispExp::Func(RispFunction::Builtin(
            RispBuiltinFunction::Xor,
        ))]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));
    }

    #[test]
    fn xor_non_bool() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Xor)),
            RispExp::String("Locutus".to_owned()),
            RispExp::Bool(false),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Xor)),
            RispExp::Nil,
            RispExp::Bool(false),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Xor)),
            RispExp::Integer(0),
            RispExp::Bool(true),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(false));

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Xor)),
            RispExp::Integer(42),
            RispExp::Bool(false),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Bool(true));
    }

    #[test]
    fn nested_builtin_eval() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Integer(37),
            RispExp::List(vec![
                RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
                RispExp::Integer(42),
                RispExp::Integer(100),
            ]),
        ]);
        assert_eq!(
            eval(exp, &mut env).unwrap(),
            RispExp::Integer(37 + 42 + 100)
        );

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::List(vec![
                RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
                RispExp::Integer(42),
                RispExp::List(vec![
                    RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
                    RispExp::Integer(42),
                    RispExp::Integer(100),
                ]),
            ]),
            RispExp::Integer(37),
        ]);
        assert_eq!(
            eval(exp, &mut env).unwrap(),
            RispExp::Integer(42 + 42 + 100 + 37)
        );
    }

    #[test]
    fn def_works() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Def)),
            RispExp::Symbol("captain".to_owned()),
            RispExp::String("picard".to_owned()),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Nil);
        assert_eq!(
            env.data.get("captain").unwrap(),
            &RispExp::String("picard".to_owned())
        );

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Def)),
            RispExp::Symbol("one".to_owned()),
            RispExp::Integer(1),
        ]);
        eval(exp, &mut env).unwrap();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Def)),
            RispExp::Symbol("two".to_owned()),
            RispExp::Integer(2),
        ]);
        eval(exp, &mut env).unwrap();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::Plus)),
            RispExp::Symbol("one".to_owned()),
            RispExp::Symbol("two".to_owned()),
        ]);
        assert_eq!(eval(exp, &mut env).unwrap(), RispExp::Integer(3));
    }

    #[test]
    fn if_works() {
        let mut env = RispEnv::default();
        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::If)),
            RispExp::Bool(true),
            RispExp::String("true".to_owned()),
            RispExp::String("false".to_owned()),
        ]);
        assert_eq!(
            eval(exp, &mut env).unwrap(),
            RispExp::String("true".to_owned())
        );

        let exp = RispExp::List(vec![
            RispExp::Func(RispFunction::Builtin(RispBuiltinFunction::If)),
            RispExp::Bool(false),
            RispExp::String("true".to_owned()),
            RispExp::String("false".to_owned()),
        ]);
        assert_eq!(
            eval(exp, &mut env).unwrap(),
            RispExp::String("false".to_owned())
        );
    }
}
