// source: https://developerlife.com/2022/02/24/rust-non-binary-tree/
// the problem is, that struct "Shape" should be Clone, but that does not work due the Parent and Children type
// i guess the RwLock is not clone, so there is that

use core::fmt::Debug;
use std::{
    fmt::{self},
    ops::Deref,
    sync::{Arc, RwLock, Weak},
};


fn main() {
    let root = Node::new( ShapeEnum::Sphere(DummyShape { x: 1.0, y: -1.0 }));
    let child_node1 = Node::new(ShapeEnum::Cube(DummyShape { x: 1.0, y: 1.0 }));
    root.add_child_and_update_its_parent(&child_node1);
    let child_node2 = Node::new(ShapeEnum::Cylinder(DummyShape { x: 1.0, y: 2.0 }));
    let child_node3 = Node::new(ShapeEnum::Plane(DummyShape { x: 1.0, y: 3.0 }));
    root.add_child_and_update_its_parent(&child_node1);
    root.add_child_and_update_its_parent(&child_node2);
    root.add_child_and_update_its_parent(&child_node3);

    let child_node11 = Node::new(ShapeEnum::Plane(DummyShape { x: 2.0, y: 1.0 }));
    let child_node12 = Node::new(ShapeEnum::Cube(DummyShape { x: 2.0, y: 2.0 }));
    let child_node13 = Node::new(ShapeEnum::Triangle(DummyShape { x: 2.0, y: 3.0 }));

    child_node1.add_child_and_update_its_parent(&child_node11);
    child_node1.add_child_and_update_its_parent(&child_node12);
    child_node1.add_child_and_update_its_parent(&child_node13);

    let child_node21 = Node::new( ShapeEnum::Sphere(DummyShape { x: 1.0, y: -1.0 }));
    child_node13.add_child_and_update_its_parent(&child_node21);

    println!("{:#?}: {:#?}", "[tree]", root); // Pretty print.
}




type NodeDataRef = Arc<Shape>;
type WeakNodeNodeRef= Weak<Shape>;
/// Parent relationship is one of non-ownership.
type Parent = RwLock<WeakNodeNodeRef>;
// not `RwLock<NodeDataRef<T>>` which would cause memory leak.
/// Children relationship is one of ownership.
type Children= RwLock<Vec<Child>>;
type Child = NodeDataRef;

#[derive(Clone, PartialEq)]
pub struct Shape {
    value: ShapeEnum,
    parent: Parent,
    children: Children,
    casts_shadow: bool,
}

#[derive(Clone, PartialEq)]
pub enum ShapeEnum {
    Sphere(DummyShape),
    Plane(DummyShape),
    Cube(DummyShape),
    Cylinder(DummyShape),
    Triangle(DummyShape),
    //   Group(Group),
}


#[derive(Clone, PartialEq)]
pub struct DummyShape {
    x: f64,
    y: f64,
}

impl DummyShape {
    pub fn new () -> DummyShape{
        DummyShape{
            x: 0.0,
            y: 0.0
        }
    }
}


#[derive(Debug)]
pub struct Node {
    arc_ref: NodeDataRef
}

impl Node

{
    pub fn new(value: ShapeEnum) -> Node {
        let new_node = Shape {
            value,
            parent: RwLock::new(Weak::new()),
            children: RwLock::new(Vec::new()),
            casts_shadow: false,
        };
        let arc_ref = Arc::new(new_node);
        Node { arc_ref }
    }

    pub fn get_copy_of_internal_arc(self: &Self) -> NodeDataRef {
        Arc::clone(&self.arc_ref)
    }

    pub fn create_and_add_child(
        self: &Self,
        value: ShapeEnum,
    ) -> NodeDataRef{
        let new_child = Node::new(value);
        self.add_child_and_update_its_parent(&new_child);
        new_child.get_copy_of_internal_arc()
    }

    /// 🔏 Write locks used.
    pub fn add_child_and_update_its_parent(
        self: &Self,
        child: &Node,
    ) {
        {
            let mut my_children = self.arc_ref.children.write().unwrap();
            my_children.push(child.get_copy_of_internal_arc());
        } // `my_children` guard dropped.

        {
            let mut childs_parent = child.arc_ref.parent.write().unwrap();
            *childs_parent = Arc::downgrade(&self.get_copy_of_internal_arc());
        } // `my_parent` guard dropped.
    }

    pub fn has_parent(self: &Self) -> bool {
        self.get_parent().is_some()
    }

