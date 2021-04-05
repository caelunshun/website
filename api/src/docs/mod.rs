mod summary;
mod summary_parser;

pub use summary::*;
pub use summary_parser::*;

use anyhow::Result;
use pulldown_cmark::{html, CowStr, Event, Parser, Tag, CodeBlockKind};
use std::{iter, str::FromStr, path::Path};
use url::Url;

use syntect::{highlighting::{Theme, ThemeSet}, html::highlighted_html_for_string, parsing::{SyntaxSet, SyntaxSetBuilder}};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref SYNTAXSET: SyntaxSet = {
        let mut builder: SyntaxSetBuilder = SyntaxSet::load_defaults_newlines().into_builder();
        builder.add_from_folder(Path::new("./syntaxes/TOML"), true).expect("Failed to load TOML Syntax. Is it up-to-date?");
        builder.build()
    };
    pub static ref THEMESET: ThemeSet = ThemeSet::load_defaults();
    pub static ref THEME: Theme = THEMESET.themes["base16-ocean.dark"].clone();

}

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
        let mut syntax = SYNTAXSET.find_syntax_by_extension("rs").unwrap();
        
        let mut new_p = Vec::new();
        let mut to_highlight = String::new();
        let mut in_code_block = false;

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
                    };
                    new_p.push(event);
                }
                Event::Start(Tag::CodeBlock(cb)) => {
                    match cb {
                        CodeBlockKind::Fenced(token) => {
                            in_code_block = true;
                            if let Some(syn) = SYNTAXSET.find_syntax_by_token(token) {
                                syntax = syn;
                            } else {
                                syntax = SYNTAXSET.find_syntax_by_extension("rs").unwrap();
                            }
                        }
                        _ => {}
                    }
                }
                Event::End(Tag::CodeBlock(_)) => {
                    if in_code_block {
                        let html = highlighted_html_for_string(&to_highlight, &SYNTAXSET, &syntax, &THEME);
                        new_p.push(Event::Html(CowStr::from(html)));
                        to_highlight = String::new();
                        in_code_block = false;
                    }
                }
                Event::Text(t) => {
                    if in_code_block {
                        to_highlight.push_str(&t);
                    } else {
                        new_p.push(event);
                    }
                }
                e => {
                    new_p.push(e.clone());
                }
            }
        }
        html::push_html(&mut output, new_p.into_iter());
        output
    }
}
