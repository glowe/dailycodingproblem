/*
   Good morning! Here's your coding interview problem for today.
   This problem was asked by Google.
   Given the root to a binary tree, implement serialize(root), which
   serializes the tree into a string, and deserialize(s), which
   deserializes the string back into the tree.
   For example, given the following Node class
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
   The following test should pass:
node = Node('root', Node('left', Node('left.left')), Node('right'))
assert deserialize(serialize(node)).left.left.val == 'left.left'
*/
use std::iter::Peekable;
use std::str::Chars;

#[derive(Default)]
pub struct Node {
    val: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn serialize(&self) -> String {
        let left_serialized = match &self.left {
            None => String::from("nil"),
            Some(left) => left.serialize(),
        };
        let right_serialized = match &self.right {
            None => String::from("nil"),
            Some(right) => right.serialize(),
        };
        format!(
            "(\"{}\" {} {})",
            self.val, left_serialized, right_serialized
        )
    }

    fn deserialize_rec(iter: &mut Peekable<Chars>) -> Result<Box<Self>, String> {
        macro_rules! expect {
            ($expected:expr) => {
                let c = iter.next();
                if c != Some($expected) {
                    let rest: String = iter.collect();
                    return Err(format!(
                        "Expected $expected, but encountered {:?} (rest: {}",
                        c, rest
                    ));
                }
            };
        }

        expect!('(');

        expect!('"');
        let mut val: String = String::new();
        loop {
            match iter.next() {
                Some('"') => break,
                Some(c) => val.push(c),
                None => return Err(String::from("Reached end of string prematurely")),
            }
        }
        expect!(' ');

        let left: Option<Box<Self>>;
        if iter.peek() == Some(&'(') {
            let left_rec = Self::deserialize_rec(iter);
            if left_rec.is_err() {
                return left_rec;
            }
            left = Some(left_rec.unwrap());
        } else if iter.peek() == Some(&'n') {
            iter.next();
            expect!('i');
            expect!('l');
            left = None;
        } else {
            return Err(String::from("Expected nil or ("));
        }

        expect!(' ');

        let right: Option<Box<Self>>;
        if iter.peek() == Some(&'(') {
            let right_rec = Self::deserialize_rec(iter);
            if right_rec.is_err() {
                return right_rec;
            }
            right = Some(right_rec.unwrap());
        } else if iter.peek() == Some(&'n') {
            iter.next();
            expect!('i');
            expect!('l');
            right = None;
        } else {
            return Err(String::from("Expected nil or ("));
        }

        expect!(')');

        Ok(Box::new(Self {
            val: val,
            left: left,
            right: right,
        }))
    }

    fn deserialize(string: &str) -> Result<Box<Self>, String> {
        let mut iter = string.chars().peekable();
        Self::deserialize_rec(&mut iter)
    }
}

mod tests {
    #[test]
    fn test_serialize() {
        use super::Node;
        let root = Node {
            val: String::from("root"),
            left: Some(Box::new(Node {
                val: String::from("left"),
                left: Some(Box::new(Node {
                    val: String::from("left.left"),
                    ..Default::default()
                })),
                ..Default::default()
            })),
            right: Some(Box::new(Node {
                val: String::from("right"),
                ..Default::default()
            })),
        };
        assert_eq!(
            root.serialize(),
            "(\"root\" (\"left\" (\"left.left\" nil nil) nil) (\"right\" nil nil))"
        );
        assert_eq!(
            Node::deserialize(&root.serialize())
                .unwrap()
                .left
                .unwrap()
                .left
                .unwrap()
                .val,
            "left.left"
        )
    }
}
