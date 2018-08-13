#[derive(Default)]
pub struct Tree {
    value: String,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

pub fn is_unival(tree: &Tree) -> bool {
    fn recur(tree: &Tree, value: &str) -> bool {
        if tree.value != value {
            return false;
        }
        if tree.left.is_some() {
            if !recur(tree.left.as_ref().unwrap(), &tree.value) {
                return false;
            }
        }
        if tree.right.is_some() {
            if !recur(tree.right.as_ref().unwrap(), &tree.value) {
                return false;
            }
        }
        true
    };
    recur(&tree, &tree.value)
}

pub fn count_unival(tree: &Tree) -> u64 {
    let count = if is_unival(&tree) { 1 } else { 0 };
    count
        + tree.left.as_ref().map_or(0, |left| count_unival(left))
        + tree.right.as_ref().map_or(0, |right| count_unival(right))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_unival_trees() {
        let tree = Tree {
            value: "a".to_string(),
            left: Some(Box::new(Tree {
                value: "a".to_string(),
                ..Default::default()
            })),
            right: Some(Box::new(Tree {
                value: "a".to_string(),
                ..Default::default()
            })),
        };
        assert!(is_unival(&tree));
        assert!(is_unival(&tree.left.as_ref().unwrap()));
        assert!(is_unival(&tree.right.as_ref().unwrap()));
        assert_eq!(count_unival(&tree), 3);
    }

    #[test]
    fn test_2_unival_trees() {
        let tree = Tree {
            value: "a".to_string(),
            left: Some(Box::new(Tree {
                value: "a".to_string(),
                ..Default::default()
            })),
            right: Some(Box::new(Tree {
                value: "a".to_string(),
                left: None,
                right: Some(Box::new(Tree {
                    value: "A".to_string(),
                    ..Default::default()
                })),
            })),
        };
        assert!(!is_unival(&tree));
        assert!(is_unival(&tree.left.as_ref().unwrap()));
        assert!(!is_unival(&tree.right.as_ref().unwrap()));
        assert!(is_unival(
            &tree.right.as_ref().unwrap().right.as_ref().unwrap()
        ));
        assert_eq!(count_unival(&tree), 2);
    }

    #[test]
    fn test_1_unival_tree() {
        let tree = Tree {
            value: "a".to_string(),
            left: Some(Box::new(Tree {
                value: "a".to_string(),
                left: Some(Box::new(Tree {
                    value: "A".to_string(),
                    ..Default::default()
                })),
                right: None,
            })),
            right: None,
        };
        assert!(!is_unival(&tree));
        assert!(!is_unival(&tree.left.as_ref().unwrap()));
        assert!(is_unival(
            &tree.left.as_ref().unwrap().left.as_ref().unwrap()
        ));
        assert_eq!(count_unival(&tree), 1);
    }

}
