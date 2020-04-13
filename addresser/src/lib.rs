// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha512;

/// Represents part of address that designates resource type
#[derive(Debug)]
pub enum Resource {
    AGENT,
    ORG
}

/// Convert resource part to byte value in hex
pub fn resource_to_byte(part: Resource) -> String {
    match part {
        Resource::AGENT => String::from("00"),
        Resource::ORG => String::from("01")
    }
}

/// Convert byte string to Resource
pub fn byte_to_resource(bytes: &str) -> Result<Resource, ResourceError>  {
    match bytes {
        "00" => Ok(Resource::AGENT),
        "01" => Ok(Resource::ORG),
        _ => Err(ResourceError::UnknownResource(
                format!("No resource found matching byte pattern {}", bytes)))
    }
}


#[derive(Debug)]
pub enum ResourceError {
    UnknownResource(String)
}

////from handler
const NAMESPACE: &'static str = "cad11d";

pub fn compute_address(name: &str, resource: Resource) -> String {
    let mut sha = Sha512::new();
    sha.input(name.as_bytes());

    String::from(NAMESPACE) + &resource_to_byte(resource) + &sha.result_str()[..62].to_string()
}

////from transaction.rs
////
/// The dgc namespace prefix for global state (cad11d)
const DGC_NAMESPACE: &'static str = "cad11d";

/// Returns a hex string representation of the supplied bytes
///
/// # Arguments
///
/// * `b` - input bytes
fn bytes_to_hex_str(b: &[u8]) -> String {
    b.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("")
}

/// Returns a state address for a given agent name
///
/// # Arguments
///
/// * `name` - the agent's name
pub fn compute_agent_address(name: &str) -> String {
    let hash: &mut [u8] = &mut [0; 64];

    let mut sha = Sha512::new();
    sha.input(name.as_bytes());
    sha.result(hash);

    String::from(DGC_NAMESPACE) + &resource_to_byte(Resource::AGENT)
        + &bytes_to_hex_str(hash)[..62]
}

/// Returns a state address for a given organization id
///
/// # Arguments
///
/// * `id` - the organization's id
fn compute_org_address(id: &str) -> String {
    let hash: &mut [u8] = &mut [0; 64];

    let mut sha = Sha512::new();
    sha.input(id.as_bytes());
    sha.result(hash);

    String::from(DGC_NAMESPACE) + &resource_to_byte(Resource::ORG)
        + &bytes_to_hex_str(hash)[..62]
}

