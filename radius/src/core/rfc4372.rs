// Code generated by machine generator; DO NOT EDIT.

use crate::core::avp::{AVPType, AVP};
use crate::core::packet::Packet;

pub const CHARGEABLE_USER_IDENTITY_TYPE: AVPType = 89;
pub fn delete_chargeable_user_identity(packet: &mut Packet) {
    packet.delete(CHARGEABLE_USER_IDENTITY_TYPE);
}
pub fn add_chargeable_user_identity(packet: &mut Packet, value: &[u8]) {
    packet.add(AVP::from_bytes(CHARGEABLE_USER_IDENTITY_TYPE, value));
}
pub fn lookup_chargeable_user_identity(packet: &Packet) -> Option<Vec<u8>> {
    packet
        .lookup(CHARGEABLE_USER_IDENTITY_TYPE)
        .map(|v| v.encode_bytes())
}
pub fn lookup_all_chargeable_user_identity(packet: &Packet) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(CHARGEABLE_USER_IDENTITY_TYPE) {
        vec.push(avp.encode_bytes())
    }
    vec
}
