use crate::headers::{Header, HeaderName, HeaderValue, Headers, IF_UNMODIFIED_SINCE};
use crate::utils::{fmt_http_date, parse_http_date};

use std::fmt::Debug;

use std::time::SystemTime;

/// Apply the HTTP method if the entity has not been modified after the
/// given date.
///
/// # Specifications
///
/// - [RFC 7232, section 3.4: If-Unmodified-Since](https://tools.ietf.org/html/rfc7232#section-3.4)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types_rs::Result<()> {
/// #
/// use http_types_rs::Response;
/// use http_types_rs::conditional::IfUnmodifiedSince;
/// use std::time::{SystemTime, Duration};
///
/// let time = SystemTime::now() + Duration::from_secs(5 * 60);
/// let expires = IfUnmodifiedSince::new(time);
///
/// let mut res = Response::new(200);
/// res.insert_header(&expires, &expires);
///
/// let expires = IfUnmodifiedSince::from_headers(res)?.unwrap();
///
/// // HTTP dates only have second-precision.
/// let elapsed = time.duration_since(expires.modified())?;
/// assert_eq!(elapsed.as_secs(), 0);
/// #
/// # Ok(()) }
/// ```
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct IfUnmodifiedSince {
    instant: SystemTime,
}

impl IfUnmodifiedSince {
    /// Create a new instance of `IfUnmodifiedSince`.
    pub fn new(instant: SystemTime) -> Self {
        Self { instant }
    }

    /// Returns the last modification time listed.
    pub fn modified(&self) -> SystemTime {
        self.instant
    }

    /// Create an instance of `IfUnmodifiedSince` from a `Headers` instance.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(IF_UNMODIFIED_SINCE) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let header = headers.iter().last().unwrap();

        let instant = parse_http_date(header.as_str())?;
        Ok(Some(Self { instant }))
    }
}

impl Header for IfUnmodifiedSince {
    fn header_name(&self) -> HeaderName {
        IF_UNMODIFIED_SINCE
    }
    fn header_value(&self) -> HeaderValue {
        let output = fmt_http_date(self.instant);

        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(output.into()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::headers::Headers;
    use std::time::Duration;

    #[test]
    fn smoke() -> crate::Result<()> {
        let time = SystemTime::now() + Duration::from_secs(5 * 60);
        let expires = IfUnmodifiedSince::new(time);

        let mut headers = Headers::new();
        expires.apply_header(&mut headers);

        let expires = IfUnmodifiedSince::from_headers(headers)?.unwrap();

        // HTTP dates only have second-precision
        let elapsed = time.duration_since(expires.modified())?;
        assert_eq!(elapsed.as_secs(), 0);
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() {
        let mut headers = Headers::new();
        headers.insert(IF_UNMODIFIED_SINCE, "<nori ate the tag. yum.>").unwrap();
        let err = IfUnmodifiedSince::from_headers(headers).unwrap_err();
        assert_eq!(err.status(), 400);
    }
}
