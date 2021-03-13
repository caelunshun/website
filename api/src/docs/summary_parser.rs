<<<<<<< HEAD
use pulldown_cmark::{Event, Parser, Tag};

use super::{Link, Summary, SummaryItem};
use anyhow::{anyhow, bail, Result};

pub struct SummaryParser<'a> {
    tokens: std::iter::Peekable<Parser<'a>>,
}

impl<'a> SummaryParser<'a> {
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

    pub fn parse(mut self) -> Result<Summary> {
        let title = self.parse_title();
        let prefix_chapters = self.parse_items()?;
        let numbered_chapters = self.parse_numbered()?;
        let suffix_chapters = self.parse_items()?;

        self.parse_end()?;

        Ok(Summary {
            title,
            prefix_chapters,
            numbered_chapters,
            suffix_chapters,
        })
    }

    fn parse_title(&mut self) -> Option<String> {
        match self.peek()? {
            Event::Start(Tag::Heading(1)) => {
                self.next();
                let events = self.take_while(|event| !matches!(event, Event::End(Tag::Heading(1))));
                Some(stringify_events(events))
            }
            _ => None,
        }
    }

    fn parse_items(&mut self) -> Result<Vec<SummaryItem>> {
        let mut items = Vec::new();
        while let Some(event) = self.peek() {
            match event {
                Event::Start(Tag::List(..)) | Event::Start(Tag::Heading(1)) => break,
                Event::Start(Tag::Link(_type, _, _title)) => {
                    items.push(SummaryItem::Link(self.parse_link().unwrap()));
                }
                Event::Rule => {
                    self.next();
                    items.push(SummaryItem::Separator)
                }
                _ => {
                    self.next();
                }
            }
        }
        Ok(items)
    }

    fn parse_numbered(&mut self) -> Result<Vec<SummaryItem>> {
        let mut items = Vec::new();

        // Start of suffix
        if let Some(Event::Start(Tag::Paragraph)) = self.peek() {
            return Ok(items);
        }

        let mut open = 0;
        while let Some(event) = self.peek() {
            if let Some(title) = self.parse_title() {
                items.push(SummaryItem::PartTitle(title));
            }
            match event {
                Event::Start(Tag::Item) => {
                    open += 1;
                    self.next();
                    let mut link = self
                        .parse_link()
                        .ok_or(anyhow!("List item should contain a link"))?;

                    link.nested_items = self.parse_numbered()?;

                    items.push(SummaryItem::Link(link))
                }
                Event::End(Tag::Item) if open > 0 => {
                    open -= 1; self.next();
                }
                Event::End(Tag::Item) => break,
                Event::Rule => { self.next(); items.push(SummaryItem::Separator)},
                _ => { self.next(); },
            }
        }

        Ok(items)
    }

