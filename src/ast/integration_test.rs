#[cfg(test)]
mod intergration_test {
    use crate::{
        ast::{parser::Parser, printer::AstPrinter, scanner::Scanner},
        lox::Lox,
    };

    #[test]
    fn number() {
        let mut lox: Lox = Lox::default();

        let source = String::from("65");
        let mut scanner = Scanner::new(&mut lox, &source);
        let tkns = scanner.scan_tokens();

        let expr = Parser::new(tkns).parse().unwrap();

        let printed = AstPrinter::default().print(&expr);

        assert_eq!(printed, "65")
    }

    #[test]
    fn unary_bang() {
        let mut lox: Lox = Lox::default();

        let source = String::from("!3");
        let mut scanner = Scanner::new(&mut lox, &source);
        let tkns = scanner.scan_tokens();

        let expr = Parser::new(tkns).parse().unwrap();

        let printed = AstPrinter::default().print(&expr);

        assert_eq!(printed, "(! 3)")
    }

    #[test]
    fn equal_with_bang() {
        let mut lox: Lox = Lox::default();

        let source = String::from("!3 == 4");
        let mut scanner = Scanner::new(&mut lox, &source);
        let tkns = scanner.scan_tokens();

        let expr = Parser::new(tkns).parse().unwrap();

        let printed = AstPrinter::default().print(&expr);
        assert_eq!(printed, "(== (! 3) 4)")
    }
}
