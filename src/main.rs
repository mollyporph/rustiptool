mod one;
mod two;
fn main() {
  let mut ip = "192.168.15.200";
  let mut cidr = "192.168.15.0/24";
  println!("{0} in range {1} : {2}", ip, cidr, ip_is_in_range(ip,cidr));
  ip = "192.168.15.200";
  cidr = "192.168.15.128/25";
  println!("{0} in range {1} : {2}", ip, cidr, ip_is_in_range(ip,cidr));
  ip = "192.168.15.200";
  cidr = "192.168.16.0/24";
  println!("{0} in range {1} : {2}", ip, cidr, ip_is_in_range(ip,cidr));
}
fn ip_is_in_range(ip: &str, cidr: &str) -> bool{
      let mut split = cidr.split('/');
      let cidr_ip = ip_to_endian(split.next().unwrap());
      let cidr_mask = cidr_to_endian(split.next().unwrap());
      (ip_to_endian(ip) & cidr_mask) == (cidr_ip & cidr_mask)}

fn swap_endian(num: u32) -> u32{((num & 0xff) << 24 | (num & 0xff00) << 8 | (num & 0xff0000) >> 8  | (num & 0xff000000) >> 24)} 
fn cidr_to_endian(cidr: &str) -> u32{swap_endian((-1 << (32 - cidr.parse::<i32>().unwrap())) as u32)}
fn ip_to_endian(ip: &str) -> u32{ip.split('.')
                                    .enumerate()
                                    .map(|(i, s)| s.parse::<u32>().unwrap() << i * 8)
                                    .fold(0, | sum:u32, x:u32 | sum | x)}