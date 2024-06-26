#[derive(Debug)]
pub struct StrSplit<'a, D> {
    remainder: Option<&'a str>,
    delimiter: D,
}

impl<'a, D> StrSplit<'a, D> {
    pub fn new(haystack: &'a str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'a, D> Iterator for StrSplit<'a, D>
where
    D: Delimiter,
{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;

        if let Some((delim_start, delim_end)) = self.delimiter.find_next(&remainder) {
            let until_delimiter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // here 's' is the remainder and self is the delimiter
        s.find(self)
            .map(|start_index| (start_index, start_index + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // here 's' is the remainder and self is the delimiter
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    // let delim = &*format!("{}", c);

    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn until_char_test() {
        assert_eq!(until_char("hello world", 'o'), "hell");
    }

    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn tail() {
        let haystack = "a b c d ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
    }

    #[test]
    fn double_delim_test() {
        let haystack = "a b c d  ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "", ""]);
    }
}
