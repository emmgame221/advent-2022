use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn print_solution() {
    let input = File::open("inputs/input7.txt").unwrap();
    let lines = BufReader::new(input).lines();
    let lines: Vec<String> = lines.map(|x| x.unwrap()).collect();
    let dirs = build_directory_tree(&lines);
    println!("Day 7 Part 1: {}", size_of_removable(&dirs));
    let total_size = dirs[0].total_size(&dirs);
    let goal = 30000000 - (70000000 - total_size);
    let mut smallest = usize::MAX;
    for dir in dirs.iter() {
        let size = dir.total_size(&dirs);
        if size >= goal && size < smallest {
            smallest = size;
        }
    }
    println!("Day 7 Part 2: {}", smallest);
}

fn build_directory_tree(lines: &Vec<String>) -> Vec<AocDirectory> {
    let root = AocDirectory {
        dirs: Vec::new(),
        files: Vec::new(),
        name: "/".to_owned(),
        parent_idx: 0,
        idx: 0,
    };
    let mut dirs: Vec<AocDirectory> = vec![root];
    let mut cur_dir = 0;
    let mut next_idx = 1;
    for line in lines.iter().skip(1) {
        if line.starts_with("$ cd") {
            let target = line.strip_prefix("$ cd ").unwrap();
            let target_idx = if target == ".." {
                dirs[cur_dir].parent_idx
            } else if dirs[cur_dir].has_subdir(target, &dirs) {
                dirs[cur_dir]
                    .get_directories(&dirs)
                    .iter()
                    .find(|d| d.name == target)
                    .unwrap()
                    .idx
            } else {
                let newdir = AocDirectory {
                    dirs: Vec::new(),
                    files: Vec::new(),
                    name: target.to_owned(),
                    parent_idx: dirs[cur_dir].idx,
                    idx: next_idx,
                };
                let idx = newdir.idx;
                next_idx += 1;
                dirs[cur_dir].dirs.push(newdir.idx);
                dirs.push(newdir);
                idx
            };
            cur_dir = target_idx;
        } else if line.starts_with("$ ls") {
            // do nothing
        } else if line.starts_with("dir") {
            let name = line.strip_prefix("dir ").unwrap().to_owned();
            let parent_idx = dirs[cur_dir].idx;
            let idx = next_idx;
            next_idx += 1;
            let newdir = AocDirectory {
                dirs: Vec::new(),
                files: Vec::new(),
                name,
                parent_idx,
                idx,
            };
            dirs[cur_dir].dirs.push(newdir.idx);
            dirs.push(newdir);
        } else {
            let (left, right) = line.split_once(" ").unwrap();
            let size: usize = left.parse().unwrap();
            let name = right.to_owned();
            dirs[cur_dir].files.push(AocFile { _name: name, size });
        }
    }
    dirs
}

fn size_of_removable(dirs: &Vec<AocDirectory>) -> usize {
    let mut tot = 0;
    for dir in dirs {
        let size = dir.total_size(dirs);
        if size <= 100000 {
            tot += size;
        }
    }
    tot
}

#[derive(Clone, Debug)]
struct AocDirectory {
    dirs: Vec<usize>,
    files: Vec<AocFile>,
    name: String,
    parent_idx: usize,
    idx: usize,
}

impl AocDirectory {
    fn total_size(&self, dirs: &Vec<AocDirectory>) -> usize {
        let size = self
            .get_directories(dirs)
            .iter()
            .fold(0, |s, dir| s + dir.total_size(dirs))
            + self.files.iter().fold(0, |s, f| s + f.size);
        size
    }

    fn has_subdir(&self, name: &str, dirs: &Vec<AocDirectory>) -> bool {
        for i in self.dirs.iter() {
            if dirs[*i].name == name {
                return true;
            }
        }
        false
    }

    fn get_directories<'a>(&'a self, dirs: &'a Vec<AocDirectory>) -> Vec<&'a AocDirectory> {
        let mut own_dirs = Vec::new();
        for dir in self.dirs.iter() {
            own_dirs.push(&dirs[*dir]);
        }
        own_dirs
    }
}

#[derive(Clone, Debug)]
struct AocFile {
    _name: String,
    size: usize,
}
