use nom::bytes::complete::{is_not, tag, take_while};
use nom::character::complete::{multispace0, space0, space1};
use nom::combinator::opt;
use nom::multi::separated_list;
use nom::sequence::{delimited, tuple};

use crate::types::Entry;

pub fn parse_cedict(src: &str) -> nom::IResult<&str, Vec<Entry>> {
    // consume a comment at the start of the file (and discard comment)
    // take_while ensures empty strings are still accepted
    // tuple means execute parsers sequentially and return
    // all results as a tuple.
    let (s, _) = opt(tuple((
        separated_list(tag("\n"), tuple((tag("#"), take_while(|x| x != '\n')))),
        multispace0, // zero or more of spaces, tabs, \r and \n 
    )))(src)?;
    // consume main content
    separated_list(tag("\n"), parse_line)(s)
}

fn parse_line(line: &str) -> nom::IResult<&str, Entry> {
    match tuple((
        is_not(" "), // consume until we reach a space
        space1,      // consume one or more spaces
        is_not(" "),
        space1,
        // consume [ then consume content until we reach a ].
        // Note: take_while accepts empty pinyin (some entries in CC Canto are marked [])
        delimited(tag("["), take_while(|x| x != ']'), tag("]")),
        space1,
        // optional jyutping
        opt(delimited(tag("{"), take_while(|x| x != '}'), tag("}"))),
        space0,
        // the rest of the line is the definition (and optional comment)
        // consume until we reach comment # or newline \n
        take_while(|x| x != '#' && x != '\n'),
        // optionally consume the comment.
        opt(tuple((tag("#"), is_not("\n")))),
    ))(line)
    {
        Ok((s, (trad, _, simp, _, pinyin, _, jyutping, _, def, _))) => {
            let entry = Entry::new(trad, simp, pinyin, jyutping.unwrap_or(""), def.trim(), 0);

            Ok((s, entry))
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use super::parse_cedict;
    use crate::types::Entry;
    #[test]
    fn test_line_comment() {
        let src = "好 好 [hao3] {hou2} |good|well| # a comment";
        assert_eq!(
            (
                "",
                vec![Entry::new("好", "好", "hao3", "hou2", "|good|well|", 0)]
            ),
            parse_cedict(src).unwrap()
        );
    }
    #[test]
    fn test_no_jyutping() {
        let src = "好 好 [hao3] |good|well|";
        assert_eq!(
            (
                "",
                vec![Entry::new("好", "好", "hao3", "", "|good|well|", 0)]
            ),
            parse_cedict(src).unwrap()
        );
    }
    #[test]
    fn test() {
        let src = "好 好 [hao3] {hou2} |good|well|   ";
        assert_eq!(
            (
                "",
                vec![Entry::new("好", "好", "hao3", "hou2", "|good|well|", 0)]
            ),
            parse_cedict(src).unwrap()
        );
    }
    #[test]
    fn test_header_comment() {
        let src = "# hi\n#\n# testing";
        assert_eq!(("", vec![]), parse_cedict(src).unwrap());
    }
    #[test]
    fn empty_pinyin() {
        let src = "好 好 [] {hou2} |good|well|";
        assert_eq!(
            (
                "",
                vec![Entry::new("好", "好", "", "hou2", "|good|well|", 0)]
            ),
            parse_cedict(src).unwrap()
        );
    }
    #[test]
    fn test_header_comment2() {
        let src = "# hi\n#\n# testing\n好 好 [hao3] {hou2} |good|well|";
        assert_eq!(
            (
                "",
                vec![Entry::new("好", "好", "hao3", "hou2", "|good|well|", 0)]
            ),
            parse_cedict(src).unwrap()
        );
    }
    #[test]
    fn test_list() {
        let src = "好 好 [hao3] {hou2} |good|well|\n一事 一事 [yi1 shi4] {jat1 si6} |A matter|";
        assert_eq!(
            (
                "",
                vec![
                    Entry::new("好", "好", "hao3", "hou2", "|good|well|", 0),
                    Entry::new("一事", "一事", "yi1 shi4", "jat1 si6", "|A matter|", 0)
                ]
            ),
            parse_cedict(src).unwrap()
        );
    }
}
