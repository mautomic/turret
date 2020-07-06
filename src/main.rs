use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    for x in 1..25 {
        let id: &str = &x.to_string();
        let url = format!("http://127.0.0.1:8500/employee/id/{}", id);
        let mut resp = reqwest::blocking::get(&url)?;
        let mut body = String::new();
        resp.read_to_string(&mut body)?;
        println!("{:#?}", body);
    }

    Ok(())
}