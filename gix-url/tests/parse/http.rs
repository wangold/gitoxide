use gix_url::Scheme;

use crate::parse::{assert_url, assert_url_roundtrip, url, url_with_pass};

#[test]
fn username_expansion_is_unsupported() -> crate::Result {
    assert_url_roundtrip(
        "http://example.com/~byron/hello",
        url(Scheme::Http, None, "example.com", None, b"/~byron/hello"),
    )
}

#[test]
fn empty_user_cannot_roundtrip() -> crate::Result {
    let actual = gix_url::parse("http://@example.com/~byron/hello".into())?;
    let expected = url(Scheme::Http, None, "example.com", None, b"/~byron/hello");
    assert_eq!(actual, expected);
    assert_eq!(
        actual.to_bstring(),
        "http://example.com/~byron/hello",
        "we cannot differentiate between empty user and no user"
    );
    Ok(())
}

#[test]
fn username_and_password() -> crate::Result {
    assert_url_roundtrip(
        "http://user:password@example.com/~byron/hello",
        url_with_pass(Scheme::Http, "user", "password", "example.com", None, b"/~byron/hello"),
    )
}

#[test]
fn username_and_password_and_port() -> crate::Result {
    assert_url_roundtrip(
        "http://user:password@example.com:8080/~byron/hello",
        url_with_pass(Scheme::Http, "user", "password", "example.com", 8080, b"/~byron/hello"),
    )
}

#[test]
fn only_password() -> crate::Result {
    assert_url_roundtrip(
        "http://:password@example.com/~byron/hello",
        url_with_pass(Scheme::Http, "", "password", "example.com", None, b"/~byron/hello"),
    )
}

#[test]
fn username_and_empty_password() -> crate::Result {
    let actual = gix_url::parse("http://user:@example.com/~byron/hello".into())?;
    let expected = url(Scheme::Http, "user", "example.com", None, b"/~byron/hello");
    assert_eq!(actual, expected);
    assert_eq!(
        actual.to_bstring(),
        "http://user@example.com/~byron/hello",
        "an empty password appears like no password to us - fair enough"
    );
    Ok(())
}

#[test]
fn secure() -> crate::Result {
    assert_url_roundtrip(
        "https://github.com/byron/gitoxide",
        url(Scheme::Https, None, "github.com", None, b"/byron/gitoxide"),
    )
}

#[test]
fn http_missing_path() -> crate::Result {
    assert_url_roundtrip("http://host.xz/", url(Scheme::Http, None, "host.xz", None, b"/"))?;
    assert_url("http://host.xz", url(Scheme::Http, None, "host.xz", None, b"/"))?;
    Ok(())
}
