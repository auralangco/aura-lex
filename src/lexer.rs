use crate::{lexeme::{accepter::{Accepter, LexemeState}, Coord, Lexeme}, LexemeKind};

/// The delimiters in the source code.
/// Those can be used to separate lexemes.
//const DELIMITERS: [char; 14] = ['(', ')', '[', ']', '{', '}', ',', ':', ';', '.', ' ', '\n', '\t', '\r'];

/// The Aura lexer. It takes a source code string and returns a vector of lexemes groups.
/// As of now, some lexemes might be ambiguous, so the lexer return all possible lexemes and
/// the parser will have to disambiguate them using the nearby lexemes.
pub fn lex<'src>(src: &'src str) -> Vec<Lexeme<'src>> {
    let chars = src.chars().enumerate();
    let mut start = 0;
    let mut start_coord = Coord { line: 1, col: 1 };
    let mut end_coord: Coord = start_coord;

    let mut candidates = LexemeState::stream();
    let mut lexemes = vec![];
    
    for (i, c) in chars {
        let next_candidates = get_next_candidates(&candidates, c);

        if acceptable_candidates_count(&next_candidates) == 0 && acceptable_candidates_count(&candidates) > 0 {
            lexemes.push(build_lexeme_from_candidates(candidates, src, start, i, start_coord, end_coord));

            candidates = get_next_candidates(&LexemeState::stream(), c);

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

/// Removes whitespace lexemes from the lexemes stream.
pub fn remove_ws<'src>(lexemes: Vec<Vec<Lexeme<'src>>>) -> Vec<Vec<Lexeme<'src>>> {
    lexemes.into_iter()
        .map(|group| group.into_iter()
            .filter(|lexeme| lexeme.kind.unambiguous_unchecked() != LexemeKind::Ws)
            .collect())
        .filter(|group: &Vec<Lexeme<'src>>| group.len() > 0)
        .collect()
}

/// Removes comments lexemes from the lexemes stream.
pub fn remove_comments<'src>(lexemes: Vec<Vec<Lexeme<'src>>>) -> Vec<Vec<Lexeme<'src>>> {
    lexemes.into_iter()
        .map(|group| group.into_iter()
            .filter(|lexeme| lexeme.kind.unambiguous_unchecked() != LexemeKind::CommentLine && lexeme.kind.unambiguous_unchecked() != LexemeKind::CommentBlock)
            .collect())
        .filter(|group: &Vec<Lexeme<'src>>| group.len() > 0)
        .collect()
}

/// Runs `accept` on every candidate in `candidates` and returns the ones that accept `c`.
#[cfg(not(feature = "parallel"))]
fn get_next_candidates<'src>(candidates: &Vec<LexemeState>, c: char) -> Vec<LexemeState> {
    candidates.into_iter()
        .filter_map(|s| s.accept(c))
        .collect()
}

/// Runs `accept` on every candidate in `candidates` in parallel and returns the ones that accept `c`.
#[cfg(feature = "parallel")]
fn get_next_candidates<'src>(candidates: &Vec<LexemeState>, c: char) -> Vec<LexemeState> {
    use rayon::prelude::*;

    candidates.into_par_iter()
        .filter_map(|s| s.accept(c))
        .collect()
}

/// Returns the number of acceptable candidates in `candidates`.
#[cfg(not(feature = "parallel"))]
fn acceptable_candidates_count(candidates: &Vec<LexemeState>) -> usize {
    candidates.iter().filter(|s| s.acceptable()).count()
}

/// Returns the number of acceptable candidates in `candidates` in parallel.
#[cfg(feature = "parallel")]
fn acceptable_candidates_count(candidates: &Vec<LexemeState>) -> usize {
    use rayon::prelude::*;

    candidates.par_iter().filter(|s| s.acceptable()).count()
}

/// Builds lexemes from the candidates in `candidates` and returns them.
#[cfg(not(feature = "parallel"))]
fn build_lexeme_from_candidates<'src>(candidates: Vec<LexemeState>, src: &'src str, start: usize, end: usize, start_coord: Coord, end_coord: Coord) -> Lexeme<'src> {
    use crate::lexeme::LexemeAmbiguity;

    let states: Vec<_> = candidates.into_iter()
        .filter(|s| s.acceptable())
        .collect();
    
    let kind = if states.len() == 1 {
        LexemeAmbiguity::Unambiguous(states[0].into())
    } else {
        let (a, b) = (states[0].into(), states[1].into());
        LexemeAmbiguity::Ambiguous(a, b)
    };

    Lexeme { 
        kind, 
        slice: &src[start..end], 
        start, 
        end, 
        start_coord, 
        end_coord 
    }
}