    /// 🔒 Read lock used.
    pub fn get_parent(self: &Self) -> Option<NodeDataRef> {
        let my_parent_weak = self.arc_ref.parent.read().unwrap();
        if let Some(my_parent_arc_ref) = my_parent_weak.upgrade() {
            Some(my_parent_arc_ref)
        } else {
            None
        }
    }
}

/// <https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types>
impl Deref for Node

{
    type Target = Shape;

    fn deref(&self) -> &Self::Target {
        &self.arc_ref
    }
}


impl fmt::Debug for Shape

{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let mut parent_msg = String::new();
        if let Some(parent) = self.parent.read().unwrap().upgrade() {
            parent_msg.push_str(format!("📦 {}", parent.value).as_str());
        } else {
            parent_msg.push_str("🚫 None");
        }
        f.debug_struct("Node")
            .field("value", &self.value)
            // .field("parent", &self.parent)
            .field("parent", &parent_msg)
            .field("children", &self.children)
            .finish()
    }
}


impl fmt::Debug for DummyShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Shape")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl fmt::Display for DummyShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Display for ShapeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ShapeEnum  {}", &"self".to_string())
    }
}

impl fmt::Debug for ShapeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ShapeEnum")
            .field("self", &"self".to_string())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{DummyShape, Node, ShapeEnum};

    #[test]
    fn test_tree_low_level_node_manipulation() {
        let cube = ShapeEnum::Cube(DummyShape::new());
        let child_node = Node::new(cube);

        {
            let sphere = ShapeEnum::Sphere(DummyShape::new());
            let parent_node = Node::new(sphere);
            parent_node.add_child_and_update_its_parent(&child_node);

            // The following is enabled by the `Deref` impl. `Node` has access to all the fields and methods
            // of `NodeData`.
            assert_eq!(parent_node.children.read().unwrap().len(), 1);
            assert!(parent_node.parent.read().unwrap().upgrade().is_none());
            // assert_eq!(parent_node.value, 5);
            assert_eq!(Arc::weak_count(&parent_node.arc_ref), 1);

            println!("{}: {:#?}", "[parent_node]", parent_node); // Pretty print.
            println!("{}: {:#?}", "[child_node]", child_node); // Pretty print.

            assert_eq!(Arc::strong_count(&child_node.get_copy_of_internal_arc()), 3); // `child_node` has 2 strong references.
            assert_eq!(Arc::weak_count(&child_node.get_copy_of_internal_arc()), 0);

            assert_eq!(
                Arc::strong_count(&parent_node.get_copy_of_internal_arc()),
                2
            ); // `parent_node` has 1 strong reference.
            assert_eq!(Arc::weak_count(&parent_node.get_copy_of_internal_arc()), 1); // `parent_node` also has 1 weak reference.

            assert!(child_node.has_parent());
            // assert_eq!(child_node.get_parent().unwrap().value, 5);
        } // `parent_node` is dropped here.

        // `child_node`'s parent is now `None`, its an orphan.
        assert!(!child_node.has_parent());
       //  assert_eq!(child_node.get_copy_of_internal_arc().value, 3);

        assert_eq!(Arc::strong_count(&child_node.get_copy_of_internal_arc()), 2); // `child_node` has 1 strong references.
        assert_eq!(Arc::weak_count(&child_node.get_copy_of_internal_arc()), 0); // `child_node` still has no weak references.
    }

// TODO: impl tree walking, find w/ comparator lambda, and print out the tree.
// TODO: impl delete, easy insert.
// TODO: impl nodelist (find multiple nodes) & return iterator.
// TODO: impl add siblings to node.

    #[test]
    fn test_tree_simple_api() {
        let cube = ShapeEnum::Cube(DummyShape::new());
        let root_node = Node::new(cube);
        // assert_eq!(root_node.get_copy_of_internal_arc().value, 5);

        {
            // ⚠️ In the following line, `Node` is not returned by `create_and_add_child()`. Instead a ref
            // (`Arc`) to the underlying `NodeData` is returned.
            let sphere = ShapeEnum::Sphere(DummyShape::new());
            let child_node_data_ref = root_node.create_and_add_child(sphere);
           //  assert_eq!(child_node_data_ref.value, 3);
            assert_eq!(
                root_node
                    .get_copy_of_internal_arc()
                    .children
                    .read()
                    .unwrap()
                    .len(),
                1
            );
            // assert_eq!(
            //     child_node_data_ref.value,
            //     root_node
            //         .get_copy_of_internal_arc()
            //         .children
            //         .read()
            //         .unwrap()[0]
            //         .value
            // );
        }
        println!("{}: {:#?}", "[tree]", root_node); // Pretty print.
    }
}
