use security::get_key;

const KEY: &str = "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX";

fn main() {
    if !get_key().eq(KEY) {
        println!("FAILED");
        std::process::exit(1);
    }

    println!("SUCCESS");
    println!("signed with: {}", KEY);
}
