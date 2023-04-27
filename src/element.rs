// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright © 2023 Probably Yes Software LLC - CC-BY-SA-4.0           ┃
// ┃ Author: Conner Blair <conner@probablyyes.software>                  ┃
// ┠─────────────────────────────────────────────────────────────────────┨
// ┃ This library is made available for public use under the             ┃
// ┃ Creative Commons Attribution Share Alike 4.0 International license. ┃
// ┃ See the top level file 'LICENSE' for more information.              ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

pub mod html;
pub mod none;

use std::{
    fmt::{self, Display, Formatter, Write},
    marker::PhantomData
};

pub trait HtmlElement: Display {
    const ELEMENT_NAME: &'static str = "";

    fn render(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

impl HtmlElement for &str {}

impl HtmlElement for fmt::Arguments<'_> {}

pub struct None;

impl HtmlElement for None {}

impl Display for None {
    fn fmt(&self, _: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

pub struct HtmlTag<TagContent, TagType>(TagContent, PhantomData<TagType>)
where
    TagContent: HtmlElement,
    TagType: tag_type::HtmlTagType;

impl<TagContent, TagType> From<TagContent> for HtmlTag<TagContent, TagType>
where
    TagContent: HtmlElement,
    TagType: tag_type::HtmlTagType
{
    fn from(content: TagContent) -> Self {
        Self(content, PhantomData)
    }
}

pub type OpeningTag<TagContent> = HtmlTag<TagContent, tag_type::OpeningTag>;

impl<TagContent> HtmlElement for OpeningTag<TagContent> where TagContent: HtmlElement {}

impl<TagContent> Display for OpeningTag<TagContent>
where
    TagContent: HtmlElement
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char('<')?;
        self.0.render(f)?;
        f.write_char('>')
    }
}

pub type ClosingTag<TagConent> = HtmlTag<TagConent, tag_type::ClosingTag>;

impl<TagContent> HtmlElement for ClosingTag<TagContent> where TagContent: HtmlElement {}

impl<TagContent> Display for ClosingTag<TagContent>
where
    TagContent: HtmlElement
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("</")?;
        self.0.render(f)?;
        f.write_char('>')
    }
}

mod tag_type {

    pub struct OpeningTag;

    pub struct ClosingTag;

    pub trait HtmlTagType: inner::HtmlTagType {}

    impl<Tag> HtmlTagType for Tag where Tag: inner::HtmlTagType {}

    mod inner {
        pub trait HtmlTagType {}

        impl HtmlTagType for super::OpeningTag {}

        impl HtmlTagType for super::ClosingTag {}
    }
}
