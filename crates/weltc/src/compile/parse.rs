use std::fs;

use html5ever::tendril::fmt::UTF8;
use html5ever::tendril::{ByteTendril, ReadExt, Tendril};
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
enum TreeNode {
    Begin(MyTag),
    Text(Tendril<UTF8>),
    End,
}

#[derive(Debug)]
struct MyTag {
    name: html5ever::LocalName,
    attributes: Vec<html5ever::Attribute>,
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
                    self.tree.push(TreeNode::Begin(MyTag {
                        name: tag.name,
                        attributes: tag.attrs,
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
