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