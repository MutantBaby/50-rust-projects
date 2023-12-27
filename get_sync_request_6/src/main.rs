use std::io::Read;

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error) #[cfg(unix)];
        HttpRequest(reqwest::Error);
    }

    errors {
        InvalidUrl(url: String) {
            description("invalid url"),
            display("invalid url: {}", url),
        }
    }

    // types {
    //     Error, ErrorKind, ResultExt, Result;
    // }

    //  links {
    //     Another(other_error::Error, other_error::ErrorKind) #[cfg(unix)];
    // }
}

fn main() -> Result<()> {
    let mut body: String = String::new();
    let mut response: reqwest::blocking::Response =
        reqwest::blocking::get("http://httpbin.org/get").unwrap();

    // access body part into body variable
    response.read_to_string(&mut body).unwrap();

    println!("Header | {:#?}", response.headers());
    println!("Status | {}", response.status());
    println!("Body | {}", body);

    Ok(())
}
