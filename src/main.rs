use std::io::Read;

fn main() {

    let base_url: &str = "http://127.0.0.1:8500/employee/id/";

    for x in 1..25 {
        let url = &create_url(base_url, x);
        let mut response_body = String::new();
        let result = hit_endpoint(url, &mut response_body);
        if result.is_ok() {
            println!("{:#?}", response_body);
        } else {
            println!("Error during GET request");
        }
    }
}

fn hit_endpoint(url: &str, response_body: &mut String) -> Result<(), Box<dyn std::error::Error>> {

    let mut resp = reqwest::blocking::get(url)?;
    resp.read_to_string(response_body)?;
    Ok(())
}

fn create_url(base_url: &str, id: i32) -> String {

    let id: &str = &id.to_string();
    let url = format!("{}{}", base_url, id);
    return url;
}