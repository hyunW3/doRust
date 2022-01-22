use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use scraper::{Html, Selector};

// const NOTICE: &'static str = "https://skb.skku.edu/semi/community/notice.do";
const URL: &'static str = "https://github.com/hyunW3?tab=repositories&type=source";

//fn get_body() -> Result<(), Box<dyn std::error::Error>> {
fn get_body() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(URL)?
        .text()?;
		// .json::<HashMap<String, String>>()?;
	let document = Html::parse_document(&resp);
	let a_selector = Selector::parse("a").unwrap();
	let titles = Selector::parse("h3.wb-break-all").unwrap();
	let mut title_elements = document.select(&titles);
	
	let root_url : Vec<&str> = URL.split("?").collect();
	let root_url = root_url[0];
	println!("root url : {}",root_url);
	let mut url_list = String::new();
	for title_element in &mut title_elements {
		let inner_a = title_element.select(&a_selector).next().unwrap();
		let href = inner_a.value().attr("href").unwrap().to_string();
		let title_split : Vec<&str> = href.split("/").collect();
		let title = title_split[2];
		// println!("title : {}",title);
        let link = root_url.to_owned() + &("/".to_owned() +title);
		// println!("Link : {}",link);
		url_list = url_list.to_owned() + &(title.to_owned() +" " + &link + "\n"); 
	}
	let path = Path::new("github_links.txt");
	let display = path.display();
	let mut file = match File::create(&path) {
		Err(why) => panic!("couldn't create {}: {}",display,why),
		Ok(file) => file,
	};
	match file.write_all(url_list.as_bytes()) {
		Err(why) => panic!("Couldn't write to {} : {} ",display,why),
		Ok(_) => println!("succesfully wrote to {}",display),
	}
    Ok(())
}
fn main(){
	let res = get_body();
}