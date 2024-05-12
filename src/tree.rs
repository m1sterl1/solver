use std::{
    fmt::Display,
    io::{stdin, Stdin},
    rc::Rc,
};

use crate::utils::{read_index, read_index_range};
use crate::utils::GroupsHM;

pub enum Kind {
    Intersect(usize), // choice with cost
    Word(String, GroupsHM<i32>),
    Root,
}

pub struct Node {
    cost: usize, // life cost
    kind: Kind,
    children: Vec<Rc<Node>>,
}

impl Node {
    pub fn new_word(word: &str, group: &GroupsHM<i32>) -> Self {
        Self {
            kind: Kind::Word(word.to_string(), group.clone()),
            children: vec![],
            cost: 0,
        }
    }

    pub fn new_intersection(choice: usize) -> Self {
        Self {
            kind: Kind::Intersect(choice),
            children: vec![],
            cost: 0,
        }
    }
    pub fn new_root() -> Self {
        Self {
            kind: Kind::Root,
            children: vec![],
            cost: 0,
        }
    }

    // Set children
    pub fn set_children(&mut self, children: Vec<Node>) {
        for child in children {
            self.append_child(child)
        }
    }

    /// Append child to node, update cost value
    pub fn append_child(&mut self, child: Node) {
        if let Kind::Word(_, _) = self.kind {
            self.cost = if child.cost < self.cost {
                self.cost
            } else {
                child.cost + 1
            }
        } else {
            self.cost = self.cost.max(child.cost);
        }
        self.children.push(Rc::new(child));
    }

    fn children(&self) -> &[Rc<Node>] {
        &self.children
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let node = match &self.kind {
            Kind::Word(word, _) => format!("Word {} cost {}", word, self.cost),
            Kind::Intersect(i) => format!("Intersect {} cost {}", i, self.cost),
            Kind::Root => format!("Root, cost {}", self.cost),
        };
        write!(f, "{node}")
    }
}

pub struct Tree {
    root: Rc<Node>,
    current: Rc<Node>,
    stdin: Stdin,
}

impl Tree {
    pub fn new(root: Node) -> Self {
        let root = Rc::new(root);
        let current = root.clone();
        Self {
            root,
            current,
            stdin: stdin(),
        }
    }
    pub fn print(&self) {
        self.print_ident(&self.root, 0);
    }

    fn print_ident(&self, node: &Rc<Node>, ident: usize) {
        println!("{}{}", "\t".repeat(ident), node);
        let ident = ident + 1;
        for child in node.children() {
            self.print_ident(child, ident);
        }
    }

    pub fn run(&mut self) {
        self.next();
    }

    fn process_intersect(&mut self) {
        println!("Word list:");
        for (i, node) in self.current.children.iter().enumerate() {
            if let Kind::Word(word, groups) = &node.kind {
                println!("{:2}. {}, max life cost {}, groups {:?}", i, word, node.cost, groups);
            }
            // let mut canditates = self.current.children.clone();
            // canditates.sort_by(|n1, n2|n1.cost.partial_cmp(&n2.cost));
            // let min_cost = canditates.first().unwrap().cost;
            // let candidates = canditates.into_iter().filter(|n|n.cost == min_cost).collect::<Vec<_>>();
        }
        println!("Choose:");
        let max_index = self.current.children.len();
        let index = read_index(&self.stdin, max_index);
        self.current = self.current.children[index].clone();
        if let Kind::Word(word, _) = &self.current.kind {
            println!("Choice: {}", word);
        }
        self.next();
    }

    fn next(&mut self) {
        match &self.current.kind {
            Kind::Root => {
                self.process_intersect();
            }
            Kind::Intersect(_) => self.process_intersect(),
            Kind::Word(_, _) => {
                println!("Write number of intercections.");
                let mut possible = vec![];
                for node in &self.current.children {
                    if let Kind::Intersect(i) = node.kind {
                        possible.push(i);
                    }
                }
                println!("Possible values: {:?}", possible);
                let intersection = read_index_range(&self.stdin, &possible);
                self.current = self
                    .current
                    .children
                    .iter()
                    .find(|ch| {
                        if let Kind::Intersect(i) = ch.kind {
                            i == intersection
                        } else {
                            false
                        }
                    })
                    .unwrap()
                    .clone();
                self.next()
            }
        }
    }
}

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use super::{Node, Tree};

    #[test]
    fn print() {
        let mut root = Node::new_root();
        let word = Node::new_word("hello", &HashMap::from([(0, vec![0,1])]));
        let mut intersection = Node::new_intersection(0);
        intersection.append_child(word);
        let word = Node::new_word("one", &HashMap::from([(0, vec![0,1])]));
        intersection.append_child(word);
        root.append_child(intersection);

        let tree = Tree::new(root);
        tree.print();
    }
}
