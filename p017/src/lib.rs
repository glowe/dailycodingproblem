/*
Good morning! Here's your coding interview problem for today.

This problem was asked by Google.

Suppose we represent our file system by a string in the following manner:

The string "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext" represents:

dir
    subdir1
    subdir2
        file.ext

The directory dir contains an empty sub-directory subdir1 and a sub-directory subdir2 containing a 
file file.ext.

The string "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext" 
represents:

dir
    subdir1
        file1.ext
        subsubdir1
    subdir2
        subsubdir2
            file2.ext

The directory dir contains two sub-directories subdir1 and subdir2. subdir1 contains a file file1.ext and 
an empty second-level sub-directory subsubdir1. subdir2 contains a second-level sub-directory subsubdir2
containing a file file2.ext.

We are interested in finding the longest (number of characters) absolute path to a file within our file
system. For example, in the second example above, the longest absolute path is 
"dir/subdir2/subsubdir2/file2.ext", and its length is 32 (not including the double quotes).

Given a string representing the file system in the above format, return the length of the longest absolute 
path to a file in the abstracted file system. If there is no file in the system, return 0.

Note:

The name of a file contains at least a period and an extension.

The name of a directory or sub-directory will not contain a period.
*/
use std::cmp;
// Seems like a tree-based problem

fn count_leading_tabs(name: &str) -> usize {
    name.find(|c: char| c != '\t').unwrap_or(0)
}

#[derive(Debug, PartialEq)]
struct Dir {
    name: String,
    files: Vec<String>,
    subdirs: Vec<Dir>,
}

// Implement parse
impl Dir {
    fn from(spec: &str) -> Self {
        let lines: Vec<&str> = spec.lines().collect();
        Self::parse(&lines, 0)
    }

    fn parse(lines: &Vec<&str>, depth: usize) -> Self {
        let name = lines[0];
        let leading_tabs = count_leading_tabs(name);
        assert_eq!(count_leading_tabs(name), depth);
        let mut dir = Dir {
            name: name.chars().skip(depth).collect(),
            files: vec![],
            subdirs: vec![],
        };
        for i in 1..lines.len() {
            let name = lines[i];
            let leading_tabs = count_leading_tabs(name);
            if leading_tabs <= depth {
                break;
            }
            if leading_tabs > depth + 1 {
                continue;
            }
            if name.contains(".") {
                dir.files.push(name.chars().skip(depth + 1).collect());
            } else {
                dir.subdirs
                    .push(Self::parse(&lines[i..].to_vec(), depth + 1));
            }
        }
        dir
    }

    fn longest_path_len(&self) -> usize {
        let max_files_len = self
            .files
            .iter()
            .map(|name| name.len() as usize)
            .max()
            .unwrap_or(0);
        let max_subdirs_len = self
            .subdirs
            .iter()
            .map(|subdir| subdir.longest_path_len())
            .max()
            .unwrap_or(0);
        let max_child_len = cmp::max(max_files_len, max_subdirs_len);
        if max_child_len == 0 {
            0
        } else {
            // include 1 char for forward slash delim between dir path and children
            self.name.len() + 1 + max_child_len
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let filesystem = Dir {
            name: "dir".to_string(),
            files: vec![],
            subdirs: vec![
                Dir {
                    name: "subdir1".to_string(),
                    files: vec!["file1.ext".to_string()],
                    subdirs: vec![Dir {
                        name: "subsubdir1".to_string(),
                        files: vec![],
                        subdirs: vec![],
                    }],
                },
                Dir {
                    name: "subdir2".to_string(),
                    files: vec![],
                    subdirs: vec![Dir {
                        name: "subsubdir2".to_string(),
                        files: vec!["file2.ext".to_string()],
                        subdirs: vec![],
                    }],
                },
            ],
        };
        // TODO: this should really be a parse and return a Result
        let parsed = Dir::from("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext");
        assert_eq!(filesystem, parsed);
        assert_eq!(filesystem.longest_path_len(), 32);
    }
}
