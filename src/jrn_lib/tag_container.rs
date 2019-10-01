use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Default)]
pub struct TagContainer {
    inner: HashMap<String, u16>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct CountAndTag(pub u16, pub String);

impl Ord for CountAndTag {
    fn cmp(&self, other: &Self) -> Ordering {
        let count_ord = self.0.cmp(&other.0);
        let s_ord = self.1.cmp(&other.1).reverse();
        count_ord.then(s_ord)
    }
}

impl PartialOrd for CountAndTag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TagContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, tag: &str) {
        if let Some(n) = self.inner.get_mut(tag) {
            *n += 1;
        } else {
            let s = String::from(tag);
            self.inner.insert(s, 1);
        }
    }

    pub fn count(&self, tag: &str) -> Option<&u16> {
        self.inner.get(tag)
    }

    pub fn sorted(&self) -> Vec<CountAndTag> {
        let mut heap: BinaryHeap<CountAndTag> = BinaryHeap::new();
        for (key, count) in self.inner.iter() {
            let item = CountAndTag(*count, String::from(key));
            heap.push(item);
        }
        let mut vec = heap.into_sorted_vec();
        vec.reverse();
        vec
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_dup() {
        let mut con = TagContainer::new();
        con.insert(&"test");
        con.insert(&"test");
        assert_eq!(con.count(&"test"), Some(&2));
    }

    #[test]
    fn can_sort() {
        let mut con = TagContainer::new();
        con.insert(&"A");
        con.insert(&"B");
        con.insert(&"B");
        con.insert(&"C");
        for (i, ct) in con.sorted().iter().enumerate() {
            match i {
                0 => assert_eq!(ct.1, "B"),
                1 => assert_eq!(ct.1, "A"),
                2 => assert_eq!(ct.1, "C"),
                _ => unreachable!()
            }
        }
    }
}
