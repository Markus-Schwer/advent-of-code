use std::{fmt, usize};

const INPUT: &str = include_str!("../../resources/2024_09.txt");

#[derive(Debug, Clone, Eq, PartialEq)]
enum Block {
    Space,
    File(usize),
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Block::Space => write!(f, "."),
            Block::File(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum FsObject {
    Space(usize, usize),        // start, lenght
    File(usize, usize, usize),  // id, start, length
}

impl fmt::Display for FsObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FsObject::Space(_, space_lenght) => {
                for _ in 0..*space_lenght {
                    if let Err(err) = write!(f, ".") {
                        return Err(err);
                    }
                }
                Ok(())
            },
            FsObject::File(id, _, file_length) => {
                for _ in 0..*file_length {
                    if let Err(err) = write!(f, "{}", id) {
                        return Err(err);
                    }
                }
                Ok(())
            },
        }
    }
}

fn print_fs(fs: &Vec<Block>) {
    for block in fs {
        print!("{}", block);
    }
    println!();
}

fn calc_checksum(fs: &Vec<Block>) -> usize {
    let mut checksum = 0;
    for (i, block) in fs.iter().enumerate() {
        if let Block::File(id) = block {
            checksum += i * id;
        }
    }
    checksum
}

fn convert_fs_objects_to_blocks(fs_objects: &Vec<FsObject>) -> Vec<Block> {
    fs_objects.iter().flat_map(|fs_object| {
        match fs_object {
            FsObject::Space(_, space_len) => (0..*space_len).map(|_| Block::Space).collect::<Vec<Block>>(),
            FsObject::File(id, _, file_len) => (0..*file_len).map(|_| Block::File(*id)).collect(),
        }
    }).collect()
}

fn main() {
    let mut fs: Vec<Block> = Vec::new();
    let mut fs_objects = Vec::new();
    let mut block_index = 0;
    for (i, c) in INPUT.chars().enumerate() {
        if c < '0' || c > '9' {
            break;
        }

        let number: u8 = c as u8 - '0' as u8;
        if i % 2 == 0 {
            // file
            for _ in 0..number {
                fs.push(Block::File(i / 2));
            }
            fs_objects.push(FsObject::File(i / 2, block_index, number as usize));
        } else {
            // free space
            for _ in 0..number {
                fs.push(Block::Space);
            }
            fs_objects.push(FsObject::Space(block_index, number as usize));
        }
        block_index += number as usize;
    }
    print_fs(&fs);

    // part 1
    let mut part1 = fs.clone();
    for i in 0..part1.len() {
        let block = &part1[i];

        if let Block::File(_) = block {
            continue;
        }

        // i is a free space, so find a file block, starting at the end
        for j in (i + 1..part1.len()).rev() {
            if let Block::File(_) = part1[j] {
                part1.swap(i, j);
                break;
            }
        }
    }
    print_fs(&part1);

    let part1_checksum = calc_checksum(&part1);
    println!("part 1 checksum: {}", part1_checksum);
    println!();

    // part 2
    for i in 0..fs_objects.len() {
        let FsObject::Space(space_pos, space_len) = fs_objects[i] else {
            continue;
        };

        for j in (i + 1..fs_objects.len()).rev() {
            let FsObject::File(file_id, file_pos, file_len) = fs_objects[j] else {
                continue;
            };

            if space_len > file_len {
                fs_objects[i] = FsObject::File(file_id, space_pos, file_len);
                fs_objects[j] = FsObject::Space(file_pos, file_len);
                fs_objects.insert(i+1, FsObject::Space(space_pos + file_len, space_len - file_len));
                break;
            } else if space_len == file_len {
                fs_objects.swap(i, j);
                break;
            }
        }
    }

    let part2 = convert_fs_objects_to_blocks(&fs_objects);
    print_fs(&part2);

    let part2_checksum = calc_checksum(&part2);
    println!("part 2 checksum: {}", part2_checksum);
}
