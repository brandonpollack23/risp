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
            _ => panic!("NOT IMPLEMENTED YET!"),
        },
        _ => panic!("NOT IMPLEMENTED YET!"),
    }
}

#[cfg(test)]
mod tests {
    // TODO NOW plus
    // TODO NOW minus
    // TODO NOW mul
    // TODO NOW div
    // TODO NOW and
    // TODO NOW or
    // TODO NOW not
    // TODO NOW xor
}
