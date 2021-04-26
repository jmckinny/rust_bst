mod binary_tree;
extern crate rand;
fn main() {
    let mut tree: binary_tree::BST<i32> = binary_tree::BST::new();
    println!("{}", tree.size());
    tree.insert(5);
    println!("{}", tree.size());
    let x = rand::random::<u8>();
    let x = x as i32;
    for i in 1..x{
        tree.insert(i);
    }
    println!("{}", tree.size());
    println!("{:?}", tree.find(5));
}
