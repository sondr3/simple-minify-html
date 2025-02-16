/// Configuration settings that can be adjusted and passed to a minification function to change the
/// minification approach.
#[derive(Clone, Default)]
pub struct Cfg {
    /// Allow unquoted attribute values in the output to contain characters prohibited by the [WHATWG specification](https://html.spec.whatwg.org/multipage/syntax.html#attributes-2). These will still be parsed correctly by almost all browsers.
    pub allow_noncompliant_unquoted_attribute_values: bool,
    /// Allow some minifications around entities that may not pass validation, but will still be parsed correctly by almost all browsers.
    pub allow_optimal_entities: bool,
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

    pub fn enable_possibly_noncompliant(&mut self) {
        self.allow_noncompliant_unquoted_attribute_values = true;
        self.allow_optimal_entities = true;
    }
}
