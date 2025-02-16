#[cfg(feature = "css")]
use std::str::from_utf8;

#[cfg(feature = "css")]
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use minify_html_common::whitespace::trimmed;

use crate::cfg::Cfg;

#[cfg(not(feature = "css"))]
pub fn minify_css(_cfg: &Cfg, out: &mut Vec<u8>, code: &[u8]) {
    out.extend_from_slice(trimmed(code));
}

#[cfg(feature = "css")]
pub fn minify_css(cfg: &Cfg, out: &mut Vec<u8>, code: &[u8]) {
    if cfg.minify_css {
        let popt = PrinterOptions {
            minify: true,
            ..Default::default()
        };
        let result = match StyleSheet::parse(
            from_utf8(code).expect("<style> content contains non-UTF-8"),
            ParserOptions::default(),
        ) {
            Ok(mut sty) => match sty.minify(MinifyOptions::default()) {
                Ok(()) => match sty.to_css(popt) {
                    Ok(out) => Some(out.code),
                    // TODO Collect error as warning.
                    Err(_err) => None,
                },
                // TODO Collect error as warning.
                Err(_err) => None,
            },
            // TODO Collect error as warning.
            Err(_err) => None,
        };
        if let Some(min) = result {
            if min.len() < code.len() {
                out.extend_from_slice(min.as_bytes());
                return;
            };
        };
    }
    out.extend_from_slice(trimmed(code));
}
