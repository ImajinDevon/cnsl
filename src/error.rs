//! A module for error handling.
use std::fmt::Display;

/// A trait for expecting a value, and panicking with the raw message if it is not present.
pub trait RawPanics<T, E> {
    /// Consumes `self`, then returns the stored value, else panics with the given message.
    /// # Examples
    /// ```
    /// use cnsl::error::RawPanics;
    /// use cnsl::stdin;
    ///
    /// let name = stdin::preadln("Enter your name: ").panics();
    /// ```
    fn panics(self) -> T;
}

/// This implementation provides a way of unwrapping a result, and panicking with
/// its error message if it contains an [Err] value.
impl<T, E> RawPanics<T, E> for Result<T, E>
where
    E: Display,
{
    /// Consumes `self`, then returns the stored value, else the error message.
    /// # Examples
    /// ```
    /// use cnsl::error::RawPanics;
    /// use cnsl::stdin;
    ///
    /// let name = stdin::preadln_raw(b"Enter your name").panics();
    /// ```
    fn panics(self) -> T {
        self.unwrap_or_else(|err| panic!("{}", err))
    }
}
