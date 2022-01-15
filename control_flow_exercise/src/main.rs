#[derive(Debug)]
enum Option<T> {
	None,
	Some(T),
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::Some(i) => Option::Some(i + 1),
		_ => x,
	}
}
fn main() {
	let five = Option::Some(5);
	let mut six = plus_one(five);
	let none = plus_one(Option::None);
	let mut counter = 0;
	println!("six : {:?} none : {:?}",six,none);
	// using if let : less typing, indentation, boilerplate code
	if let Option::Some(value) = six {
		println!("The value of six variable : {}",value);
		six = Option::Some(value+1);
	}else {
		counter +=1;
	}
	println!("six variable : {:?}",six);
}
