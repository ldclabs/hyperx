use header::HttpDate;

header! {
    /// `Date` header, defined in [RFC7231](http://tools.ietf.org/html/rfc7231#section-7.1.1.2)
    ///
    /// The `Date` header field represents the date and time at which the
    /// message was originated.
    ///
    /// # ABNF
    ///
    /// ```text
    /// Date = HTTP-date
    /// ```
    ///
    /// # Example values
    ///
    /// * `Tue, 15 Nov 1994 08:12:31 GMT`
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate http;
    /// use hyperx::header::{Date, TypedHeaders};
    /// use std::time::SystemTime;
    ///
    /// let mut headers = http::HeaderMap::new();
    /// headers.encode(&Date(SystemTime::now().into()));
    /// ```
    (Date, "Date") => [HttpDate]

    test_date {
        test_header!(test1, [b"Tue, 15 Nov 1994 08:12:31 GMT"]);
    }
}

bench_header!(imf_fixdate, Date, {
    vec![b"Mon, 07 Nov 1994 08:48:37 GMT".to_vec()]
});
bench_header!(rfc_850, Date, {
    vec![b"Sunday, 06-Nov-94 08:49:37 GMT".to_vec()]
});
bench_header!(asctime, Date, {
    vec![b"Sun Nov  6 08:49:37 1994".to_vec()]
});

standard_header!(Date, DATE);
