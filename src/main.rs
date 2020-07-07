use std::io::Read;
use std::time::Instant;
#[macro_use]
extern crate clap;

fn main() {

    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "mautomic")
        (@arg URL: -u --url +required +takes_value "Sets the url to fire requests at")
        (@arg NUM: -n --num +required +takes_value "Number of requests")
    ).get_matches();

    let url = matches.value_of("URL").unwrap();
    let num = matches.value_of("NUM").unwrap();
    let requests_to_fire = num.parse::<u128>().unwrap();

    println!("{} will be blasted with {} requests", url, requests_to_fire);
    deploy_turret(url, requests_to_fire, true);
}

fn deploy_turret(base_url: &str, requests_to_fire: u128, param_requires_num: bool) {

    let mut total_time = 0;
    for x in 1..requests_to_fire {
        let url = & if param_requires_num {
           create_url_with_id(base_url, x)
        } else {
           create_url_with_param(base_url, &determine_occupation(x))
        };

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