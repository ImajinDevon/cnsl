//! A module for reading from the standard input.
use std::io;
use std::io::Write;

/// A macro to read until a newline from stdin.
/// # Examples
/// ```
/// use cnsl::readln;
///
/// match readln!("Enter your name: ") {
///     Ok(name) => println!("Hello, {}!", name),
///     Err(e) => eprintln!("{}", e),
/// }
/// ```
#[macro_export]
macro_rules! readln {
    () => {
        cnsl::stdin::readln()
    };
    ($fmt:expr) => {
        cnsl::stdin::preadln($fmt)
    };
    ($fmt:expr, $($arg:tt)*) => {
        cnsl::stdin::preadln(print!($fmt, $($arg)*))
    }
}

/// Reads a line from stdin, then if present, pop the trailing newline.
/// This can be either `\n` or `\r\n`.
/// # Examples
/// ```
/// use cnsl::error::Panics;
/// use cnsl::stdin;
///
/// let name = stdin::readln().panics();
/// println!("Hello, {}!", name);
/// ```
pub fn readln() -> io::Result<String> {
    let mut string = String::new();
    io::stdin().read_line(&mut string)?;

    if string.ends_with('\n') {
        string.pop();

        if string.ends_with('\r') {
            string.pop();
        }
    }
    Ok(string)
}

/// Prints the given prompt to stdout, then returns [readln].
/// # Examples
/// ```
/// use cnsl::error::Panics;
/// use cnsl::stdin;
///
/// let name = stdin::preadln("Enter your name: ").panics();
/// println!("Hello, {}!", name);
/// ```
#[inline]
pub fn preadln<D>(prompt: D) -> io::Result<String>
where
    D: AsRef<[u8]>,
{
    preadln_raw(prompt.as_ref())
}

/// Prints the given bytes to stdout, then returns [readln].
/// # Examples
/// ```
/// use cnsl::error::Panics;
/// use cnsl::stdin;
///
/// let name = stdin::preadln_raw(b"Enter your name: ").panics();
/// println!("Hello, {}!", name);
/// ```
pub fn preadln_raw(bytes: &[u8]) -> io::Result<String> {
    io::stdout().write_all(bytes)?;
    io::stdout().flush()?;
    readln()
}
