use std::{
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
    str::FromStr,
    time::SystemTime,
};

use num_bigint::{BigUint, ModInverse, RandPrime};
use num_traits::{FromPrimitive, One};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use rand_core::{CryptoRng, RngCore};

// Default exponent for RSA keys.
const EXP: u64 = 65537;

struct Rsa {
    n: BigUint,
    e: BigUint,
    d: BigUint,
}

impl Rsa {
    fn new<R: RngCore + CryptoRng + RandPrime>(rng: &mut R, bit_size: usize, e: &BigUint) -> Rsa {
        let mut p: BigUint;
        let mut q: BigUint;
        let n_final: BigUint;
        let d_final: BigUint;

        'next: loop {
            p = rng.gen_prime(bit_size / 2);
            q = rng.gen_prime(bit_size - p.bits());

            if p == q {
                continue 'next;
            }

            let n = p.clone() * q.clone();
            let f_euler = (p.clone() - BigUint::one()) * (q.clone() - BigUint::one());

            if n.bits() != bit_size {
                continue 'next;
            }

            // e * d + f_euler * y = nod(e, f_euler) = 1
            // (f_euler % e) * d_1 + e * y_1 = gcd(e, f_euler) = 1
            if let Some(d) = e.mod_inverse(f_euler) {
                n_final = n;
                d_final = d.to_biguint().unwrap();
                break;
            }
        }

        Rsa {
            n: n_final,
            e: e.clone(),
            d: d_final,
        }
    }

    fn as_public(&self) -> RsaPublic {
        RsaPublic {
            n: self.n.clone(),
            e: self.e.clone(),
        }
    }

    fn as_private(&self) -> RsaPrivate {
        RsaPrivate {
            n: self.n.clone(),
            d: self.d.clone(),
        }
    }
}

struct RsaPublic {
    n: BigUint,
    e: BigUint,
}

impl RsaPublic {
    fn to_file(&self, path: PathBuf) {
        let mut f = fs::File::create(path).unwrap();
        f.write(format!("{}\n", self.n).as_bytes()).unwrap();
        f.write(format!("{}\n", self.e).as_bytes()).unwrap();
    }

    fn from_file(path: PathBuf) -> RsaPublic {
        let f = fs::File::open(path).unwrap();
        let mut lines = io::BufReader::new(f).lines();

        let n = lines.next().unwrap().unwrap();
        let n = BigUint::from_str(&n).unwrap();

        let e = lines.next().unwrap().unwrap();
        let e = BigUint::from_str(&e).unwrap();

        RsaPublic { n, e }
    }

    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        data.chunks(self.size())
            .flat_map(|m| {
                let m = BigUint::from_bytes_le(m);
                let mut x = m.modpow(&self.e, &self.n).to_bytes_le();
                x.extend((x.len()..self.size()).map(|_| 0));
                x
            })
            .collect()
    }

    fn size(&self) -> usize {
        self.n.bits() / 8
    }
}

struct RsaPrivate {
    n: BigUint,
    d: BigUint,
}

impl RsaPrivate {
    fn to_file(&self, path: PathBuf) {
        let mut f = fs::File::create(path).unwrap();
        f.write(format!("{}\n", self.n).as_bytes()).unwrap();
        f.write(format!("{}\n", self.d).as_bytes()).unwrap();
    }

    fn from_file(path: PathBuf) -> RsaPrivate {
        let f = fs::File::open(path).unwrap();
        let mut lines = io::BufReader::new(f).lines();

        let n = lines.next().unwrap().unwrap();
        let n = BigUint::from_str(&n).unwrap();

        let d = lines.next().unwrap().unwrap();
        let d = BigUint::from_str(&d).unwrap();

        RsaPrivate { n, d }
    }

    fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        data.chunks(self.size())
            .flat_map(|m| {
                let m = BigUint::from_bytes_le(m);
                let mut x = m.modpow(&self.d, &self.n).to_bytes_le();
                x.extend((x.len()..self.size()).map(|_| 0));
                x
            })
            .collect()
    }

    fn size(&self) -> usize {
        self.n.bits() / 8
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
    Gen {
        #[arg(short, long)]
        seed: Option<u64>,
        #[arg(short, long, default_value_t = 64)]
        byte_size: usize,
        #[arg(short, long, default_value = "pub_key.txt")]
        pub_key: PathBuf,
        #[arg(short = 'P', long, default_value = "private_key.txt")]
        private_key: PathBuf,
    },
    Enc {
        #[arg(short, long, default_value = "pub_key.txt")]
        pub_key: PathBuf,
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    Dec {
        #[arg(short = 'P', long, default_value = "private_key.txt")]
        private_key: PathBuf,
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn make_match_size(data: &mut Vec<u8>, target: usize) {
    let r = target - data.len() % target;
    let r: u8 = r.try_into().unwrap();
    data.extend((0..r).into_iter().map(|_| r))
}

fn make_original_size(data: &mut Vec<u8>) {
    let r = data.last().unwrap();
    let s = data.len() - usize::from(*r);
    data.drain(s..data.len());
}

fn main() {
    match Cli::parse().command {
        Commands::Gen {
            seed,
            byte_size,
            private_key,
            pub_key,
        } => {
            let seed = if let Some(s) = seed {
                s
            } else {
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            };

            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let r = Rsa::new(&mut rng, byte_size * 8, &BigUint::from_u64(EXP).unwrap());

            r.as_public().to_file(pub_key);
            r.as_private().to_file(private_key);
        }
        Commands::Enc {
            pub_key,
            input,
            output,
        } => {
            let key = RsaPublic::from_file(pub_key);

            let mut input = std::fs::read(input).unwrap();
            make_match_size(&mut input, key.size());

            let data = key.encrypt(&input);

            std::fs::write(output, data).unwrap();
        }
        Commands::Dec {
            private_key,
            input,
            output,
        } => {
            let key = RsaPrivate::from_file(private_key);

            let input = std::fs::read(input).unwrap();

            let mut data = key.decrypt(&input);
            make_original_size(&mut data);

            std::fs::write(output, &data).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let message =
            "hello world!                                                    \n".as_bytes();

        let mut rng = ChaCha8Rng::seed_from_u64(12345);
        let r = Rsa::new(&mut rng, 512, &BigUint::from_u64(EXP).unwrap());

        let mut data = Vec::from(message);
        make_match_size(&mut data, r.as_public().size());

        let data = r.as_public().encrypt(&data);
        let mut data = r.as_private().decrypt(&data);
        make_original_size(&mut data);

        assert_eq!(data, message);
    }
}
