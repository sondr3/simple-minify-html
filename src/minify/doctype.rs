pub fn minify_doctype(out: &mut Vec<u8>, ended: bool) {
    out.extend_from_slice(b"<!doctype");
    out.push(b' ');
    out.extend_from_slice(b"html");
    if ended {
        out.extend_from_slice(b">");
    };
}
