use error_chain::error_chain;
use std::io::Read;
use serde::Deserialize;
use reqwest::header::USER_AGENT;

#[derive(Deserialize,Debug)]
struct User{
    login:String,
    id:u32
}

#[tokio::main]
async fn main()-> Result<()>{
    let owner:String = "rust-lang-nursery".to_string();
    let repo:String = "rust-cookbook".to_string();
    let requestUrl:String = format!("https://api.github.com/repos/{owner}/{repo}/stargazers");
    println!("{}",requestUrl);
    let client = reqwest::Client::new();
    let response = client.get(&requestUrl).header(USER_AGENT,"rust web api client demo task").send().await?;
    let users:Vec<User>= response.json().await?;
    println!(" Users  are---------------------{:?}",users);
    Ok(())
}


error_chain! {
    foreign_links {
        Io(
            std::io::Error
        );
        HttpRequest(reqwest::Error);
    }
}
#[tokio::main]
async fn mxxain() -> Result<()> {
let res=reqwest::get("https://www.rust-lang.org/en-US/").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    let body = res.text().await?;
    println!("Body:\n{}", body);
    println!("COmpleted");
    Ok(())

}

fn xmain() -> Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body= String::new();
    res.read_to_string(&mut body).unwrap();
    println!("status {}", res.status());
    println!("headers {:?}", res.headers());
    println!("body {}", body);
    Ok(())
}


