use std::{fmt::Display, rc::Rc};

pub enum Kind{
    Intersect(usize),       // choice with cost
    Word {
        s:String,           // word
        cost: usize},       // life cost,       
    Root
}

pub struct Node{
    kind: Kind,
    children: Vec<Rc<Node>>
}

impl Node{
    pub fn new_word(word:&str, cost: usize) -> Self{
        Self { 
            kind: Kind::Word{s: word.to_string(), cost}, 
            children: vec![]
        }
    }
    
    pub fn new_choice(choice:usize) -> Self{
        Self { kind: Kind::Intersect(choice), children: vec![]}
    }
    pub fn new_root(children: Vec<Node>) -> Self{
        let mut node = Self{kind: Kind::Root, children:vec![]};
        node.set_children(children);
        node
    }

    pub fn set_children(&mut self, children: Vec<Node>){
        let children = children
        .into_iter()
        .map(|ch|Rc::new(ch))
        .collect();
        self.children = children;
    }

    fn children(&self) -> &[Rc<Node>]{
        &self.children
    }
}

pub struct Tree{
    root: Rc<Node>,
    current: Rc<Node>,
}


