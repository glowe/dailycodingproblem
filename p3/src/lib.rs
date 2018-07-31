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
            None => "?".to_string(),
            Some(left) => left.serialize(),
        };
        let right_serialized = match &self.right {
            None => "?".to_string(),
            Some(right) => right.serialize(),
        };
        format!("\"{}\" {} {}", self.val, left_serialized, right_serialized)
    }

    fn parse_node_rec(iter: &mut Peekable<Chars>) -> Result<Option<Box<Self>>, String> {
        match iter.peek() {
            Some(&'"') => match Self::parse_node(iter) {
                Ok(node) => Ok(Some(node)),
                Err(err) => return Err(err),
            },
            Some(&'?') => {
                iter.next();
                Ok(None)
            }
            Some(&c) => {
                let rest: String = iter.collect();
                return Err(format!(
                    "Expected \" or ?, but encountered {} ({})",
                    c, rest
                ));
            }
            None => return Err("Expected \" or ?, but reached end of string instead".to_string()),
        }
    }

    fn parse_node(iter: &mut Peekable<Chars>) -> Result<Box<Self>, String> {
        macro_rules! expect {
            ($expected:expr) => {
                let c = iter.next();
                if c != Some($expected) {
                    let rest: String = iter.collect();
                    return Err(format!(
                        "Expected {}, but encountered {:?} (rest: {}",
                        $expected, c, rest
                    ));
                }
            };
        }

        expect!('"');
        let mut val: String = String::new();
        loop {
            match iter.next() {
                Some('"') => break,
                Some(c) => val.push(c),
                None => return Err("Reached end of string prematurely".to_string()),
            }
        }
        expect!(' ');

        let left = match Self::parse_node_rec(iter) {
            Ok(node) => node,
            Err(err) => return Err(err),
        };

        expect!(' ');

        let right = match Self::parse_node_rec(iter) {
            Ok(node) => node,
            Err(err) => return Err(err),
        };

        Ok(Box::new(Self {
            val: val,
            left: left,
            right: right,
        }))
    }

    fn deserialize(string: &str) -> Result<Box<Self>, String> {
        let mut iter = string.chars().peekable();
        Self::parse_node(&mut iter)
    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    #[test]
    fn test_serialize() {
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
            "\"root\" \"left\" \"left.left\" ? ? ? \"right\" ? ?"
        );
    }

    #[test]
    fn test_deserialize() {
        assert_eq!(
            Node::deserialize("\"root\" \"left\" \"left.left\" ? ? ? \"right\" ? ?")
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
