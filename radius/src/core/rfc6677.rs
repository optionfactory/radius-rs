// Code generated by machine generator; DO NOT EDIT.

use crate::core::avp::{AVPError, AVPType, AVP};
use crate::core::packet::Packet;

pub const EAP_LOWER_LAYER_TYPE: AVPType = 163;
pub fn delete_eap_lower_layer(packet: &mut Packet) {
    packet.delete(EAP_LOWER_LAYER_TYPE);
}
pub fn add_eap_lower_layer(packet: &mut Packet, value: EapLowerLayer) {
    packet.add(AVP::from_u32(EAP_LOWER_LAYER_TYPE, value as u32));
}
pub fn lookup_eap_lower_layer(packet: &Packet) -> Option<Result<EapLowerLayer, AVPError>> {
    packet
        .lookup(EAP_LOWER_LAYER_TYPE)
        .map(|v| Ok(v.encode_u32()? as EapLowerLayer))
}
pub fn lookup_all_eap_lower_layer(packet: &Packet) -> Result<Vec<EapLowerLayer>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(EAP_LOWER_LAYER_TYPE) {
        vec.push(avp.encode_u32()? as EapLowerLayer)
    }
    Ok(vec)
}

pub type EapLowerLayer = u32;
pub const EAP_LOWER_LAYER_WIRED_IEEE_802_1X: EapLowerLayer = 1;
pub const EAP_LOWER_LAYER_IEEE_802_1X_NO_PREAUTH: EapLowerLayer = 2;
pub const EAP_LOWER_LAYER_IEEE_802_1X_PREAUTH: EapLowerLayer = 3;
pub const EAP_LOWER_LAYER_IEEE_802_1_6E: EapLowerLayer = 4;
pub const EAP_LOWER_LAYER_IK_EV_2: EapLowerLayer = 5;
pub const EAP_LOWER_LAYER_PPP: EapLowerLayer = 6;
pub const EAP_LOWER_LAYER_PANA_NO_PREAUTH: EapLowerLayer = 7;
pub const EAP_LOWER_LAYER_GSS_API: EapLowerLayer = 8;
pub const EAP_LOWER_LAYER_PANA_PREAUTH: EapLowerLayer = 9;
