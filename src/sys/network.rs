use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};

use eui48::MacAddress;
use nix;

use super::super::errors::Result;

pub fn interfaces() -> Result<Vec<String>> {
    let mut items = nix::ifaddrs::getifaddrs()?
        .filter(|x| {
            // SIOCGIWNAME to test wifi
            x.flags.contains(nix::net::if_::InterfaceFlags::IFF_UP)
                && x.flags.contains(nix::net::if_::InterfaceFlags::IFF_RUNNING)
                && x.flags
                    .contains(nix::net::if_::InterfaceFlags::IFF_BROADCAST)
                && x.flags
                    .contains(nix::net::if_::InterfaceFlags::IFF_MULTICAST)
        })
        .map(|x| x.interface_name)
        .collect::<Vec<_>>();

    items.sort();
    items.dedup();
    Ok(items)
}

pub fn ip4(name: &String) -> Result<Option<Ipv4Addr>> {
    let items = nix::ifaddrs::getifaddrs()?
        .filter(|x| x.interface_name == *name)
        .map(|x| {
            if let Some(addr) = x.address {
                if let nix::sys::socket::SockAddr::Inet(addr) = addr {
                    if let SocketAddr::V4(addr) = addr.to_std() {
                        return Some(addr.ip().clone());
                    }
                }
            }
            None
        })
        .filter(|x| *x != None)
        .collect::<Vec<_>>();

    Ok(match items.first() {
        Some(it) => *it,
        None => None,
    })
}

pub fn ip6(name: &String) -> Result<Option<Ipv6Addr>> {
    let items = nix::ifaddrs::getifaddrs()?
        .filter(|x| x.interface_name == *name)
        .map(|x| {
            if let Some(addr) = x.address {
                if let nix::sys::socket::SockAddr::Inet(addr) = addr {
                    if let SocketAddr::V6(addr) = addr.to_std() {
                        return Some(addr.ip().clone());
                    }
                }
            }
            None
        })
        .filter(|x| *x != None)
        .collect::<Vec<_>>();

    Ok(match items.first() {
        Some(it) => *it,
        None => None,
    })
}

pub fn mac(name: &String) -> Result<Option<MacAddress>> {
    let items = nix::ifaddrs::getifaddrs()?
        .filter(|x| x.interface_name == *name)
        .map(|x| {
            if let Some(addr) = x.address {
                if let nix::sys::socket::SockAddr::Link(addr) = addr {
                    return Some(MacAddress::new(addr.addr()));
                }
            }
            None
        })
        .filter(|x| *x != None)
        .collect::<Vec<_>>();

    Ok(match items.first() {
        Some(it) => *it,
        None => None,
    })
}
