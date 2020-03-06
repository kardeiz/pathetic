#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uri(url::Url);

impl From<&str> for Uri {
    fn from(t: &str) -> Self {
        Self::new(t)
    }
}

impl AsRef<str> for Uri {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Uri {
    fn base_url() -> url::Url {
        use once_cell::sync::Lazy;
        static URL: Lazy<url::Url> =
            Lazy::new(|| "http://_".parse().expect("`http://_` is a valid `URL`"));
        URL.clone()
    }

    /// Create a new `URI`, from a path-query-fragment `str`.
    pub fn new(input: &str) -> Self {
        let mut url = Self::base_url();

        let mut query_start = None;
        let mut fragment_start = None;

        for (idx, m) in input.match_indices(|c| c == '?' || c == '#') {
            match m {
                "?" => {
                    query_start = Some(idx);
                }
                "#" => {
                    fragment_start = Some(idx);
                }
                _ => {}
            }
        }

        match (query_start, fragment_start) {
            (None, None) => {
                url.set_path(input);
            }
            (Some(q), None) => {
                url.set_path(&input[..q]);
                url.set_query(Some(&input[q + 1..]));
            }
            (Some(q), Some(f)) => {
                url.set_path(&input[..q]);
                url.set_query(Some(&input[q + 1..f]));
                url.set_fragment(Some(&input[f + 1..]));
            }
            (None, Some(f)) => {
                url.set_path(&input[..f]);
                url.set_fragment(Some(&input[f + 1..]));
            }
        }

        Self(url)
    }
    
    /// Return the serialization of this URL.    
    pub fn as_str(&self) -> &str {
        &self.0[url::Position::BeforePath..]
    }

    /// Return this URL's fragment identifier, if any.
    pub fn fragment(&self) -> Option<&str> {
        self.0.fragment()
    }

    /// Return the path for this URL, as a percent-encoded ASCII string.
    pub fn path(&self) -> &str {
        self.0.path()
    }

    /// Return an iterator of '/' slash-separated path segments, each as a percent-encoded ASCII string.
    pub fn path_segments(&self) -> std::str::Split<char> {
        self.0.path_segments().expect("`URI` is always-a-base")
    }

    /// Return an object with methods to manipulate this URL’s path segments.
    pub fn path_segments_mut(&mut self) -> url::PathSegmentsMut {
        self.0.path_segments_mut().expect("`URI` is always-a-base")
    }

    /// Return this URL's query string, if any, as a percent-encoded ASCII string.
    pub fn query(&self) -> Option<&str> {
        self.0.query()
    }

    /// Parse the URL's query string, if any, as application/x-www-form-urlencoded and return an iterator of (key, value) pairs.
    pub fn query_pairs(&self) -> url::form_urlencoded::Parse {
        self.0.query_pairs()
    }

    /// Manipulate this URL's query string, viewed as a sequence of name/value pairs in application/x-www-form-urlencoded syntax.
    pub fn query_pairs_mut(&mut self) -> url::form_urlencoded::Serializer<url::UrlQuery> {
        self.0.query_pairs_mut()
    }

    /// Change this URL's fragment identifier.
    pub fn set_fragment(&mut self, fragment: Option<&str>) {
        self.0.set_fragment(fragment)
    }

    /// Change this URL's path.
    pub fn set_path(&mut self, path: &str) {
        self.0.set_path(path)
    }

    /// Change this URL’s query string.
    pub fn set_query(&mut self, query: Option<&str>) {
        self.0.set_query(query)
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let mut uri = Uri::new("/foo.html");

        uri.query_pairs_mut()
            .clear()
            .append_pair("foo", "bar & baz");
        println!("{:?}", &uri.as_str());
    }
}
