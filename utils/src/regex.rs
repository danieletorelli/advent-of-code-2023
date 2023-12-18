use pcre2::bytes::Regex;

pub fn extract_groups<'a>(
    input: &'a str,
    regex: &Regex,
    groups: Vec<&'a str>,
) -> Vec<Option<&'a str>> {
    match regex.captures(input.as_bytes()).unwrap() {
        Some(captures) => groups
            .iter()
            .map(|name| {
                captures
                    .name(name)
                    .and_then(|m| std::str::from_utf8(m.as_bytes()).ok())
            })
            .collect(),
        None => vec![],
    }
}

pub fn extract_group<'a>(input: &'a str, regex: &Regex, group: &'a str) -> Option<&'a str> {
    extract_groups(input, regex, vec![group])[0]
}
