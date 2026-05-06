/// Deordinalizes a `&str`
///
/// Strips a single trailing ordinal suffix (`st`, `nd`, `rd`, `th`) only when
/// it is preceded by an ASCII digit. Inputs that look like decimals (contain a
/// `.`) are returned unchanged. Strings that do not match the
/// `<digit><suffix>` pattern are returned unchanged, so `deordinalize("south")`
/// returns `"south"` rather than `"sou"`.
///
/// ```
/// use cruet::number::deordinalize::deordinalize;
///
/// assert!(deordinalize("0.1") == "0.1");
/// assert!(deordinalize("-1st") == "-1");
/// assert!(deordinalize("0th") == "0");
/// assert!(deordinalize("1st") == "1");
/// assert!(deordinalize("2nd") == "2");
/// assert!(deordinalize("3rd") == "3");
/// assert!(deordinalize("9th") == "9");
/// assert!(deordinalize("12th") == "12");
/// assert!(deordinalize("12000th") == "12000");
/// assert!(deordinalize("12001th") == "12001");
/// assert!(deordinalize("12002nd") == "12002");
/// assert!(deordinalize("12003rd") == "12003");
/// assert!(deordinalize("12004th") == "12004");
/// assert!(deordinalize("3rd") == "3");
/// assert!(deordinalize("3rd") == "3");
/// assert!(deordinalize("") == "");
/// ```
///
/// Non-ordinal trailing characters are preserved:
/// ```
/// use cruet::number::deordinalize::deordinalize;
///
/// assert_eq!(deordinalize("south"), "south");
/// assert_eq!(deordinalize("ststst"), "ststst");
/// ```
pub fn deordinalize(non_ordinalized_string: &str) -> String {
    if non_ordinalized_string.contains('.') {
        return non_ordinalized_string.to_owned();
    }
    if let Some(stripped) = strip_one_ordinal_suffix(non_ordinalized_string) {
        return stripped.to_owned();
    }
    non_ordinalized_string.to_owned()
}

fn strip_one_ordinal_suffix(s: &str) -> Option<&str> {
    const SUFFIXES: [&str; 4] = ["st", "nd", "rd", "th"];
    for suffix in SUFFIXES {
        if let Some(prefix) = s.strip_suffix(suffix)
            && prefix.bytes().last().is_some_and(|b| b.is_ascii_digit())
        {
            return Some(prefix);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::deordinalize;

    #[test]
    fn strips_each_valid_suffix() {
        assert_eq!(deordinalize("1st"), "1");
        assert_eq!(deordinalize("2nd"), "2");
        assert_eq!(deordinalize("3rd"), "3");
        assert_eq!(deordinalize("4th"), "4");
        assert_eq!(deordinalize("12000th"), "12000");
    }

    #[test]
    fn preserves_non_ordinal_words() {
        assert_eq!(deordinalize("south"), "south");
        assert_eq!(deordinalize("band"), "band");
        assert_eq!(deordinalize("hard"), "hard");
        assert_eq!(deordinalize("nest"), "nest");
    }

    #[test]
    fn does_not_repeat_strip() {
        // Previously `trim_end_matches` would chain strip multiple times,
        // turning "ststst" into "" because "st" repeats.
        assert_eq!(deordinalize("ststst"), "ststst");
        // "1stst" — only one suffix is stripped, and the remaining "st"
        // is not preceded by a digit so it stays.
        assert_eq!(deordinalize("1stst"), "1stst");
    }

    #[test]
    fn preserves_decimal_input() {
        assert_eq!(deordinalize("0.1"), "0.1");
        assert_eq!(deordinalize("3.14th"), "3.14th");
    }

    #[test]
    fn empty_input() {
        assert_eq!(deordinalize(""), "");
    }
}
