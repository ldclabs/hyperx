use header::internals::VecMap;
use header::{Header, RawLike};
use std::borrow::Cow;
use std::fmt;
use std::str::from_utf8;

/// `Cookie` header, defined in [RFC6265](http://tools.ietf.org/html/rfc6265#section-5.4)
///
/// If the user agent does attach a Cookie header field to an HTTP
/// request, the user agent must send the cookie-string
/// as the value of the header field.
///
/// When the user agent generates an HTTP request, the user agent MUST NOT
/// attach more than one Cookie header field.
///
/// # Example values
/// * `SID=31d4d96e407aad42`
/// * `SID=31d4d96e407aad42; lang=en-US`
///
/// Consider using the _cookie_ crate for parsing/decoding or encoding
/// cookie values.
///
/// # Example
/// ```
/// # extern crate http;
/// use hyperx::header::{Cookie, TypedHeaders};
///
/// let mut headers = http::HeaderMap::new();
/// let mut cookie = Cookie::new();
/// cookie.append("foo", "bar");
///
/// assert_eq!(cookie.get("foo"), Some("bar"));
///
/// headers.encode(&cookie);
/// ```
#[derive(Clone)]
pub struct Cookie(VecMap<Cow<'static, str>, Cow<'static, str>>);

impl Default for Cookie {
    fn default() -> Self {
        Self::new()
    }
}

impl Cookie {
    /// Creates a new `Cookie` header.
    pub fn new() -> Cookie {
        Cookie(VecMap::with_capacity(0))
    }

    /// Sets a name and value for the `Cookie`.
    ///
    /// # Note
    ///
    /// This will remove all other instances with the same name,
    /// and insert the new value.
    pub fn set<K, V>(&mut self, key: K, value: V)
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let key = key.into();
        let value = value.into();
        self.0.remove_all(&key);
        self.0.append(key, value);
    }

    /// Append a name and value for the `Cookie`.
    ///
    /// # Note
    ///
    /// Cookies are allowed to set a name with a
    /// a value multiple times. For example:
    ///
    /// ```
    /// use hyperx::header::Cookie;
    /// let mut cookie = Cookie::new();
    /// cookie.append("foo", "bar");
    /// cookie.append("foo", "quux");
    /// assert_eq!(cookie.to_string(), "foo=bar; foo=quux");
    pub fn append<K, V>(&mut self, key: K, value: V)
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        self.0.append(key.into(), value.into());
    }

    /// Get a value for the name, if it exists.
    ///
    /// # Note
    ///
    /// Only returns the first instance found. To access
    /// any other values associated with the name, parse
    /// the `str` representation.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(AsRef::as_ref)
    }

    /// Iterate cookies.
    ///
    /// Iterate cookie (key, value) in insertion order.
    ///
    /// ```
    /// use hyperx::header::Cookie;
    /// let mut cookie = Cookie::new();
    /// cookie.append("foo", "bar");
    /// cookie.append(String::from("dyn"), String::from("amic"));
    ///
    /// let mut keys = Vec::new();
    /// let mut values = Vec::new();
    /// for (k, v) in cookie.iter() {
    ///     keys.push(k);
    ///     values.push(v);
    /// }
    /// assert_eq!(keys, vec!["foo", "dyn"]);
    /// assert_eq!(values, vec!["bar", "amic"]);
    /// ```
    pub fn iter(&self) -> CookieIter {
        CookieIter(self.0.iter())
    }
}

impl Header for Cookie {
    fn header_name() -> &'static str {
        static NAME: &str = "Cookie";
        NAME
    }

    fn parse_header<'a, T>(raw: &'a T) -> ::Result<Cookie>
    where
        T: RawLike<'a>,
    {
        let mut vec_map = VecMap::with_capacity(raw.len());
        for cookies_raw in raw.iter() {
            let cookies_str = from_utf8(cookies_raw)?;
            for cookie_str in cookies_str.split(';') {
                let mut key_val = cookie_str.splitn(2, '=');
                let key_val = (key_val.next(), key_val.next());
                if let (Some(key), Some(val)) = key_val {
                    vec_map.insert(key.trim().to_owned().into(), val.trim().to_owned().into());
                }
            }
        }

        if vec_map.len() != 0 {
            Ok(Cookie(vec_map))
        } else {
            Err(::Error::Header)
        }
    }

    fn fmt_header(&self, f: &mut ::header::Formatter) -> fmt::Result {
        f.fmt_line(self)
    }
}

