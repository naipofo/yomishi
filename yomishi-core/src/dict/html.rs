use super::parser::{
    structured::{ItemData, StructuredContent, StructuredItem},
    GlossaryDetailed, GlossaryEntry,
};
use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};
use std::io::Cursor;

pub fn render_glossary(glossary: GlossaryEntry) -> quick_xml::Result<(String, Vec<String>)> {
    let mut writer = Writer::new(Cursor::new(vec![]));
    let mut paths = vec![];

    match glossary {
        GlossaryEntry::Text(t) => {
            text(&mut writer, &t)?;
        }
        GlossaryEntry::Detailed(d) => match d {
            GlossaryDetailed::Text { text } => return render_glossary(GlossaryEntry::Text(text)),
            GlossaryDetailed::Image { path } => {
                writer
                    .create_element("img")
                    .with_attribute::<(&str, &str)>(("src", &path))
                    .write_empty()?;
                paths.push(path);
            }
            GlossaryDetailed::StructuredContent { content } => {
                render_structured(&mut writer, content)?
            }
        },
    }

    Ok((
        String::from_utf8(writer.into_inner().into_inner()).unwrap(),
        vec![], // TODO: gather media
    ))
}

fn render_structured(
    writer: &mut Writer<Cursor<Vec<u8>>>,
    c: StructuredContent,
) -> quick_xml::Result<()> {
    match c {
        StructuredContent::Text(t) => text(writer, &t),
        StructuredContent::Multiple(m) => m.into_iter().map(|e| render_item(writer, e)).collect(),
        StructuredContent::Content(c) => render_item(writer, *c),
    }
}

fn render_item(writer: &mut Writer<Cursor<Vec<u8>>>, i: StructuredItem) -> quick_xml::Result<()> {
    match i {
        StructuredItem::Text(t) => text(writer, &t)?,
        StructuredItem::Object { tag, data, variant } => {
            let mut element: BytesStart<'_> = BytesStart::new(&tag);
            let ItemData {
                content,
                style,
                data,
            } = data;
            if let Some(data) = data {
                for (key, val) in data {
                    element.push_attribute::<(&str, &str)>((&format!("data-{key}"), &val));
                }
            };
            if let Some(style) = style {
                for _ in style {
                    // TODO: style
                }
            }

            writer.write_event(Event::Start(element))?;

            if let Some(content) = content {
                render_structured(writer, content)?;
            }
            writer.write_event(Event::End(BytesEnd::new(&tag)))?;
        }
    }
    Ok(())
}

fn text(writer: &mut Writer<Cursor<Vec<u8>>>, text: &str) -> quick_xml::Result<()> {
    let s = text.split("\n").collect::<Vec<_>>();
    for (i, l) in s.iter().enumerate() {
        writer.write_event(Event::Text(BytesText::new(l)))?;
        if i != s.len() - 1 {
            writer.create_element("br").write_empty()?;
        }
    }
    Ok(())
}