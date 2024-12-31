use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(
            std::io::Error
        );
        HttpRequest(reqwest::Error);
    }
}
fn main() -> Result<()> {
   let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body= String::new();
    res.read_to_string(&mut body).unwrap();
    println!("status {}", res.status());
println!("headers {:?}", res.headers());
    println!("body {}", body);
    Ok(())
}
