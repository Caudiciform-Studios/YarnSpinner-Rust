//! The C# implementation uses inheritance to do this.
//! More specifically, the lexer generated by ANTLR derives from the `IndentAwareLexer`
//! directly, and the `IndentAwareLexer` derives from the ANTLR Lexer base class.
//! Instead of this, we use a proxy/wrapper around the generated lexer to handle everything correctly.
//! TODO: Decide if we want to hide the generated lexer to make sure no one accidentially uses it.

use antlr_rust::{
    char_stream::CharStream,
    token_factory::{CommonTokenFactory, TokenFactory},
    TokenSource,
};

use super::generated::yarnspinnerlexer::{LocalTokenFactory, YarnSpinnerLexer};

/// A Lexer subclass that detects newlines and generates indent and dedent tokens accordingly.
pub struct IndentAwareYarnSpinnerLexer<
    'input,
    Input: CharStream<From<'input>>,
    TF: TokenFactory<'input> = CommonTokenFactory,
> {
    base: YarnSpinnerLexer<'input, Input>, // TODO: needed?
    pub token: Option<TF::Tok>,
}

impl<'input, Input: CharStream<From<'input>> + std::ops::Deref> std::ops::Deref
    for IndentAwareYarnSpinnerLexer<'input, Input>
{
    type Target = YarnSpinnerLexer<'input, Input>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// better_any::tid! {IndentAwareYarnSpinnerLexer} // TODO: needed?

impl<'input, Input: CharStream<From<'input>>> TokenSource<'input>
    for IndentAwareYarnSpinnerLexer<'input, Input>
{
    type TF = CommonTokenFactory; // TODO: correct?

    fn next_token(&mut self) -> <Self::TF as antlr_rust::token_factory::TokenFactory<'input>>::Tok {
        self.base.next_token()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn antlr_rust::int_stream::IntStream> {
        self.base.get_input_stream()
    }

    fn get_source_name(&self) -> String {
        self.base.get_source_name()
    }

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
    }
}

/// Copied from generated/yarnspinnerlexer.rs
type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a>>::From;

impl<'input, Input: CharStream<From<'input>>> IndentAwareYarnSpinnerLexer<'input, Input>
where
    &'input LocalTokenFactory<'input>: Default,
{
    pub fn new(input: Input) -> Self {
        IndentAwareYarnSpinnerLexer {
            // TODO: is that correct? Is ::new sufficient whithout the LocalTokenFactory as param?
            base: YarnSpinnerLexer::new_with_token_factory(
                input,
                <&LocalTokenFactory<'input> as Default>::default(),
            ),
            token: Default::default(), // TODO: correct?
        }
    }
}

#[cfg(test)]
mod test {
    use antlr_rust::{
        common_token_stream::CommonTokenStream, int_stream::IntStream, token::TOKEN_EOF,
        InputStream,
    };

    use crate::prelude::generated::yarnspinnerlexer::YarnSpinnerLexer;

    use super::*;

    const MINIMAL_INPUT: &str = "title: Minimal Yarn
---
This is the one and only line
===";

    #[test]
    fn behaves_like_lexer_for_unindented_input() {
        let generated_lexer = YarnSpinnerLexer::new(InputStream::new(MINIMAL_INPUT));
        let indent_aware_lexer = IndentAwareYarnSpinnerLexer::new(InputStream::new(MINIMAL_INPUT));

        let mut reference_token_stream = CommonTokenStream::new(generated_lexer);
        let mut indent_aware_token_stream = CommonTokenStream::new(indent_aware_lexer);

        assert_eq!(
            reference_token_stream.size(),
            indent_aware_token_stream.size()
        );

        // Sanity check: Make sure at least one token is read: We do have input.
        assert_eq!(
            reference_token_stream.iter().next(),
            indent_aware_token_stream.iter().next()
        );

        // Can not do this, as trying to read EOF panics...
        // Iterator::eq(
        //     reference_token_stream.iter(),
        //     indent_aware_token_stream.iter(),
        // );

        while reference_token_stream.la(1) != TOKEN_EOF {
            assert_eq!(
                reference_token_stream.iter().next(),
                indent_aware_token_stream.iter().next()
            );
        }

        assert_eq!(TOKEN_EOF, reference_token_stream.la(1));
        assert_eq!(TOKEN_EOF, indent_aware_token_stream.la(1));
    }
}
