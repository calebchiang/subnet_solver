use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::utils;

pub async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Subnet Solver server!")
}

pub async fn calculate_subnet(
    path: web::Path<(String, u8)>,
) -> impl Responder {
    let (ip, subnet_mask) = path.into_inner();

    if !utils::validate_ip(&ip) {
        return HttpResponse::BadRequest().body("Invalid IP address format. ");
    }

    if subnet_mask > 32 {
        return HttpResponse::BadRequest().body("Invalid subnet mask. Must be between 0 and 32. ");
    }

    let network_address = utils::calculate_network_address(&ip, subnet_mask);
    let first_host_address = utils::calculate_first_host_address(&network_address);
    let broadcast_address = utils::calculate_broadcast_address(&network_address, subnet_mask);
    let last_host_address = utils::calculate_last_host_address(&broadcast_address);
    let next_subnet_address = utils::calculate_next_subnet_address(&network_address, subnet_mask);


    let response = json!({
        "ip": ip,
        "subnet_mask": subnet_mask,
        "network_address": network_address,
        "first_host_address": first_host_address,
        "broadcast_address": broadcast_address,
        "last_host_address": last_host_address,
        "next_subnet_address": next_subnet_address
    });

    HttpResponse::Ok().json(response)
}

