use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
    pss::{BlindedSigningKey, VerifyingKey},
    RsaPrivateKey, RsaPublicKey,
};
use sha2::Sha256;
use signature::{RandomizedSigner, Signature, Verifier};
use std::path::PathBuf;

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
        #[arg(short, long, default_value_t = 2048)]
        bits_size: usize,
        #[arg(short, long, default_value = "public_key.pem")]
        public_key: PathBuf,
        #[arg(short = 'P', long, default_value = "private_key.pem")]
        private_key: PathBuf,
    },
    Sig {
        #[arg(short = 'P', long, default_value = "private_key.pem")]
        private_key: PathBuf,
        #[arg(short, long)]
        data: PathBuf,
        #[arg(short, long, default_value = "sign.sig")]
        signature: PathBuf,
    },
    Ver {
        #[arg(short, long, default_value = "public_key.pem")]
        public_key: PathBuf,
        #[arg(short, long)]
        data: PathBuf,
        #[arg(short, long, default_value = "sign.sig")]
        signature: PathBuf,
    },
}

fn main() {
    match Cli::parse().command {
        Commands::Gen {
            bits_size,
            private_key: private_key_path,
            public_key: public_key_path,
        } => {
            let mut rng = rand::thread_rng();
            let private_key =
                RsaPrivateKey::new(&mut rng, bits_size).expect("failed to generate key");

            private_key
                .write_pkcs1_pem_file(private_key_path, rsa::pkcs8::LineEnding::CRLF)
                .expect("failed to store private key");

            private_key
                .to_public_key()
                .write_pkcs1_pem_file(public_key_path, rsa::pkcs8::LineEnding::CRLF)
                .expect("failed to store public key");
            println!("GENERATED!");
        }
        Commands::Sig {
            private_key,
            data,
            signature: sign_path,
        } => {
            let mut rng = rand::thread_rng();

            let private_key = RsaPrivateKey::read_pkcs1_pem_file(private_key)
                .expect("failed to read private key");

            let signing_key = BlindedSigningKey::<Sha256>::new(private_key);

            let data = std::fs::read(data).expect("failed to read input data");
            let sign = signing_key
                .try_sign_with_rng(&mut rng, &data)
                .expect("failed to sign data");

            std::fs::write(sign_path, sign.as_bytes()).expect("failed to write signature");
            println!("SIGNED!");
        }
        Commands::Ver {
            public_key,
            data,
            signature: sign_path,
        } => {
            let public_key =
                RsaPublicKey::read_pkcs1_pem_file(public_key).expect("faild to read public key");
            let verifying_key = VerifyingKey::<Sha256>::from(public_key);

            let data = std::fs::read(data).expect("failed to read input data");
            let sign = std::fs::read(sign_path).expect("failed to read signature file");
            let sign = Signature::from_bytes(&sign).expect("failed to convert to signature");

            if let Ok(_) = verifying_key.verify(&data, &sign) {
                println!("OK!");
            } else {
                println!("FAILED!");
            }
        }
    }
}
