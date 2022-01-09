// https://doc.rust-lang.org/book/ch05-02-example-structs.html#refactoring-with-structs-adding-more-meaning
#[derive(Debug)]
struct Rectangle {
	width : u32,
	height : u32,
}
fn main() {
	let scale = 2;
	let rect1 = Rectangle{
		width : dbg!(30 * scale),
		height : 50
	};
	println!("rect1 is {:#?}",rect1);
	println!(
		"The area of rectangle is {} square pixels",
		area(&rect1)
	);
}
fn area(dimension : &Rectangle) -> u32 {
	dimension.width * dimension.height
}
