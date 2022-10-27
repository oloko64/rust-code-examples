/// Parse the given string reference as json and return the result.
pub fn parse_to_json(s: &str) -> Result<serde_json::Value, serde_json::Error> {
    let mapped = serde_json::from_str(s)?;
    Ok(mapped)
}