impl PartialEq for Cookie {
    fn eq(&self, other: &Cookie) -> bool {
        if self.0.len() == other.0.len() {
            for (k, v) in self.0.iter() {
                if other.get(k) != Some(v) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

impl fmt::Debug for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map()
            .entries(self.0.iter().map(|(k, v)| (k, v)))
            .finish()
    }
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iter = self.0.iter();
        if let Some((key, val)) = iter.next() {
            write!(f, "{}={}", key, val)?;
        }
        for (key, val) in iter {
            write!(f, "; {}={}", key, val)?;
        }
        Ok(())
    }
}

/// Iterator for cookie.
#[derive(Debug)]
pub struct CookieIter<'a>(::std::slice::Iter<'a, (Cow<'static, str>, Cow<'static, str>)>);

impl<'a> Iterator for CookieIter<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|kv| (kv.0.as_ref(), kv.1.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use super::Cookie;
    use header::{Header, Raw};

    #[test]
    fn test_set_and_get() {
        let mut cookie = Cookie::new();
        cookie.append("foo", "bar");
        cookie.append(String::from("dyn"), String::from("amic"));

        assert_eq!(cookie.get("foo"), Some("bar"));
        assert_eq!(cookie.get("dyn"), Some("amic"));
        assert!(cookie.get("nope").is_none());

        cookie.append("foo", "notbar");
        assert_eq!(cookie.get("foo"), Some("bar"));

        cookie.set("foo", "hi");
        assert_eq!(cookie.get("foo"), Some("hi"));
        assert_eq!(cookie.get("dyn"), Some("amic"));
    }

    #[test]
    fn test_eq() {
        let mut cookie = Cookie::new();
        let mut cookie2 = Cookie::new();

        // empty is equal
        assert_eq!(cookie, cookie2);

        // left has more params
        cookie.append("foo", "bar");
        assert_ne!(cookie, cookie2);

        // same len, different params
        cookie2.append("bar", "foo");
        assert_ne!(cookie, cookie2);

        // right has more params, and matching KV
        cookie2.append("foo", "bar");
        assert_ne!(cookie, cookie2);

        // same params, different order
        cookie.append("bar", "foo");
        assert_eq!(cookie, cookie2);
    }

    #[test]
    fn test_parse() {
        let mut cookie = Cookie::new();
        let r: Raw = b"foo=bar".to_vec().into();
        let parsed = Cookie::parse_header(&r).unwrap();
        cookie.append("foo", "bar");
        assert_eq!(cookie, parsed);

        let parsed = Cookie::parse_header(&r).unwrap();
        assert_eq!(cookie, parsed);

        let r: Raw = b"foo=bar; baz=quux".to_vec().into();
        let parsed = Cookie::parse_header(&r).unwrap();
        cookie.append("baz", "quux");
        assert_eq!(cookie, parsed);

        let r: Raw = b"foo=bar;; baz=quux".to_vec().into();
        let parsed = Cookie::parse_header(&r).unwrap();
        assert_eq!(cookie, parsed);

        let r: Raw = b"foo=bar; invalid ; bad; ;; baz=quux".to_vec().into();
        let parsed = Cookie::parse_header(&r).unwrap();
        assert_eq!(cookie, parsed);

        let r: Raw = b" foo  =    bar;baz= quux  ".to_vec().into();
        let parsed = Cookie::parse_header(&r).unwrap();
        assert_eq!(cookie, parsed);

        let r: Raw = vec![b"foo  =    bar".to_vec(), b"baz= quux  ".to_vec()].into();
        let parsed = Cookie::parse_header(&r).unwrap();
        assert_eq!(cookie, parsed);
        let r: Raw = b"foo=bar; baz=quux ; empty=".to_vec().into();
        let parsed = Cookie::parse_header(&r).unwrap();
        cookie.append("empty", "");
        assert_eq!(cookie, parsed);

        let mut cookie = Cookie::new();
        let r: Raw = b"middle=equals=in=the=middle".to_vec().into();
        let parsed = Cookie::parse_header(&r).unwrap();
        cookie.append("middle", "equals=in=the=middle");
        assert_eq!(cookie, parsed);

        let r: Raw = b"middle=equals=in=the=middle; double==2".to_vec().into();
        let parsed = Cookie::parse_header(&r).unwrap();
        cookie.append("double", "=2");
        assert_eq!(cookie, parsed);
    }
}

bench_header!(bench, Cookie, { vec![b"foo=bar; baz=quux".to_vec()] });

standard_header!(Cookie, COOKIE);
