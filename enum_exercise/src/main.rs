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
fn main() {
	// println!("four : {:?}, six : {:?}",four,six);
	let home = IpAddr::V4( Ipv4Addr {
		ip :(127, 0, 0, 1),
		MAC : 123134134,
	});
	
	let loopback = IpAddr::V6(String::from("::1"));
	// Option practice
	println!("home : {:?}, loopback : {:?}",home,loopback);
}

