use header::{parsing, Header, RawLike};
use std::fmt;

/// `Referrer-Policy` header, part of
/// [Referrer Policy](https://www.w3.org/TR/referrer-policy/#referrer-policy-header)
///
/// The `Referrer-Policy` HTTP header specifies the referrer
/// policy that the user agent applies when determining what
/// referrer information should be included with requests made,
/// and with browsing contexts created from the context of the
/// protected resource.
///
/// # ABNF
///
/// ```text
/// Referrer-Policy: 1#policy-token
/// policy-token   = "no-referrer" / "no-referrer-when-downgrade"
///                  / "same-origin" / "origin"
///                  / "origin-when-cross-origin" / "unsafe-url"
/// ```
///
/// # Example values
///
/// * `no-referrer`
///
/// # Example
///
/// ```
/// # extern crate http;
/// use hyperx::header::{ReferrerPolicy, TypedHeaders};
///
/// let mut headers = http::HeaderMap::new();
/// headers.encode(&ReferrerPolicy::NoReferrer);
/// ```
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ReferrerPolicy {
    /// `no-referrer`
    NoReferrer,
    /// `no-referrer-when-downgrade`
    NoReferrerWhenDowngrade,
    /// `same-origin`
    SameOrigin,
    /// `origin`
    Origin,
    /// `origin-when-cross-origin`
    OriginWhenCrossOrigin,
    /// `unsafe-url`
    UnsafeUrl,
    /// `strict-origin`
    StrictOrigin,
    ///`strict-origin-when-cross-origin`
    StrictOriginWhenCrossOrigin,
}

impl Header for ReferrerPolicy {
    fn header_name() -> &'static str {
        static NAME: &str = "Referrer-Policy";
        NAME
    }

    fn parse_header<'a, T>(raw: &'a T) -> ::Result<ReferrerPolicy>
    where
        T: RawLike<'a>,
    {
        use self::ReferrerPolicy::*;
        // See https://www.w3.org/TR/referrer-policy/#determine-policy-for-token
        let headers: Vec<String> = parsing::from_comma_delimited(raw)?;

        for h in headers.iter().rev() {
            let slice = &h.to_ascii_lowercase()[..];
            match slice {
                "no-referrer" | "never" => return Ok(NoReferrer),
                "no-referrer-when-downgrade" | "default" => return Ok(NoReferrerWhenDowngrade),
                "same-origin" => return Ok(SameOrigin),
                "origin" => return Ok(Origin),
                "origin-when-cross-origin" => return Ok(OriginWhenCrossOrigin),
                "strict-origin" => return Ok(StrictOrigin),
                "strict-origin-when-cross-origin" => return Ok(StrictOriginWhenCrossOrigin),
                "unsafe-url" | "always" => return Ok(UnsafeUrl),
                _ => continue,
            }
        }

        Err(::Error::Header)
    }

    fn fmt_header(&self, f: &mut ::header::Formatter) -> fmt::Result {
        f.fmt_line(self)
    }
}

impl fmt::Display for ReferrerPolicy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ReferrerPolicy::*;
        f.write_str(match *self {
            NoReferrer => "no-referrer",
            NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
            SameOrigin => "same-origin",
            Origin => "origin",
            OriginWhenCrossOrigin => "origin-when-cross-origin",
            StrictOrigin => "strict-origin",
            StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
            UnsafeUrl => "unsafe-url",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ReferrerPolicy;
    use header::{Header, Raw};

    #[test]
    fn test_parse_header() {
        let r: Raw = "origin".into();
        let a: ReferrerPolicy = Header::parse_header(&r).unwrap();
        let b = ReferrerPolicy::Origin;
        assert_eq!(a, b);

        let r: Raw = "foobar".into();
        let e: ::Result<ReferrerPolicy> = Header::parse_header(&r);
        assert!(e.is_err());
    }

    #[test]
    fn test_rightmost_header() {
        let r: Raw = "same-origin, origin, foobar".into();
        let a: ReferrerPolicy = Header::parse_header(&r).unwrap();
        let b = ReferrerPolicy::Origin;
        assert_eq!(a, b);
    }
}

standard_header!(ReferrerPolicy, REFERRER_POLICY);
