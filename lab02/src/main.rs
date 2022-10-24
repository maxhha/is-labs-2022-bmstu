mod enigma;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let seed = u64::from_str_radix(&args[1], 10).unwrap();
    let inf = &args[2];
    let outf: &String = &args[3];

    let mut enigma = enigma::Enigma::from_seed(3, seed);

    let input = std::fs::read(inf).unwrap();
    std::fs::write(outf, enigma.run(&input)).unwrap();
}
