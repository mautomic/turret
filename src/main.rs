use std::io::Read;
use std::time::Instant;

fn main() {

    deploy_turret("http://127.0.0.1:8500/employee/occupation/", 50);
    deploy_turret("http://127.0.0.1:8500/employee/salary/", 50);
    deploy_turret_with_id("http://127.0.0.1:8500/employee/id/", 50);
}

fn deploy_turret(base_url: &str, requests_to_fire: u128) {

    let mut total_time = 0;
    for x in 1..requests_to_fire {
        let url = &create_url_with_param(base_url, &determine_occupation(x));
        let mut response_body = String::new();
        let result = hit_endpoint(url, &mut response_body, &mut total_time);
        if !result.is_ok() {
            println!("Error during GET request");
        }
    }

    let avg_request_time = total_time/requests_to_fire;
    println!("Average request time was {} ms", avg_request_time);
}

fn deploy_turret_with_id(base_url: &str, requests_to_fire: u128) {

    let mut total_time = 0;
    for x in 1..requests_to_fire {
        let url = &create_url_with_id(base_url, x);
        let mut response_body = String::new();
        let result = hit_endpoint(url, &mut response_body, &mut total_time);
        if !result.is_ok() {
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

fn create_url_with_id(base_url: &str, id: u128) -> String {
    let id: &str = &id.to_string();
    let url = format!("{}{}", base_url, id);
    return url;
}

fn create_url_with_param(base_url: &str, param: &str) -> String {
    return format!("{}{}", base_url, param);
}

fn determine_occupation(index: u128) -> String {
    if index % 4 == 0 { return String::from("Manager"); }
    else if index % 3 == 0 { return  String::from("FullStackDeveloper"); }
    else if index % 2 == 0 { return  String::from("FrontendDeveloper"); }
    else { return  String::from("BackendDeveloper"); }
}