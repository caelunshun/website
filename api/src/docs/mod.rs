mod summary;
mod summary_parser;

pub use summary::*;
pub use summary_parser::*;

use anyhow::Result;
use pulldown_cmark::{html, CowStr, Event, Parser, Tag};
use std::{iter, str::FromStr};
use url::Url;

pub struct DocsParser<'a> {
    base: Url,
    events: Parser<'a>,
}

impl<'a> DocsParser<'a> {
    pub fn new(src: &'a str, base: Url) -> Self {
        Self {
            base,
            events: Parser::new(src),
        }
    }

    pub fn parse(mut self) -> String {
        let mut output = String::new();
        for mut event in self.events {
            match &mut event {
                Event::Start(Tag::Link(_, href, _)) => {
                    *href = match Url::from_str(href) {
                        Err(url::ParseError::RelativeUrlWithoutBase) => {
                            let url = self.base.join(href.trim_end_matches(".md")).unwrap();
                            CowStr::from(url.to_string())
                        },
                        Ok(url) => CowStr::from(url.to_string()),
                        _ => CowStr::from("foo"),
                    }
                }
                _ => {}
            }
            html::push_html(&mut output, iter::once(event));
        }
        output
    }
}
