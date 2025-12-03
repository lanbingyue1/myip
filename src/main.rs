use clap::Parser;
use colored::*;
use getifs::{interfaces, local_addrs};
use std::collections::HashMap;
use std::net::{IpAddr, UdpSocket};

/// A simple tool to display local IP addresses
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Only display IPv6 addresses
    #[arg(short = '6', long)]
    ipv6: bool,
    
    /// Display all IP addresses (both IPv4 and IPv6)
    #[arg(short = 'a', long)]
    all: bool,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();
    
    // Get main IP address (the one used for internet connection)
    let main_ip = get_main_ip();
    
    // Get all network interfaces and create index to name mapping
    let interfaces = interfaces().unwrap();
    let mut interface_map = HashMap::new();
    
    for interface in interfaces {
        interface_map.insert(interface.index(), interface.name().to_string());
    }
    
    // Get local IP addresses
    let local_ips = local_addrs().unwrap();
    
    // Process each IP address
    for ip_str in local_ips {
        let ip_str = ip_str.to_string();
        
        // Parse the IP string to extract the interface index
        // Format: "192.168.10.6/24 (15)"
        if let Some(paren_idx) = ip_str.find('(') {
            // Extract IP address without the interface index
            let ip_addr = &ip_str[0..paren_idx].trim();
            
            // Determine if we should display this IP based on arguments
            let is_ipv6 = ip_addr.contains(':');
            let should_display = if args.all {
                true
            } else if args.ipv6 {
                is_ipv6
            } else {
                // Default: only IPv4
                !is_ipv6
            };
            
            if should_display {
                // Extract interface index from the parenthesis
                let index_str = &ip_str[paren_idx + 1..ip_str.len() - 1];
                if let Ok(index) = index_str.parse::<u32>() {
                    // Find the interface name in the map
                    if let Some(interface_name) = interface_map.get(&index) {
                        // Check if this IP is the main IP
                        let is_main_ip = if let Some(main_ip_val) = &main_ip {
                            ip_addr.starts_with(&main_ip_val.to_string())
                        } else {
                            false
                        };
                        
                        // Print with color if it's the main IP
                        if is_main_ip {
                            println!("{}: {}", 
                                     interface_name.green(), 
                                     ip_addr.green(),
                                    );
                        } else {
                            println!("{}: {}", interface_name, ip_addr);
                        }
                    }
                }
            }
        }
    }
}

/// Get the main IP address used for internet connection
fn get_main_ip() -> Option<IpAddr> {
    // Create UDP socket (bind to all IPv4 interfaces, auto-assign port)
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => socket,
        Err(_) => return None,
    };
    
    // Connect to Alibaba Cloud DNS (avoid Google DNS failure in China)
    if let Err(_) = socket.connect("223.5.5.5:53") {
        return None;
    }
    
    // Get local IP address from the connected socket
    socket.local_addr().ok().map(|addr| addr.ip())
}