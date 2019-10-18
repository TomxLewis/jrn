
pub trait Deliminated<T> {
    fn deliminate(self) -> Option<Vec<T>>;
}

static DELIMINATORS: [char; 6] = [',', '_', '-', '/', '\\', ' '];

pub trait IsDeliminator {
    fn is_deliminator(&self) -> bool;
}

impl IsDeliminator for char {
    fn is_deliminator(&self) -> bool {
        DELIMINATORS.contains(&self)
    }
}

impl <'a> Deliminated<&'a str> for &'a str {

    fn deliminate(self) -> Option<Vec<&'a str>> {
        let mut indices: Vec<usize> = Vec::new();
        for (i, char) in self.char_indices() {
            if char.is_deliminator() {
                indices.push(i);
            }
        }

        if indices.is_empty() {
            return Some(vec!(self))
        }

        let mut v = Vec::new();
        let mut prev_i = 0;
        for i in indices {
            if prev_i == 0 {
                v.push(&self[0..i])
            } else {
                v.push(&self[prev_i+1..i]);
            }
            prev_i = i;
        }

        if prev_i < self.len() {
            v.push(&self[prev_i+1..])
        }

        let mut result = Vec::new();
        for s in v {
            if let Some(char) = s.chars().next() {
                if char.is_deliminator() {
                    result.push(&s[1..])
                } else {
                    result.push(s);
                }
            }
        }
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deliminate_space() {
        let s = "+one two three";
        let d = s.deliminate().unwrap();
        assert_eq!(d, vec!("+one", "two", "three"));
    }

    #[test]
    fn leading_deliminator() {
        let s = " one two three";
        let d = s.deliminate().unwrap();
        assert_eq!(d, vec!("one", "two", "three"));
    }

    #[test]
    fn single_no_delim() {
        let s = "+one";
        let d = s.deliminate().unwrap();
        assert_eq!(d, vec!("+one"));
    }

    #[test]
    fn single_leading_delim() {
        let s = ",one";
        let d = s.deliminate().unwrap();
        assert_eq!(d, vec!("one"));
    }

    #[test]
    fn single_value_multi_delim() {
        let s = ",one,";
        let d = s.deliminate().unwrap();
        assert_eq!(d, vec!("one"));
    }

    #[test]
    fn multi_value_multi_delim() {
        let s = "one,,two, three";
        let d = s.deliminate().unwrap();
        assert_eq!(d, vec!("one", "two", "three"));
    }
}