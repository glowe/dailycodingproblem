#[derive(Default, Clone)]
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

pub fn count_unival_exponential(tree: &Tree) -> u64 {
    let count = if is_unival(&tree) { 1 } else { 0 };
    count
        + tree
            .left
            .as_ref()
            .map_or(0, |left| count_unival_exponential(left))
        + tree
            .right
            .as_ref()
            .map_or(0, |right| count_unival_exponential(right))
}

pub fn count_unival_linear(tree: &Tree) -> u64 {
    fn recur(root: &Option<Box<Tree>>) -> (u64, bool) {
        match root {
            None => (0, true),
            Some(root) => {
                let (left_count, is_left_unival) = recur(&root.left);
                let (right_count, is_right_unival) = recur(&root.right);
                let total_count = left_count + right_count;

                if !is_left_unival || !is_right_unival {
                    return (total_count, false);
                }

                if root.left.is_some() {
                    if root.value != root.left.as_ref().unwrap().value {
                        return (total_count, false);
                    }
                }

                if root.right.is_some() {
                    if root.value != root.right.as_ref().unwrap().value {
                        return (total_count, false);
                    }
                }

                return (total_count + 1, true);
            }
        }
    }
    let (count, _) = recur(&Some(Box::new(tree.clone())));
    count
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
        assert_eq!(count_unival_exponential(&tree), 3);
        assert_eq!(count_unival_linear(&tree), 3);
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
        assert_eq!(count_unival_exponential(&tree), 2);
        assert_eq!(count_unival_linear(&tree), 2);
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
        assert_eq!(count_unival_exponential(&tree), 1);
        assert_eq!(count_unival_linear(&tree), 1);
    }

}
