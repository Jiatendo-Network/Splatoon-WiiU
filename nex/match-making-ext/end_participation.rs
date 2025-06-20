use std::error::Error;
use nex_rs::{client::Client, packet::{PacketV1, DataPacket}, rmc::RMCResponse, stream::StreamOut};
use globals::NEXServer;
use match_making_ext::{ProtocolID, MethodEndParticipation};

pub fn end_participation(
    err: Option<Box<dyn Error>>,
    client: &Client,
    call_id: u32,
    id_gathering: u32,
    str_message: String,
) {
    // Remove player from room in database
    database::remove_player_from_room(id_gathering, client.jid());

    // Create response stream
    let mut rmc_response_stream = StreamOut::new(NEXServer::get_instance());
    
    // Write success boolean
    rmc_response_stream.write_bool(true).unwrap(); // %retval%

    let rmc_response_body = rmc_response_stream.bytes();

    // Create RMC response
    let mut rmc_response = RMCResponse::new(ProtocolID, call_id);
    rmc_response.set_success(MethodEndParticipation, rmc_response_body);

    let rmc_response_bytes = rmc_response.bytes();

    // Create response packet
    let mut response_packet = PacketV1::new(client, None).unwrap();
    
    response_packet.set_version(1);
    response_packet.set_source(0xA1);
    response_packet.set_destination(0xAF);
    response_packet.set_type(DataPacket);
    response_packet.set_payload(rmc_response_bytes);

    response_packet.add_flag(nex_rs::Flag::NeedsAck);
    response_packet.add_flag(nex_rs::Flag::Reliable);

    // Send packet
    NEXServer::get_instance().send(response_packet);
}

