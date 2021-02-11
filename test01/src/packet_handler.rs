//extern crate packet;

//use pnet::datalink::{ NetworkInterface};

//use pnet::packet::arp::ArpPacket;
//use pnet::packet::ethernet::{EtherTypes, EthernetPacket };
use pnet::packet::icmp::{self, echo_reply, echo_request, IcmpPacket, IcmpTypes};
//use pnet::packet::icmpv6::Icmpv6Packet;
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv4::{self, MutableIpv4Packet};
//use pnet::packet::ipv6::Ipv6Packet;
//use pnet::packet::tcp::TcpPacket;
//use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
//use pnet::util::MacAddr;

//use tokio_tun::Tun;
//use tokio::io::WriteHalf;
//use tokio::io::AsyncReadExt;
//use tokio::io::AsyncWriteExt;
//use tokio::io::Write;
//use tokio_tun::TunBuilder;

//use std::env;
//use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
//use std::process;

/*
fn handle_udp_packet(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8], writer: &tokio::io::WriteHalf<Tun>) {
    let udp = UdpPacket::new(packet);

    if let Some(udp) = udp {
        println!(
            "[{}]: UDP Packet: {}:{} > {}:{}; length: {}",
            interface_name,
            source,
            udp.get_source(),
            destination,
            udp.get_destination(),
            udp.get_length()
        );
    } else {
        println!("[{}]: Malformed UDP Packet", interface_name);
    }
}
*/
/*
fn send_ipv4_packet(
    source: Ipv4Addr,
    destination: Ipv4Addr,
    protocol: IpNextHeaderProtocol,
    payload: &[u8],
    //vlan_id: u16,
    //chan: Arc<Chan>,
    //tx: &TxSender,
    writer: &mut tokio::io::WriteHalf<Tun>,
) {
    println!("send");
    let buf_size = MutableIpv4Packet::minimum_packet_size() + payload.len();
    let mut ip_packet = MutableIpv4Packet::owned(vec![0u8; buf_size]).unwrap();

    ip_packet.set_version(4);
    ip_packet.set_header_length(5); // 5 × 32 bits = 160 bits = 20 bytes
    ip_packet.set_dscp(0); // DF - Default Forwarding
    ip_packet.set_ecn(0);
    ip_packet.set_total_length(buf_size as u16);
    ip_packet.set_identification(0);
    ip_packet.set_flags(2); // 010 - DF bit set
    ip_packet.set_fragment_offset(0);
    ip_packet.set_ttl(64);
    ip_packet.set_next_level_protocol(protocol);
    ip_packet.set_source(source);
    ip_packet.set_destination(destination);
    ip_packet.set_payload(payload);
    let checksum = ipv4::checksum(&ip_packet.to_immutable());
    ip_packet.set_checksum(checksum);

    //rx.WriteHalf::write(ip_packet.packet());
    //writer.write(ip_packet.packet());
    //let r = writer.write(ip_packet.packet());
    //writer.flush();
    writer.write_all(ip_packet.packet());
    writer.flush();
    //println!("send:{:?}", ret);

    //match ret {
    //            Ok(n) => println!("ok"),
    //            Err(err) => println!("err"),
    //        }
    //let neighbor_info = NeighborInfo::ResolveIp(IpAddr::V4(destination), vlan_id);

    //if vlan_id == 0 {
    //    send_ethernet_packet(
    //        EtherTypes::Ipv4,
    //        ip_packet.packet(),
    //        chan,
    //        tx,
    //        neighbor_info,
    //    );
    //} else {
    //    send_vlan_packet(
    //        EtherTypes::Ipv4,
    //        vlan_id,
    //        ip_packet.packet(),
    //        chan,
    //        tx,
    //        neighbor_info,
    //    );
    //};
}
*/

fn make_ipv4_packet(
    source: Ipv4Addr,
    destination: Ipv4Addr,
    protocol: IpNextHeaderProtocol,
    payload: &[u8],
) -> Vec<u8> {
    println!("send");
    let buf_size = MutableIpv4Packet::minimum_packet_size() + payload.len();
    let mut ip_packet = MutableIpv4Packet::owned(vec![0u8; buf_size]).unwrap();

    ip_packet.set_version(4);
    ip_packet.set_header_length(5); // 5 × 32 bits = 160 bits = 20 bytes
    ip_packet.set_dscp(0); // DF - Default Forwarding
    ip_packet.set_ecn(0);
    ip_packet.set_total_length(buf_size as u16);
    ip_packet.set_identification(0);
    ip_packet.set_flags(2); // 010 - DF bit set
    ip_packet.set_fragment_offset(0);
    ip_packet.set_ttl(64);
    ip_packet.set_next_level_protocol(protocol);
    ip_packet.set_source(source);
    ip_packet.set_destination(destination);
    ip_packet.set_payload(payload);
    let checksum = ipv4::checksum(&ip_packet.to_immutable());
    ip_packet.set_checksum(checksum);

    ip_packet.packet().to_vec()
}

