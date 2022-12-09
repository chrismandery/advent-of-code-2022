use anyhow::{bail, Context, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

struct Dir {
    subdirs: HashMap<String, Dir>,
    files: HashMap<String, usize>
}

impl Dir {
    fn find_size_of_smallest_dir_to_delete(&self, total_disk_space: usize, required_disk_space: usize) -> Option<usize> {
        let free_disk_space = total_disk_space - self.get_total_dir_size();
        let required_to_free = required_disk_space - free_disk_space;
        let mut dir_sizes = self.gather_all_dir_sizes();
        dir_sizes.sort_unstable();
        dir_sizes.into_iter().filter(|s| *s >= required_to_free).next()
    }

    fn gather_all_dir_sizes(&self) -> Vec<usize> {
        let mut dir_sizes = vec!(self.get_total_dir_size());
        for subdir in self.subdirs.values() {
            dir_sizes.extend(subdir.gather_all_dir_sizes());
        }
        dir_sizes
    }

    fn get_total_dir_size(&self) -> usize {
        let size_subdirs: usize = self.subdirs.values().map(|d| d.get_total_dir_size()).sum();
        let size_files: usize = self.files.values().sum();
        size_subdirs + size_files
    }

    fn get_total_dir_size_if_below_threshold(&self, threshold: usize) -> usize {
        let own_size = self.get_total_dir_size();
        let subdir_sizes: usize = self.subdirs.values().map(|d| d.get_total_dir_size_if_below_threshold(threshold)).sum();
        if own_size <= threshold {
            own_size + subdir_sizes
        } else {
            subdir_sizes
        }
    }
}

fn main() -> Result<()> {
    let root = read_input_file("../inputs/day7_input.txt")?;
    println!("Sum of total sizes of dirs with size <= 100000 is: {}", root.get_total_dir_size_if_below_threshold(100000));
    println!("Size of smallest dir that would be sufficient to delete is: {}", root.find_size_of_smallest_dir_to_delete(70000000, 30000000).unwrap());

    Ok(())
}

fn read_input_file<P: AsRef<Path>>(input_path: P) -> Result<Dir> {
    let re_ls_output = Regex::new(r"^(dir|\d+) ([a-zA-Z.]+)$")?;
    let input = read_to_string(input_path)?;

    let mut root = Dir {
        subdirs: HashMap::new(),
        files: HashMap::new()
    };

    let mut cur_path: Vec<String> = vec!();

    for line in input.lines() {
        let mut cur_dir = &mut root;
        for path_element in cur_path.iter() {
            cur_dir = cur_dir.subdirs.get_mut(path_element).with_context(|| format!("Subdirectory \"{}\" not found!", path_element))?;
        }

        // If we do it that way, the parser does not have to be stateful (except storing the current dir)
        if line == "$ cd .." {
            if cur_path.is_empty() {
                bail!("Tried to cd .., but we are already in the root dir!");
            }

            cur_path.pop();
        } else if line == "$ cd /" {
            cur_path = vec!();
        } else if line.starts_with("$ cd ") {
            let name: String = line.chars().skip(5).collect();
            if !cur_dir.subdirs.contains_key(&name) {
                bail!("Tried to switch into subdir {} which is not known!", name);
            }

            cur_path.push(name);
        } else if line == "$ ls" {
            // ls command can be ignored: If a line does not match any of the cd/ls commands, it must be the output of a preceding ls command anyway
        } else {
            let c = re_ls_output.captures(line).with_context(|| format!("Could not parse line: {}", line))?;
            let filesize_or_dir = c.get(1).unwrap().as_str();
            let name = c.get(2).unwrap().as_str().to_string();

            if filesize_or_dir == "dir" {
                let dir = Dir {
                    subdirs: HashMap::new(),
                    files: HashMap::new()
                };
                cur_dir.subdirs.insert(name, dir);
            } else {
                let filesize = filesize_or_dir.parse().with_context(|| format!("Could not parse \"{}\" as number!", filesize_or_dir))?;
                cur_dir.files.insert(name, filesize);
            }
        }
    }

    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let root = read_input_file("../inputs/day7_example.txt").unwrap();
        assert_eq!(root.get_total_dir_size(), 48381165);
        assert_eq!(root.get_total_dir_size_if_below_threshold(100000), 95437);
        assert_eq!(root.find_size_of_smallest_dir_to_delete(70000000, 30000000), Some(24933642));
    }
}
