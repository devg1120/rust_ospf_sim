use std::env;

//use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use futures_util::StreamExt;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};

use std::sync::Arc;

//use std::str::FromStr;

use futures_util::SinkExt;
use log::*;
use std::{net::SocketAddr, time::Duration};
use tokio::net::{TcpListener, TcpStream};
//use tokio_tungstenite::{accept_async, tungstenite::Error, tungstenite::Message, tungstenite::Result};
use tokio_tungstenite::{accept_async, tungstenite::Message, tungstenite::Result};

//use std::str::FromStr;


fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    return a.to_string();
}

pub async fn start(arc_map: Arc<std::sync::Mutex<HashMap<IpAddr, super::Target>>>) -> Result<()> {
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");

        tokio::spawn(handle_connection(arc_map.clone(), peer, stream));
    }

    Ok(())
}

async fn handle_connection(
    arc_map: Arc<std::sync::Mutex<HashMap<IpAddr, super::Target>>>,
    peer: SocketAddr,
    stream: TcpStream,
) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    //let interval = tokio::time::interval(Duration::from_millis(1000));
    let _interval = tokio::time::interval(Duration::from_millis(1000));

    loop {
        tokio::select! {
                    msg = ws_receiver.next() => {
                        match msg {
                            Some(msg) => {
                                let msg = msg?;
                                if msg.is_text() ||msg.is_binary() {
                                   println!("msg : {}",msg);
                                   println!("type : {}",type_of(&msg));

                                   let some_cmd_target = match &msg {
                                            Message::Text(s) => {
                                             let args: Vec<String> = s.split(' ').map(|x| x.to_string())
                                                                                .collect();
                                                       println!("{:?}", args);
                                                      None
                                            },
                                            Message::Binary(v) => {
                                                       println!("{:?}",v);
                                                       let str = v.iter().map(|&s| s as char).collect::<String>();
                                                       println!("{}",str);
                                                       let args: Vec<String> = str.trim().split(' ').map(|x| x.to_string())
                                                                                .collect();
                                                       println!("{:?}", args);
                                                       Some(args)


                                            },
                                            _ => {
                                                      None
                                            },
                                   };

                                   let target = match some_cmd_target {
                                         Some(t) => t,
                                         None => continue,
                                   };

                                   let cmd = target.get(0);
                                   let ipa = target.get(1);
                                   println!("cmd:{:?}", cmd);
                                   println!("ipaddr:{:?}", ipa);

                                   let (cf_, cmd_) = match cmd {
                                                Some(cmd2) => {
                                                        match cmd2.as_str() {
                                                                 "start" =>(true, "start"),
                                                                 "stop" => (true, "stop"),
                                                                 _ => (false,""),
                                                                }
                                                        },
                                                 None => (false, ""),
                                        };
                                   
                                                                      
                                   if !cf_ {
                                       println!("--- cmd error continue");
                                       continue;
                                   };

                                   let d_addr_ = "127.0.0.1".parse::<IpAddr>();
                                   let d_ipaddr = 
                                                match d_addr_ {
                                                   Ok(a) => a,
                                                   Err(v) => continue,
                                                            };

                                   let (if_, ipaddr) = match ipa {
                                                Some(ipa2) => {
                                                         //let addr_ = "10.1.0.2".parse::<IpAddr>();
                                                         let addr_ = ipa2.parse::<IpAddr>();
                                                                      match addr_ {
                                                                         Ok(a) => (true,a),
                                                                         Err(v) => (false,d_ipaddr),
                                                                                  }
                                                              },

                                                 None => (false, d_ipaddr),
                                        };
                                   
                                                                      
                                   if !if_ {
                                       println!("--- ipaddr error continue");
                                       continue;
                                   };

                                   ws_sender.send(msg).await?;
                                   ws_sender.send(Message::text("Ok".to_string())).await?;

                                   let rtx_ = {
                                                 //let  ipaddr = IpAddr::V4(Ipv4Addr::new(10, 1, 0, 2));
                                                 //let  ipaddr = IpAddr::from_str("10.1.0.2");
                                                 //let addr_ = "10.1.0.2".parse::<IpAddr>();
                                                 //let ipaddr = 
                                                 //             match addr_ {
                                                 //                Ok(a) => a,
                                                 //                Err(v) => continue,
                                                 //           };
                                                 let map = arc_map.lock().unwrap();
                                                 let tmp_tx = if map.contains_key(&ipaddr) {
                                                               match map.get(&ipaddr) {
                                                                   Some(target) => {
                                                                             println!("map ok ");
                                                                             //Some(target.tx.clone())
                                                                             Some(target.tx.clone())
                                                                           },
                                                                   None => {
                                                                             println!("map ng ");
                                                                             None
                                                                   }
                                                               }
                                                     } else {
                                                          None
                                                     };
                                              tmp_tx
                                             }; // arc_map.locked  drop ....

                                   match rtx_ {

                                       Some(tx) => {
                                                     let cmd = super::Command::Cmd {
                                                         //key: "*command*".to_string(),
                                                         key: cmd_.to_string(),
                                                     };
                                                     tx.send(cmd).await.unwrap();
                                                                println!("tx send ok ");

                                                  },
                                       None => {
                                              continue;
                                             },

                                 };
                                 

                                } else if msg.is_close() {
                                    break;
                                }
                            }
                            None => break,
                        }
                    }
                   // _ = interval.tick() => {
                   //     ws_sender.send(Message::Text("tick".to_owned())).await?;
                   // }
                }
    }

    Ok(())
}
