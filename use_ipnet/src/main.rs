use ipnet::{IpNet,Ipv4Net,Ipv6Net};
use std::{net::{Ipv4Addr,Ipv6Addr}, str::FromStr};

fn main() -> std::io::Result<()>{
    let _v4 = Ipv4Net::new(Ipv4Addr::new(10,1,1,0),24).unwrap();
    let _v6 = Ipv6Net::new(Ipv6Addr::new(0xfd,0,0,0,0,0,0,0),24).unwrap();

    let v4_2 = Ipv4Net::from_str("10.1.1.0/24").unwrap();
    let v6_2 = Ipv6Net::from_str("fd00::/24").unwrap();

    let v4_3:Ipv4Net = "10.1.1.0/24".parse().unwrap();
    let v6_3:Ipv6Net = "fd00::/24".parse().unwrap();

    assert_eq!(v4_2, v4_3);
    assert_eq!(v6_2, v6_3);

    let net1 = IpNet::from(v4_2);
    let net2 = IpNet::from_str("10.1.1.0/24").unwrap();
    let net3:IpNet = "10.1.1.0/24".parse().unwrap();
    assert_eq!(net1,net2);
    assert_eq!(net1,net3);

    println!("{}, hostmask={}",net1,net1.hostmask());
    println!("{}, netmask={}",net1,net1.netmask());

    assert_eq!(
        "192.168.12.34/16".parse::<Ipv4Net>().unwrap().trunc(),
        "192.168.0.0/16".parse().unwrap()
    );
    Ok(())
}
