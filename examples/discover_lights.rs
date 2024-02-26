extern crate lifx_lan;
extern crate lifx_serialization;

use std::{collections::HashSet, net::{SocketAddr, UdpSocket}};

use lifx_lan::{messages::Message, request_options::LifxRequestOptions, serialize_lifx_packet, deserialize_lifx_packet};

const LIGHT_COUNT: usize = 4;

fn main () {
    let local_address = "192.168.1.21:56700";
    let broadcast_address = "255.255.255.255:56700";

    let socket = UdpSocket::bind(local_address).unwrap();
    socket.set_broadcast(true).unwrap();

    let mut message_buffer = [0u8; 36];

    let mut req_options = LifxRequestOptions {
        tagged: true,
        source: 0,
        target: [0; 8],
        ack_required: false,
        res_required: false,
        sequence: 0,
    };

    serialize_lifx_packet(&req_options, &Message::GetService, &mut message_buffer);

    match socket.send_to(&message_buffer, broadcast_address) {
        Ok(_) => println!("Broadcasted discovery message!"),
        Err(e) => eprintln!("Failed to send message: {}", e),
    }

    req_options.increment_sequence();

    let mut buf = [0; 128];

    println!("Listening for replies...");

    let mut light_addresses = HashSet::<SocketAddr>::new();

    while light_addresses.len() < LIGHT_COUNT {
        match socket.recv_from(&mut buf) {
            Ok((_size, src)) => {
                let (_header, payload) = deserialize_lifx_packet(&buf).unwrap();

                match payload {
                    Message::Service { service, port: _ } => {
                        if service == 1 {
                            println!("Got UDP Service advertisement from {}", src);

                            light_addresses.insert(src);
                        }
                    },
                    _ => {}
                }
            }
            Err(e) => {
                eprintln!("Failed to receive a datagram: {}", e);
                break;
            }
        }
    }

    serialize_lifx_packet(&req_options, &Message::GetLabel, &mut message_buffer);

    for light_address in light_addresses {
        match socket.send_to(&message_buffer, light_address) {
            Ok(_) => println!("Requesting label from {}", light_address),
            Err(e) => eprintln!("Failed to send message: {}", e),
        }
    }

    loop {
        match socket.recv_from(&mut buf) {
            Ok((_size, src)) => {
                let (_header, payload) = deserialize_lifx_packet(&buf).unwrap();

                match payload {
                    Message::Label { label } => {
                        println!("Got label from {}: {}", src, label);
                    },
                    _ => {}
                }
            }
            Err(e) => {
                eprintln!("Failed to receive a datagram: {}", e);
                break;
            }
        }
    }
    
}