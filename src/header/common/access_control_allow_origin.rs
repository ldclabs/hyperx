use header::{Header, RawLike};
use std::fmt::{self, Display};
use std::str;

/// The `Access-Control-Allow-Origin` response header,
/// part of [CORS](http://www.w3.org/TR/cors/#access-control-allow-origin-response-header)
///
/// The `Access-Control-Allow-Origin` header indicates whether a resource
/// can be shared based by returning the value of the Origin request header,
/// `*`, or `null` in the response.
///
/// # ABNF
///
/// ```text
/// Access-Control-Allow-Origin = "Access-Control-Allow-Origin" ":" origin-list-or-null | "*"
/// ```
///
/// # Example values
/// * `null`
/// * `*`
/// * `http://google.com/`
///
/// # Examples
/// ```
/// # extern crate http;
/// use hyperx::header::{AccessControlAllowOrigin, TypedHeaders};
///
/// let mut headers = http::HeaderMap::new();
/// headers.encode(
///     &AccessControlAllowOrigin::Any
/// );
/// ```
/// ```
/// # extern crate http;
/// use hyperx::header::{AccessControlAllowOrigin, TypedHeaders};
///
/// let mut headers = http::HeaderMap::new();
/// headers.encode(
///     &AccessControlAllowOrigin::Null,
/// );
/// ```
/// ```
/// # extern crate http;
/// use hyperx::header::{AccessControlAllowOrigin, TypedHeaders};
///
/// let mut headers = http::HeaderMap::new();
/// headers.encode(
///     &AccessControlAllowOrigin::Value("http://hyper.rs".to_owned())
/// );
/// ```
#[derive(Clone, PartialEq, Debug)]
pub enum AccessControlAllowOrigin {
    /// Allow all origins
    Any,
    /// A hidden origin
    Null,
    /// Allow one particular origin
    Value(String),
}

impl Header for AccessControlAllowOrigin {
    fn header_name() -> &'static str {
        "Access-Control-Allow-Origin"
    }

    fn parse_header<'a, T>(raw: &'a T) -> ::Result<AccessControlAllowOrigin>
    where
        T: RawLike<'a>,
    {
        if let Some(line) = raw.one() {
            Ok(match line {
                b"*" => AccessControlAllowOrigin::Any,
                b"null" => AccessControlAllowOrigin::Null,
                _ => AccessControlAllowOrigin::Value(str::from_utf8(line)?.into()),
            })
        } else {
            Err(::Error::Header)
        }
    }

    fn fmt_header(&self, f: &mut ::header::Formatter) -> fmt::Result {
        f.fmt_line(self)
    }
}

impl Display for AccessControlAllowOrigin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            AccessControlAllowOrigin::Any => f.write_str("*"),
            AccessControlAllowOrigin::Null => f.write_str("null"),
            AccessControlAllowOrigin::Value(ref url) => Display::fmt(url, f),
        }
    }
}

#[cfg(test)]
mod test_access_control_allow_origin {
    use super::AccessControlAllowOrigin as HeaderField;
    use header::*;
    test_header!(test1, [b"null"]);
    test_header!(test2, [b"*"]);
    test_header!(test3, [b"http://google.com/"]);
}

standard_header!(AccessControlAllowOrigin, ACCESS_CONTROL_ALLOW_ORIGIN);
