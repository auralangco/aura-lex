use crate::lexeme::{state::{Accepter, LexemeState}, Coord, Lexeme};

/// The delimiters in the source code.
/// Those can be used to separate lexemes.
//const DELIMITERS: [char; 14] = ['(', ')', '[', ']', '{', '}', ',', ':', ';', '.', ' ', '\n', '\t', '\r'];


pub fn lex<'src>(src: &'src str) -> Vec<Vec<Lexeme<'src>>> {
    let chars = src.chars().enumerate();
    let mut start = 0;
    let mut start_coord = Coord { line: 1, col: 1 };
    let mut end_coord: Coord = start_coord;

    let mut candidates = LexemeState::stream();
    let mut lexemes = vec![];
    
    for (i, c) in chars {
        let next_candidates = candidates.iter()
                .filter_map(|s| s.accept(c))
                .collect::<Vec<_>>();
        
        if next_candidates.iter().filter(|s| s.acceptable()).count() == 0 {
            lexemes.push(candidates.into_iter()
                .filter(|s| s.acceptable())
                .map(|state| Lexeme { 
                    kind: state.into(), 
                    slice: &src[start..i], 
                    start, 
                    end: i, 
                    start_coord, 
                    end_coord 
                })
                .collect::<Vec<_>>());

            candidates = LexemeState::stream().into_iter()
                .filter_map(|s| s.accept(c))
                .collect();
            start = i;
            start_coord = end_coord;
        } else {
            candidates = next_candidates;
        }

        if c == '\n' {
            end_coord.line += 1;
            end_coord.col = 1;
        } else {
            end_coord.col += 1;
        }
    }

    lexemes
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexeme_test() {
        let src = "val   x:=10     ;// line comment
        ";
        let chars = src.chars().enumerate();
        let mut start = 0;

        let mut candidates = LexemeState::stream();
        let mut lexemes = vec![];

        for (i, c) in chars {
            println!("{}: {} - `{}`", i, c, &src[start..i]);

            let next_candidates = candidates.iter()
                .filter_map(|s| s.accept(c))
                .collect::<Vec<_>>();
            dbg!(&next_candidates);

            if next_candidates.iter().filter(|s| s.acceptable()).count() == 0 {
                lexemes.push(candidates.into_iter()
                    .filter(|s| s.acceptable())
                    .map(|s| (s, &src[start..i], start, i))
                    .collect::<Vec<_>>());

                candidates = LexemeState::stream().into_iter()
                    .filter_map(|s| s.accept(c))
                    .collect();
                start = i;
            } else {
                candidates = next_candidates;
            }
        }

        println!("{:?}", lexemes);
    }

    #[test]
    fn test_lex() {
        let src = r#"val x := 10
        val name := "John Doe"

        type Person := @enum (name String, full_name (first String, last String))

        main -> 'main:{
            parts := @mut ["John", "Doe"];
            parts[0] = "Jane";
        }
        "#;
        let lexemes = lex(src);
        dbg!(&lexemes);
    }
}