#![deny(unsafe_code)]

use std::io::Write;

pub use crate::cfg::Cfg;
use crate::{
    ast::c14n::c14n_serialise_ast,
    minify::content::minify_content,
    parse::{content::parse_content, Code},
    spec::tag::{ns::Namespace, EMPTY_SLICE},
};

mod ast;
mod cfg;
mod entity;
mod gen;
mod minify;
mod parse;
mod pattern;
mod spec;
mod tag;
#[cfg(test)]
mod tests;
mod whitespace;

/// Minifies UTF-8 HTML code, represented as an array of bytes.
///
/// # Arguments
///
/// * `code` - A slice of bytes representing the source code to minify.
/// * `cfg` - Configuration object to adjust minification approach.
///
/// # Examples
///
/// ```
/// use simple_minify_html::{Cfg, minify};
///
/// let mut code: &[u8] = b"<p>  Hello, world!  </p>";
/// let mut cfg = Cfg::new();
/// cfg.keep_comments = true;
/// let minified = minify(&code, &cfg);
/// assert_eq!(minified, b"<p>Hello, world!".to_vec());
/// ```
#[must_use]
pub fn minify(src: &[u8], cfg: &Cfg) -> Vec<u8> {
    let mut code = Code::new_with_opts(src);
    let parsed = parse_content(&mut code, Namespace::Html, EMPTY_SLICE, EMPTY_SLICE);
    let mut out = Vec::with_capacity(src.len());
    minify_content(
        cfg,
        &mut out,
        Namespace::Html,
        false,
        EMPTY_SLICE,
        parsed.children,
    );
    out.shrink_to_fit();
    out
}

pub fn canonicalize<T: Write>(out: &mut T, src: &[u8]) -> std::io::Result<()> {
    let mut code = Code::new(src);
    let parsed = parse_content(&mut code, Namespace::Html, EMPTY_SLICE, EMPTY_SLICE);
    for c in parsed.children {
        c14n_serialise_ast(out, &c)?;
    }
    Ok(())
}
