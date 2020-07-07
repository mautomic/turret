use std::io::Read;
use std::time::Instant;
#[macro_use]
extern crate clap;

// Main function that kicks off turret
fn main() {

    // turret requires two command line args, -u for the url and -n for the number of requests
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
    deploy_turret(url, requests_to_fire);
}

// Function that fires a specified number of requests to the provided url
// and keeps an accumulation of the total time the response takes to arrive
// back. Once all requests are finished, the total time is divided by the num
// of requests in order to get the average time.
fn deploy_turret(base_url: &str, requests_to_fire: u128) {

    let mut total_time = 0;
    for _x in 0..requests_to_fire {
        let mut response_body = String::new();
        let result = hit_endpoint(base_url, &mut response_body, &mut total_time);
        if !result.is_ok() {
            println!("Error during GET request");
        }
    }

    let avg_request_time = total_time/requests_to_fire;
    println!("Average request time was {} ms", avg_request_time);
}

// Function that marks the time using Instant, makes a synchronous request
// to the url, and writes the response to the provided response_body reference
// and adds the time taken to the time reference. These variables are used in
// the caller of this function.
fn hit_endpoint(url: &str, response_body: &mut String, time: &mut u128) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let mut resp = reqwest::blocking::get(url)?;
    resp.read_to_string(response_body)?;
    *time = *time + start.elapsed().as_millis();
    Ok(())
}