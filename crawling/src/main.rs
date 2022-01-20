// const NOTICE: &'static str = "https://skb.skku.edu/semi/community/notice.do";
const NOTICE: &'static str = "https://rustrepo.com/repo/seanmonstar-reqwest-rust-http-client";

use std::collections::HashMap;
fn get_body() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(NOTICE)?
        .text();
		//.json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}
fn main(){
	get_body();
}