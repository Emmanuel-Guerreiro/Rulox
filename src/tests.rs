#[cfg(test)]
mod end_to_end_tests {
    use crate::lox::Lox;
    #[test]
    fn excecute_value_negation() {
        let src = String::from("-3");
        let mut lox: Lox = Lox::default();
        lox.run(src);
    }
}
