/*
Given two singly linked lists that intersect at some point, find the intersecting node. The lists are
non-cyclical.

For example, given A = 3 -> 7 -> 8 -> 10 and B = 99 -> 1 -> 8 -> 10, return the node with value 8.

In this example, assume nodes with the same value are the exact same node objects.

Do this in O(M + N) time (where M and N are the lengths of the lists) and constant space.
*/
use std::rc::Rc;

#[derive(Debug)]
struct List {
    head: Option<Rc<Node>>,
}

impl List {
    fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            Node::len(&self.head)
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

#[derive(Debug)]
struct Node {
    value: u64,
    next: Option<Rc<Node>>,
}

impl Node {
    fn len(next: &Option<Rc<Node>>) -> usize {
        let mut i = 0;
        let mut n = next;
        while n.is_some() {
            i += 1;
            n = &n.as_ref().unwrap().next;
        }
        i
    }

    fn skip(next: &Option<Rc<Node>>, skip: usize) -> &Option<Rc<Self>> {
        if next.is_none() {
            return next;
        }
        let mut next = next;
        for _ in 0..skip {
            next = &next.as_ref().unwrap().next;
            if next.is_none() {
                break;
            }
        }
        next
    }
}

fn find_intersection<'b>(a: &List, b: &'b List) -> Option<&'b Rc<Node>> {
    if a.is_empty() || b.is_empty() {
        return None;
    }

    let mut node_a = &a.head;
    let mut node_b = &b.head;

    // Discard prefix of longer list.
    let len_a = Node::len(node_a);
    let len_b = Node::len(node_b);
    if len_a > len_b {
        let n = len_a - len_b;
        node_a = Node::skip(node_a, n);
    } else if len_b > len_a {
        let n = len_b - len_a;
        node_a = Node::skip(node_b, n);
    }

    // Compare suffixes
    let len = len_a.min(len_b);
    for _ in 0..len {
        let rc_a = node_a.as_ref().unwrap();
        let rc_b = node_b.as_ref().unwrap();
        if Rc::ptr_eq(&rc_a, &rc_b) {
            // We return rc_b since we are using lifetime 'b
            return Some(rc_b);
        }
        // Advance both nodes
        node_a = Node::skip(node_a, 1);
        node_b = Node::skip(node_b, 1);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let c = Rc::new(Node {
            value: 8,
            next: Some(Rc::new(Node {
                value: 10,
                next: None,
            })),
        });
        let a = List {
            head: Some(Rc::new(Node {
                value: 3,
                next: Some(Rc::new(Node {
                    value: 7,
                    next: Some(c.clone()),
                })),
            })),
        };
        assert_eq!(a.len(), 4);
        let b = List {
            head: Some(Rc::new(Node {
                value: 99,
                next: Some(Rc::new(Node {
                    value: 1,
                    next: Some(c.clone()),
                })),
            })),
        };
        assert_eq!(b.len(), 4);
        // Verify that we are sharing the same node
        assert!(Rc::ptr_eq(
            &a.head
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap(),
            &b.head
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
        ));
        let intersection = find_intersection(&a, &b);
        assert!(Rc::ptr_eq(intersection.unwrap(), &c));
    }
}
