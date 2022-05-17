//! A module for error handling.
use std::io;

pub trait Panics<T> {
    /// Consumes `self`, then returns the stored value, else panics with the given message.
    /// # Examples
    /// ```
    /// use cnsl::error::Panics;
    /// use cnsl::stdin;
    ///
    /// let name = stdin::preadln("Enter your name: ").panics();
    /// ```
    fn panics(self) -> T;
}

impl Panics<String> for io::Result<String> {
    /// Consumes `self`, then returns the stored value, else the error message.
    /// # Examples
    /// ```
    /// use cnsl::error::Panics;
    /// use cnsl::stdin;
    ///
    /// let name = stdin::preadln_raw(b"Enter your name").panics();
    /// ```
    fn panics(self) -> String {
        self.unwrap_or_else(|err| panic!("{}", err))
    }
}
