#[derive(Debug)]
struct Ipv4Addr {
	ip : (u8, u8, u8, u8),
	MAC : u32,
}
#[derive(Debug)]
enum IpAddr {
	V4(Ipv4Addr),
	V6(String),
}
enum Option<T> {
	None,
	Some(T),
}
#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska
}
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}
fn value_in_Cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => {
			println!("Lucky Penny!");
			1
		}
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State quarter from {:?}",state);
			25
		},
	}
}

fn main() {
	let home = IpAddr::V4( Ipv4Addr {
		ip :(127, 0, 0, 1),
		MAC : 123134134,
	});
	let loopback = IpAddr::V6(String::from("::1"));
	
	println!("home : {:?}, loopback : {:?}",home,loopback);
	// 6.1 Option practice
	let some_number = Option::Some(5);
	let some_string = Option::Some("a string");
	
	let absent_number : Option<i32> = Option::None;
	
	// 6.2 match Control Flow
	let quarter_alaska = Coin::Quarter(UsState::Alaska);
	value_in_Cents(quarter_alaska);
}

