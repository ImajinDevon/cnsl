use std::fmt::Display;
use std::io;
use std::io::Write;

/// Reads until a newline character is found.
/// This can be either `\n` or `\r\n`.
/// Returns the string `"<null>"` if stdout could not be read.
pub fn readln() -> String {
    let mut string = String::new();

    match io::stdin().read_line(&mut string) {
        Ok(_) => (),
        Err(_) => return "<null>".to_string(),
    }

    if string.ends_with('\n') {
        string.pop();

        if string.ends_with('\r') {
            string.pop();
        }
    }
    string
}

/// Prints the given prompt to stdout, then returns [readln].
#[inline]
pub fn preadln<D>(prompt: D) -> String
where
    D: AsRef<[u8]>,
{
    preadln_raw(prompt.as_ref())
}

pub fn preadln_raw(bytes: &[u8]) -> String {
    let _ = io::stdout().write_all(bytes);
    let _ = io::stdout().flush();
    readln()
}
