fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>().chars().next().unwrap()
            } else {
                c.to_uppercase().collect::<String>().chars().next().unwrap()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

        data.iter().for_each(|(a, b)| {
            assert_eq!(invert_the_case(a.to_string()), b.to_string());
            assert_eq!(invert_the_case(b.to_string()), a.to_string());
        });
    }
}
