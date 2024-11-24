use crate::{lexeme::{state::{Accepter, LexemeState}, Coord, Lexeme}, LexemeKind};

/// The delimiters in the source code.
/// Those can be used to separate lexemes.
//const DELIMITERS: [char; 14] = ['(', ')', '[', ']', '{', '}', ',', ':', ';', '.', ' ', '\n', '\t', '\r'];

/// The Aura lexer. It takes a source code string and returns a vector of lexemes groups.
/// As of now, some lexemes might be ambiguous, so the lexer return all possible lexemes and
/// the parser will have to disambiguate them using the nearby lexemes.
pub fn lex<'src>(src: &'src str) -> Vec<Vec<Lexeme<'src>>> {
    let chars = src.chars().enumerate();
    let mut start = 0;
    let mut start_coord = Coord { line: 1, col: 1 };
    let mut end_coord: Coord = start_coord;

    let mut candidates = LexemeState::stream();
    let mut lexemes = vec![];
    
    for (i, c) in chars {
        let next_candidates = get_next_candidates(&candidates, c);

        if acceptable_candidates_count(&next_candidates) == 0 && acceptable_candidates_count(&candidates) > 0 {
            lexemes.push(build_lexemes_from_candidates(candidates, src, start, i, start_coord, end_coord));

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
            .filter(|lexeme| lexeme.kind != LexemeKind::Ws)
            .collect())
        .filter(|group: &Vec<Lexeme<'src>>| group.len() > 0)
        .collect()
}

/// Removes comments lexemes from the lexemes stream.
pub fn remove_comments<'src>(lexemes: Vec<Vec<Lexeme<'src>>>) -> Vec<Vec<Lexeme<'src>>> {
    lexemes.into_iter()
        .map(|group| group.into_iter()
            .filter(|lexeme| lexeme.kind != LexemeKind::CommentLine && lexeme.kind != LexemeKind::CommentBlock)
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
fn build_lexemes_from_candidates<'src>(candidates: Vec<LexemeState>, src: &'src str, start: usize, end: usize, start_coord: Coord, end_coord: Coord) -> Vec<Lexeme<'src>> {
    candidates.into_iter()
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
        
        for (i, lexeme) in lexemes.iter().enumerate() {
            println!("Lexeme group {}", i);
            for l in lexeme {
                println!("{:?}", l);
            }
        }
        assert_eq!(lexemes.len(), 77);
        assert_eq!(lexemes[0].len(), 2);
        assert_eq!(lexemes[0][0].kind, LexemeKind::KwVal);
        assert_eq!(lexemes[0][0].slice, "val");
        assert_eq!(lexemes[0][1].kind, LexemeKind::IdentVal);
        assert_eq!(lexemes[0][1].slice, "val");
        assert_eq!(lexemes[1].len(), 1);
        assert_eq!(lexemes[1][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[2].len(), 1);
        assert_eq!(lexemes[2][0].kind, LexemeKind::IdentVal);
        assert_eq!(lexemes[2][0].slice, "x");
        assert_eq!(lexemes[3].len(), 1);
        assert_eq!(lexemes[3][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[4].len(), 1);
        assert_eq!(lexemes[4][0].kind, LexemeKind::OpDecl);
        assert_eq!(lexemes[5].len(), 1);
        assert_eq!(lexemes[5][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[6].len(), 1);
        assert_eq!(lexemes[6][0].kind, LexemeKind::LitIntDec);
        assert_eq!(lexemes[6][0].slice, "10");
        assert_eq!(lexemes[7].len(), 1);
        assert_eq!(lexemes[7][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[8].len(), 2);
        assert_eq!(lexemes[8][0].kind, LexemeKind::KwVal);
        assert_eq!(lexemes[8][0].slice, "val");
        assert_eq!(lexemes[8][1].kind, LexemeKind::IdentVal);
        assert_eq!(lexemes[8][1].slice, "val");
        assert_eq!(lexemes[9].len(), 1);
        assert_eq!(lexemes[9][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[10].len(), 1);
        assert_eq!(lexemes[10][0].kind, LexemeKind::IdentVal);
        assert_eq!(lexemes[10][0].slice, "name");
        assert_eq!(lexemes[11].len(), 1);
        assert_eq!(lexemes[11][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[12].len(), 1);
        assert_eq!(lexemes[12][0].kind, LexemeKind::OpDecl);
        assert_eq!(lexemes[13].len(), 1);
        assert_eq!(lexemes[13][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[14].len(), 1);
        assert_eq!(lexemes[14][0].kind, LexemeKind::LitStr);
        assert_eq!(lexemes[14][0].slice, "\"John Doe\"");
        assert_eq!(lexemes[15].len(), 1);
        assert_eq!(lexemes[15][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[16].len(), 2);
        assert_eq!(lexemes[16][0].kind, LexemeKind::KwType);
        assert_eq!(lexemes[16][0].slice, "type");
        assert_eq!(lexemes[16][1].kind, LexemeKind::IdentVal);
        assert_eq!(lexemes[16][1].slice, "type");
        assert_eq!(lexemes[17].len(), 1);
        assert_eq!(lexemes[17][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[18].len(), 1);
        assert_eq!(lexemes[18][0].kind, LexemeKind::IdentType);
        assert_eq!(lexemes[18][0].slice, "Person");
        assert_eq!(lexemes[19].len(), 1);
        assert_eq!(lexemes[19][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[20].len(), 1);
        assert_eq!(lexemes[20][0].kind, LexemeKind::OpDecl);
        assert_eq!(lexemes[21].len(), 1);
        assert_eq!(lexemes[21][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[22].len(), 1);
        assert_eq!(lexemes[22][0].kind, LexemeKind::IdentMacro);
        assert_eq!(lexemes[22][0].slice, "@enum");
        assert_eq!(lexemes[23].len(), 1);
        assert_eq!(lexemes[23][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[24].len(), 1);
        assert_eq!(lexemes[24][0].kind, LexemeKind::DelimOParen);
        assert_eq!(lexemes[25].len(), 1);
        assert_eq!(lexemes[25][0].kind, LexemeKind::IdentVal);
        assert_eq!(lexemes[25][0].slice, "name");
        assert_eq!(lexemes[26].len(), 1);
        assert_eq!(lexemes[26][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[27].len(), 1);
        assert_eq!(lexemes[27][0].kind, LexemeKind::IdentType);
        assert_eq!(lexemes[27][0].slice, "String");
        assert_eq!(lexemes[28].len(), 1);
        assert_eq!(lexemes[28][0].kind, LexemeKind::PtComma);
        assert_eq!(lexemes[29].len(), 1);
        assert_eq!(lexemes[29][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[30].len(), 1);
        assert_eq!(lexemes[30][0].kind, LexemeKind::IdentVal);
        assert_eq!(lexemes[30][0].slice, "full_name");
        assert_eq!(lexemes[31].len(), 1);
        assert_eq!(lexemes[31][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[32].len(), 1);
        assert_eq!(lexemes[32][0].kind, LexemeKind::DelimOParen);
        assert_eq!(lexemes[33].len(), 1);
        assert_eq!(lexemes[33][0].kind, LexemeKind::IdentVal);
        assert_eq!(lexemes[33][0].slice, "first");
        assert_eq!(lexemes[34].len(), 1);
        assert_eq!(lexemes[34][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[35].len(), 1);
        assert_eq!(lexemes[35][0].kind, LexemeKind::IdentType);
        assert_eq!(lexemes[35][0].slice, "String");
        assert_eq!(lexemes[36].len(), 1);
        assert_eq!(lexemes[36][0].kind, LexemeKind::PtComma);
        assert_eq!(lexemes[37].len(), 1);
        assert_eq!(lexemes[37][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[38].len(), 1);
        assert_eq!(lexemes[38][0].kind, LexemeKind::IdentVal);
        assert_eq!(lexemes[38][0].slice, "last");
        assert_eq!(lexemes[39].len(), 1);
        assert_eq!(lexemes[39][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[40].len(), 1);
        assert_eq!(lexemes[40][0].kind, LexemeKind::IdentType);
        assert_eq!(lexemes[40][0].slice, "String");
        assert_eq!(lexemes[41].len(), 1);
        assert_eq!(lexemes[41][0].kind, LexemeKind::DelimCParen);
        assert_eq!(lexemes[42].len(), 1);
        assert_eq!(lexemes[42][0].kind, LexemeKind::DelimCParen);
        assert_eq!(lexemes[43].len(), 1);
        assert_eq!(lexemes[43][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[44].len(), 2);
        assert_eq!(lexemes[44][0].kind, LexemeKind::KwMain);
        assert_eq!(lexemes[44][0].slice, "main");
        assert_eq!(lexemes[44][1].kind, LexemeKind::IdentVal);
        assert_eq!(lexemes[44][1].slice, "main");
        assert_eq!(lexemes[45].len(), 1);
        assert_eq!(lexemes[45][0].kind, LexemeKind::Ws);
        assert_eq!(lexemes[46].len(), 1);
        assert_eq!(lexemes[46][0].kind, LexemeKind::OpRArw);

    }
}