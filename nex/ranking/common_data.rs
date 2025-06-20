use nex_rs::{client::Client, packet::v1::PacketV1, rmc::RMCResponse};
use ranking::protocol::{RankingProtocol, RankingMethod};
use std::error::Error;

pub fn upload_common_data(
    err: Option<Box<dyn Error>>,
    client: &Client,
    call_id: u32,
    common_data: &[u8],
    unique_id: u64,
) {
    // Create RMC response
    let mut rmc_response = RMCResponse::new(RankingProtocol::PROTOCOL_ID, call_id);
    rmc_response.set_success(RankingMethod::UploadCommonData, None);

    let rmc_response_bytes = rmc_response.to_bytes();

    // Create response packet
    let mut response_packet = PacketV1::new(client, None);
    
    response_packet.set_version(1);
    response_packet.set_source(0xA1);
    response_packet.set_destination(0xAF);
    response_packet.set_type(nex_rs::packet::PacketType::Data);
    response_packet.set_payload(&rmc_response_bytes);

    response_packet.add_flag(nex_rs::packet::PacketFlag::NeedsAck);
    response_packet.add_flag(nex_rs::packet::PacketFlag::Reliable);

    // Send packet
    globals::nex_server().send(response_packet);
}
