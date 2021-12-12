use rustyline::validate::{
    MatchingBracketValidator, ValidationContext, ValidationResult, Validator,
};
use rustyline::Result;
use rustyline_derive::{Completer, Helper, Highlighter, Hinter};

#[derive(Completer, Helper, Highlighter, Hinter)]
pub struct RispValidator {
    bracket_validator: MatchingBracketValidator,
}

impl RispValidator {
    pub fn new() -> Self {
        RispValidator {
            bracket_validator: MatchingBracketValidator::new(),
        }
    }
}

impl Validator for RispValidator {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult> {
        // TODO check if all delimiters balanced and dont be valid in that case
        self.bracket_validator.validate(ctx)
    }
}
