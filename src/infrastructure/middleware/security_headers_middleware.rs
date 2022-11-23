use actix_web::{
    dev::ServiceResponse,
    http::header::{self, HeaderName, HeaderValue},
    Result,
};

pub fn add_security_headers(mut r: ServiceResponse) -> Result<ServiceResponse> {
    r.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );

    r.response_mut().headers_mut().insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static("default-src 'self'"),
    );

    r.response_mut().headers_mut().insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );

    r.response_mut().headers_mut().insert(
        header::X_XSS_PROTECTION,
        HeaderValue::from_static("1; mode=block"),
    );

    r.response_mut().headers_mut().insert(
        HeaderName::from_static("X-Permitted-Cross-Domain-Policies"),
        HeaderValue::from_static("none"),
    );

    Ok(r)
}
