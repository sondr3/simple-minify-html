/// Configuration settings that can be adjusted and passed to a minification function to change the
/// minification approach.
#[derive(Clone, Default)]
pub struct Cfg {
    /// Do not omit closing tags when possible.
    pub keep_closing_tags: bool,
    /// Keep all comments.
    pub keep_comments: bool,
    /// Do not omit `<html>` and `<head>` opening tags when they don't have attributes.
    pub keep_html_and_head_opening_tags: bool,
    /// Keep `type=text` attribute name and value on `<input>` elements.
    pub keep_input_type_text_attr: bool,
    /// Keep SSI comments.
    pub keep_ssi_comments: bool,
    /// Remove all bangs.
    pub remove_bangs: bool,
    /// Remove all processing instructions.
    pub remove_processing_instructions: bool,
}

impl Cfg {
    pub fn new() -> Cfg {
        Cfg::default()
    }
}
