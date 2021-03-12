use std::io::Write;
use std::iter;

use anyhow::Result;
use pulldown_cmark::{Parser, Event, Tag};

struct Resource();

pub struct HtmlParser<'a> {
    tokens: std::iter::Peekable<Parser<'a>>,
}

impl<'a> HtmlParser<'a> {
    pub fn new(src: &'a str) -> Self {
        let parser = Parser::new(src).peekable();

        Self { tokens: parser }
    }

    fn next(&mut self) -> std::option::Option<pulldown_cmark::Event<'a>> {
        self.tokens.next()
    }

    fn peek(&mut self) -> std::option::Option<pulldown_cmark::Event<'a>> {
        self.tokens.peek().map(Clone::clone)
    }

    pub fn parse<W>(mut self) -> Result<()>
    where
        W: Write
    {
        while let Some(event) = self.next() {
            match event {
                Event::Start(Tag::Image(_, href, _)) => todo!(),
                _ => { }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::HtmlParser;

    #[test]
    fn foo() {

    }
}