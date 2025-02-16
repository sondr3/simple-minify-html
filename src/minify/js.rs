#[cfg(feature = "js")]
use oxc_allocator::Allocator;
#[cfg(feature = "js")]
use oxc_codegen::{CodeGenerator, CodegenOptions};
#[cfg(feature = "js")]
use oxc_mangler::MangleOptions;
#[cfg(feature = "js")]
use oxc_minifier::{CompressOptions, Minifier, MinifierOptions};
#[cfg(feature = "js")]
use oxc_parser::Parser;
#[cfg(feature = "js")]
use oxc_span::SourceType;

use crate::whitespace::trimmed;

#[cfg(feature = "js")]
pub fn minify_js(out: &mut Vec<u8>, code: &[u8]) {
    let allocator = Allocator::default();
    let source_type = SourceType::cjs();
    let code = std::str::from_utf8(code).expect("js contained invalid utf-8");
    let minified = minify(&allocator, code, source_type);
    out.extend_from_slice(trimmed(minified.as_bytes()));
}

#[cfg(feature = "js")]
fn minify(allocator: &Allocator, source: &str, source_type: SourceType) -> String {
    let ret = Parser::new(allocator, source, source_type).parse();
    let mut program = ret.program;
    let options = MinifierOptions {
        mangle: Some(MangleOptions::default()),
        compress: Some(CompressOptions::default()),
    };
    let ret = Minifier::new(options).build(allocator, &mut program);
    CodeGenerator::new()
        .with_options(CodegenOptions {
            minify: true,
            ..CodegenOptions::default()
        })
        .with_symbol_table(ret.symbol_table)
        .build(&program)
        .code
}

#[cfg(not(feature = "js"))]
pub fn minify_js(out: &mut Vec<u8>, code: &[u8]) {
    out.extend_from_slice(trimmed(code));
}
