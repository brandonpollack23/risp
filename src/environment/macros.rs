#[macro_export]
macro_rules! number_list_apply {
    ($args:ident, $method:expr) => {{
        let t = RispEnv::check_for_illegal_arithmetic_input($args);
        if (t.is_err()) {
            Err(t.unwrap_err())
        } else {
            if $args.iter().any(|arg| matches!(arg, RispExp::Float(_))) {
                Ok(RispExp::Float($method(
                    $args.iter().map(|arg| RispEnv::exp_to_float(arg)),
                )))
            } else {
                Ok(RispExp::Integer($method(
                    $args.iter().map(RispEnv::exp_to_int),
                )))
            }
        }
    }};
}
#[macro_export]
macro_rules! number_list_subtractive_apply {
    ($args:ident, $default:expr, $operation:expr, $others_combiner:expr) => {{
        let t = RispEnv::check_for_illegal_arithmetic_input($args);
        if (t.is_err()) {
            Err(t.unwrap_err())
        } else {
            let first = $args.first();

            if first.is_none() {
                Ok($default)
            } else {
                let first = first.unwrap();
                let rest = &$args[1..];
                if $args.iter().any(|arg| matches!(arg, RispExp::Float(_))) {
                    let first = RispEnv::exp_to_float(first);
                    let sub: f64 = $others_combiner(rest.iter().map(RispEnv::exp_to_float));
                    Ok(RispExp::Float($operation(first, sub)))
                } else {
                    let first = RispEnv::exp_to_int(first);
                    let sub: i32 = $others_combiner(rest.iter().map(RispEnv::exp_to_int));
                    Ok(RispExp::Integer($operation(first, sub)))
                }
            }
        }
    }};
}
