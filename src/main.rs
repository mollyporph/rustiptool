fn main() {
    let ip = "192.168.0.16";
    let cidr_parts = split_cidr("192.168.0.0/24");
    let cidrendian = ip_to_endian(cidr_parts.0);
    let cidrmask = cidr_to_endian(cidr_parts.1);
    let ipendian = ip_to_endian(ip);
    println!("{:b}", 192);
    println!("{:b}", 168);
    println!("{:b}", 0);
    println!("{:b}", 16);
    println!("{:032b}", ipendian);

    println!("{:032b}", cidrmask);
    println!("{:032b}", cidrendian);

    println!("{:032b}", (ipendian & cidrmask));
    println!("{:032b}", (cidrendian & cidrmask));
    let is_in:bool = (ipendian & cidrmask) == (cidrendian & cidrmask);
    println!("{}",is_in);

}
fn split_cidr(cidr: &str) -> (&str,&str) {let mut split = cidr.split('/'); (split.next().unwrap(),split.next().unwrap())}
fn swap_endian(num: u32) -> u32{((num & 0xff) << 24 | (num & 0xff00) << 8 | (num & 0xff0000) >> 8  | (num & 0xff000000) >> 24)} 
fn cidr_to_endian(cidr: &str) -> u32{swap_endian((-1 << (32 - cidr.parse::<i32>().unwrap())) as u32)}
fn ip_to_endian(ip: &str) -> u32{ip.split('.')
                                    .enumerate()
                                    .map(|(i, s)| s.parse::<u32>().unwrap() << i * 8)
                                    .fold(0, | sum:u32, x:u32 | sum | x)}

