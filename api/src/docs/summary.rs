use serde::{Serialize};

/// A struct representing an entry in the `SUMMARY.md`, possibly with nested
/// entries.
#[derive(Debug, PartialEq, Eq, Default, Serialize)]
pub struct Link {
    /// The name of the chapter.
    pub name: String,
    /// The location of the chapter's source file, taking the book's `src`
    /// directory as the root.
    pub location: Option<String>,
    /// Any nested items this chapter may contain.
    pub nested_items: Vec<SummaryItem>,
}

/// An item in `SUMMARY.md` which could be either a separator, part title or a `Link`.
#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum SummaryItem {
    /// A link to a chapter.
    Link(Link),
    /// A separator (`---`).
    Separator,
    /// A part title.
    PartTitle(String),
}

/// The parsed `SUMMARY.md`, specifying how the book should be laid out.
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Summary {
    /// An optional title for the `SUMMARY.md`, currently just ignored.
    pub title: Option<String>,
    /// Chapters before the main text (e.g. an introduction).
    pub prefix_chapters: Vec<SummaryItem>,
    /// The main numbered chapters of the book, broken into one or more possibly named parts.
    pub numbered_chapters: Vec<SummaryItem>,
    /// Items which come after the main document (e.g. a conclusion).
    pub suffix_chapters: Vec<SummaryItem>,
}
