use crate::{
    ast::{NodeData, RcdataContentType},
    entity::decode::decode_entities,
    parse::{Code, content::ParsedContent},
    tag::TAG_TEXTAREA_END,
};

pub fn parse_textarea_content(code: &mut Code) -> ParsedContent {
    let (len, closing_tag_omitted) = match TAG_TEXTAREA_END.find(code.as_slice()) {
        Some(m) => (m.start(), false),
        None => (code.rem(), true),
    };
    ParsedContent {
        closing_tag_omitted,
        children: vec![NodeData::RcdataContent {
            typ: RcdataContentType::Textarea,
            text: decode_entities(code.slice_and_shift(len), false),
        }],
    }
}
