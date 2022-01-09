// https://doc.rust-lang.org/book/ch05-02-example-structs.html#refactoring-with-structs-adding-more-meaning
#[derive(Debug)]
struct Rectangle {
	width : u32,
	height : u32,
}
impl Rectangle {
	fn square(size: u32) -> Rectangle { // associated function that isn't method : often used for constructors
		Rectangle {
			width : size,
			height : size,
		}
	}
	fn area(&self) -> u32 { // short for self: &self
		self.width * self.height
	}
	fn width(&self) -> u32 { // method getter
		self.width
	}
	fn available(&self) -> bool {
		self.width >0 && self.height >0
	}
	fn can_hold(&self, other : &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}
fn main() {
	let scale = 2;
	let rect1 = Rectangle{
		width : dbg!(30 * scale),
		height : 40
	};
	if rect1.available() {
		println!("The rectangle has a non-zero width & height; it is {} x {}",rect1.width,rect1.height)
	}else {
		return;
	}
	println!("rect1 is {:#?}",rect1);
	println!(
		"The area of rectangle is {} square pixels",
		rect1.area()
	);
	
	let rect2 = Rectangle {
		width : 30,
		height : 30
	};
	println!("Rect1 can hold ? : {}",rect1.can_hold(&rect2));
	let sq = Rectangle::square(3);
	println!("square constructor : {:#?}",sq);
}
