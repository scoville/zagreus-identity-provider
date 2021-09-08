use validator::ValidationError;

pub fn validate_terms_accepted(terms_accepted: &bool) -> Result<(), ValidationError> {
    if !terms_accepted {
        return Err(ValidationError::new("terms_not_accepted"));
    }

    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let password_chars = password.chars();

    if password.len() < 8 {
        return Err(ValidationError::new("password_too_short"));
    }

    if !password_chars.clone().any(char::is_lowercase) {
        return Err(ValidationError::new("must_contain_lower_cased_chars"));
    }

    if !password_chars.clone().any(char::is_uppercase) {
        return Err(ValidationError::new("must_contain_upper_cased_chars"));
    }

    if !password_chars.clone().any(char::is_numeric) {
        return Err(ValidationError::new("must_contain_numbers"));
    }

    Ok(())
}

/// This macro will automatically validate anything
/// (that implements the `validator::Validate` trait)
/// and return a 400 error if an error occured.
macro_rules! validate {
    ($s:ident) => {
        if let Err(validation_errors) = $s.validate() {
            let mut response = ::actix_web::HttpResponse::with_body(
                ::actix_web::http::StatusCode::BAD_REQUEST,
                ::actix_web::body::AnyBody::from_slice(
                    ::serde_json::to_string(validation_errors.errors())
                        .unwrap()
                        .as_bytes(),
                ),
            );

            let headers = response.headers_mut();

            headers.append(
                ::actix_web::http::header::CONTENT_TYPE,
                ::actix_web::http::HeaderValue::from_static("application/json"),
            );

            return Ok(response);
        };
    };
}

pub(crate) use validate;

#[cfg(test)]
mod tests {
    use validator::ValidationError;

    use super::validate_password;
    use super::validate_terms_accepted;

    #[test]
    fn it_validates_terms_accepted() {
        assert_eq!(validate_terms_accepted(&true), Ok(()));
        assert_eq!(
            validate_terms_accepted(&false),
            Err(ValidationError::new("terms_not_accepted"))
        );
    }

    #[test]
    fn it_validates_password() {
        assert_eq!(validate_password("Val1d_passw0rd"), Ok(()));
        assert_eq!(
            validate_password("1nVal1d"),
            Err(ValidationError::new("password_too_short"))
        );
        assert_eq!(
            validate_password("L00DPASSW0RD"),
            Err(ValidationError::new("must_contain_lower_cased_chars"))
        );
        assert_eq!(
            validate_password("s1lentpassw0rd"),
            Err(ValidationError::new("must_contain_upper_cased_chars"))
        );
        assert_eq!(
            validate_password("Boringpassword"),
            Err(ValidationError::new("must_contain_numbers"))
        );
    }
}