    fn parse_end(&mut self) -> Result<()> {
        while let Some(event) = self.next() {
            match event {
                Event::Start(Tag::List(..)) | Event::Start(Tag::Heading(..)) => {
                    bail!("Did not expect this...")
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn parse_link(&mut self) -> Option<Link> {
        if let Some(Event::Start(Tag::Link(_, href, _))) = self.next() {
            let location = if href.is_empty() {
                None
            } else {
                Some(href.to_string())
            };

            let events = self.take_while(|event| !matches!(event, Event::End(Tag::Link(..))));
            let name = stringify_events(events);
            Some(Link {
                name,
                location,
                nested_items: Vec::new(),
            })
        } else {
            None
        }
    }

    /// Takes until token, inclusive
    fn take_while<P>(&mut self, mut predicate: P) -> Vec<Event>
    where
        P: FnMut(&Event) -> bool,
    {
        let mut tokens = Vec::new();
        while let Some(event) = self.next() {
            if !predicate(&event) {
                tokens.push(event);
                break;
            }
            tokens.push(event);
        }
        tokens
    }
}

/// Removes the styling from a list of Markdown events and returns just the
/// plain text.
fn stringify_events(events: Vec<Event<'_>>) -> String {
    events
        .into_iter()
        .filter_map(|event| match event {
            Event::Text(text) | Event::Code(text) => Some(text.to_string()),
            Event::SoftBreak => Some(String::from(" ")),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_initial_title() {
        let src = "# Summary";
        let should_be = String::from("Summary");

        let mut parser = SummaryParser::new(src);
        let got = parser.parse_title().unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_title_with_styling() {
        let src = "# My **Awesome** Summary";
        let should_be = String::from("My Awesome Summary");

        let mut parser = SummaryParser::new(src);
        let got = parser.parse_title().unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn convert_markdown_events_to_a_string() {
        let src = "Hello *World*, `this` is some text [and a link](./path/to/link)";
        let should_be = "Hello World, this is some text and a link";

        let events = pulldown_cmark::Parser::new(src).collect();
        let got = stringify_events(events);

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_some_prefix_items() {
        let src = "[First](./first.md)\n[Second](./second.md)\n";
        let mut parser = SummaryParser::new(src);

        let should_be = vec![
            SummaryItem::Link(Link {
                name: String::from("First"),
                location: Some("./first.md".to_owned()),
                ..Default::default()
            }),
            SummaryItem::Link(Link {
                name: String::from("Second"),
                location: Some("./second.md".to_owned()),
                ..Default::default()
            }),
        ];

        let got = parser.parse_items().unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_prefix_items_with_a_separator() {
        let src = "[First](./first.md)\n\n---\n\n[Second](./second.md)\n";
        let mut parser = SummaryParser::new(src);

        let got = parser.parse_items().unwrap();

        assert_eq!(got.len(), 3);
        assert_eq!(got[1], SummaryItem::Separator);
    }

    #[test]
    fn suffix_items_cannot_be_followed_by_a_list() {
        let src = "[First](./first.md)\n- [Second](./second.md)\n";
        let mut parser = SummaryParser::new(src);

        parser.parse_items().unwrap();
        let got = parser.parse_end();

        assert!(got.is_err());
    }

    #[test]
    fn parse_a_link() {
        let src = "[First](./first.md)";
        let should_be = Link {
            name: String::from("First"),
            location: Some("./first.md".to_owned()),
            ..Default::default()
        };

        let mut parser = SummaryParser::new(src);
        let _ = parser.next(); // Discard opening paragraph

        let got = parser.parse_link();
        assert_eq!(got, Some(should_be));
    }

    #[test]
    fn parse_a_numbered_chapter() {
        let src = "- [First](./first.md)\n";
        let link = Link {
            name: String::from("First"),
            location: Some("./first.md".to_owned()),
            ..Default::default()
        };
        let should_be = vec![SummaryItem::Link(link)];

        let mut parser = SummaryParser::new(src);
        let got = parser.parse_numbered().unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_nested_numbered_chapters() {
        let src = "- [First](./first.md)\n  - [Nested](./nested.md)\n- [Second](./second.md)";

        let should_be = vec![
            SummaryItem::Link(Link {
                name: String::from("First"),
                location: Some("./first.md".to_owned()),
                nested_items: vec![SummaryItem::Link(Link {
                    name: String::from("Nested"),
                    location: Some("./nested.md".to_owned()),
                    nested_items: Vec::new(),
                })],
            }),
            SummaryItem::Link(Link {
                name: String::from("Second"),
                location: Some("./second.md".to_owned()),
                nested_items: Vec::new(),
            }),
        ];

        let mut parser = SummaryParser::new(src);
        let got = parser
            .parse_numbered()
            .unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_numbered_chapters_separated_by_comment() {
        let src = "- [First](./first.md)\n<!-- this is a comment -->\n- [Second](./second.md)";

        let should_be = vec![
            SummaryItem::Link(Link {
                name: String::from("First"),
                location: Some("./first.md".to_owned()),
                nested_items: Vec::new(),
            }),
            SummaryItem::Link(Link {
                name: String::from("Second"),
                location: Some("./second.md".to_owned()),
                nested_items: Vec::new(),
            }),
        ];

        let mut parser = SummaryParser::new(src);
        let got = parser
            .parse_numbered()
            .unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_titled_parts() {
        let src = "- [First](./first.md)\n- [Second](./second.md)\n\
                   # Title 2\n- [Third](./third.md)\n\t- [Fourth](./fourth.md)";

        let should_be = vec![
            SummaryItem::Link(Link {
                name: String::from("First"),
                location: Some("./first.md".to_owned()),
                nested_items: Vec::new(),
            }),
            SummaryItem::Link(Link {
                name: String::from("Second"),
                location: Some("./second.md".to_owned()),
                nested_items: Vec::new(),
            }),
            SummaryItem::PartTitle(String::from("Title 2")),
            SummaryItem::Link(Link {
                name: String::from("Third"),
                location: Some("./third.md".to_owned()),
                nested_items: vec![SummaryItem::Link(Link {
                    name: String::from("Fourth"),
                    location: Some("./fourth.md".to_owned()),
                    nested_items: Vec::new(),
                })],
            }),
        ];

        let mut parser = SummaryParser::new(src);
        let got = parser.parse_numbered().unwrap();

        assert_eq!(got, should_be);
    }

    /// This test ensures the book will continue to pass because it breaks the
    /// `SUMMARY.md` up using level 2 headers ([example]).
    ///
    /// [example]: https://github.com/rust-lang/book/blob/2c942dc094f4ddcdc7aba7564f80782801197c99/second-edition/src/SUMMARY.md#basic-rust-literacy
    #[test]
    fn can_have_a_subheader_between_nested_items() {
        let src = "- [First](./first.md)\n\n## Subheading\n\n- [Second](./second.md)\n";
        let should_be = vec![
            SummaryItem::Link(Link {
                name: String::from("First"),
                location: Some("./first.md".to_owned()),
                nested_items: Vec::new(),
            }),
            SummaryItem::Link(Link {
                name: String::from("Second"),
                location: Some("./second.md".to_owned()),
                nested_items: Vec::new(),
            }),
        ];

        let mut parser = SummaryParser::new(src);
        let got = parser
            .parse_numbered()
            .unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn an_empty_link_location_is_a_draft_chapter() {
        let src = "- [Empty]()\n";
        let mut parser = SummaryParser::new(src);

        let got = parser.parse_numbered();
        let should_be = vec![SummaryItem::Link(Link {
            name: String::from("Empty"),
            location: None,
            nested_items: Vec::new(),
        })];

        assert!(got.is_ok());
        assert_eq!(got.unwrap(), should_be);
    }

    /// Ensure section numbers are correctly incremented after a horizontal separator.
    #[test]
    fn keep_numbering_after_separator() {
        let src =
            "- [First](./first.md)\n---\n- [Second](./second.md)\n---\n- [Third](./third.md)\n";
        let should_be = vec![
            SummaryItem::Link(Link {
                name: String::from("First"),
                location: Some("./first.md".to_owned()),
                nested_items: Vec::new(),
            }),
            SummaryItem::Separator,
            SummaryItem::Link(Link {
                name: String::from("Second"),
                location: Some("./second.md".to_owned()),
                nested_items: Vec::new(),
            }),
            SummaryItem::Separator,
            SummaryItem::Link(Link {
                name: String::from("Third"),
                location: Some("./third.md".to_owned()),
                nested_items: Vec::new(),
            }),
        ];

        let mut parser = SummaryParser::new(src);
        let got = parser
            .parse_numbered()
            .unwrap();

        assert_eq!(got, should_be);
    }

    /// Ensure chapter names spread across multiple lines have spaces between all the words.
    #[test]
    fn add_space_for_multi_line_chapter_names() {
        let src = "- [Chapter\ntitle](./chapter.md)";
        let should_be = vec![SummaryItem::Link(Link {
            name: String::from("Chapter title"),
            location: Some("./chapter.md".to_owned()),
            nested_items: Vec::new(),
        })];

        let mut parser = SummaryParser::new(src);
        let got = parser
            .parse_numbered()
            .unwrap();

        assert_eq!(got, should_be);
    }
}
=======
use pulldown_cmark::{Event, Parser, Tag};

use super::{Link, Summary, SummaryItem};
use anyhow::{anyhow, bail, Result};

pub struct SummaryParser<'a> {
    tokens: std::iter::Peekable<Parser<'a>>,
}

impl<'a> SummaryParser<'a> {
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

    pub fn parse(mut self) -> Result<Summary> {
        let title = self.parse_title();
        let prefix_chapters = self.parse_items()?;
        let numbered_chapters = self.parse_numbered()?;
        let suffix_chapters = self.parse_items()?;

        self.parse_end()?;

        Ok(Summary {
            title,
            prefix_chapters,
            numbered_chapters,
            suffix_chapters,
        })
    }

    fn parse_title(&mut self) -> Option<String> {
        match self.peek()? {
            Event::Start(Tag::Heading(1)) => {
                self.next();
                let events = self.take_while(|event| !matches!(event, Event::End(Tag::Heading(1))));
                Some(stringify_events(events))
            }
            _ => None,
        }
    }

    fn parse_items(&mut self) -> Result<Vec<SummaryItem>> {
        let mut items = Vec::new();
        while let Some(event) = self.peek() {
            match event {
                Event::Start(Tag::List(..)) | Event::Start(Tag::Heading(1)) => break,
                Event::Start(Tag::Link(_type, _, _title)) => {
                    items.push(SummaryItem::Link(self.parse_link().unwrap()));
                }
                Event::Rule => {
                    self.next();
                    items.push(SummaryItem::Separator)
                }
                _ => {
                    self.next();
                }
            }
        }
        Ok(items)
    }

    fn parse_numbered(&mut self) -> Result<Vec<SummaryItem>> {
        let mut items = Vec::new();

        // Start of suffix
        if let Some(Event::Start(Tag::Paragraph)) = self.peek() {
            return Ok(items);
        }

        let mut open = 0;
        while let Some(event) = self.peek() {
            if let Some(title) = self.parse_title() {
                items.push(SummaryItem::PartTitle(title));
            }
            match event {
                Event::Start(Tag::Item) => {
                    open += 1;
                    self.next();
                    let mut link = self
                        .parse_link()
                        .ok_or(anyhow!("List item should contain a link"))?;

                    link.nested_items = self.parse_numbered()?;

                    items.push(SummaryItem::Link(link))
                }
                Event::End(Tag::Item) if open > 0 => {
                    open -= 1; self.next();
                }
                Event::End(Tag::Item) => break,
                Event::Rule => { self.next(); items.push(SummaryItem::Separator)},
                _ => { self.next(); },
            }
        }

        Ok(items)
    }

    fn parse_end(&mut self) -> Result<()> {
        while let Some(event) = self.next() {
            match event {
                Event::Start(Tag::List(..)) | Event::Start(Tag::Heading(..)) => {
                    bail!("Did not expect this...")
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn parse_link(&mut self) -> Option<Link> {
        if let Some(Event::Start(Tag::Link(_, href, _))) = self.next() {
            let location = if href.is_empty() {
                None
            } else {
                Some(href.to_string())
            };

            let events = self.take_while(|event| !matches!(event, Event::End(Tag::Link(..))));
            let name = stringify_events(events);
            Some(Link {
                name,
                location,
                nested_items: Vec::new(),
            })
        } else {
            None
        }
    }

    /// Takes until token, inclusive
    fn take_while<P>(&mut self, mut predicate: P) -> Vec<Event>
    where
        P: FnMut(&Event) -> bool,
    {
        let mut tokens = Vec::new();
        while let Some(event) = self.next() {
            if !predicate(&event) {
                tokens.push(event);
                break;
            }
            tokens.push(event);
        }
        tokens
    }
}

/// Removes the styling from a list of Markdown events and returns just the
/// plain text.
fn stringify_events(events: Vec<Event<'_>>) -> String {
    events
        .into_iter()
        .filter_map(|event| match event {
            Event::Text(text) | Event::Code(text) => Some(text.to_string()),
            Event::SoftBreak => Some(String::from(" ")),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_initial_title() {
        let src = "# Summary";
        let should_be = String::from("Summary");

        let mut parser = SummaryParser::new(src);
        let got = parser.parse_title().unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_title_with_styling() {
        let src = "# My **Awesome** Summary";
        let should_be = String::from("My Awesome Summary");

        let mut parser = SummaryParser::new(src);
        let got = parser.parse_title().unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn convert_markdown_events_to_a_string() {
        let src = "Hello *World*, `this` is some text [and a link](./path/to/link)";
        let should_be = "Hello World, this is some text and a link";

        let events = pulldown_cmark::Parser::new(src).collect();
        let got = stringify_events(events);

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_some_prefix_items() {
        let src = "[First](./first.md)\n[Second](./second.md)\n";
        let mut parser = SummaryParser::new(src);

        let should_be = vec![
            SummaryItem::Link(Link {
                name: String::from("First"),
                location: Some("./first.md".to_owned()),
                ..Default::default()
            }),
            SummaryItem::Link(Link {
                name: String::from("Second"),
                location: Some("./second.md".to_owned()),
                ..Default::default()
            }),
        ];

        let got = parser.parse_items().unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_prefix_items_with_a_separator() {
        let src = "[First](./first.md)\n\n---\n\n[Second](./second.md)\n";
        let mut parser = SummaryParser::new(src);

        let got = parser.parse_items().unwrap();

        assert_eq!(got.len(), 3);
        assert_eq!(got[1], SummaryItem::Separator);
    }

    #[test]
    fn suffix_items_cannot_be_followed_by_a_list() {
        let src = "[First](./first.md)\n- [Second](./second.md)\n";
        let mut parser = SummaryParser::new(src);

        parser.parse_items().unwrap();
        let got = parser.parse_end();

        assert!(got.is_err());
    }

    #[test]
    fn parse_a_link() {
        let src = "[First](./first.md)";
        let should_be = Link {
            name: String::from("First"),
            location: Some("./first.md".to_owned()),
            ..Default::default()
        };

        let mut parser = SummaryParser::new(src);
        let _ = parser.next(); // Discard opening paragraph

        let got = parser.parse_link();
        assert_eq!(got, Some(should_be));
    }

    #[test]
    fn parse_a_numbered_chapter() {
        let src = "- [First](./first.md)\n";
        let link = Link {
            name: String::from("First"),
            location: Some("./first.md".to_owned()),
            ..Default::default()
        };
        let should_be = vec![SummaryItem::Link(link)];

        let mut parser = SummaryParser::new(src);
        let got = parser.parse_numbered().unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_nested_numbered_chapters() {
        let src = "- [First](./first.md)\n  - [Nested](./nested.md)\n- [Second](./second.md)";

        let should_be = vec![
            SummaryItem::Link(Link {
                name: String::from("First"),
                location: Some("./first.md".to_owned()),
                nested_items: vec![SummaryItem::Link(Link {
                    name: String::from("Nested"),
                    location: Some("./nested.md".to_owned()),
                    nested_items: Vec::new(),
                })],
            }),
            SummaryItem::Link(Link {
                name: String::from("Second"),
                location: Some("./second.md".to_owned()),
                nested_items: Vec::new(),
            }),
        ];

        let mut parser = SummaryParser::new(src);
        let got = parser
            .parse_numbered()
            .unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_numbered_chapters_separated_by_comment() {
        let src = "- [First](./first.md)\n<!-- this is a comment -->\n- [Second](./second.md)";

        let should_be = vec![
            SummaryItem::Link(Link {
                name: String::from("First"),
                location: Some("./first.md".to_owned()),
                nested_items: Vec::new(),
            }),
            SummaryItem::Link(Link {
                name: String::from("Second"),
                location: Some("./second.md".to_owned()),
                nested_items: Vec::new(),
            }),
        ];

        let mut parser = SummaryParser::new(src);
        let got = parser
            .parse_numbered()
            .unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_titled_parts() {
        let src = "- [First](./first.md)\n- [Second](./second.md)\n\
                   # Title 2\n- [Third](./third.md)\n\t- [Fourth](./fourth.md)";

        let should_be = vec![
            SummaryItem::Link(Link {
                name: String::from("First"),
                location: Some("./first.md".to_owned()),
                nested_items: Vec::new(),
            }),
            SummaryItem::Link(Link {
                name: String::from("Second"),
                location: Some("./second.md".to_owned()),
                nested_items: Vec::new(),
            }),
            SummaryItem::PartTitle(String::from("Title 2")),
            SummaryItem::Link(Link {
                name: String::from("Third"),
                location: Some("./third.md".to_owned()),
                nested_items: vec![SummaryItem::Link(Link {
                    name: String::from("Fourth"),
                    location: Some("./fourth.md".to_owned()),
                    nested_items: Vec::new(),
                })],
            }),
        ];

        let mut parser = SummaryParser::new(src);
        let got = parser.parse_numbered().unwrap();

        assert_eq!(got, should_be);
    }

    /// This test ensures the book will continue to pass because it breaks the
    /// `SUMMARY.md` up using level 2 headers ([example]).
    ///
    /// [example]: https://github.com/rust-lang/book/blob/2c942dc094f4ddcdc7aba7564f80782801197c99/second-edition/src/SUMMARY.md#basic-rust-literacy
    #[test]
    fn can_have_a_subheader_between_nested_items() {
        let src = "- [First](./first.md)\n\n## Subheading\n\n- [Second](./second.md)\n";
        let should_be = vec![
            SummaryItem::Link(Link {
                name: String::from("First"),
                location: Some("./first.md".to_owned()),
                nested_items: Vec::new(),
            }),
            SummaryItem::Link(Link {
                name: String::from("Second"),
                location: Some("./second.md".to_owned()),
                nested_items: Vec::new(),
            }),
        ];

        let mut parser = SummaryParser::new(src);
        let got = parser
            .parse_numbered()
            .unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn an_empty_link_location_is_a_draft_chapter() {
        let src = "- [Empty]()\n";
        let mut parser = SummaryParser::new(src);

        let got = parser.parse_numbered();
        let should_be = vec![SummaryItem::Link(Link {
            name: String::from("Empty"),
            location: None,
            nested_items: Vec::new(),
        })];

        assert!(got.is_ok());
        assert_eq!(got.unwrap(), should_be);
    }

    /// Ensure section numbers are correctly incremented after a horizontal separator.
    #[test]
    fn keep_numbering_after_separator() {
        let src =
            "- [First](./first.md)\n---\n- [Second](./second.md)\n---\n- [Third](./third.md)\n";
        let should_be = vec![
            SummaryItem::Link(Link {
                name: String::from("First"),
                location: Some("./first.md".to_owned()),
                nested_items: Vec::new(),
            }),
            SummaryItem::Separator,
            SummaryItem::Link(Link {
                name: String::from("Second"),
                location: Some("./second.md".to_owned()),
                nested_items: Vec::new(),
            }),
            SummaryItem::Separator,
            SummaryItem::Link(Link {
                name: String::from("Third"),
                location: Some("./third.md".to_owned()),
                nested_items: Vec::new(),
            }),
        ];

        let mut parser = SummaryParser::new(src);
        let got = parser
            .parse_numbered()
            .unwrap();

        assert_eq!(got, should_be);
    }

    /// Ensure chapter names spread across multiple lines have spaces between all the words.
    #[test]
    fn add_space_for_multi_line_chapter_names() {
        let src = "- [Chapter\ntitle](./chapter.md)";
        let should_be = vec![SummaryItem::Link(Link {
            name: String::from("Chapter title"),
            location: Some("./chapter.md".to_owned()),
            nested_items: Vec::new(),
        })];

        let mut parser = SummaryParser::new(src);
        let got = parser
            .parse_numbered()
            .unwrap();

        assert_eq!(got, should_be);
    }
}
>>>>>>> 4a79665933737d77310f562f62776e2200b9174f
