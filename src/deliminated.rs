struct Deliminator;

pub trait Deliminated<T> {
    fn deliminate(self) -> Option<Vec<T>>;
}

static DELIMINATORS: [char; 6] = [',', '_', '-', '/', '\\', ' '];

trait IsDeliminator {
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

        let mut v = Vec::new();
        let mut prev_i = 0;
        for i in indices {
            if i > prev_i {
                v.push(&self[prev_i..i-1]);
                prev_i = i+1;
            }
        }

        let mut result = Vec::new();
        for s in v {
            if s != "" {
                result.push(s);
            }
        }
        Some(result)
    }
}

