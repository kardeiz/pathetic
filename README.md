# pathetic

[![Docs](https://docs.rs/pathetic/badge.svg)](https://docs.rs/crate/pathetic/)
[![Crates.io](https://img.shields.io/crates/v/pathetic.svg)](https://crates.io/crates/pathetic)

A library for working with relative URIs, based on the `url` crate.

## Usage:

```rust
fn main() {
    let uri = pathetic::Uri::default()
        .with_path_segments_mut(|p| p.extend(&["foo", "bar"]))
        .with_query_pairs_mut(|q| q.append_pair("foo", "bar"))
        .with_fragment(Some("baz"));

    assert_eq!("/foo/bar?foo=bar#baz", uri.as_str());
}
```

<hr/>

Current version: 0.3.0

License: MIT
