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
fn main() {
	let home = IpAddr::V4( Ipv4Addr {
		ip :(127, 0, 0, 1),
		MAC : 123134134,
	});
	let loopback = IpAddr::V6(String::from("::1"));
	
	println!("home : {:?}, loopback : {:?}",home,loopback);
	// Option practice
	let some_number = Some(5);
	let some_string = Some("a string");
	
	let absent_number : Option<i32> = None;
}

