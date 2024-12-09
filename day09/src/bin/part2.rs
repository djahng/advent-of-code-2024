use std::{env, fs, iter::repeat};

#[derive(Debug, PartialEq)]
struct DiskMap {
    // Some(file_id) or free space (None)
    blocks: Vec<Option<usize>>,
    compressed: Vec<Option<usize>>,
}

impl DiskMap {
    fn from(input: &String) -> Self {
        let mut disk_map = DiskMap {
            blocks: Vec::new(),
            compressed: Vec::new(),
        };

        for (i, ch) in input.chars().enumerate() {
            if i % 2 == 0 {
                // Even indexes are files
                disk_map
                    .blocks
                    .extend(repeat(Some(i / 2)).take(ch.to_digit(10).unwrap() as usize));
            } else {
                // Odd indexes are free space
                disk_map
                    .blocks
                    .extend(repeat(None).take(ch.to_digit(10).unwrap() as usize));
            }
        }

        disk_map
    }

    fn compress(&mut self) -> u64 {
        self.compressed = self.blocks.clone();
        let mut file_ids = self.get_file_indices();
        file_ids.reverse();

        // Skip last file
        for i in 0..file_ids.len() - 1 {
            let ids = &file_ids[i];
            let blanks = self.get_blank_indices();
            if let Some(blank_idxs) = blanks.iter().position(|v| v.len() >= ids.len()) {
                for (i, _) in ids.iter().enumerate() {
                    if blanks[blank_idxs][i] < ids[i] {
                        self.compressed.swap(blanks[blank_idxs][i], ids[i]);
                    }
                }
            }
        }

        self.calculate_checksum()
    }

    fn calculate_checksum(&self) -> u64 {
        let mut checksum = 0;

        for (i, &val) in self.compressed.iter().enumerate() {
            if let Some(val) = val {
                checksum += i * val;
            }
        }

        checksum as u64
    }

    fn get_blank_indices(&self) -> Vec<Vec<usize>> {
        let mut indices = Vec::new();
        let mut group = Vec::new();

        for (i, &val) in self.compressed.iter().enumerate() {
            if val.is_none() {
                group.push(i);
            } else if !group.is_empty() && val.is_some() {
                indices.push(group.clone());
                group.clear();
            }
        }

        indices
    }

    fn get_file_indices(&self) -> Vec<Vec<usize>> {
        let mut indices = Vec::new();
        let mut group = Vec::new();

        for (i, &val) in self.blocks.iter().enumerate() {
            if let Some(id) = val {
                if group.is_empty() {
                    group.push(i);
                } else if self.blocks[*group.last().unwrap()].unwrap() == id {
                    group.push(i);
                } else {
                    indices.push(group.clone());
                    group.clear();

                    group.push(i);
                }
            } else {
                if !group.is_empty() {
                    indices.push(group.clone());
                    group.clear();
                }
            }
        }

        // Push last group if necessary
        if !group.is_empty() {
            indices.push(group.clone());
        }

        indices
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filename = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(filename)
        .expect("Could not read file")
        .trim()
        .to_string();

    let mut disk_map = DiskMap::from(&input);
    let checksum = disk_map.compress();

    println!("Checksum: {checksum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_input() {
        let input = "12345".to_string();
        let disk_map = DiskMap::from(&input);

        assert_eq!(
            disk_map,
            DiskMap {
                blocks: vec![
                    Some(0),
                    None,
                    None,
                    Some(1),
                    Some(1),
                    Some(1),
                    None,
                    None,
                    None,
                    None,
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2),
                    Some(2)
                ],
                compressed: Vec::new(),
            }
        );

        let input = "2333133121414131402".to_string();
        let disk_map = DiskMap::from(&input);

        assert_eq!(
            disk_map,
            DiskMap {
                blocks: vec![
                    Some(0),
                    Some(0),
                    None,
                    None,
                    None,
                    Some(1),
                    Some(1),
                    Some(1),
                    None,
                    None,
                    None,
                    Some(2),
                    None,
                    None,
                    None,
                    Some(3),
                    Some(3),
                    Some(3),
                    None,
                    Some(4),
                    Some(4),
                    None,
                    Some(5),
                    Some(5),
                    Some(5),
                    Some(5),
                    None,
                    Some(6),
                    Some(6),
                    Some(6),
                    Some(6),
                    None,
                    Some(7),
                    Some(7),
                    Some(7),
                    None,
                    Some(8),
                    Some(8),
                    Some(8),
                    Some(8),
                    Some(9),
                    Some(9),
                ],
                compressed: Vec::new(),
            }
        );
    }

    #[test]
    fn it_gets_blank_indices() {
        let input = "2333133121414131402".to_string();
        let mut disk_map = DiskMap::from(&input);
        disk_map.compressed = disk_map.blocks.clone();
        let blank_indicies = disk_map.get_blank_indices();

        assert_eq!(
            blank_indicies,
            vec![
                vec![2, 3, 4],
                vec![8, 9, 10],
                vec![12, 13, 14],
                vec![18],
                vec![21],
                vec![26],
                vec![31],
                vec![35],
            ]
        );
    }

    #[test]
    fn it_gets_files_list() {
        let input = "2333133121414131402".to_string();
        let disk_map = DiskMap::from(&input);
        let file_indices = disk_map.get_file_indices();

        assert_eq!(
            file_indices,
            vec![
                vec![0, 1],
                vec![5, 6, 7],
                vec![11],
                vec![15, 16, 17],
                vec![19, 20],
                vec![22, 23, 24, 25],
                vec![27, 28, 29, 30],
                vec![32, 33, 34],
                vec![36, 37, 38, 39],
                vec![40, 41],
            ]
        );
    }

    #[test]
    fn it_compresses() {
        let input = "12345".to_string();
        let mut disk_map = DiskMap::from(&input);
        let _checksum = disk_map.compress();

        assert_eq!(
            disk_map.compressed,
            vec![
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                Some(2),
                Some(2),
                Some(2),
                Some(2),
                Some(2)
            ]
        );

        let input = "2333133121414131402".to_string();
        let mut disk_map = DiskMap::from(&input);
        let _checksum = disk_map.compress();

        assert_eq!(
            disk_map.compressed,
            vec![
                Some(0),
                Some(0),
                Some(9),
                Some(9),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(7),
                Some(7),
                Some(7),
                None,
                Some(4),
                Some(4),
                None,
                Some(3),
                Some(3),
                Some(3),
                None,
                None,
                None,
                None,
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                None,
                Some(6),
                Some(6),
                Some(6),
                Some(6),
                None,
                None,
                None,
                None,
                None,
                Some(8),
                Some(8),
                Some(8),
                Some(8),
                None,
                None,
            ]
        );
    }

    #[test]
    fn it_calculates_checksum() {
        let input = "12345".to_string();
        let mut disk_map = DiskMap::from(&input);
        let checksum = disk_map.compress();

        assert_eq!(checksum, 132);

        let input = "2333133121414131402".to_string();
        let mut disk_map = DiskMap::from(&input);
        let checksum = disk_map.compress();

        assert_eq!(checksum, 2858);
    }
}