fn make_handle_icmp_packet(
    interface_name: &str,
    source: IpAddr,
    destination: IpAddr,
    packet: &[u8],
) -> Option<Vec<u8>> {
    let icmp_packet = IcmpPacket::new(packet);
    if let Some(icmp_packet) = icmp_packet {
        match icmp_packet.get_icmp_type() {
            IcmpTypes::EchoReply => {
                let echo_reply_packet = echo_reply::EchoReplyPacket::new(packet).unwrap();
                println!(
                    "[{}]: ICMP echo reply {} -> {} (seq={:?}, id={:?})",
                    interface_name,
                    source,
                    destination,
                    echo_reply_packet.get_sequence_number(),
                    echo_reply_packet.get_identifier()
                );
                None
            }
            IcmpTypes::EchoRequest => {
                println!("recv");
                let echo_request_packet = echo_request::EchoRequestPacket::new(packet).unwrap();
                println!(
                    "[{}]: ICMP echo request {} -> {} (seq={:?}, id={:?})",
                    interface_name,
                    source,
                    destination,
                    echo_request_packet.get_sequence_number(),
                    echo_request_packet.get_identifier()
                );
                let mut echo_reply_packet =
                    echo_reply::MutableEchoReplyPacket::owned(packet.to_vec()).unwrap();
                echo_reply_packet.set_icmp_type(IcmpTypes::EchoReply);
                let icmp_packet = IcmpPacket::new(echo_reply_packet.packet()).unwrap();
                let checksum = icmp::checksum(&icmp_packet);
                echo_reply_packet.set_checksum(checksum);
                match (source, destination) {
                    (IpAddr::V4(source), IpAddr::V4(destination)) => Some(make_ipv4_packet(
                        destination,
                        source,
                        IpNextHeaderProtocols::Icmp,
                        echo_reply_packet.packet(),
                    )),
                    _ => None,
                }
                //////////////////////////////////////////
            }
            _ => {
                println!(
                    "[{}]: ICMP packet {} -> {} (type={:?})",
                    interface_name,
                    source,
                    destination,
                    icmp_packet.get_icmp_type()
                );
                None
            }
        }
    } else {
        println!("[{}]: Malformed ICMP Packet", interface_name);
        None
    }
}

