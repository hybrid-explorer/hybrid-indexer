use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedSender},
    watch::Receiver,
};
use tokio_tungstenite::tungstenite;

use log::{error, info};

use subxt::backend::legacy::LegacyRpcMethods;
use zerocopy::FromBytes;

use crate::shared::*;

pub fn process_msg_status(trees: &Trees) -> Result<ResponseMessage, IndexError> {
    Ok(ResponseMessage::Status {
        last_head_block: match trees.root.get("last_head_block")? {
            Some(value) => u32::from_be_bytes(value.as_ref().try_into().unwrap()),
            None => 0,
        },
        last_batch_block: match trees.root.get("last_batch_block")? {
            Some(value) => u32::from_be_bytes(value.as_ref().try_into().unwrap()),
            None => 0,
        },
        batch_indexing_complete: match trees.root.get("batch_indexing_complete")? {
            Some(value) => value.to_vec()[0] == 1,
            None => false,
        },
    })
}

pub async fn process_msg_variants<R: RuntimeIndexer>(
    rpc: &LegacyRpcMethods<R::RuntimeConfig>,
) -> Result<ResponseMessage, IndexError> {
    let metadata = rpc.state_get_metadata(None).await?;
    let mut pallets = Vec::new();

    for pallet in metadata.pallets() {
        let mut pallet_meta = PalletMeta {
            index: pallet.index(),
            name: pallet.name().to_owned(),
            events: Vec::new(),
        };

        if let Some(variants) = pallet.event_variants() {
            for variant in variants {
                pallet_meta.events.push(EventMeta {
                    index: variant.index,
                    name: variant.name.clone(),
                })
            }
            pallets.push(pallet_meta);
        }
    }
    Ok(ResponseMessage::Variants(pallets))
}

