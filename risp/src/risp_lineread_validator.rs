use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::Result;

struct RispValidator;
impl Validator for RispValidator {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult> {
        // TODO check if all delimiters balanced and dont be valid in that case
        let input = ctx.input();
        if input.ends_with("\\\n") {
            return Ok(ValidationResult::Invalid(None));
        }

        Ok(ValidationResult::Valid(None))
    }
}
