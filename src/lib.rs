pub mod error;
pub mod stdin;

#[cfg(test)]
mod tests {
    use crate::readln;

    #[test]
    fn stdin() {
        readln!();
        readln!("prompted");
        readln!("formatted: {}", true);
    }
}