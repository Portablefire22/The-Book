
// Enums allow for a type to be defined by listing the various possible variants
// They can be used to for pattern matching


// In this example, the context of an IP address will be used.
// The IP address will be represented by an enum.
// Since IP addresses only have two possible types (IPV6 or IPV4) and is mutually exlusive, an enum is used.

enum IpAddrKind{
    V4,
    V6,
}


fn main() {
    enum IpAddr{
        V4(u8, u8, u8, u8), // A benefit of using enums is that you can have multiple variants, structs cannot do this
        V6(String),
    }    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}


fn route(ip_kind: IpAddrKind){

}