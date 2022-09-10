// based on a comment here: https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370


use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Display;

fn main() {
    let mut root = BTree::new();


    root.insert(10);
    root.insert(2);
    root.insert(12);
    root.insert(4);
    root.insert(2);
    root.insert(99);
    root.insert(23);
    root.insert(1);


    println!("tree {:?}", &root);

    println!("IN ORDER");
    root.print_inorder();
    println!();

    println!("LEVEL ORDER");
    root.print_levelorder();
    println!();

    println!("POST ORDER");
    root.print_postorder();
    println!();

    println!("PRE ORDER");
    root.print_preorder();
    println!();

    println!("DONE");
}


#[derive(Debug)]
enum BTree<T: Ord + Display> {
    Leaf {
        v: T,
        l: Box<BTree<T>>,
        r: Box<BTree<T>>,
    },
    Empty,
}

impl<T: Ord + Display> BTree<T> {
    fn new() -> BTree<T> {
        BTree::Empty
    }

    fn insert(&mut self, nv: T) {
        match self {
            &mut BTree::Leaf { ref v, ref mut l, ref mut r } => {
                match nv.cmp(v) {
                    Ordering::Less => r.insert(nv),
                    Ordering::Greater => l.insert(nv),
                    _ => return
                }
            }
            &mut BTree::Empty => {
                *self = BTree::Leaf { v: nv, l: Box::new(BTree::Empty), r: Box::new(BTree::Empty) }
            }
        };
    }

    fn is_empty(&self) -> bool {
        match self {
            &BTree::Leaf { .. } => false,
            &BTree::Empty => true,
        }
    }

    fn find(&self, fv: T) -> bool {
        match self {
            &BTree::Leaf { ref v, ref l, ref r } => {
                match fv.cmp(v) {
                    Ordering::Less => r.find(fv),
                    Ordering::Greater => l.find(fv),
                    _ => true
                }
            }
            &BTree::Empty => false,
        }
    }

    fn print_preorder(&self) {
        match self {
            &BTree::Empty => return,
            &BTree::Leaf {
                ref v,
                ref l,
                ref r,
            } => {
                print!("{}, ", v);
                l.print_preorder();
                r.print_preorder();
            }
        };
    }
    fn print_postorder(&self) {
        match self {
            &BTree::Empty => return,
            &BTree::Leaf {
                ref v,
                ref l,
                ref r,
            } => {
                l.print_postorder();
                r.print_postorder();
                print!("{}, ", v);
            }
        };
    }

    fn print_inorder(&self) {
        match self {
            &BTree::Empty => return,
            &BTree::Leaf {
                ref v,
                ref l,
                ref r,
            } => {
                l.print_inorder();
                print!("{}, ", v);
                r.print_inorder();
            }
        };
    }
    fn get_left(&self) -> Option<&Box<BTree<T>>> {
        match self {
            &BTree::Empty => None,
            &BTree::Leaf {
                v: _,
                ref l,
                r: _,
            } => Some(l),
        }
    }

    fn get_right(&self) -> Option<&Box<BTree<T>>> {
        match self {
            &BTree::Empty => None,
            &BTree::Leaf {
                v: _,
                l: _,
                ref r,
            } => Some(r),
        }
    }
    fn get_data(&self) -> Option<&T> {
        match self {
            &BTree::Empty => None,
            &BTree::Leaf {
                ref v,
                l: _,
                r: _,
            } => Some(v),
        }
    }

    fn print_levelorder(&self) {
        match self {
            &BTree::Empty => return,
            &BTree::Leaf {
                v: _,
                l: _,
                r: _,
            } => {
                let mut q = VecDeque::new();
                q.push_front(self);
                while !q.is_empty() {
                    let node = q.pop_front().unwrap();
                    if let Some(v) = node.get_data() {
                        print!("{}, ", v);
                    }
                    if let Some(l) = node.get_left() {
                        q.push_back(l);
                    }
                    if let Some(r) = node.get_right() {
                        q.push_back(r);
                    }
                }
                println!();
            }
        }
    }
}
