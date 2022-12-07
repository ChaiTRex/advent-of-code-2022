use std::collections::BTreeMap;

use self::FileTree::*;
use self::Line::*;

fn main() {
    let mut filesystem = BTreeMap::new();
    let mut current_path = Vec::new();

    for line in include_str!("../../../day07.txt").lines().map(Line::new) {
        match line {
            ToRoot => {
                current_path.clear();
            }
            ToParent => {
                current_path.pop();
            }
            ToChild(name) => {
                current_path.push(name);
            }
            Ls => (),
            Dir(name) => {
                let mut cwd = &mut filesystem;
                for dir in current_path.iter().copied() {
                    cwd = match cwd.get_mut(dir).unwrap() {
                        Directory(_, _, contents) => contents,
                        _ => unreachable!(),
                    }
                }
                cwd.insert(name, Directory(name, 0, BTreeMap::new()));
            }
            Fil(name, size) => {
                let mut cwd = &mut filesystem;
                for dir in current_path.iter().copied() {
                    cwd = match cwd.get_mut(dir).unwrap() {
                        Directory(_, ref mut dir_size, contents) => {
                            *dir_size += size;
                            contents
                        }
                        _ => unreachable!(),
                    }
                }
                cwd.insert(name, File(name, size));
            }
        }
    }

    println!("Part 1: {}", part_1_fn(&filesystem));
    println!("Part 2: {}", part_2_fn(&filesystem));
}

fn part_1_fn<'a>(filesystem: &BTreeMap<&'a str, FileTree<'a>>) -> u64 {
    let mut total_size = 0;
    for entry in filesystem.values() {
        match entry {
            File(_, _) => (),
            Directory(_, size, subtree) => {
                if *size <= 100_000 {
                    total_size += size;
                }
                total_size += part_1_fn(subtree);
            }
        }
    }
    total_size
}

fn part_2_fn<'a>(filesystem: &BTreeMap<&'a str, FileTree<'a>>) -> u64 {
    fn go<'a>(filesystem: &BTreeMap<&'a str, FileTree<'a>>, deletion_needed: u64) -> u64 {
        let mut best_result = u64::MAX;
        for entry in filesystem.values() {
            match entry {
                File(_, _) => (),
                Directory(_, size, subtree) => {
                    if *size >= deletion_needed && *size < best_result {
                        best_result = *size;
                    }
                    let subtree_best = go(subtree, deletion_needed);
                    if subtree_best < best_result {
                        best_result = subtree_best;
                    }
                }
            }
        }
        best_result
    }

    let mut filesystem_size = 0;
    for root_entry in filesystem.values() {
        match root_entry {
            Directory(_, size, _) => filesystem_size += size,
            _ => (),
        }
    }
    go(filesystem, filesystem_size - 40_000_000)
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum FileTree<'a> {
    File(&'a str, u64),
    Directory(&'a str, u64, BTreeMap<&'a str, FileTree<'a>>),
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Line<'a> {
    ToRoot,
    ToParent,
    ToChild(&'a str),
    Ls,
    Dir(&'a str),
    Fil(&'a str, u64),
}

impl<'a> Line<'a> {
    fn new(line: &'a str) -> Line<'a> {
        match line.bytes().next().unwrap() {
            b'$' => {
                let line = &line[2..];
                if line == "ls" {
                    Ls
                } else if line == "cd .." {
                    ToParent
                } else if line == "cd /" {
                    ToRoot
                } else {
                    ToChild(&line[3..])
                }
            }
            b'0'..=b'9' => {
                let (size, name) = line.split_once(' ').unwrap();
                Fil(name, size.parse::<u64>().unwrap())
            }
            b'd' => Dir(&line[4..]),
            _ => unreachable!(),
        }
    }
}
