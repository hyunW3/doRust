use std::io::{stdout, Write};
use curl::easy::Easy;

// const NOTICE: &'static str = "https://skb.skku.edu/semi/community/notice.do";
const NOTICE: &'static str = "https://www.skku.edu/skku/campus/skk_comm/notice01.do";
fn get_link(url: &str) {
}
fn main() {
	// get_link(NOTICE);
	let mut easy = Easy::new();
	easy.url(NOTICE).unwrap();
	easy.write_function(|data| {
		stdout().write_all(data).unwrap();
		Ok(data.len())
	}).unwrap();
	easy.perform().unwrap();
	
	println!("{}",easy.response_code().unwrap());
}