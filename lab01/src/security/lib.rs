use wmi::{COMLibrary, WMIConnection};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename = "Win32_OperatingSystem")]
#[serde(rename_all = "PascalCase")]
struct OperatingSystem {
    uuid: String,
}

pub fn get_key() -> String {
    let wmi_con = WMIConnection::new(COMLibrary::new().unwrap().into()).unwrap();
    let results: Vec<OperatingSystem> = wmi_con
        .raw_query("SELECT UUID FROM Win32_ComputerSystemProduct")
        .unwrap();

    results.into_iter().nth(0).unwrap().uuid
}
