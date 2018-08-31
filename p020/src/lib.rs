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

#[derive(Debug)]
struct Node {
    value: u64,
    next: Option<Rc<Node>>,
}

fn find_intersecting_node<'b>(a: &Rc<Node>, b: &'b Option<Rc<Node>>) -> Option<&'b Rc<Node>> {
    match b {
        None => return None,
        Some(rc) => {
            if Rc::ptr_eq(a, rc) {
                return Some(rc);
            } else {
                return find_intersecting_node(a, &rc.next);
            }
        }
    }
}

fn find_intersection<'b>(a: &List, b: &'b List) -> Option<&'b Rc<Node>> {
    if a.head.is_none() {
        return None;
    }
    let mut next = &a.head;
    while next.is_some() {
        let next_rc = next.as_ref().unwrap();
        let intersection = find_intersecting_node(&next_rc, &b.head);
        if intersection.is_some() {
            return intersection;
        }
        next = &next_rc.next;
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
        let b = List {
            head: Some(Rc::new(Node {
                value: 99,
                next: Some(Rc::new(Node {
                    value: 1,
                    next: Some(c.clone()),
                })),
            })),
        };
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
