use crate::environment::RispEnv;
use crate::error::{RispResult};
use crate::parser::{RispExp, RispFunction};

// TODO AFTER TESTS fully implement
// TODO "define" symbols into env
// TODO lambdas/function definitions
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
            RispFunction::Plus => env.plus(rest),
            _ => panic!("NOT IMPLEMENTED YET!"),
        },
        _ => panic!("NOT IMPLEMENTED YET!"),
    }
}
