use crate::parser::RispExp;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct RispEnv {
    data: HashMap<String, RispExp>,
}
