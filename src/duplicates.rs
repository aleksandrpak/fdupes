use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Result};
use std::collections::HashMap;
use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn find(paths: Vec<PathBuf>) -> Vec<Vec<PathBuf>> {
    let mut duplicates = vec![];
    for (_, ps) in calculate_hashes(paths) {
        if ps.len() > 1 {
            // TODO: Compare bytes
            duplicates.push(ps);
        }
    }

    duplicates
}

fn calculate_hashes(paths: Vec<PathBuf>) -> HashMap<(usize, [u8; 16]), Vec<PathBuf>> {
    let mut hashes = HashMap::new();
    let mut buffer = [0; 8192];

    // TODO: Parallel iterator?
    for path in paths {
        File::open(&path)
            .and_then(|mut f| calculate_hash(&mut f, &mut buffer))
            .ok()
            .map(|k| hashes.entry(k).or_insert_with(|| vec![]).push(path));
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
