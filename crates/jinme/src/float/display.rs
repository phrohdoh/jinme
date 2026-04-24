use crate::prelude::*;
use ::std::fmt;

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted = format!("{}", f64::from(self));
        write!(
            f,
            "{}",
            if formatted.contains('.') {
                formatted
            } else {
                format!("{}.0", formatted)
            }
        )
    }
}
