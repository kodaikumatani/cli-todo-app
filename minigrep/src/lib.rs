/// Search for lines containing `pattern` in `contents`.
/// Returns a vector of (line_number, line) tuples for matching lines.
/// Line numbers are 1-based.
pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(pattern))
        .map(|(i, line)| (i + 1, line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let contents = "safe, fast, productive.\npick three.";
        let results = search("fast", contents);
        assert_eq!(results, vec![(1, "safe, fast, productive.")]);
    }

    #[test]
    fn multiple_results() {
        let contents = "Hello\nworld\nHello world";
        let results = search("Hello", contents);
        assert_eq!(results, vec![(1, "Hello"), (3, "Hello world")]);
    }

    #[test]
    fn no_results() {
        let contents = "nothing here\nmove along";
        let results = search("missing", contents);
        assert!(results.is_empty());
    }
}
