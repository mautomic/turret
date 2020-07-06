use std::io::Read;
use std::time::Instant;

fn main() {

    let base_url: &str = "http://127.0.0.1:8500/employee/id/";

    let mut total_time = 0;
    let requests_to_fire:u128 = 50;

    for x in 1..requests_to_fire {
        let url = &create_url(base_url, x);
        let mut response_body = String::new();
        let result = hit_endpoint(url, &mut response_body, &mut total_time);
        if result.is_ok() {
            println!("{:#?}", response_body);
        } else {
            println!("Error during GET request");
        }
    }

    let avg_request_time = total_time/requests_to_fire;
    println!("Average request time was {} ms", avg_request_time);
}

fn hit_endpoint(url: &str, response_body: &mut String, time: &mut u128) -> Result<(), Box<dyn std::error::Error>> {

    let start = Instant::now();
    let mut resp = reqwest::blocking::get(url)?;
    resp.read_to_string(response_body)?;
    *time = *time + start.elapsed().as_millis();
    Ok(())
}

fn create_url(base_url: &str, id: u128) -> String {

    let id: &str = &id.to_string();
    let url = format!("{}{}", base_url, id);
    return url;
}