use std::fmt::Display;

use crate::utils::{Groups, GroupsStruct};

#[derive(Clone)]
pub struct Metric{
    // Number of groups
    g_number: usize,
    // Max(group_elements) - Min(group_elements)
    delta: usize,
    // groups
    groups: GroupsStruct<i32>
}

impl Metric{
    pub fn from_group(mut groups: GroupsStruct<i32>) -> Self{
        groups.groups_mut().remove(&-1);
        let g = groups.groups();
        let g_number = g.len();
        let delta = g.values().map(|v|v.len()).max().unwrap() 
        - g.values().map(|v|v.len()).min().unwrap();
        Self { g_number, delta, groups}
        
    }
    pub fn groups(&self) -> &GroupsStruct<i32>{
        &self.groups
    }
}

impl Display for Metric{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Number: {}, Delta: {}, Groups: {:?}", self.g_number, self.delta, self.groups.groups())
    }
}

impl PartialEq for Metric{
    fn eq(&self, other: &Self) -> bool {
        (self.g_number,self.delta) == (other.g_number, other.delta)
    }
}

// (ng, delta)
// (4, 2) > (3, 1)
// (3,1) > (3, 2) 
impl PartialOrd for Metric{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.g_number == other.g_number{
            return <usize as PartialOrd<usize>>::partial_cmp(&self.delta, &other.delta).map(|o|o.reverse())
        }
        else{
            return <usize as PartialOrd<usize>>::partial_cmp(&self.g_number, &other.g_number)
        }
    }
}

#[cfg(test)]
mod test{
    use crate::metric::{GroupsStruct, Metric};

    use super::Groups;

    #[test]
    fn test_main(){
        let mut g1 = vec![0, 0, 1, -1, 2, 1].into_iter().groups();
        let mut g2 = vec![0, 0, 0, -1, 2, 1].into_iter().groups();
        println!("{:?}\n{:?}", g1.groups(), g2.groups());
        let m1 = Metric::from_group(g1);
        let m2 = Metric::from_group(g2);
        println!("M1 {m1}, M2 {m2}, {:?}", m1.partial_cmp(&m2));
        
    }
}


