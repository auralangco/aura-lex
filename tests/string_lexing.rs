#[cfg(test)]
mod tests {
    use aura_lex::{lexeme::Coord, lexer::lex};

    #[test]
    fn lex_string() {
        let src = r#""Hello World" "#;
        let lexemes = lex(src);

        assert!(lexemes.len() == 1);
        assert!(lexemes[0].len() == 1);
        assert!(lexemes[0][0].kind == aura_lex::lexeme::kind::LexemeKind::LitStr);
        assert!(lexemes[0][0].slice == "\"Hello World\"");
        assert!(lexemes[0][0].start == 0);
        assert!(lexemes[0][0].end == 13);
        assert!(lexemes[0][0].start_coord == Coord { line: 1, col: 1 });
        assert!(lexemes[0][0].end_coord == Coord { line: 1, col: 14 });
    }
}