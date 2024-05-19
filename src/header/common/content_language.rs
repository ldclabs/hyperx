use header::QualityItem;
use language_tags::LanguageTag;

header! {
    /// `Content-Language` header, defined in
    /// [RFC7231](https://tools.ietf.org/html/rfc7231#section-3.1.3.2)
    ///
    /// The `Content-Language` header field describes the natural language(s)
    /// of the intended audience for the representation.  Note that this
    /// might not be equivalent to all the languages used within the
    /// representation.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Content-Language = 1#language-tag
    /// ```
    ///
    /// # Example values
    ///
    /// * `da`
    /// * `mi, en`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate http;
    /// # extern crate hyperx;
    /// extern crate language_tags;
    /// # use hyperx::header::{ContentLanguage, qitem, TypedHeaders};
    /// #
    /// # fn main() {
    /// let mut headers = http::HeaderMap::new();
    /// headers.encode(
    ///     &ContentLanguage(vec![
    ///         qitem("en".parse().unwrap()),
    ///     ])
    /// );
    /// # }
    /// ```
    ///
    /// ```
    /// # extern crate http;
    /// # extern crate hyperx;
    /// extern crate language_tags;
    /// # use hyperx::header::{ContentLanguage, qitem, TypedHeaders};
    /// # fn main() {
    ///
    /// let mut headers = http::HeaderMap::new();
    /// headers.encode(
    ///     &ContentLanguage(vec![
    ///         qitem("da".parse().unwrap()),
    ///         qitem("en-GB".parse().unwrap()),
    ///     ])
    /// );
    /// # }
    /// ```
    (ContentLanguage, "Content-Language") => (QualityItem<LanguageTag>)+

    test_content_language {
        test_header!(test1, [b"da"]);
        test_header!(test2, [b"mi, en"]);
    }
}

standard_header!(ContentLanguage, CONTENT_LANGUAGE);
