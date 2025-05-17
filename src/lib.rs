// These are the substreams imports
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::{
    Block, ConfirmedTransaction, Transaction, InnerInstructions, Message
};

// Base58 for pubkey encoding
use bs58;

// Include the generated protobuf code
pub mod pb {
    pub mod wormhole_bridge {
        pub mod message {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/wormhole_bridge.message.v1.rs"));
            }
        }
        pub mod token {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/wormhole_bridge.token.v1.rs"));
            }
        }
        pub mod nft {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/wormhole_bridge.nft.v1.rs"));
            }
        }
    }
}

use pb::wormhole_bridge::message::v1::{WormholeMessage, WormholeMessages};
use pb::wormhole_bridge::token::v1::{WormholeTokenTransfer, WormholeTokenTransfers};
use pb::wormhole_bridge::nft::v1::{WormholeNftTransfer, WormholeNftTransfers};

const CORE_BRIDGE_PROGRAM: &str = "worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth";
const TOKEN_BRIDGE_PROGRAM: &str = "wormDTUJ6AWPNvk59vGQbDvGJmqbDTdgWgAqcLBCgUb";
const NFT_BRIDGE_PROGRAM: &str = "WnFt12ZrnzZrFZkt2xsNsaNWoQribnuQ5B5FrDbwDhD";

fn extract_core_message(
    instruction: &InnerInstructions,
    blk: &Block,
    tx: &Transaction,
) -> Option<WormholeMessage> {
    for inst in &instruction.instructions {
        let data = &inst.data;
        if data.get(0) != Some(&1) {  // PostMessage instruction
            continue;
        }

        let len = u32::from_le_bytes(data[1..5].try_into().ok()?);
        let payload = data[5..5 + len as usize].to_vec();
        let seq_off = 5 + len as usize;
        let sequence = u32::from_le_bytes(data[seq_off..seq_off + 4].try_into().ok()?);
        let nonce = u32::from_le_bytes(data[seq_off + 4..seq_off + 8].try_into().ok()?);

        return Some(WormholeMessage {
            transaction_id: bs58::encode(&tx.signatures[0]).into_string(),
            sequence_number: sequence,
            nonce_value: nonce,
            source_address: bs58::encode(&tx.message.as_ref()?.account_keys[inst.accounts[1] as usize]).into_string(),
            consistency_level: data[seq_off + 8] as u32,
            message_payload: payload,
            block_time: blk.block_time.as_ref()?.timestamp as u64,
            block_height: blk.slot,
            source_chain: 1,  // Solana chain ID
        });
    }
    None
}

