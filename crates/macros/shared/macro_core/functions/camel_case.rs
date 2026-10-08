pub fn to_camel_case_str(val: &str) -> String {
    val.split('_')
        .filter(|word| !word.is_empty())
        .flat_map(|word| {
            word.chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
        })
        .collect()
}
