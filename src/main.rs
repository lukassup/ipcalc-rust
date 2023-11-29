use std::convert::TryInto;

use clap::Parser;

/// IPv4 address calculator
#[derive(Parser)]
struct Cli {
    /// IPv4 address in CIDR notation (e.g. 1.2.3.4/24)
    ipv4_address_cidr: String,
}

fn main() {
    let args = Cli::parse();

    let (ip_addr_str, subnet_prefix_str) = args.ipv4_address_cidr
        .split_once('/')
        .expect("expected address in format: A.B.C.D/E");

    let ip_addr_bytes: [u8; 4] = ip_addr_str
        .split('.')
        .map(|s| s.parse::<u8>().unwrap()) // TODO
        .collect::<Vec<u8>>()
        .try_into()
        .expect("expected 4 digits separated by periods: A.B.C.D, got");

    // FIXME: only allow 0..=32 subnet mask
    let subnet_mask_prefix: u32 = subnet_prefix_str
        .parse::<u32>()
        .expect("expected subnet mask in cidr notation: /E, got");

    let ip_address: u32 = u32::from_be_bytes(ip_addr_bytes);
    println!(
        "{:<16}: {:>3?}, {:#010x}",
        "ip_address",
        ip_address.to_be_bytes(),
        ip_address,
    );

    // NOTE: for `<< 0` compiler throws: attempt to shift left with overflow, check & default to 0
    // let subnet_mask: u32 = u32::MAX << u32::BITS-subnet_mask_prefix;
    let subnet_mask: u32 = u32::MAX
        .checked_shl(u32::BITS - subnet_mask_prefix)
        .unwrap_or(0);
    println!(
        "{:<16}: {:>3?}, {:#010x}, /{}",
        "subnet_mask",
        subnet_mask.to_be_bytes(),
        subnet_mask,
        subnet_mask_prefix,
    );

    let subnet_addr: u32 = ip_address & subnet_mask;
    println!(
        "{:<16}: {:>3?}, {:#010x}",
        "subnet_addr",
        subnet_addr.to_be_bytes(),
        subnet_addr,
    );

    let broadcast_addr: u32 = subnet_addr | !subnet_mask;
    println!(
        "{:<16}: {:>3?}, {:#010x}",
        "broadcast_addr",
        broadcast_addr.to_be_bytes(),
        broadcast_addr,
    );

    let subnet_first: u32 = match subnet_mask {
        u32::MAX => subnet_addr, // /32 is a special case
        _ => subnet_addr + 1,
    };
    println!(
        "{:<16}: {:>3?}, {:#010x}",
        "subnet_first",
        subnet_first.to_be_bytes(),
        subnet_first,
    );

    let subnet_last: u32 = match subnet_mask {
        u32::MAX => broadcast_addr, // /32 is a special case
        _ => broadcast_addr - 1,
    };
    println!(
        "{:<16}: {:>3?}, {:#010x}",
        "subnet_last",
        subnet_last.to_be_bytes(),
        subnet_last,
    );
}
