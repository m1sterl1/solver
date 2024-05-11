use std::collections::HashMap;
use std::hash::Hash;
use std::iter::zip;

//////// Groups trait ////////

#[derive(Clone)]
pub struct GroupsStruct<T> {
    groups: HashMap<T, Vec<usize>>,
}

impl<T> GroupsStruct<T>
where
    T: Eq + Hash,
{
    pub fn new<I: Iterator<Item = T>>(i: I) -> Self {
        let mut groups = HashMap::new();
        for (i, el) in i.enumerate() {
            groups
                .entry(el)
                .and_modify(|v: &mut Vec<usize>| v.push(i))
                .or_insert(vec![i]);
        }
        Self { groups }
    }

    pub fn groups(&self) -> &HashMap<T, Vec<usize>> {
        &self.groups
    }

    pub fn groups_mut(&mut self) -> &mut HashMap<T, Vec<usize>> {
        &mut self.groups
    }
}

pub trait Groups<T>
where
    Self: Sized + Iterator<Item = T>,
    T: Eq + Hash,
{
    fn groups(self) -> GroupsStruct<T> {
        GroupsStruct::new(self)
    }
}

impl<T, I> Groups<T> for I
where
    T: Hash + Eq,
    I: Iterator<Item = T>,
{
}

//////// Groups trait ////////

/// Return number ofequal elements in the same position
/// for two strings
pub fn get_coincidence(s1: &str, s2: &str) -> usize {
    zip(s1.chars(), s2.chars()).fold(0, |acc, (c1, c2)| if c1.eq(&c2) { acc + 1 } else { acc })
}

#[cfg(test)]
mod test {
    use super::get_coincidence;

    #[test]
    fn test_compare() {
        let s1 = "processor";
        let s2 = "durasteel";
        assert_eq!(get_coincidence(s1, s2), 0);

        let s2 = "consisted";
        assert_eq!(get_coincidence(s1, s2), 1);
    }
}
