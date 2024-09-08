use endoflife::rust;
use serde_json;

fn main() {

     let json_str = r#"{
    "releaseDate": "2024-09-05",
    "eol": false,
    "latest": "1.81.0",
    "latestReleaseDate": "2024-09-05",
    "lts": false
}"#;

    let json_object = serde_json::from_str::<rust::RustSingleCycle>(json_str);

    print!("{:?}", json_object);

}