pub fn process_msg_get_events(trees: &Trees, key: Key) -> ResponseMessage {
    let mut events = Vec::new();
    match key {
        Key::Variant(pallet_id, variant_id) => {
            let mut iter = trees.variant.scan_prefix([pallet_id, variant_id]).keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key: VariantKey = VariantKey::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::AccountId(account_id) => {
            let mut iter = trees.account_id.scan_prefix(account_id).keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = Bytes32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::AccountIndex(account_index) => {
            let mut iter = trees
                .account_index
                .scan_prefix(account_index.to_be_bytes())
                .keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = U32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::AuctionIndex(auction_index) => {
            let mut iter = trees
                .auction_index
                .scan_prefix(auction_index.to_be_bytes())
                .keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = U32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::BountyIndex(bounty_index) => {
            let mut iter = trees
                .bounty_index
                .scan_prefix(bounty_index.to_be_bytes())
                .keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = U32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::CandidateHash(candidate_hash) => {
            let mut iter = trees.candidate_hash.scan_prefix(candidate_hash).keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = Bytes32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::EraIndex(era_index) => {
            let mut iter = trees.era_index.scan_prefix(era_index.to_be_bytes()).keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = U32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::MessageId(message_id) => {
            let mut iter = trees.message_id.scan_prefix(message_id).keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = Bytes32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::ParaId(para_id) => {
            let mut iter = trees.para_id.scan_prefix(para_id.to_be_bytes()).keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = U32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::PoolId(pool_id) => {
            let mut iter = trees.pool_id.scan_prefix(pool_id.to_be_bytes()).keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = U32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::PreimageHash(preimage_hash) => {
            let mut iter = trees.preimage_hash.scan_prefix(preimage_hash).keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = Bytes32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::ProposalHash(proposal_hash) => {
            let mut iter = trees.proposal_hash.scan_prefix(proposal_hash).keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = Bytes32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::ProposalIndex(proposal_index) => {
            let mut iter = trees
                .proposal_index
                .scan_prefix(proposal_index.to_be_bytes())
                .keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = U32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::RefIndex(ref_index) => {
            let mut iter = trees.ref_index.scan_prefix(ref_index.to_be_bytes()).keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = U32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::RegistrarIndex(registrar_index) => {
            let mut iter = trees
                .registrar_index
                .scan_prefix(registrar_index.to_be_bytes())
                .keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = U32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::SessionIndex(session_index) => {
            let mut iter = trees
                .session_index
                .scan_prefix(session_index.to_be_bytes())
                .keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = U32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
        Key::TipHash(tip_hash) => {
            let mut iter = trees.tip_hash.scan_prefix(tip_hash).keys();

            while let Some(Ok(key)) = iter.next_back() {
                let key = Bytes32Key::read_from(&key).unwrap();

                events.push(Event {
                    block_number: key.block_number.into(),
                    event_index: key.event_index.into(),
                });

                if events.len() == 100 {
                    break;
                }
            }
        }
    };
    ResponseMessage::Events { key, events }
}

pub async fn process_msg<R: RuntimeIndexer>(
    rpc: &LegacyRpcMethods<R::RuntimeConfig>,
    trees: &Trees,
    msg: RequestMessage,
    sub_tx: UnboundedSender<SubscribeMessage>,
    sub_response_tx: UnboundedSender<ResponseMessage>,
) -> Result<ResponseMessage, IndexError> {
    Ok(match msg {
        RequestMessage::Status => process_msg_status(trees)?,
        RequestMessage::Variants => process_msg_variants::<R>(rpc).await?,
        RequestMessage::GetEvents { key } => process_msg_get_events(trees, key),
        RequestMessage::SubscribeEvents { key } => {
            let msg = SubscribeMessage {
                key,
                sub_response_tx,
            };
            sub_tx.send(msg).unwrap();
            ResponseMessage::Subscribed
        }
    })
}

async fn handle_connection<R: RuntimeIndexer>(
    rpc: LegacyRpcMethods<R::RuntimeConfig>,
    raw_stream: TcpStream,
    addr: SocketAddr,
    trees: Trees,
    sub_tx: UnboundedSender<SubscribeMessage>,
) -> Result<(), IndexError> {
    info!("Incoming TCP connection from: {}", addr);
    let ws_stream = tokio_tungstenite::accept_async(raw_stream).await?;
    info!("WebSocket connection established: {}", addr);

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    // Create the channel for the substrate thread to send event messages to this thread.
    let (sub_events_tx, mut sub_events_rx) = unbounded_channel();

    loop {
        tokio::select! {
            Some(Ok(msg)) = ws_receiver.next() => {
                if msg.is_text() || msg.is_binary() {
                    match serde_json::from_str(msg.to_text()?) {
                        Ok(request_json) => {
                            let response_msg = process_msg::<R>(&rpc, &trees, request_json, sub_tx.clone(), sub_events_tx.clone()).await?;
                            let response_json = serde_json::to_string(&response_msg).unwrap();
                            ws_sender.send(tungstenite::Message::Text(response_json)).await?;
                        },
                        Err(error) => error!("{}", error),
                    }
                }
            },
            Some(msg) = sub_events_rx.recv() => {
                let response_json = serde_json::to_string(&msg).unwrap();
                ws_sender.send(tungstenite::Message::Text(response_json)).await?;
            },
        }
    }
}

pub async fn websockets_listen<R: RuntimeIndexer + 'static>(
    trees: Trees,
    rpc: LegacyRpcMethods<R::RuntimeConfig>,
    port: u16,
    mut exit_rx: Receiver<bool>,
    sub_tx: UnboundedSender<SubscribeMessage>,
) {
    let mut addr = "0.0.0.0:".to_string();
    addr.push_str(&port.to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    loop {
        tokio::select! {
            biased;

            _ = exit_rx.changed() => {
                break;
            }
            Ok((stream, addr)) = listener.accept() => {
                tokio::spawn(handle_connection::<R>(
                    rpc.clone(),
                    stream,
                    addr,
                    trees.clone(),
                    sub_tx.clone(),
                ));
            }
        }
    }
}
