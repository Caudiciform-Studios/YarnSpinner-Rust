//! Contains the modules that were generated by ANTLR.

#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(warnings)]
#[allow(clippy)]
pub mod yarnspinnerlexer;

#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(warnings)]
#[allow(clippy)]
pub mod yarnspinnerparser;

#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(warnings)]
#[allow(clippy)]
pub mod yarnspinnerparserlistener;

#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(warnings)]
#[allow(clippy)]
pub mod yarnspinnerparservisitor;

#[cfg(test)]
mod tests {
    use super::*;
    use antlr_rust::tree::ParseTree;
    use antlr_rust::{common_token_stream::CommonTokenStream, *};
    use yarnspinnerlexer::*;
    use yarnspinnerparser::*;

    const MINIMAL_INPUT: &str = "title: Minimal Yarn
---
This is the one and only line
===";

    #[test]
    fn parses_root() {
        let lexer = YarnSpinnerLexer::new(InputStream::new(MINIMAL_INPUT));
        let mut parser = YarnSpinnerParser::new(CommonTokenStream::new(lexer));
        let dialogue = parser.dialogue().unwrap();
        let expected_string_tree = "(dialogue (node (header title :  Minimal Yarn) --- (body (statement (line_statement (line_formatted_text T his is the one and only line) \\n))) ===))";
        assert_eq!(expected_string_tree, dialogue.to_string_tree(&*parser));
    }

    #[test]
    fn does_random_stuffs() {
        let lexer = YarnSpinnerLexer::new(InputStream::new(
            "# hello
# nonono
title: Node_Title
---
Here are some lines!
That's weird?
Wow!
===",
        ));
        let mut parser = YarnSpinnerParser::new(CommonTokenStream::new(lexer));

        let dialogue_context = parser.dialogue().unwrap();
        let hashtags = dialogue_context.file_hashtag_all();

        let output = hashtags[0].get_text();
        assert_eq!(output, "#hello");
        let output = hashtags[0].HASHTAG_TEXT().unwrap().get_text();
        assert_eq!(output, "hello");

        let output = hashtags[1].HASHTAG_TEXT().unwrap().get_text();
        assert_eq!(output, "nonono");

        let first_node = dialogue_context.node(0).unwrap();
        let statements = first_node.body().unwrap().statement_all();

        let output = statements[0]
            .line_statement()
            .unwrap()
            .line_formatted_text()
            .unwrap()
            .get_text();
        assert_eq!(output, "Here are some lines!");

        let output = statements[1]
            .line_statement()
            .unwrap()
            .line_formatted_text()
            .unwrap()
            .get_text();

        assert_eq!(output, "That's weird?");
        let output = statements[2]
            .line_statement()
            .unwrap()
            .line_formatted_text()
            .unwrap()
            .get_text();
        assert_eq!(output, "Wow!");

        let title = first_node.header(0);
        let output = title.unwrap().REST_OF_LINE().unwrap().get_text();
        assert_eq!(output, "Node_Title");
    }
}
