use header::{Header, RawLike};
use std::fmt::{self, Display};
use std::str;
use unicase;

/// `Access-Control-Allow-Credentials` header, part of
/// [CORS](http://www.w3.org/TR/cors/#access-control-allow-headers-response-header)
///
/// > The Access-Control-Allow-Credentials HTTP response header indicates whether the
/// > response to request can be exposed when the credentials flag is true. When part
/// > of the response to an preflight request it indicates that the actual request can
/// > be made with credentials. The Access-Control-Allow-Credentials HTTP header must
/// > match the following ABNF:
///
/// # ABNF
///
/// ```text
/// Access-Control-Allow-Credentials: "Access-Control-Allow-Credentials" ":" "true"
/// ```
///
/// Since there is only one acceptable field value, the header struct does not accept
/// any values at all. Setting an empty `AccessControlAllowCredentials` header is
/// sufficient. See the examples below.
///
/// # Example values
/// * "true"
///
/// # Examples
///
/// ```
/// # extern crate http;
/// # extern crate hyperx;
/// # fn main() {
///
/// use hyperx::header::{AccessControlAllowCredentials, TypedHeaders};
///
/// let mut headers = http::HeaderMap::new();
/// headers.encode(&AccessControlAllowCredentials);
/// # }
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct AccessControlAllowCredentials;

const ACCESS_CONTROL_ALLOW_CREDENTIALS_TRUE: &str = "true";

impl Header for AccessControlAllowCredentials {
    fn header_name() -> &'static str {
        static NAME: &str = "Access-Control-Allow-Credentials";
        NAME
    }

    fn parse_header<'a, T>(raw: &'a T) -> ::Result<AccessControlAllowCredentials>
    where
        T: RawLike<'a>,
    {
        if let Some(line) = raw.one() {
            let text = unsafe {
                // safe because:
                // 1. we don't actually care if it's utf8, we just want to
                //    compare the bytes with the "case" normalized. If it's not
                //    utf8, then the byte comparison will fail, and we'll return
                //    None. No big deal.
                str::from_utf8_unchecked(line)
            };
            if unicase::eq_ascii(text, ACCESS_CONTROL_ALLOW_CREDENTIALS_TRUE) {
                return Ok(AccessControlAllowCredentials);
            }
        }
        Err(::Error::Header)
    }

    fn fmt_header(&self, f: &mut ::header::Formatter) -> fmt::Result {
        f.fmt_line(self)
    }
}

impl Display for AccessControlAllowCredentials {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.write_str("true")
    }
}

#[cfg(test)]
mod test_access_control_allow_credentials {
    use super::AccessControlAllowCredentials as HeaderField;
    use header::*;
    use std::str;
    test_header!(works, vec![b"true"], Some(HeaderField));
    test_header!(ignores_case, [b"True"]);
    test_header!(not_bool, vec![b"false"], None);
    test_header!(only_single, vec![b"true", b"true"], None);
    test_header!(
        no_gibberish,
        vec!["\u{645}\u{631}\u{62d}\u{628}\u{627}".as_bytes()],
        None
    );
}

standard_header!(
    AccessControlAllowCredentials,
    ACCESS_CONTROL_ALLOW_CREDENTIALS
);
