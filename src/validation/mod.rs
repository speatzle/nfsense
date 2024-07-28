use {
    crate::definitions::config::Config, garde::rules::pattern::Matcher, once_cell::sync::Lazy,
    regex::Regex,
};

const REGEX_NAME: &str = r"^[a-zA-Z0-9._/-]*$";

pub fn validate_name(value: &str, _: &Config) -> garde::Result {
    if value.len() > 32 {
        return Err(garde::Error::new("name is longer than 32"));
    }

    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX_NAME).unwrap());
    if !RE.is_match(value) {
        return Err(garde::Error::new("name must only contain a-zA-Z0-9._/-"));
    }
    Ok(())
}
