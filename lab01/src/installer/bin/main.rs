use security::get_key;
use std::fs::File;
use std::io::Write;

const KEY_TEMPLATE: &[u8] = b"XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX";

fn main() {
    let app_exe_bytes = include_bytes!(env!("APP_EXE"));

    println!("Installing {} bytes...", app_exe_bytes.len());
    println!("Signing app...");
    let key_pos = app_exe_bytes.windows(KEY_TEMPLATE.len()).position(|w| w == KEY_TEMPLATE).unwrap();

    let mut app = File::create("app.exe").unwrap();
    app.write(&app_exe_bytes[..key_pos]).unwrap();
    app.write(&get_key().into_bytes()).unwrap();
    app.write(&app_exe_bytes[key_pos+KEY_TEMPLATE.len()..]).unwrap();

    println!("Finished!");
}
