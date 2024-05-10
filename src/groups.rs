use std::collections::HashMap;
use std::hash::Hash;
struct GroupsStructure<T>{
    groups: HashMap<T, Vec<usize>>
}

impl<T> GroupsStructure<T>
where   T:Eq+Hash{
    fn new<I:Iterator<Item=T>>(i: I) -> Self{
        let mut groups = HashMap::new();
        for (i, el) in i.enumerate(){    
            groups.entry(el)
            .and_modify(|v:&mut Vec<usize>|v.push(i))
            .or_insert(vec![i]);
        }
        Self{groups}
    }

    fn groups(&self) -> &HashMap<T, Vec<usize>>{
        &self.groups
    }
}

trait Groups<T>
where   Self:Sized + Iterator<Item = T>,
        T: Eq + Hash{
    fn groups(self) -> GroupsStructure<T>{
        GroupsStructure::new(self)
    }
}

impl<T,I> Groups<T> for I
where   T: Hash+Eq, 
        I:Iterator<Item = T>{}

#[cfg(test)]
mod test{
    use super::Groups;

    #[test]
    fn test_main(){
        let v = vec![0, 0, 1, -1, 2, 1];
        let g = v.iter().groups();
        println!("{:?}", g.groups())
    }
}


