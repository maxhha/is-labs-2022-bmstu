use postcard::{from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

fn bytes_to_binary(s: &[u8]) -> Vec<bool> {
    s.into_iter()
        .flat_map(|x| (0..8).into_iter().rev().map(|i| (*x >> i) & 1 != 0))
        .collect()
}

fn binary_to_bytes(s: &[bool]) -> Vec<u8> {
    s.chunks(8)
        .map(|x| x.into_iter().fold(0u8, |acc, b| (acc << 1) | u8::from(*b)))
        .collect()
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct Leaf {
    letter: u8,
    count: i32,
}

#[derive(Clone, Serialize, Deserialize)]
enum Tree {
    Leaf(Leaf),
    Node {
        count: i32,
        left: Box<Self>,
        right: Box<Self>,
    },
}

impl Tree {
    fn from_data(data: &[u8]) -> Tree {
        let mut alphabet: Vec<Option<Leaf>> = vec![None; 1 << u8::BITS];

        for i in data.iter() {
            alphabet
                .get_mut(usize::from(*i))
                .unwrap()
                .get_or_insert_with(|| Leaf {
                    letter: *i,
                    count: 0,
                })
                .count += 1;
        }

        let mut trees: Vec<_> = alphabet
            .into_iter()
            .filter_map(|x| x.map(|x| Box::new(Tree::Leaf(x))))
            .collect();

        while trees.len() > 1 {
            let mut min1 = 0;
            let mut min2 = 1;

            if trees[min1].count() > trees[min2].count() {
                let tmp = min1;
                min1 = min2;
                min2 = tmp;
            }

            for (i, n) in trees.iter().enumerate().skip(2) {
                if n.count() < trees[min1].count() {
                    min2 = min1;
                    min1 = i;
                } else if n.count() < trees[min2].count() {
                    min2 = i;
                }
            }

            if min1 < min2 {
                let tmp = min1;
                min1 = min2;
                min2 = tmp;
            }

            let min1 = trees.drain(min1..min1 + 1).next().unwrap();
            let min2 = trees.drain(min2..min2 + 1).next().unwrap();

            trees.push(Box::new(Tree::Node {
                count: min1.count() + min2.count(),
                left: min1,
                right: min2,
            }));
        }

        *trees[0].clone()
    }

    fn count(&self) -> i32 {
        match self {
            Self::Leaf(Leaf { letter: _, count }) => *count,
            Self::Node {
                count,
                left: _,
                right: _,
            } => *count,
        }
    }

    fn _to_encode_map(&self, map: &mut HashMap<u8, Vec<bool>>, prefix: &mut Vec<bool>) {
        match self {
            Self::Leaf(Leaf { letter, count: _ }) => {
                map.insert(*letter, prefix.clone());
            }
            Self::Node {
                count: _,
                left,
                right,
            } => {
                prefix.push(false);
                left._to_encode_map(map, prefix);
                *prefix.last_mut().unwrap() = true;
                right._to_encode_map(map, prefix);
                prefix.pop().unwrap();
            }
        }
    }

    fn to_encode_map(&self) -> HashMap<u8, Vec<bool>> {
        let mut map = HashMap::with_capacity(usize::from(u8::MAX) + 1);

        self._to_encode_map(&mut map, &mut Vec::with_capacity(usize::from(u8::MAX) + 1));
        map
    }

    fn compress(&self, data: &[u8]) -> Vec<u8> {
        let map = self.to_encode_map();

        let mut data: Vec<_> = data
            .into_iter()
            .flat_map(|x| {
                map.get(x)
                    .expect(&format!("failed to find byte {} in tree", *x))
                    .iter()
                    .cloned()
            })
            .collect();

        let n_bits_to_add = 8 - data.len() % 8;
        let payload = bytes_to_binary(std::slice::from_ref(&u8::try_from(n_bits_to_add).unwrap()));

        data.extend((0..n_bits_to_add).map(|_| false).chain(payload.into_iter()));

        binary_to_bytes(&data)
    }

    fn _decompress(&self, iter: &mut dyn Iterator<Item = bool>) -> u8 {
        match self {
            Self::Leaf(Leaf { letter, count: _ }) => *letter,
            Self::Node {
                count: _,
                left,
                right,
            } => {
                if iter
                    .next()
                    .expect("Failed to decompress: unexpected number of bits")
                {
                    right._decompress(iter)
                } else {
                    left._decompress(iter)
                }
            }
        }
    }

    fn decomress(&self, data: &[u8]) -> Vec<u8> {
        let mut data = bytes_to_binary(data);
        let n_bits_to_remove = binary_to_bytes(&data[data.len() - 8..])[0];
        data.drain(data.len() - 8 - usize::from(n_bits_to_remove)..);

        let mut result = Vec::new();
        let mut data_iter = data.into_iter().peekable();

        while data_iter.peek().is_some() {
            result.push(self._decompress(&mut data_iter));
        }

        result
    }

    fn write_file(&self, path: &std::path::Path) {
        std::fs::write(path, &to_allocvec(&self).expect("Failed to serialize tree"))
            .expect("Failed to write tree to file");
    }

    fn from_file(path: &std::path::Path) -> Self {
        from_bytes(&std::fs::read(path).expect("Failed to read tree file"))
            .expect("Failed to parse tree")
    }
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Com {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long, default_value = "tree.bin")]
        tree: PathBuf,
    },
    Dec {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long, default_value = "tree.bin")]
        tree: PathBuf,
    },
}

fn main() {
    match Cli::parse().command {
        Commands::Com {
            input,
            output,
            tree: tree_path,
        } => {
            let data = std::fs::read(&input).expect("Failed to read input file");
            let tree = Tree::from_data(&data);

            tree.write_file(&tree_path);
            std::fs::write(&output, &tree.compress(&data))
                .expect("Failed to write compressed data");
        }
        Commands::Dec {
            input,
            output,
            tree: tree_path,
        } => {
            let data = std::fs::read(&input).expect("Failed to read input file");
            let tree = Tree::from_file(&tree_path);
            std::fs::write(output, tree.decomress(&data))
                .expect("failed to store decompressed data");
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_compress_decompress() {
        let message = "hello world!\n".as_bytes();

        let tree = Tree::from_data(&message);

        let comressed_data = tree.compress(&message);
        let decomressed_data = tree.decomress(&comressed_data);

        assert_eq!(&decomressed_data, &message);
    }
}
