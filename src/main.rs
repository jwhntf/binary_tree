use std::collections::VecDeque;
use std::fmt::Formatter;
use std::fmt::Display;
struct Node<T> {
    data: T,
    left_child: Option<Box<Node<T>>>,
    right_child: Option<Box<Node<T>>>,
}
impl<T: Display> std::fmt::Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.data)?;
        Ok(())
    }
}
impl<T> Node<T> {
    fn build_from_raw(data: T) -> Self {
        Self {
            data,
            left_child: None,
            right_child: None,
        }
    }
    fn build_whole(
        data: T,
        left_child: Option<Box<Node<T>>>,
        right_child: Option<Box<Node<T>>>,
    ) -> Self {
        Self {
            data,
            left_child,
            right_child,
        }
    }

    fn borrow_value(&self) -> &T {
        &self.data
    }

    fn get_left_child(&self) -> &Option<Box<Node<T>>> {
        &self.left_child
    }

    fn get_right_child(&self) -> &Option<Box<Node<T>>> {
        &self.right_child
    }

    fn set_left_child(&mut self, data: T) {
        if let Some(ref mut left_child) = self.left_child {
            left_child.data = data;
        } else {
            self.left_child = Some(Box::new(Node::build_from_raw(data)))
        }
    }
    fn set_right_child(&mut self, data: T) {
        if let Some(ref mut right_child) = self.right_child {
            right_child.data = data;
        } else {
            self.right_child = Some(Box::new(Node::build_from_raw(data)))
        }
    }

    fn is_leaf(&self) -> bool {
        self.left_child.is_none() && self.right_child.is_none()
    }
}

struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Display> BinaryTree<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn pre_order(&self, root: &Option<Box<Node<T>>>) {
        if let Some(node) = root {
            visit(node);
            self.pre_order(node.get_left_child());
            self.pre_order(node.get_right_child());
        }
    }

    fn in_order(&self, root: &Option<Box<Node<T>>>) {
        if let Some(node) = root {
            self.in_order(node.get_left_child());
            visit(node);
            self.in_order(node.get_right_child());
        }
    }

    fn post_order(&self, root: &Option<Box<Node<T>>>) {
        if let Some(node) = root {
            self.post_order(node.get_left_child());
            self.post_order(node.get_right_child());
            visit(node);
        }
    }

    fn pre_order_non_recursive(&self, root: &Option<Box<Node<T>>>) {
        let mut astack: Vec<&Option<Box<Node<T>>>> = Vec::new();
        let mut pointer = root;
        astack.push(&None);
        while let Some(node) = pointer {
            visit(node);
            if node.get_right_child().is_some() {
                astack.push(node.get_right_child());
            }
            if node.get_left_child().is_some() {
                pointer = node.get_left_child();
            } else {
                pointer = astack.pop().unwrap();
            }
        }
    }

    fn in_order_non_recursive(&self, root: &Option<Box<Node<T>>>) {
        let mut astack: Vec<&Option<Box<Node<T>>>> = Vec::new();
        let mut pointer = root;
        while !astack.is_empty() || pointer.is_some() {
            if let Some(node) = pointer {
                astack.push(pointer);
                pointer = node.get_left_child();
            }
            else {
                pointer = astack.pop().unwrap();
                // if let Some(node) = pointer {
                //     visit(node);
                //     pointer = node.get_right_child();
                // }
                let node = pointer.as_ref().unwrap();
                visit(node);
                pointer = node.get_right_child();
            }
        }
    }

    fn post_order_non_recursive(&self, root: &Option<Box<Node<T>>>) {
        let mut astack: Vec<StackElement<T>> = Vec::new();
        let mut pointer = root;
        while !astack.is_empty() || pointer.is_some() {
            while let Some(node) = pointer {
                astack.push(StackElement{pointer, tag: Tags::Left});
                pointer = node.get_left_child();
            }
            let mut element = astack.pop().unwrap();
            pointer = element.pointer;
            if element.tag == Tags::Left {
                element.tag = Tags::Right;
                pointer = pointer.as_ref().unwrap().get_right_child();
                astack.push(element);
            }
            else {
                visit(pointer.as_ref().unwrap());
                pointer = &None;
            }
        }
    }

    fn level_order(&self, root: &Option<Box<Node<T>>>) {
        let mut aqueue: VecDeque<&Option<Box<Node<T>>>> = VecDeque::new();
        let mut pointer = root;
        if pointer.is_some() {
            aqueue.push_back(pointer);
        }
        while !aqueue.is_empty() {
            pointer = aqueue.pop_front().unwrap();
            let node = pointer.as_ref().unwrap();
            visit(node);
            if node.get_left_child().is_some() {
                aqueue.push_back(node.get_left_child());
            }
            if node.get_right_child().is_some() {
                aqueue.push_back(node.get_right_child());
            }
        }
    }

}

#[derive(PartialEq)]
enum Tags {Left, Right}
struct StackElement<'a, T> {
    pointer: &'a Option<Box<Node<T>>>,
    tag: Tags,
}
fn visit<T: Display>(data: T) {
    println!("{}", data);
}
fn get_a_binarytree() -> BinaryTree<i64> {
    let tree: BinaryTree<i64> = BinaryTree {
        root: Some(Box::new(Node {
            data: 1,
            left_child: Some(Box::new(Node {
                data: 2,
                left_child: Some(Box::new(Node {
                    data: 4,
                    left_child: Some(Box::new(Node {
                        data: 8,
                        left_child: None,
                        right_child: None,
                    })),
                    right_child: Some(Box::new(Node {
                        data: 9,
                        left_child: None,
                        right_child: None,
                    })),
                })),
                right_child: Some(Box::new(Node {
                    data: 5,
                    left_child: Some(Box::new(Node {
                        data: 10,
                        left_child: None,
                        right_child: None,
                    })),
                    right_child: Some(Box::new(Node {
                        data: 11,
                        left_child: None,
                        right_child: None,
                    })),
                })),
            })),
            right_child: Some(Box::new(Node {
                data: 3,
                left_child: Some(Box::new(Node {
                    data: 6,
                    left_child: Some(Box::new(Node {
                        data: 12,
                        left_child: None,
                        right_child: None,
                    })),
                    right_child: Some(Box::new(Node {
                        data: 13,
                        left_child: None,
                        right_child: None,
                    })),
                })),
                right_child: Some(Box::new(Node {
                    data: 7,
                    left_child: Some(Box::new(Node {
                        data: 14,
                        left_child: None,
                        right_child: None,
                    })),
                    right_child: Some(Box::new(Node {
                        data: 15,
                        left_child: None,
                        right_child: None,
                    })),
                })),
            })),
        })),
    };
    tree
}
fn main() {
    let tree = get_a_binarytree();
    tree.level_order(&tree.root);
}