/// Builds lexemes from the candidates in `candidates` in parallel and returns them.
#[cfg(feature = "parallel")]
fn build_lexemes_from_candidates<'src>(candidates: Vec<LexemeState>, src: &'src str, start: usize, end: usize, start_coord: Coord, end_coord: Coord) -> Vec<Lexeme<'src>> {
    use rayon::prelude::*;

    candidates.into_par_iter()
        .filter(|s| s.acceptable())
        .map(|state| Lexeme { 
            kind: state.into(), 
            slice: &src[start..end], 
            start, 
            end, 
            start_coord, 
            end_coord 
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::lexeme::kind::LexemeKind;

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
        
        assert_eq!(lexemes.len(), 77);
        assert_eq!(lexemes[0].kind.ambiguous_unchecked(), (LexemeKind::KwVal, LexemeKind::IdentVal));
        assert_eq!(lexemes[0].slice, "val");
        assert_eq!(lexemes[1].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[2].kind.unambiguous_unchecked(), LexemeKind::IdentVal);
        assert_eq!(lexemes[2].slice, "x");
        assert_eq!(lexemes[3].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[4].kind.unambiguous_unchecked(), LexemeKind::OpDecl);
        assert_eq!(lexemes[5].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[6].kind.unambiguous_unchecked(), LexemeKind::LitIntDec);
        assert_eq!(lexemes[6].slice, "10");
        assert_eq!(lexemes[7].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[8].kind.ambiguous_unchecked(), (LexemeKind::KwVal, LexemeKind::IdentVal));
        assert_eq!(lexemes[8].slice, "val");
        assert_eq!(lexemes[9].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[10].kind.unambiguous_unchecked(), LexemeKind::IdentVal);
        assert_eq!(lexemes[10].slice, "name");
        assert_eq!(lexemes[11].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[12].kind.unambiguous_unchecked(), LexemeKind::OpDecl);
        assert_eq!(lexemes[13].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[14].kind.unambiguous_unchecked(), LexemeKind::LitStr);
        assert_eq!(lexemes[14].slice, "\"John Doe\"");
        assert_eq!(lexemes[15].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[16].kind.ambiguous_unchecked(), (LexemeKind::KwType, LexemeKind::IdentVal));
        assert_eq!(lexemes[16].slice, "type");
        assert_eq!(lexemes[17].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[18].kind.unambiguous_unchecked(), LexemeKind::IdentType);
        assert_eq!(lexemes[18].slice, "Person");
        assert_eq!(lexemes[19].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[20].kind.unambiguous_unchecked(), LexemeKind::OpDecl);
        assert_eq!(lexemes[21].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[22].kind.unambiguous_unchecked(), LexemeKind::IdentMacro);
        assert_eq!(lexemes[22].slice, "@enum");
        assert_eq!(lexemes[23].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[24].kind.unambiguous_unchecked(), LexemeKind::DelimOParen);
        assert_eq!(lexemes[25].kind.unambiguous_unchecked(), LexemeKind::IdentVal);
        assert_eq!(lexemes[25].slice, "name");
        assert_eq!(lexemes[26].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[27].kind.unambiguous_unchecked(), LexemeKind::IdentType);
        assert_eq!(lexemes[27].slice, "String");
        assert_eq!(lexemes[28].kind.unambiguous_unchecked(), LexemeKind::PtComma);
        assert_eq!(lexemes[29].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[30].kind.unambiguous_unchecked(), LexemeKind::IdentVal);
        assert_eq!(lexemes[30].slice, "full_name");
        assert_eq!(lexemes[31].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[32].kind.unambiguous_unchecked(), LexemeKind::DelimOParen);
        assert_eq!(lexemes[33].kind.unambiguous_unchecked(), LexemeKind::IdentVal);
        assert_eq!(lexemes[33].slice, "first");
        assert_eq!(lexemes[34].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[35].kind.unambiguous_unchecked(), LexemeKind::IdentType);
        assert_eq!(lexemes[35].slice, "String");
        assert_eq!(lexemes[36].kind.unambiguous_unchecked(), LexemeKind::PtComma);
        assert_eq!(lexemes[37].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[38].kind.unambiguous_unchecked(), LexemeKind::IdentVal);
        assert_eq!(lexemes[38].slice, "last");
        assert_eq!(lexemes[39].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[40].kind.unambiguous_unchecked(), LexemeKind::IdentType);
        assert_eq!(lexemes[40].slice, "String");
        assert_eq!(lexemes[41].kind.unambiguous_unchecked(), LexemeKind::DelimCParen);
        assert_eq!(lexemes[42].kind.unambiguous_unchecked(), LexemeKind::DelimCParen);
        assert_eq!(lexemes[43].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[44].kind.ambiguous_unchecked(), (LexemeKind::KwMain, LexemeKind::IdentVal));
        assert_eq!(lexemes[44].slice, "main");
        assert_eq!(lexemes[45].kind.unambiguous_unchecked(), LexemeKind::Ws);
        assert_eq!(lexemes[46].kind.unambiguous_unchecked(), LexemeKind::OpRArw);
    }
}