#[substreams::handlers::map]
fn map_bridge_messages(blk: Block) -> Result<WormholeMessages, Error> {
    let mut messages = vec![];

    // Get block context data once per block
    let block_slot = blk.slot;
    let block_timestamp = blk.block_time.as_ref().map(|t| t.timestamp).unwrap_or(0);

    // Iterate through all transactions in the block
    // Filter out failed transactions early and get a reference to the transaction and its meta
    for (tx_index, (transaction, meta)) in blk.transactions().filter_map(|tx| {
        tx.meta.as_ref()
            .filter(|meta| meta.err.is_none())
            .map(|meta| (tx, meta))
    }).enumerate() {
        // Get the transaction ID (signature) in Base58 format
        let tx_id = bs58::encode(&transaction.id()).into_string();
        
        // Check if this transaction involves the Core Bridge program
        let involves_core_bridge = if let Some(tx_msg) = transaction.transaction.as_ref().and_then(|tx| tx.message.as_ref()) {
            tx_msg.account_keys.iter().any(|key| {
                bs58::encode(key).into_string() == CORE_BRIDGE_PROGRAM
            })
        } else {
            false
        };

        if involves_core_bridge {
            if let Some(tx) = &transaction.transaction {
                if let Some(meta) = &transaction.meta {
                    for (idx, inner_instructions) in meta.inner_instructions.iter().enumerate() {
                        if let Some(instruction) = tx.message.as_ref().and_then(|m| m.instructions.get(idx)) {
                            if let Some(program_id) = tx.message.as_ref().map(|m| &m.account_keys[instruction.program_id_index as usize]) {
                                if bs58::encode(program_id).into_string() == CORE_BRIDGE_PROGRAM {
                                    if let Some(data) = extract_core_message(inner_instructions, &blk, tx) {
                                        messages.push(data);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(WormholeMessages { messages })
}

fn extract_token_transfer(
    instruction: &InnerInstructions,
    blk: &Block,
    tx: &Transaction,
) -> Option<WormholeTokenTransfer> {
    for inst in &instruction.instructions {
        let data = &inst.data;
        if data.is_empty() {
            continue;
        }

        // Extract token transfer data based on instruction
        return Some(WormholeTokenTransfer {
            transaction_id: bs58::encode(&tx.signatures[0]).into_string(),
            transfer_amount: 0,  // Extract from data
            asset_address: String::new(),  // Extract from accounts
            source_chain: "solana".to_string(),
            recipient_address: String::new(),  // Extract from data/accounts
            target_chain: String::new(),  // Extract from data
            sender_address: bs58::encode(&tx.message.as_ref()?.account_keys[0]).into_string(),
            block_time: blk.block_time.as_ref()?.timestamp as u64,
            block_height: blk.slot,
            sequence_number: 0,  // Extract from data
            token_symbol: String::new(),
            token_decimals: 0,
            token_name: String::new(),
        });
    }
    None
}

fn extract_nft_transfer(
    instruction: &InnerInstructions,
    blk: &Block,
    tx: &Transaction,
) -> Option<WormholeNftTransfer> {
    for inst in &instruction.instructions {
        let data = &inst.data;
        if data.is_empty() {
            continue;
        }

        // Extract NFT transfer data based on instruction
        return Some(WormholeNftTransfer {
            transaction_id: bs58::encode(&tx.signatures[0]).into_string(),
            nft_address: String::new(),  // Extract from accounts
            nft_chain: String::new(),
            recipient_address: String::new(),  // Extract from data/accounts
            target_chain: String::new(),  // Extract from data
            sender_address: bs58::encode(&tx.message.as_ref()?.account_keys[0]).into_string(),
            block_time: blk.block_time.as_ref()?.timestamp as u64,
            block_height: blk.slot,
            sequence_number: 0,  // Extract from data
            nft_name: String::new(),
            nft_symbol: String::new(),
            nft_uri: String::new(),
            token_id: 0,
        });
    }
    None
}

#[substreams::handlers::map]
fn map_token_transfers(blk: Block) -> Result<WormholeTokenTransfers, Error> {
    let mut transfers = vec![];

    // Get block context data once per block
    let block_slot = blk.slot;
    let block_timestamp = blk.block_time.as_ref().map(|t| t.timestamp).unwrap_or(0);

    // Iterate through all transactions in the block
    // Filter out failed transactions early and get a reference to the transaction and its meta
    for (tx_index, (transaction, meta)) in blk.transactions().filter_map(|tx| {
        tx.meta.as_ref()
            .filter(|meta| meta.err.is_none())
            .map(|meta| (tx, meta))
    }).enumerate() {
        // Check if this transaction involves the Token Bridge program
        let involves_token_bridge = if let Some(tx_msg) = transaction.transaction.as_ref().and_then(|tx| tx.message.as_ref()) {
            tx_msg.account_keys.iter().any(|key| {
                bs58::encode(key).into_string() == TOKEN_BRIDGE_PROGRAM
            })
        } else {
            false
        };

        if involves_token_bridge {
            if let Some(tx) = &transaction.transaction {
                if let Some(meta) = &transaction.meta {
                    for (idx, inner_instructions) in meta.inner_instructions.iter().enumerate() {
                        if let Some(instruction) = tx.message.as_ref().and_then(|m| m.instructions.get(idx)) {
                            if let Some(program_id) = tx.message.as_ref().map(|m| &m.account_keys[instruction.program_id_index as usize]) {
                                if bs58::encode(program_id).into_string() == TOKEN_BRIDGE_PROGRAM {
                                    if let Some(transfer) = extract_token_transfer(inner_instructions, &blk, tx) {
                                        transfers.push(transfer);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(WormholeTokenTransfers { transfers })
}

#[substreams::handlers::map]
fn map_nft_transfers(blk: Block) -> Result<WormholeNftTransfers, Error> {
    let mut transfers = vec![];

    // Get block context data once per block
    let block_slot = blk.slot;
    let block_timestamp = blk.block_time.as_ref().map(|t| t.timestamp).unwrap_or(0);

    // Iterate through all transactions in the block
    // Filter out failed transactions early and get a reference to the transaction and its meta
    for (tx_index, (transaction, meta)) in blk.transactions().filter_map(|tx| {
        tx.meta.as_ref()
            .filter(|meta| meta.err.is_none())
            .map(|meta| (tx, meta))
    }).enumerate() {
        // Check if this transaction involves the NFT Bridge program
        let involves_nft_bridge = if let Some(tx_msg) = transaction.transaction.as_ref().and_then(|tx| tx.message.as_ref()) {
            tx_msg.account_keys.iter().any(|key| {
                bs58::encode(key).into_string() == NFT_BRIDGE_PROGRAM
            })
        } else {
            false
        };

        if involves_nft_bridge {
            if let Some(tx) = &transaction.transaction {
                if let Some(meta) = &transaction.meta {
                    for (idx, inner_instructions) in meta.inner_instructions.iter().enumerate() {
                        if let Some(instruction) = tx.message.as_ref().and_then(|m| m.instructions.get(idx)) {
                            if let Some(program_id) = tx.message.as_ref().map(|m| &m.account_keys[instruction.program_id_index as usize]) {
                                if bs58::encode(program_id).into_string() == NFT_BRIDGE_PROGRAM {
                                    if let Some(transfer) = extract_nft_transfer(inner_instructions, &blk, tx) {
                                        transfers.push(transfer);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(WormholeNftTransfers { transfers })
}
