pub use {
    id_url_display::LiteralIdUrlDisplay,
    this::Literal,
    url_display::LiteralUrlDisplay,
    value::LiteralValue,
};

mod id_url_display;
#[cfg(test)]
mod tests;
mod this;
mod url_display;
mod value;

// Unfortunately, the hyper:Uri, if it has an empty path
// prints itself with a slash at the end, which is not
// what we want for an RDF resource identifier.
pub fn write_iri(f: &mut std::fmt::Formatter<'_>, iri: &hyper::Uri) -> std::fmt::Result {
    if iri.path() == "/" {
        write!(
            f,
            "{}",
            iri.to_string().as_str().strip_suffix("/").unwrap()
        )
    } else {
        write!(f, "{}", iri)
    }
}
