use std::collections::HashMap;
use std::fmt::{self, Formatter};
use std::rc::Rc;
use std::{cell::RefCell, fmt::Debug};

use advent_of_code::helpers::Input;
use anyhow::{anyhow, Context, Result};

pub fn part_one(input: Input) -> Result<u32> {
    let fs = populate_fs(input)?;
    let sum: usize = fs_dir_sizes(&fs)?
        .iter()
        .skip(1)
        .filter(|s| **s <= 100000)
        .sum();
    Ok(sum as u32)
}

pub fn part_two(input: Input) -> Result<u32> {
    let fs = populate_fs(input)?;
    let mut sizes = fs_dir_sizes(&fs)?;
    sizes.sort();
    let total_size = sizes.last().context("no sizes")?;
    let needed = total_size - (70000000 - 30000000);
    sizes.iter().find(|s| **s >= needed).map(|i| *i as u32).context("no dir found")
}

fn populate_fs(input: Input) -> Result<Filesystem> {
    let mut fs = Filesystem::new();

    for l in input.as_str().lines() {
        let parts: Vec<&str> = l.split_whitespace().collect();
        match parts[0] {
            "$" => match parts[1] {
                "cd" => fs.cd(parts[2])?,
                "ls" => {}
                _ => return Err(anyhow!("command {:?} not recognized", parts[1])),
            },
            "dir" => fs.mkdir(parts[1])?,
            _ => fs.touch(parts[1], parts[0].parse()?)?,
        }
    }

    Ok(fs)
}

fn fs_dir_sizes(fs: &Filesystem) -> Result<Vec<usize>> {
    let mut ret = vec![];
    let mut dirs_todo = vec![fs.root.clone()];
    while !dirs_todo.is_empty() {
        let cur = dirs_todo.pop().context("bad pop")?;
        let curb = cur.borrow();
        let size = curb.get_size()?;
        ret.push(size);

        if let Some(c) = &curb.children {
            dirs_todo.extend(
                c.iter()
                    .map(|(_, v)| v.clone())
                    .filter(|n| n.borrow().is_dir()),
            );
        } else {
            return Err(anyhow!("directory without children"));
        };
    }
    Ok(ret)
}

struct Filesystem {
    root: Rc<RefCell<Node>>,
    cur: Rc<RefCell<Node>>,
}

impl Filesystem {
    fn new() -> Self {
        let _root = Rc::new(RefCell::new(Node::new_dir_node(None, "root")));
        Self {
            root: _root.clone(),
            cur: _root,
        }
    }

    fn mkdir(&mut self, name: &str) -> Result<()> {
        self.cur
            .borrow_mut()
            .add_child(Rc::new(RefCell::new(Node::new_dir_node(
                Some(self.cur.clone()),
                name,
            ))))
    }

    fn touch(&mut self, name: &str, size: usize) -> Result<()> {
        self.cur
            .borrow_mut()
            .add_child(Rc::new(RefCell::new(Node::new_file_node(
                self.cur.clone(),
                name,
                size,
            ))))
    }

    fn cd(&mut self, cmd: &str) -> Result<()> {
        self.cur = match cmd {
            "/" => self.root.clone(),
            ".." => self.cur.borrow().get_parent()?,
            _ => self.cur.borrow().child(cmd)?,
        };

        Ok(())
    }
}

struct Node {
    name: String,
    size: Option<usize>,
    parent: Option<Rc<RefCell<Node>>>,
    children: Option<HashMap<String, Rc<RefCell<Node>>>>,
}

impl Node {
    fn new_dir_node(parent: Option<Rc<RefCell<Node>>>, name: &str) -> Self {
        Self {
            name: name.to_string(),
            size: None,
            parent,
            children: Some(HashMap::new()),
        }
    }

    fn new_file_node(parent: Rc<RefCell<Node>>, name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size: Some(size),
            parent: Some(parent),
            children: None,
        }
    }

    fn is_dir(&self) -> bool {
        self.children.is_some()
    }

    fn add_child(&mut self, node: Rc<RefCell<Node>>) -> Result<()> {
        let children = self
            .children
            .as_mut()
            .context("not a directory, cannot add child")?;
        let name = node.borrow().name.clone();
        children.insert(name, node);
        Ok(())
    }

    fn get_parent(&self) -> Result<Rc<RefCell<Node>>> {
        self.parent.clone().context("No parent, root node")
    }

    fn child(&self, name: &str) -> Result<Rc<RefCell<Node>>> {
        if let Some(children) = &self.children {
            match children.get(name) {
                Some(node) => Ok(node.clone()),
                None => Err(anyhow!("Child not found")),
            }
        } else {
            Err(anyhow!("File has no children"))
        }
    }

    fn get_size(&self) -> Result<usize> {
        if let Some(children) = &self.children {
            let mut sum = 0;
            for (_, n) in children.iter() {
                sum += n.borrow().get_size()?;
            }
            Ok(sum)
        } else {
            self.size.context("file without size")
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(Input::new(&input)).unwrap(), 95437);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(Input::new(&input)).unwrap(), 24933642);
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, Input::new(input.as_str()));
    advent_of_code::solve!(2, part_two, Input::new(input.as_str()));
}
