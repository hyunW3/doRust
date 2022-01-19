use std::process::Command;
use std::io::{self, Write};
// const NOTICE: &'static str = "https://skb.skku.edu/semi/community/notice.do";
const NOTICE: &'static str = "https://www.skku.edu/skku/campus/skk_comm/notice01.do";
fn get_link(url: &str) {
}
fn print_type_of<T>(_ : &T) {
	println!("type is : {}",std::any::type_name::<T>())
}
fn main() {
	// get_link(NOTICE);
	
	let output = Command::new("curl").args(&[NOTICE, "|", "tee","output.txt"]).output();
	// io::stdout().write_all(&output.stdout).unwrap();
	print_type_of(&output);
}