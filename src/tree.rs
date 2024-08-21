use std::{fmt::Display, rc::Rc};

use crate::utils::GroupsHM;

pub type Answer = Vec<(String, usize)>; // Answer from solver, word, cost
pub type Guess = (String, usize); // (word, intercection)

enum Kind {
    Intersect(usize), // choice with cost
    Word(String, GroupsHM<i32>),
    Root,
}

pub(crate) struct Node {
    cost: usize, // life cost
    kind: Kind,
    children: Vec<Rc<Node>>,
}

impl Node {
    pub(crate) fn new_word(word: &str, group: &GroupsHM<i32>) -> Self {
        Self {
            kind: Kind::Word(word.to_string(), group.clone()),
            children: vec![],
            cost: 0,
        }
    }

    pub(crate) fn new_intersection(choice: usize) -> Self {
        Self {
            kind: Kind::Intersect(choice),
            children: vec![],
            cost: 0,
        }
    }
    pub(crate) fn new_root() -> Self {
        Self {
            kind: Kind::Root,
            children: vec![],
            cost: 0,
        }
    }

    // Set children
    pub(crate) fn set_children(&mut self, children: Vec<Node>) {
        for child in children {
            self.append_child(child)
        }
    }

    /// Append child to node, update cost value
    pub(crate) fn append_child(&mut self, child: Node) {
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

pub(crate) struct Tree {
    root: Rc<Node>,
    current: Rc<Node>,
}

impl Tree {
    pub(crate) fn new(root: Node) -> Self {
        let root = Rc::new(root);
        let current = root.clone();
        Self { root, current }
    }
    fn print(&self) {
        self.print_ident(&self.root, 0);
    }

    fn print_ident(&self, node: &Rc<Node>, ident: usize) {
        println!("{}{}", "\t".repeat(ident), node);
        let ident = ident + 1;
        for child in node.children() {
            self.print_ident(child, ident);
        }
    }

    /// Propagate state two steps and print answer for the
    /// Intercection node
    pub fn next_answer(&mut self, guess: &Guess) -> Answer {
        self.next(guess);
        self.next(guess);
        self.answer()
    }

    /// Return Answer, current node must be Root or Interction
    pub fn answer(&self) -> Answer {
        let mut answer = Answer::default();
        match &self.current.kind {
            Kind::Root | Kind::Intersect(_) => {
                for node in self.current.children.iter() {
                    if let Kind::Word(word, _) = &node.kind {
                        answer.push((word.clone(), node.cost));
                    }
                }
            }
            _ => {}
        };
        answer
    }

    /// Propagate inner state one step
    fn next(&mut self, guess: &Guess) {
        let (word, num_inter) = guess;
        match &self.current.kind {
            Kind::Intersect(_) | Kind::Root => {
                // search children: word == child.word
                self.current = self
                    .current
                    .children()
                    .iter()
                    .find(|&node| {
                        if let Kind::Word(w, _) = &node.kind {
                            w == word
                        } else {
                            false
                        }
                    })
                    .unwrap()
                    .clone();
            }
            Kind::Word(_, _) => {
                self.current = self
                    .current
                    .children
                    .iter()
                    .find(|&ch| {
                        if let Kind::Intersect(i) = &ch.kind {
                            i == num_inter
                        } else {
                            false
                        }
                    })
                    .unwrap()
                    .clone();
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
        let word = Node::new_word("hello", &HashMap::from([(0, vec![0, 1])]));
        let mut intersection = Node::new_intersection(0);
        intersection.append_child(word);
        let word = Node::new_word("one", &HashMap::from([(0, vec![0, 1])]));
        intersection.append_child(word);
        root.append_child(intersection);

        let tree = Tree::new(root);
        tree.print();
    }
}
