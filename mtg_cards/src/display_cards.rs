#![deny(missing_docs)]
use std::fmt;

/// Divider out of a specified char
pub fn divider(max: usize, ch: char, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for _ in 0..max {
        write!(f, "{}", ch)?;
    }
    Ok(())
}

/// Two columns with spaces used as padding between.
pub fn cols(left: &str, right: &str, max: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut pad = " ".to_string();
    while pad.len() + left.len() + right.len() < max {
        pad += " ";
    }
    write!(f, "\n{}{}{}\n", left, pad, right)?;
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