/*
fn handle_icmp_packet(interface_name: &str, source: IpAddr, destination: IpAddr,
packet: &[u8],
recv_packet: &[u8],
 writer: &mut tokio::io::WriteHalf<Tun>) {

    writer.write_all(&recv_packet);

    let icmp_packet = IcmpPacket::new(packet);
    if let Some(icmp_packet) = icmp_packet {
        match icmp_packet.get_icmp_type() {
            IcmpTypes::EchoReply => {
                let echo_reply_packet = echo_reply::EchoReplyPacket::new(packet).unwrap();
                println!(
                    "[{}]: ICMP echo reply {} -> {} (seq={:?}, id={:?})",
                    interface_name,
                    source,
                    destination,
                    echo_reply_packet.get_sequence_number(),
                    echo_reply_packet.get_identifier()
                );
            }
            IcmpTypes::EchoRequest => {
                 println!("recv");
                 writer.write_all(&recv_packet);
                let echo_request_packet = echo_request::EchoRequestPacket::new(packet).unwrap();
                println!(
                    "[{}]: ICMP echo request {} -> {} (seq={:?}, id={:?})",
                    interface_name,
                    source,
                    destination,
                    echo_request_packet.get_sequence_number(),
                    echo_request_packet.get_identifier()
                );
         let mut echo_reply_packet =

                    echo_reply::MutableEchoReplyPacket::owned(packet.to_vec()).unwrap();
                echo_reply_packet.set_icmp_type(IcmpTypes::EchoReply);
                let icmp_packet = IcmpPacket::new(echo_reply_packet.packet()).unwrap();
                let checksum = icmp::checksum(&icmp_packet);
                echo_reply_packet.set_checksum(checksum);
                match (source, destination) {
                       (IpAddr::V4(source), IpAddr::V4(destination)) => {
                                         send_ipv4_packet(
                                             destination,
                                             source,
                                             IpNextHeaderProtocols::Icmp,
                                             echo_reply_packet.packet(),
                                             //vlan_id,
                                             //chan.clone(),
                                             writer,
                                         );
                       },
                        _ => {
                        }
                }
                //////////////////////////////////////////
            }
            _ => println!(
                "[{}]: ICMP packet {} -> {} (type={:?})",
                interface_name,
                source,
                destination,
                icmp_packet.get_icmp_type()
            ),
        }
    } else {
        println!("[{}]: Malformed ICMP Packet", interface_name);
    }
}
*/
/*
fn handle_icmpv6_packet(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8], writer: &tokio::io::WriteHalf<Tun>) {
    let icmpv6_packet = Icmpv6Packet::new(packet);
    if let Some(icmpv6_packet) = icmpv6_packet {
        println!(
            "[{}]: ICMPv6 packet {} -> {} (type={:?})",
            interface_name,
            source,
            destination,
            icmpv6_packet.get_icmpv6_type()
        )
    } else {
        println!("[{}]: Malformed ICMPv6 Packet", interface_name);
    }
}
*/
/*
fn handle_tcp_packet(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8], writer: &tokio::io::WriteHalf<Tun>) {
    let tcp = TcpPacket::new(packet);
    if let Some(tcp) = tcp {
        println!(
            "[{}]: TCP Packet: {}:{} > {}:{}; length: {}",
            interface_name,
            source,
            tcp.get_source(),
            destination,
            tcp.get_destination(),
            packet.len()
        );
    } else {
        println!("[{}]: Malformed TCP Packet", interface_name);
    }
}
*/
/*
pub fn handle_transport_protocol(
    interface_name: &str,
    source: IpAddr,
    destination: IpAddr,
    protocol: IpNextHeaderProtocol,
    packet: &[u8],
    recv_packet: &[u8],
    writer: &mut tokio::io::WriteHalf<Tun>,
) {
    match protocol {
        IpNextHeaderProtocols::Udp => {
            handle_udp_packet(interface_name, source, destination, packet, writer)
        }
        IpNextHeaderProtocols::Tcp => {
            handle_tcp_packet(interface_name, source, destination, packet, writer)
        }
        IpNextHeaderProtocols::Icmp => {
            handle_icmp_packet(interface_name, source, destination, packet, recv_packet, writer)
        }
        IpNextHeaderProtocols::Icmpv6 => {
            handle_icmpv6_packet(interface_name, source, destination, packet, writer)
        }
        _ => println!(
            "[{}]: Unknown {} packet: {} > {}; protocol: {:?} length: {}",
            interface_name,
            match source {
                IpAddr::V4(..) => "IPv4",
                _ => "IPv6",
            },
            source,
            destination,
            protocol,
            packet.len()
        ),
    }
}
*/

