//! A library for working with relative URIs, based on the `url` crate.
//! 
//! # Usage:
//! 
//! ```rust
//! fn main() {
//!     let uri = pathetic::Uri::default()
//!         .with_path_segments_mut(|p| p.extend(&["foo", "bar"]))
//!         .with_query_pairs_mut(|q| q.append_pair("foo", "bar"))
//!         .with_fragment(Some("baz"));
//! 
//!     assert_eq!("/foo/bar?foo=bar#baz", uri.as_str());
//! } 
//! ```

/// Create a new base `Uri`
pub fn uri() -> Uri {
    Uri::default()
}

/// The relative URI container
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uri(url::Url);

impl std::fmt::Debug for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Uri").field(&self.as_str()).finish()
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl std::convert::TryFrom<&str> for Uri {
    type Error = url::ParseError;
    fn try_from(t: &str) -> Result<Self, Self::Error> {
        Self::new(t)
    }
}

#[cfg(feature = "actix-web")]
impl From<&actix_web::http::uri::Uri> for Uri {
    fn from(t: &actix_web::http::uri::Uri) -> Self {
        Self::default()
            .with_path(t.path())
            .with_query(t.query())
    }
}

impl AsRef<str> for Uri {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Default for Uri {
    fn default() -> Self {
        Self(Self::base_url())
    }
}

impl Uri {
    fn base_url() -> url::Url {
        use once_cell::sync::Lazy; 
        static URL: Lazy<url::Url> =
            Lazy::new(|| "http://_".parse().expect("`http://_` is a valid `URL`"));
        URL.clone()
    }

    /// Create a new `Uri`, from a path-query-fragment `str`.
    pub fn new(input: &str) -> Result<Self, url::ParseError> {
        Self::base_url().join(input).map(Self)
    }

    /// Parse a string as an URL, with this URL as the base URL.
    pub fn join(&self, input: &str) -> Result<Self, url::ParseError> {
        self.0.join(input).map(Self)
    }

    /// Return the serialization of this URL.    
    pub fn as_str(&self) -> &str {
        &self.0[url::Position::BeforePath..]
    }    

    /// Return the path for this URL, as a percent-encoded ASCII string.
    pub fn path(&self) -> &str {
        self.0.path()
    }

    /// Return this URL's query string, if any, as a percent-encoded ASCII string.
    pub fn query(&self) -> Option<&str> {
        self.0.query()
    }

    /// Return this URL's fragment identifier, if any.
    pub fn fragment(&self) -> Option<&str> {
        self.0.fragment()
    }

    /// Return an iterator of '/' slash-separated path segments, each as a percent-encoded ASCII string.
    pub fn path_segments(&self) -> std::str::Split<char> {
        self.0.path_segments().expect("`Uri` is always-a-base")
    }

    /// Return an object with methods to manipulate this URL's path segments.
    pub fn path_segments_mut(&mut self) -> url::PathSegmentsMut {
        self.0.path_segments_mut().expect("`Uri` is always-a-base")
    }

    /// Parse the URL's query string, if any, as application/x-www-form-urlencoded and return an iterator of (key, value) pairs.
    pub fn query_pairs(&self) -> url::form_urlencoded::Parse {
        self.0.query_pairs()
    }

    /// Manipulate this URL's query string, viewed as a sequence of name/value pairs in application/x-www-form-urlencoded syntax.
    pub fn query_pairs_mut(&mut self) -> url::form_urlencoded::Serializer<url::UrlQuery> {
        self.0.query_pairs_mut()
    }

    /// Change this URL's path.
    pub fn set_path(&mut self, path: &str) {
        self.0.set_path(path)
    }

    /// Change this URL's query string.
    pub fn set_query(&mut self, query: Option<&str>) {
        self.0.set_query(query)
    }

    /// Change this URL's fragment identifier.
    pub fn set_fragment(&mut self, fragment: Option<&str>) {
        self.0.set_fragment(fragment)
    }

    /// Modify the path inline.
    pub fn with_path(mut self, path: &str) -> Self {
        self.set_path(path);
        self
    }

    /// Modify the fragment inline.
    pub fn with_query(mut self, query: Option<&str>) -> Self {
        self.set_query(query);
        self
    }

    /// Modify the fragment inline.
    pub fn with_fragment(mut self, fragment: Option<&str>) -> Self {
        self.set_fragment(fragment);
        self
    }

    /// Modify the path segments inline.
    pub fn with_path_segments_mut<F>(mut self, cls: F) -> Self
    where
        F: for<'a, 'b> Fn(&'b mut url::PathSegmentsMut<'a>) -> &'b mut url::PathSegmentsMut<'a>,
    {
        {
            let mut path_segments_mut = self.path_segments_mut();
            cls(&mut path_segments_mut);
        }
        self
    }

    /// Modify the query pairs inline.
    pub fn with_query_pairs_mut<F>(mut self, cls: F) -> Self
    where
        F: for<'a, 'b> Fn(
            &'b mut url::form_urlencoded::Serializer<'a, url::UrlQuery<'a>>,
        )
            -> &'b mut url::form_urlencoded::Serializer<'a, url::UrlQuery<'a>>,
    {
        {
            let mut query_pairs_mut = self.query_pairs_mut();
            cls(&mut query_pairs_mut);
        }
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {

        let uri = Uri::default()
            .with_path_segments_mut(|p| p.push("foo"));

        assert_eq!("Uri(\"/foo\")", format!("{:?}", &uri));

        let mut uri = Uri::new("../../../foo.html?lorem=ipsum").unwrap();

        assert_eq!("/foo.html?lorem=ipsum", uri.as_str());

        uri.query_pairs_mut().clear().append_pair("foo", "bar & baz");

        assert_eq!("/foo.html?foo=bar+%26+baz", uri.as_str());

        let mut uri = Uri::default();

        uri.path_segments_mut().extend(&["foo", "bar", "baz"]);

        assert_eq!("/foo/bar/baz", uri.as_str());

        let uri = uri.join("/baz").unwrap();

        assert_eq!("/baz", uri.as_str());

        let mut uri = uri.join("?foo=bar").unwrap();

        assert_eq!("/baz?foo=bar", uri.as_str());

        uri.path_segments_mut().clear();

        assert_eq!("/?foo=bar", uri.as_str());

        let uri = Uri::default()
            .with_path_segments_mut(|p| p.extend(&["foo", "bar"]))
            .with_query_pairs_mut(|q| q.append_pair("foo", "bar"))
            .with_fragment(Some("baz"));

        assert_eq!("/foo/bar?foo=bar#baz", uri.as_str());

        let uri = Uri::default().with_path("/a/b/c/d/e/f/g");

        let uri = uri.join("../../../../..").unwrap();

        assert_eq!("/a/", uri.as_str());

    }
}
