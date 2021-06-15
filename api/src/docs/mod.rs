mod builder;
mod summary;

pub use builder::*;
pub use summary::*;

use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Parser, Tag};
use std::{path::Path};

use syntect::{
    highlighting::{Theme, ThemeSet},
    html::highlighted_html_for_string,
    parsing::{SyntaxSet, SyntaxSetBuilder},
};

use lazy_static::lazy_static;

use crate::featherurl::FeatherUrl;

lazy_static! {
    pub static ref SYNTAXSET: SyntaxSet = {
        let mut builder: SyntaxSetBuilder = SyntaxSet::load_defaults_newlines().into_builder();
        builder
            .add_from_folder(Path::new("./markdown/TOML"), true)
            .expect("Failed to load TOML Syntax. Is it up-to-date?");
        builder.build()
    };
    pub static ref THEMESET: ThemeSet = {
        let mut ts = ThemeSet::load_defaults();
        ts.add_from_folder(Path::new("./markdown/Themes"))
            .expect("Failed to load Themes!");
        ts
    };
    pub static ref THEME: Theme = THEMESET.themes["GitHub Dark"].clone();
}

pub struct DocsParser<'a> {
    base: FeatherUrl,
    events: Parser<'a>,
}

impl<'a> DocsParser<'a> {
    pub fn new(src: &'a str, base: FeatherUrl) -> Self {
        Self {
            base,
            events: Parser::new(src),
        }
    }

    pub fn parse(self) -> String {
        let mut syntax = SYNTAXSET.find_syntax_by_extension("rs").unwrap();

        let mut new_p = Vec::new();
        let mut to_highlight = String::new();
        let mut in_code_block = false;

        let mut cur_heading = String::new();
        let mut in_heading = false;

        let mut output = String::new();
        for mut event in self.events {
            match &mut event {
                Event::Start(Tag::Link(_, href, _)) => {
                    /* *href = match Url::from_str(href) {
                        Err(url::ParseError::RelativeUrlWithoutBase) => {
                            let url = self.base.join(href.trim_end_matches(".md")).unwrap();
                            CowStr::from(url.to_string())
                        }
                        Ok(url) => CowStr::from(url.to_string()),
                        _ => CowStr::from("foo"),
                    };*/
                    if !href.starts_with("http") {
                        let mut abc = self.base.clone();
                        abc.join(href.trim_end_matches(".md"));
                        *href = CowStr::from(abc.to_string_basic());
                    }
                    new_p.push(event);
                }
                Event::Start(Tag::CodeBlock(cb)) => {
                    if let CodeBlockKind::Fenced(token) = cb {
                        in_code_block = true;
                        if let Some(syn) = SYNTAXSET.find_syntax_by_token(token) {
                            syntax = syn;
                        } else {
                            syntax = SYNTAXSET.find_syntax_by_extension("rs").unwrap();
                        }
                    }
                }
                Event::End(Tag::CodeBlock(_)) => {
                    if in_code_block {
                        let html =
                            highlighted_html_for_string(&to_highlight, &SYNTAXSET, &syntax, &THEME);
                        new_p.push(Event::Html(CowStr::from(html)));
                        to_highlight = String::new();
                        in_code_block = false;
                    }
                }
                Event::Start(Tag::Heading(_)) => {
                    in_heading = true;
                }
                Event::End(Tag::Heading(level)) => {
                    if in_heading {
                        let html = format!(
                            "<h{} id=\"h-{}\">{}</h{}>",
                            level,
                            crate::featherurl::encode_uri_component(&cur_heading.to_lowercase()),
                            cur_heading,
                            level
                        );
                        new_p.push(Event::Html(CowStr::from(html)));
                        cur_heading = String::new();
                        in_heading = false;
                    }
                }
                Event::Text(t) => {
                    if in_code_block {
                        to_highlight.push_str(&t);
                    } else if in_heading {
                        cur_heading.push_str(&t);
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

    pub fn static_parse_links(base: FeatherUrl, src: String) -> (String, Vec<String>, Vec<crate::Topic>) {
        let events = Parser::new(src.as_str());
        let mut links: Vec<String> = Vec::new();
        let mut topics: Vec<crate::Topic> = Vec::new();
        let mut topic_index = 0;

        let mut syntax = SYNTAXSET.find_syntax_by_extension("rs").unwrap();

        let mut to_highlight = String::new();
        let mut in_code_block = false;

        let mut cur_heading = String::new();
        let mut in_heading = false;

        let mut output = String::new();
        let events = events.filter_map(|mut event| {
            match event {
                Event::Start(Tag::Link(_, ref mut href, _)) => {
                    if !href.starts_with("http") {
                        let mut abc = base.clone();
                        abc.join(href.trim_end_matches(".md"));
                        let finished_url = abc.to_string_basic();
                        *href = CowStr::from(finished_url.clone());
                        if !links.contains(&finished_url) {
                            links.push(finished_url.clone());
                        }
                    }
                    Some(event)
                }
                Event::Start(Tag::CodeBlock(cb)) => {
                    if let CodeBlockKind::Fenced(token) = cb {
                        in_code_block = true;
                        if let Some(syn) = SYNTAXSET.find_syntax_by_token(&token) {
                            syntax = syn;
                        } else {
                            syntax = SYNTAXSET.find_syntax_by_extension("rs").unwrap();
                        }
                    }
                    None
                }
                Event::End(Tag::CodeBlock(_)) => {
                    if in_code_block {
                        let html =
                            highlighted_html_for_string(&to_highlight, &SYNTAXSET, &syntax, &THEME);
                        to_highlight = String::new();
                        in_code_block = false;
                        Some(Event::Html(CowStr::from(html)))
                    } else {
                        None
                    }
                }
                Event::Start(Tag::Heading(_)) => {
                    in_heading = true;
                    Some(event)
                }
                Event::End(Tag::Heading(level)) => {
                    if in_heading {
                        let html = format!(
                            "<h{} id=\"h-{}-{}\">{}</h{}>",
                            level,
                            crate::featherurl::encode_uri_component(&cur_heading.to_lowercase()),
                            topic_index,
                            cur_heading,
                            level
                        );
                        topics.push(crate::Topic {hash: format!("h-{}-{}", crate::featherurl::encode_uri_component(&cur_heading.to_lowercase()), topic_index), name: cur_heading.clone()});
                        cur_heading = String::new();
                        in_heading = false;
                        topic_index+=1;
                        Some(Event::Html(CowStr::from(html)))
                    } else {
                        None
                    }
                }
                Event::Text(ref t) => {
                    if in_code_block {
                        to_highlight.push_str(t);
                        None
                    } else if in_heading {
                        cur_heading.push_str(t);
                        None
                    } else {
                        Some(event)
                    }
                }
                e => {
                    Some(e)
                }
            }
        });
        html::push_html(&mut output, events);
        (output, links, topics)
    }
}

#[cfg(test)]
mod tests {
    use crate::docs::DocsParser;

    #[test]
    fn heading() {
        let to_parse = "# Lol";
        let parser = DocsParser::new(
            to_parse,
            crate::featherurl::FeatherUrl::from("http://localhost:3000/"),
        );
        assert_eq!("<h1 id=\"h-lol\">Lol</h1>", parser.parse());
    }
}
