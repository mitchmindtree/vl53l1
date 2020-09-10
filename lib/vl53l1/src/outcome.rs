use crate::error::{StError, Warning};
use core::convert::TryFrom;

/// An alternative to the `Ok` variant of `Result` that allows for handling warnings.
#[derive(Debug, Default)]
#[must_use = "this `Outcome` may contain a warning, which should be handled"]
pub struct Outcome<T> {
    value: T,
    pub warning: Option<Warning>,
}

impl<T> Outcome<T> {
    /// Create a new `Outcome` instance.
    pub fn new(value: T, warning: Option<Warning>) -> Self {
        Self { value, warning }
    }

    /// An outcome with no warning.
    pub fn ok(value: T) -> Self {
        Self::new(value, None)
    }

    /// An outcome with a warning.
    pub fn warn(value: T, warning: Warning) -> Self {
        Self::new(value, Some(warning))
    }

    /// Map the outcome to a new type with the given function.
    pub fn map<F, U>(self, f: F) -> Outcome<U>
    where
        F: FnOnce(T) -> U,
    {
        let Outcome { value, warning } = self;
        let value = f(value);
        Outcome { value, warning }
    }

    /// Returns `Ok` if the warning was in the set of warnings to be ignored, otherwise returns the
    /// `Warning` as an `Err`.
    pub fn ignore_warnings(self, to_ignore: &[Warning]) -> Result<T, Warning> {
        match self.warning {
            Some(warning) => match to_ignore.iter().any(|&w| warning == w) {
                true => Ok(self.value),
                false => Err(warning),
            },
            None => Ok(self.value),
        }
    }

    /// Ignore warnings if any were emitted and return the value.
    pub fn ignore_all_warnings(self) -> T {
        self.value
    }

    /// Return an error in the case that a warning was produced.
    pub fn err_on_warning(self) -> Result<T, Warning> {
        match self.warning {
            Some(warning) => Err(warning),
            None => Ok(self.value),
        }
    }
}

impl<T, E> From<Outcome<T>> for Result<Outcome<T>, E> {
    fn from(outcome: Outcome<T>) -> Self {
        Ok(outcome)
    }
}

/// Produce a `Result` from an error value from the original API.
pub fn from_st(status: i8) -> Result<Outcome<()>, StError> {
    // Check for OK.
    if status == 0 {
        return Outcome::ok(()).into();
    }

    // Check for warning.
    let res = Warning::try_from(status).map(|w| Outcome::warn((), w));
    if let Ok(outcome) = res {
        return Ok(outcome);
    }

    // Check for error.
    match StError::try_from(status) {
        Ok(err) => return Err(err),
        // TODO: Not in original, but at least indicates unhandled error?
        _ => Err(StError::NOT_IMPLEMENTED),
    }
}
