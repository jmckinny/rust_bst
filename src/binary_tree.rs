use std::cmp::Ord;
use std::cmp::Ordering;
pub enum BST<T: Ord>{
    Leaf{
        data: T,
        left: Box<BST<T>>,
        right: Box<BST<T>>
    },
    Empty
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self{
        BST::Empty
    }

    pub fn size(&self) -> u32{
        match self {
            BST::Empty => 0,
            BST::Leaf{
                data: _,
                ref left,
                ref right,
            } => {1 + left.size() + right.size()}
        }
    }
    pub fn insert(&mut self, value:T){
        match self{
            BST::Empty => *self = BST::Leaf{
                data: value,
                left: Box::new(BST::Empty),
                right: Box::new(BST::Empty)
            },
            BST::Leaf {
                ref data,
                ref mut left,
                ref mut right,
            } => match value.cmp(data){
                Ordering::Less | Ordering::Equal => left.insert(value),
                Ordering::Greater => right.insert(value),
            }

        }
    }
    pub fn find(&self, value:T) -> Option<&T>{
        match self {
            BST::Empty => None,
            BST::Leaf{
                ref data,
                ref left,
                ref right,
            } => match value.cmp(data){
                Ordering::Less => left.find(value),
                Ordering::Greater => right.find(value),
                Ordering::Equal => Some(data),
            }
        }
    }
}
