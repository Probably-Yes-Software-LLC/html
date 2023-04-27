// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright © 2023 Probably Yes Software LLC - CC-BY-SA-4.0           ┃
// ┃ Author: Conner Blair <conner@probablyyes.software>                  ┃
// ┠─────────────────────────────────────────────────────────────────────┨
// ┃ This library is made available for public use under the             ┃
// ┃ Creative Commons Attribution Share Alike 4.0 International license. ┃
// ┃ See the top level file 'LICENSE' for more information.              ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::fmt::{self, Display, Formatter};

use crate::element::{ClosingTag, HtmlElement, None, OpeningTag};

pub struct Html<Head = None, Body = None>
where
    Head: HtmlElement,
    Body: HtmlElement
{
    head: Head,
    body: Body
}

impl Html {
    pub fn head<Head>(head: Head) -> Html<Head>
    where
        Head: HtmlElement
    {
        Html { head, body: None }
    }
}

impl<Head> Html<Head>
where
    Head: HtmlElement
{
    pub fn body<Body>(self, body: Body) -> Html<Head, Body>
    where
        Body: HtmlElement
    {
        Html {
            head: self.head,
            body
        }
    }
}

impl<Head, Body> HtmlElement for Html<Head, Body>
where
    Head: HtmlElement,
    Body: HtmlElement
{
    const ELEMENT_NAME: &'static str = "html";
}

impl<Head, Body> Display for Html<Head, Body>
where
    Head: HtmlElement,
    Body: HtmlElement
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        OpeningTag::from(format_args!("!DOCTYPE {}", Self::ELEMENT_NAME)).render(f)?;
        OpeningTag::from(Self::ELEMENT_NAME).render(f)?;
        self.head.render(f)?;
        self.body.render(f)?;
        ClosingTag::from(Self::ELEMENT_NAME).render(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn render_basic_html() {
        let head = "<head></head>";
        let body = "<body></body>";

        let html = Html::head(head).body(body);

        assert_eq!(
            html.to_string(),
            format!("<!DOCTYPE html><html>{}{}</html>", head, body)
        );
    }
}
