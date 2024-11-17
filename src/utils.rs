pub fn validate_ip(ip: &str) -> bool {
    ip.split('.')
        .filter_map(|part| part.parse::<u8>().ok()) // Check if each part is a valid u8
        .count()
        == 4 // Must have exactly 4 parts
}

pub fn calculate_network_address(ip: &str, subnet_mask: u8) -> String {
    // split IP into parts and stores in vector
    let octets: Vec<u8> = ip
        .split('.')
        .filter_map(|part| part.parse::<u8>().ok())
        .collect();

    let mut mask = [0u8; 4];
    for i in 0..(subnet_mask / 8) {
        mask[i as usize] = 255;
    }
    if subnet_mask % 8 != 0 {
        mask[(subnet_mask / 8) as usize] = !((1 << (8 - subnet_mask % 8)) - 1);
    }

    let network_octets: Vec<u8> = octets 
        .iter()
        .zip(mask.iter())
        .map(|(octet, mask)| octet & mask)
        .collect();

    network_octets 
        .iter()
        .map(|octet| octet.to_string())
        .collect::<Vec<_>>()
        .join(".")
}

pub fn calculate_first_host_address(network_address: &str) -> String {
    let mut octets: Vec<u8> = network_address
        .split('.')
        .filter_map(|part| part.parse::<u8>().ok())
        .collect();

    for i in (0..4).rev() {
        if octets[i] < 255 {
            octets[i] += 1;
            break;
        } else {
            octets[i] = 0
        }
    }

    octets
        .iter()
        .map(|octet| octet.to_string())
        .collect::<Vec<_>>()
        .join(".")
    }

pub fn calculate_broadcast_address(network_address: &str, subnet_mask: u8) -> String {
    let mut octets: Vec<u8> = network_address
        .split('.')
        .filter_map(|part| part.parse::<u8>().ok())
        .collect();

        let mut mask = [0u8; 4]; 
        for i in 0..(subnet_mask / 8) {
            mask[i as usize] = 255; 
        }
        if subnet_mask % 8 != 0 {
            mask[(subnet_mask / 8) as usize] = 255 << (8 - subnet_mask % 8);
        }
    
        // Perform bitwise OR between the network address and the inverted mask
        for i in 0..4 {
            octets[i] |= !mask[i];
        }
    
        // Convert the result back to a dotted-decimal string
        octets
            .iter()
            .map(|octet| octet.to_string())
            .collect::<Vec<_>>()
            .join(".")
    
}

pub fn calculate_last_host_address(broadcast_address: &str) -> String {
    let mut octets: Vec<u8> = broadcast_address
        .split('.')
        .filter_map(|part| part.parse::<u8>().ok())
        .collect();

    for i in (0..4).rev() {
        if octets[i] > 0 {
            octets[i] -= 1;
            break;
        } else {
            octets[i] = 255;
        }
    }

    octets
        .iter()
        .map(|octet| octet.to_string())
        .collect::<Vec<_>>()
        .join(".")
}


pub fn calculate_next_subnet_address(network_address: &str, subnet_mask: u8) -> String {
    let octets: Vec<u8> = network_address
        .split('.')
        .filter_map(|part| part.parse::<u8>().ok())
        .collect();

    // Convert the octets into a single 32-bit integer
    let mut ip_as_int = (octets[0] as u32) << 24
        | (octets[1] as u32) << 16
        | (octets[2] as u32) << 8
        | (octets[3] as u32);

    // Calculate the block size
    let block_size = 1 << (32 - subnet_mask);

    // Add the block size to get the next subnet address
    ip_as_int += block_size;

    // Convert the 32-bit integer back into four octets
    let next_octets = [
        ((ip_as_int >> 24) & 0xFF) as u8,
        ((ip_as_int >> 16) & 0xFF) as u8,
        ((ip_as_int >> 8) & 0xFF) as u8,
        (ip_as_int & 0xFF) as u8,
    ];

    next_octets
        .iter()
        .map(|octet| octet.to_string())
        .collect::<Vec<_>>()
        .join(".")
}
