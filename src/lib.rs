pub mod error;
pub mod stdin;

#[cfg(test)]
mod tests {
    use crate::readln;

    #[test]
    fn macro_build_tests() {
        readln!();
        readln!("prompted");
        readln!("formatted: {}", true);
        readln!("a, b, c: {}, {}, {}", 1, 2, 3);
    }
}
