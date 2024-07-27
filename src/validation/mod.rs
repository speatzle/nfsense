use {
    crate::definitions::config::Config, garde::rules::pattern::Matcher, once_cell::sync::Lazy,
    regex::Regex,
};

pub fn validate_name(value: &str, _: &Config) -> garde::Result {
    if value.len() > 32 {
        return Err(garde::Error::new("name is longer than 32"));
    }

    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"/^[0-9A-Za-z_-]*$/g").unwrap());
    if !RE.is_match(value) {
        return Err(garde::Error::new("name must only contain 0-9A-Za-z_-"));
    }
    Ok(())
}
