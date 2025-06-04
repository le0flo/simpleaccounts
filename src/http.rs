pub fn get_header_value(request: &actix_web::HttpRequest, header_name: &'static str) -> Result<String, ()> {
    let _header = actix_web::http::header::HeaderName::from_static(header_name);

    return match request.headers().get(_header) {
        Some(value) => Ok(String::from(value.to_str().unwrap())),
        None => Err(()),
    };
}
