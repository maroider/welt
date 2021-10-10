extern crate std;

use std::fs;

use html5ever::tendril::fmt::UTF8;
use html5ever::tendril::{ByteTendril, ReadExt, StrTendril, Tendril};
use html5ever::tokenizer::{BufferQueue, TagKind, Token, TokenSink, TokenSinkResult, Tokenizer};

pub fn parse(mut file: fs::File) {
    let mut chunk = ByteTendril::new();
    file.read_to_tendril(&mut chunk).unwrap();
    let mut input = BufferQueue::new();
    input.push_back(chunk.try_reinterpret().unwrap());

    let mut tok = Tokenizer::new(Sink::new(), Default::default());
    let _ = tok.feed(&mut input);
    tok.end();

    dbg!(tok.sink);
}

#[derive(Debug)]
struct Sink {
    tree: Vec<TreeNode>,
}

#[derive(Debug)]
pub enum TreeNode {
    Begin(Tag),
    Text(Tendril<UTF8>),
    End,
}

#[derive(Debug)]
pub struct Tag {
    pub name: html5ever::LocalName,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Attribute {
    pub name_span: Span,
    pub name: html5ever::QualName,
    pub value_span: Span,
    pub value: AttributeValue,
}

#[derive(Debug)]
pub enum AttributeValue {
    String(StrTendril),
    EtString(Vec<FeString>),
}

// NOTE: `Fe` means `FormatExpression`, as the contents of the template is a rust expression with a
//        formatting speicifier.

#[derive(Debug)]
pub struct FeString {
    pub span: Span,
    pub kind: FeElementKind,
}

#[derive(Debug)]
pub enum FeElementKind {
    Text {
        span: Span,
        text: StrTendril,
    },
    Template {
        opening_bracket: Span,
        closing_bracket: Span,
        content_span: Span,
        content: StrTendril,
    },
}

#[derive(Debug)]
pub struct Span {
    pub start: usize,
    pub len: usize,
}

impl Sink {
    fn new() -> Self {
        Self { tree: Vec::new() }
    }
}

impl TokenSink for Sink {
    type Handle = ();

    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        match token {
            Token::TagToken(tag) => match tag.kind {
                TagKind::StartTag => {
                    self.tree.push(TreeNode::Begin(Tag {
                        name: tag.name,
                        attributes: parse_attributes(tag.attrs),
                    }));
                }
                TagKind::EndTag => {
                    if self
                        .tree
                        .iter()
                        .rev()
                        .find_map(|node| {
                            if let TreeNode::Begin(tag) = node {
                                Some(tag)
                            } else {
                                None
                            }
                        })
                        .map(|state| state.name == tag.name)
                        .unwrap_or(false)
                    {
                        self.tree.push(TreeNode::End);
                    }
                }
            },
            Token::CharacterTokens(characters) => {
                if let Some(TreeNode::Text(text)) = self.tree.last_mut() {
                    text.push_tendril(&characters);
                } else {
                    self.tree.push(TreeNode::Text(characters));
                }
            }
            Token::ParseError(err) => {
                dbg!(err);
                // TODO: Report errors!
            }
            _ => {}
        }

        TokenSinkResult::Continue
    }
}

fn parse_attributes(attributes: Vec<html5ever::Attribute>) -> Vec<Attribute> {
    todo!()
}
