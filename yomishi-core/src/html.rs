use crate::{
    dict::parser::{
        structured::{ItemData, StructuredContent, StructuredItem},
        tag::Tag,
        term::{GlossaryDetailed, GlossaryEntry},
        term_meta::TermMeta,
    },
    dictionary::search::{DictionaryTagged, SearchResult, TermWithTags},
    japanese::ruby::{try_from_reading, Segment},
};

use handlebars::{handlebars_helper, Handlebars};
use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Cursor;

#[derive(Debug, Serialize, Deserialize)]
pub struct GlossaryTemplateData {
    pub ruby: Vec<Segment>,
    pub inflection_rules: Vec<String>,
    pub tags: Vec<Tag>,
    pub meta: Vec<DictionaryTagged<TermMeta>>,
    pub glossaries: Vec<DictionaryTagged<TermWithTags>>,
}

pub struct HandlebarsRenderer<'a>(Handlebars<'a>);

impl HandlebarsRenderer<'_> {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("t1", include_str!("html/templates.hbs"))
            .unwrap();

        handlebars_helper!(FormatGlossary: |entry: GlossaryEntry| { render_entry(entry) });
        handlebars.register_helper("formatGlossary", Box::new(FormatGlossary));

        handlebars_helper!(CheckArray: |entry: Value| { entry.is_array() });
        handlebars.register_helper("isArray", Box::new(CheckArray));

        Self(handlebars)
    }

    pub fn render_marker<T: Serialize>(&self, marker: &str, data: T) -> String {
        self.0
            .render(
                "t1",
                &serde_json::json!({ "marker": marker, "data": &data }),
            )
            .unwrap()
    }

    pub fn render_glossary(&self, glossaries: &GlossaryTemplateData) -> String {
        self.render_marker("full-glossary", glossaries)
    }
}

fn render_entry(entry: GlossaryEntry) -> String {
    match entry {
        GlossaryEntry::Text(t) => render_pure_text(&t),
        GlossaryEntry::Detailed(_) => render_glossary_old(entry).unwrap().0,
    }
}

fn render_pure_text(text: &str) -> String {
    text.split("\n").collect::<Vec<_>>().join("<br>")
}

pub fn render_glossary_old(glossary: GlossaryEntry) -> quick_xml::Result<(String, Vec<String>)> {
    let mut writer = Writer::new(Cursor::new(vec![]));
    let mut paths = vec![];

    match glossary {
        GlossaryEntry::Text(t) => {
            text(&mut writer, &t)?;
        }
        GlossaryEntry::Detailed(d) => match d {
            GlossaryDetailed::Text { text } => return Ok((render_pure_text(&text), vec![])),
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

pub fn search_to_template_data(result: SearchResult) -> GlossaryTemplateData {
    let SearchResult {
        deinflection,
        glossaries,
        tags,
        meta,
    } = result;

    GlossaryTemplateData {
        ruby: try_from_reading(
            glossaries.get(0).unwrap().data.term.expression.to_string(),
            glossaries.get(0).unwrap().data.term.reading.to_string(),
        ),
        inflection_rules: deinflection.reasons.iter().map(|e| e.to_string()).collect(),
        tags,
        meta,
        glossaries,
    }
}
