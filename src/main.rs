use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let base_url: &str = "http://127.0.0.1:8500/employee/id/";

    for x in 1..25 {
        let mut resp = reqwest::blocking::get(&create_url(base_url, x))?;
        let mut body = String::new();
        resp.read_to_string(&mut body)?;
        println!("{:#?}", body);
    }

    Ok(())
}

fn create_url(base_url: &str, id: i32) -> String {

    let id: &str = &id.to_string();
    let url = format!("{}{}", base_url, id);
    return url;
}