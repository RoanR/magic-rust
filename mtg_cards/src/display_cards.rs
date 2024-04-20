#![deny(missing_docs)]
use std::fmt;

/// Divider out of a specified char
pub fn divider(max: usize, ch: char, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for _ in 0..max {
        write!(f, "{}", ch)?;
    }
    write!(f, "\n")?;
    Ok(())
}

/// Two columns with spaces used as padding between.
pub fn cols(left: &str, right: &str, max: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut pad = "".to_string();
    while pad.len() + left.len() + right.len() < max {
        pad += " ";
    }
    write!(f, "{}{}{}\n", left, pad, right)?;
    Ok(())
}

/// Wrap block of text to a line limit.
///
/// TODO: Wrap nicely around whole words
pub fn wrap(body: &str, max: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut count = 0;
    for ch in body.chars() {
        if ch == '\n' {
            count = 0;
            write!(f, "{}", ch)?;
        } else if count % max == 0 {
            write!(f, "\n{}", ch)?;
            count += 1;
        } else {
            write!(f, "{}", ch)?;
            count += 1;
        }
    }
    write!(f, "\n")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use fmt::Display;

    struct Foo {
        left: String,
        right: String,
        body: String,
        line: usize,
        div: char,
    }

    impl Display for Foo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            cols(&self.left, &self.right, self.line, f)?;
            wrap(&self.body, self.line, f)?;
            divider(self.line, self.div, f)?;
            Ok(())
        }
    }

    #[test]
    fn format_length() {
        let mut tester = Foo {
            left: "".to_owned(),
            right: "".to_owned(),
            body: "".to_owned(),
            line: 3,
            div: '*',
        };

        assert_eq!(format!("{tester}").len(), 9);
        tester.line = 0;
        assert_eq!(format!("{tester}").len(), 3);
        tester.line = 25;
        assert_eq!(format!("{tester}").len(), 53);
    }

    #[test]
    fn format_col() {
        let mut tester = Foo {
            left: "l".to_owned(),
            right: "r".to_owned(),
            body: "".to_owned(),
            line: 3,
            div: '.',
        };

        assert_eq!(&format!("{tester}")[..4], "l r\n");
        tester.line = 0;
        assert_eq!(&format!("{tester}")[..3], "lr\n");
        tester.line = 12;
        assert_eq!(&format!("{tester}")[..13], "l          r\n");
    }

    #[test]
    fn format_divider() {
        let mut tester = Foo {
            left: "".to_owned(),
            right: "".to_owned(),
            body: "".to_owned(),
            line: 3,
            div: '.',
        };

        assert_eq!(&format!("{tester}")[5..], "...\n");
        tester.line = 0;
        assert_eq!(&format!("{tester}"), "\n\n\n");
        tester.line = 5;
        tester.div = 'a';
        assert_eq!(&format!("{tester}")[..13], "     \n\naaaaa\n");
    }

    #[test]
    fn format_wrap() {
        let mut tester = Foo {
            left: "".to_owned(),
            right: "".to_owned(),
            body: "This is a test".to_owned(),
            line: 5,
            div: '.',
        };

        assert_eq!(&format!("{tester}"), "     \n\nThis \nis a \ntest\n.....\n");
        tester.line = 1;
        assert_eq!(&format!("{tester}")[..10], " \n\nT\nh\ni\ns");
        tester.line = 5;
    }
}