pub fn make_handle_transport_protocol(
    interface_name: &str,
    source: IpAddr,
    destination: IpAddr,
    protocol: IpNextHeaderProtocol,
    packet: &[u8],
) -> Option<Vec<u8>> {
    match protocol {
        IpNextHeaderProtocols::Udp => {
            //handle_udp_packet(interface_name, source, destination, packet, writer)
            None
        }
        IpNextHeaderProtocols::Tcp => {
            //handle_tcp_packet(interface_name, source, destination, packet, writer)
            None
        }
        IpNextHeaderProtocols::Icmp => {
            make_handle_icmp_packet(interface_name, source, destination, packet)
        }
        IpNextHeaderProtocols::Icmpv6 => {
            // handle_icmpv6_packet(interface_name, source, destination, packet, writer)
            None
        }
        _ => None,
    }
}
/*
pub async fn handle_transport_protocol2(
    interface_name: &str,
    source: IpAddr,
    destination: IpAddr,
    protocol: IpNextHeaderProtocol,
    packet: &[u8],
    recv_packet: &[u8],
    //writer: &mut tokio::io::WriteHalf<Tun>,
    tun:  Tun,
)  {
      let (mut reader, mut _writer) = tokio::io::split(tun);

       //_writer.write(&recv_packet);
}

pub fn handle_ipv4_packet(interface_name: &str, ethernet: &EthernetPacket, writer: &mut tokio::io::WriteHalf<Tun>) {
    let header = Ipv4Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            interface_name,
            IpAddr::V4(header.get_source()),
            IpAddr::V4(header.get_destination()),
            header.get_next_level_protocol(),
            header.payload(),
            header.payload(),
            writer,
        );
    } else {
        println!("[{}]: Malformed IPv4 Packet", interface_name);
    }
}

pub fn handle_ipv6_packet(interface_name: &str, ethernet: &EthernetPacket, writer: &mut tokio::io::WriteHalf<Tun>) {
    let header = Ipv6Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            interface_name,
            IpAddr::V6(header.get_source()),
            IpAddr::V6(header.get_destination()),
            header.get_next_header(),
            header.payload(),
            header.payload(),
            writer,
        );
    } else {
        println!("[{}]: Malformed IPv6 Packet", interface_name);
    }
}

pub fn handle_arp_packet(interface_name: &str, ethernet: &EthernetPacket,  writer: &mut tokio::io::WriteHalf<Tun>) {
    let header = ArpPacket::new(ethernet.payload());
    if let Some(header) = header {
        println!(
            "[{}]: ARP packet: {}({}) > {}({}); operation: {:?}",
            interface_name,
            ethernet.get_source(),
            header.get_sender_proto_addr(),
            ethernet.get_destination(),
            header.get_target_proto_addr(),
            header.get_operation()
        );
    } else {
        println!("[{}]: Malformed ARP Packet", interface_name);
    }
}

pub fn handle_ethernet_frame(interface: &NetworkInterface, ethernet: &EthernetPacket, writer: &mut tokio::io::WriteHalf<Tun>) {
    let interface_name = &interface.name[..];
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => handle_ipv4_packet(interface_name, ethernet, writer),
        EtherTypes::Ipv6 => handle_ipv6_packet(interface_name, ethernet, writer),
        EtherTypes::Arp => handle_arp_packet(interface_name, ethernet, writer),
        _ => println!(
            "[{}]: Unknown packet: {} > {}; ethertype: {:?} length: {}",
            interface_name,
            ethernet.get_source(),
            ethernet.get_destination(),
            ethernet.get_ethertype(),
            ethernet.packet().len()
        ),
    }
}
*/

/*
fn main() {
    use pnet::datalink::Channel::Ethernet;

    let iface_name = match env::args().nth(1) {
        Some(n) => n,
        None => {
            writeln!(io::stderr(), "USAGE: packetdump <NETWORK INTERFACE>").unwrap();
            process::exit(1);
        }
    };
    let interface_names_match = |iface: &NetworkInterface| iface.name == iface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap_or_else(|| panic!("No such network interface: {}", iface_name));

    // Create a channel to receive on
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("packetdump: unhandled channel type: {}"),
        Err(e) => panic!("packetdump: unable to create channel: {}", e),
    };

    loop {
        let mut buf: [u8; 1600] = [0u8; 1600];
        let mut fake_ethernet_frame = MutableEthernetPacket::new(&mut buf[..]).unwrap();
        match rx.next() {
            Ok(packet) => {
                let payload_offset;
                if cfg!(any(target_os = "macos", target_os = "ios"))
                    && interface.is_up()
                    && !interface.is_broadcast()
                    && ((!interface.is_loopback() && interface.is_point_to_point())
                        || interface.is_loopback())
                {
                    if interface.is_loopback() {
                        // The pnet code for BPF loopback adds a zero'd out Ethernet header
                        payload_offset = 14;
                    } else {
                        // Maybe is TUN interface
                        payload_offset = 0;
                    }
                    if packet.len() > payload_offset {
                        let version = Ipv4Packet::new(&packet[payload_offset..])
                            .unwrap()
                            .get_version();
                        if version == 4 {
                            fake_ethernet_frame.set_destination(MacAddr(0, 0, 0, 0, 0, 0));
                            fake_ethernet_frame.set_source(MacAddr(0, 0, 0, 0, 0, 0));
                            fake_ethernet_frame.set_ethertype(EtherTypes::Ipv4);
                            fake_ethernet_frame.set_payload(&packet[payload_offset..]);
                            handle_ethernet_frame(&interface, &fake_ethernet_frame.to_immutable());
                            continue;
                        } else if version == 6 {
                            fake_ethernet_frame.set_destination(MacAddr(0, 0, 0, 0, 0, 0));
                            fake_ethernet_frame.set_source(MacAddr(0, 0, 0, 0, 0, 0));
                            fake_ethernet_frame.set_ethertype(EtherTypes::Ipv6);
                            fake_ethernet_frame.set_payload(&packet[payload_offset..]);
                            handle_ethernet_frame(&interface, &fake_ethernet_frame.to_immutable());
                            continue;
                        }
                    }
                }
                handle_ethernet_frame(&interface, &EthernetPacket::new(packet).unwrap());
            }
            Err(e) => panic!("packetdump: unable to receive packet: {}", e),
        }
    }
}
*/
