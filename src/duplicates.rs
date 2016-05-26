use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Result};
use std::collections::HashMap;
use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn find(paths: Vec<PathBuf>) -> Vec<Vec<PathBuf>> {
    let mut duplicates = vec![];
    for (_, groups) in calculate_hashes(paths) {
        for group in groups {
            if group.len() > 1 {
                duplicates.push(group);
            }
        }
    }

    duplicates
}

fn calculate_hashes(paths: Vec<PathBuf>) -> HashMap<(usize, [u8; 16]), Vec<Vec<PathBuf>>> {
    let mut hashes = HashMap::new();
    let mut buffer = [0; 8192];

    // TODO: Parallel iterator?
    for path in paths {
        File::open(&path)
            .and_then(|mut f| calculate_hash(&mut f, &mut buffer))
            .ok()
            .map(|k| add_to_group(hashes.entry(k).or_insert_with(|| vec![]), path));
    }

    hashes
}

fn calculate_hash(file: &mut File, buffer: &mut [u8]) -> Result<(usize, [u8; 16])> {
    let mut hasher = Md5::new();
    let mut size = 0;

    loop {
        let count = try!(file.read(buffer));
        if count == 0 {
            break;
        }

        size += count;
        hasher.input(&buffer[0..count]);
    }

    let mut output = [0; 16];
    hasher.result(&mut output);

    Ok((size, output))
}

fn add_to_group(groups: &mut Vec<Vec<PathBuf>>, path: PathBuf) {
    // TODO: Parallel comparison in the end
    if groups.len() == 0 {
        groups.push(vec![path]);
        return;
    }

    for group in groups {
        if are_same_bytes(&group[0], &path) {
            group.push(path);
            break;
        }
    }
}

fn are_same_bytes(path1: &PathBuf, path2: &PathBuf) -> bool {
    File::open(path1)
        .and_then(|f1| File::open(path2).map(|f2| (f1, f2)))
        .and_then(|(mut f1, mut f2)| {
            let mut buffer1 = [0; 8192];
            let mut buffer2 = [0; 8192];

            loop {
                let count1 = try!(f1.read(&mut buffer1));
                let count2 = try!(f2.read(&mut buffer2));

                if count1 == 0 && count2 == 0 {
                    return Ok(true);
                } else if &buffer1[0..count1] != &buffer2[0..count2] {
                    return Ok(false);
                }
            }
        })
        .unwrap_or(false)
}
