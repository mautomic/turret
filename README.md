# turret
Turret is a simple tool for web service performance testing, specifically GET request
speed for a particular endpoint. The idea is to quickly blast N requests and measure
how long it takes to get the response, so the developer is utilizing a big enough
sample size to measure the performance of that particular query. This is useful especially
for requests over the wire, for unreliable services, or for APIs under various loads.

### Usage  
To run turret from the command line, navigate to wherever you've cloned the repo, cd into
turret/src/ and run:
```
cargo run -- --url <URL> --num <NUM>
```

To run from an IDE like CLion, create a run configuration using 'Cargo Command' and
set the command to:
```
run --bin turret -- -u <URL> -n <NUM>
```

### About
Turret utilizes reqwest, and a blocking GET request at that.

```Rust
let mut resp = reqwest::blocking::get(url)?;
resp.read_to_string(response_body)?;
```

In the future, more advanced functionality my be introduced such as:
* Asynchronous flooding
* Reading URLs from a config file
* Further metrics, range, std. deviation, 25th/75th percentile, etc...
