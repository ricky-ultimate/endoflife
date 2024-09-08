use endoflife::request::{
    api_request_all_rust_cycles, api_request_single_rust_cycle
};

fn main() {
    match api_request_single_rust_cycle("1.81") {
        Ok(data) => {
            // If the request is successful, serialize and print the result
            let json_str = serde_json::to_string_pretty(&data).unwrap();
            println!("{}", json_str);
        }
        Err(e) => {
            // If there's an error, print the error message
            eprintln!("Error: {}", e);
        }
    }
}
