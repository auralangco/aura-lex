# Aura Lang Lexer

This is the implementation of the [Aura Lang](https://github.com/auralangco/aura) lexer

## How to Use

This parser is actually meant to be used within Aura parser, but if you wanna use it in your project just use:

```
$ cargo add aura-lex
```

And then call

```rs
let Vec<Lexeme<'_> = aura_lex::lex(src);
```

This basic function takes in a `&str` and lexes it producing a vector of lexemes

## Lexeme

A lexeme is a sequence of chars that have some meaning for our language. They are to Aura as words are to English.

Basically a lexeme has 3 main components:

- kind: specifies which kind of lexeme this is
- slice: the string slice from the source corresponding to the lexeme
- location: numbers informing where the lexeme can be found in the source text

## Luthor

If you just wanna test this lexer we provide a minimal executable called `luthor`

```
$ cargo install aura-lex
$ luthor some-aura-file.aura
```

This will print the lexeme stream parsed from the file

## Known Issues

- keyword and value identifiers rules are ambiguous
- no error handling