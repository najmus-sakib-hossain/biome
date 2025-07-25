#![cfg(test)]

mod block;
mod flow;

use super::TextSize;
use crate::lexer::YamlLexer;
use biome_parser::lexer::Lexer;
use quickcheck_macros::quickcheck;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

// Assert the result of lexing a piece of source code,
// and make sure the tokens yielded are fully lossless and the source can be reconstructed from only the tokens
#[macro_export]
macro_rules! assert_lex {
    ($src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let mut lexer = $crate::lexer::YamlLexer::from_str($src);
        let mut idx = 0;
        let mut tok_idx = biome_rowan::TextSize::default();

        let mut new_str = String::with_capacity($src.len());
        let mut tokens = vec![];

        use biome_parser::lexer::Lexer;
        while lexer.next_token(()) != biome_yaml_syntax::YamlSyntaxKind::EOF {
            tokens.push((lexer.current(), lexer.current_range()));
        }

        $(
            assert_eq!(
                tokens[idx].0,
                biome_yaml_syntax::YamlSyntaxKind::$kind,
                "expected token kind {}, but found {:?}",
                stringify!($kind),
                tokens[idx].0,
            );

            assert_eq!(
                tokens[idx].1.len(),
                biome_rowan::TextSize::from($len),
                "expected token length of {}, but found {:?} for token {:?}",
                $len,
                tokens[idx].1.len(),
                tokens[idx].0,
            );

            new_str.push_str(&$src[tokens[idx].1]);
            tok_idx += tokens[idx].1.len();

            idx += 1;
        )*

        if idx < tokens.len() {
            panic!(
                "expected {} tokens but lexer returned {}, first unexpected token is '{:?}'",
                idx,
                tokens.len(),
                tokens[idx].0
            );
        } else {
            assert_eq!(idx, tokens.len());
        }

        assert_eq!($src, new_str, "Failed to reconstruct input");
    }};
}

// This is for testing if the lexer is truly lossless
// It parses random strings and puts them back together with the produced tokens and compares
#[quickcheck]
fn losslessness(string: String) -> bool {
    // using an mpsc channel allows us to spawn a thread and spawn the lexer there, then if
    // it takes more than 2 seconds we panic because it is 100% infinite recursion
    let cloned = string.clone();
    let (sender, receiver) = channel();
    thread::spawn(move || {
        let mut lexer = YamlLexer::from_str(&cloned);
        let mut tokens = vec![];

        while lexer.next_token(()) != biome_yaml_syntax::YamlSyntaxKind::EOF {
            tokens.push(lexer.current_range());
        }

        sender
            .send(tokens)
            .expect("Could not send tokens to receiver");
    });
    let token_ranges = receiver
        .recv_timeout(Duration::from_secs(2))
        .unwrap_or_else(|_| panic!("Lexer is infinitely recursing with this code: ->{string}<-"));

    let mut new_str = String::with_capacity(string.len());
    let mut idx = TextSize::from(0);

    for range in token_ranges {
        new_str.push_str(&string[range]);
        idx += range.len();
    }

    string == new_str
}
