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
                // Odd indexes is free space
                disk_map
                    .blocks
                    .extend(repeat(None).take(ch.to_digit(10).unwrap() as usize));
            }
        }

        disk_map
    }

    fn compress(&mut self) -> u64 {
        self.compressed = self.blocks.clone();

        while let Some(free_space_idx) = self.compressed.iter().position(|&block| block.is_none()) {
            if free_space_idx == self.compressed.len() - 1 {
                // Remove None as the last element in the vector
                self.compressed.pop();
                break;
            }

            // Get last element
            while let Some(val) = self.compressed.pop() {
                match val {
                    Some(n) => {
                        self.compressed[free_space_idx] = Some(n);
                        break;
                    }
                    None => continue,
                }
            }
        }

        self.calculate_checksum()
    }

    fn calculate_checksum(&self) -> u64 {
        let mut checksum = 0;

        for (i, &val) in self.compressed.iter().enumerate() {
            let val = val.unwrap();
            checksum += i * val;
        }

        checksum as u64
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
    fn it_compresses() {
        let input = "12345".to_string();
        let mut disk_map = DiskMap::from(&input);
        let _checksum = disk_map.compress();

        assert_eq!(
            disk_map.compressed,
            vec![
                Some(0),
                Some(2),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
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
                Some(8),
                Some(1),
                Some(1),
                Some(1),
                Some(8),
                Some(8),
                Some(8),
                Some(2),
                Some(7),
                Some(7),
                Some(7),
                Some(3),
                Some(3),
                Some(3),
                Some(6),
                Some(4),
                Some(4),
                Some(6),
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                Some(6),
                Some(6),
            ]
        );
    }

    #[test]
    fn it_calculates_checksum() {
        let input = "12345".to_string();
        let mut disk_map = DiskMap::from(&input);
        let checksum = disk_map.compress();

        assert_eq!(checksum, 60);

        let input = "2333133121414131402".to_string();
        let mut disk_map = DiskMap::from(&input);
        let checksum = disk_map.compress();

        assert_eq!(checksum, 1928);
    }
}
