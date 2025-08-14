enum IpAddrKind {
    v4 (u8,u8,u8.u8),
    v6 (String)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String, 
}

fn main() {
    let localhost =  IpAddrKind::V4 (127.0.0.1);
    


}
