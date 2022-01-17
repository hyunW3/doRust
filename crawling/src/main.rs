extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;

const NOTICE: &'static str = "https://skb.skku.edu/semi/community/notice.do";

fn main() {
	let body = reqwest::get("https://skb.skku.edu/semi/community/notice.do")
		.await?
		.text()
		.await?;
	
	println!("body = {:?}",body);
}
