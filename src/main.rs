use std::{io::Read, result::Result};

// * This is a simple program that makes a GET request to a server that returns the client's IP address.
#[derive(serde::Deserialize)]
struct Response {
    origin: String,
}

// * This function prints the status code, the status message and the headers of the response.
fn print_response_infos(
    res: &reqwest::blocking::Response,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Status: {}", res.status());
    println!("Status message: {}", return_status_message(res.status()));
    println!("Headers:\n{:#?}", res.headers());

    Ok(())
}

// * This function returns a message based on the status code.
fn return_status_message(status: reqwest::StatusCode) -> String {
    match status {
        reqwest::StatusCode::OK => "Everything is fine",
        reqwest::StatusCode::CREATED => "Resource created",
        reqwest::StatusCode::ACCEPTED => "Request accepted",
        reqwest::StatusCode::NO_CONTENT => "No content",
        reqwest::StatusCode::BAD_REQUEST => "Bad request",
        reqwest::StatusCode::UNAUTHORIZED => "Unauthorized",
        reqwest::StatusCode::FORBIDDEN => "Forbidden",
        reqwest::StatusCode::NOT_FOUND => "Not found",
        reqwest::StatusCode::METHOD_NOT_ALLOWED => "Method not allowed",
        reqwest::StatusCode::INTERNAL_SERVER_ERROR => "Internal server error",
        _ => "Unknown status",
    }
    .to_string()
}

// * This function extracts the IP address from the JSON response.
fn get_ip_from_json(json: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response: Response = serde_json::from_str(json)?;
    Ok(response.origin)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // * Make a GET request to the server.
    let mut res = reqwest::blocking::get("https://httpbin.org/ip")?;

    // * Read the response body.
    let mut body = String::new();

    // * Read the response body.
    res.read_to_string(&mut body)?;

    // * Print the status code, the status message and the headers of the response.
    print_response_infos(&res)?;

    // * Extract the IP address from the JSON response.
    let ip = get_ip_from_json(&body)?;

    // * Print the IP address.
    println!("My IP is: {}", ip);

    Ok(())
}
