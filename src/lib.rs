// These are the substreams imports
use substreams::handlers::map;
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

// Base58 for pubkey encoding
use bs58;

// To include the generated Protobuf modules
pub mod pb {
    pub mod wormhole {
        include!(concat!(env!("OUT_DIR"), "/wormhole.v1.rs"));
    }
    pub mod spl {
        include!(concat!(env!("OUT_DIR"), "/spl.v1.rs"));
    }
    pub mod mydata {
        include!(concat!(env!("OUT_DIR"), "/mydata.v1.rs"));
    }
}

use pb::wormhole::{Instruction as WhInst, Instructions, WormholeMessage};
use pb::spl::MintOrBurnEvent;
use pb::mydata::MyData;

// This filters Wormhole Core & Bridge instructions
#[map]
fn map_filter_wormhole(blk: Block) -> Result<Instructions, Error> {
    let mut out = Instructions { instructions: Vec::new() };
    for tx in blk.transactions {
        let msg = tx.transaction.as_ref().unwrap().message.as_ref().unwrap();
        let keys = tx.resolved_accounts();
        for inst in &msg.instructions {
            let pid = bs58::encode(&keys[inst.program_id_index as usize]).into_string();
            if pid == "worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth"
               || pid == "wormDTUJ6AWPNvk59vGQbDvGJmqbDTdgWgAqcLBCgUb"
            {
                out.instructions.push(WhInst {
                    program_id: pid,
                    accounts: inst.accounts.iter()
                        .map(|i| bs58::encode(&keys[*i as usize]).into_string())
                        .collect(),
                    data: inst.data.clone(),
                });
            }
        }
    }
    Ok(out)
}

// To decode first Wormhole “PostMessage” per block → WormholeMessage
#[map]
fn map_decode_wormhole(insts: Instructions) -> Result<WormholeMessage, Error> {
    for inst in insts.instructions {
        if inst.program_id == "worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth"
           && inst.data.get(0) == Some(&1)
        {
            let len = u32::from_le_bytes(inst.data[1..5].try_into().unwrap()) as usize;
            let payload = inst.data[5..5 + len].to_vec();
            let seq_off = 5 + len;
            let sequence = u64::from_le_bytes(inst.data[seq_off..seq_off + 8].try_into().unwrap());
            return Ok(WormholeMessage {
                emitter: inst.accounts[1].clone(),
                sequence,
                payload,
            });
        }
    }
    Ok(WormholeMessage::default())
}

// Manually decoding SPL Token mint/burns
#[map]
fn map_spl_mint_burn(blk: Block) -> Result<MintOrBurnEvent, Error> {
    // Loop for traversing each instruction where program_id == SPL Token program
    for tx in blk.transactions {
        let msg = tx.transaction.as_ref().unwrap().message.as_ref().unwrap();
        let keys = tx.resolved_accounts();
        for inst in &msg.instructions {
            let pid = bs58::encode(&keys[inst.program_id_index as usize]).into_string();
            if pid == "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" {
                let data = &inst.data;
                if !data.is_empty() {
                    // 7 = MintTo, 8 = Burn
                    match data[0] {
                        7 if data.len() >= 9 => {
                            let amount = u64::from_le_bytes(data[1..9].try_into().unwrap());
                            let mint = bs58::encode(&keys[inst.accounts[0] as usize]).into_string();
                            let owner = bs58::encode(&keys[inst.accounts[1] as usize]).into_string();
                            return Ok(MintOrBurnEvent {
                                mint,
                                owner,
                                amount,
                                is_mint: true,
                            });
                        }
                        8 if data.len() >= 9 => {
                            let amount = u64::from_le_bytes(data[1..9].try_into().unwrap());
                            let owner = bs58::encode(&keys[inst.accounts[0] as usize]).into_string();
                            let mint = bs58::encode(&keys[inst.accounts[1] as usize]).into_string();
                            return Ok(MintOrBurnEvent {
                                mint,
                                owner,
                                amount,
                                is_mint: false,
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    // No SPL mint/burn ⇒ default
    Ok(MintOrBurnEvent::default())
}

// This is the basic block metadata
#[map]
fn map_my_data(blk: Block) -> MyData {
    MyData {
        block_hash:      blk.blockhash.clone(),
        block_slot:      blk.slot,
        block_timestamp: blk.block_time.clone().unwrap_or_default().timestamp as u64,
        transactions_len: blk.transactions.len() as u64,
        instructions_len: blk.walk_instructions().count() as u64,
    }
}
