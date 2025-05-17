use std::collections::HashMap;
use std::sync::OnceLock;

// Cache for known token metadata
static TOKEN_METADATA_CACHE: OnceLock<HashMap<String, TokenMetadata>> = OnceLock::new();

/// Token metadata structure
#[derive(Clone, Debug)]
pub struct TokenMetadata {
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
}

/// Convert Wormhole chain ID to a human-readable name
pub fn chain_id_to_name(chain_id: u16) -> String {
    match chain_id {
        1 => "Solana".to_string(),
        2 => "Ethereum".to_string(),
        3 => "Terra".to_string(),
        4 => "BSC".to_string(),
        5 => "Polygon".to_string(),
        6 => "Avalanche".to_string(),
        7 => "Oasis".to_string(),
        8 => "Algorand".to_string(),
        9 => "Aurora".to_string(),
        10 => "Fantom".to_string(),
        11 => "Karura".to_string(),
        12 => "Acala".to_string(),
        13 => "Klaytn".to_string(),
        14 => "Celo".to_string(),
        15 => "NEAR".to_string(),
        16 => "Moonbeam".to_string(),
        17 => "Neon".to_string(),
        18 => "Terra2".to_string(),
        19 => "Injective".to_string(),
        20 => "Sui".to_string(),
        21 => "Aptos".to_string(),
        22 => "Arbitrum".to_string(),
        23 => "Optimism".to_string(),
        24 => "Gnosis".to_string(),
        26 => "Cosmos".to_string(),
        28 => "Osmosis".to_string(),
        30 => "Base".to_string(),
        _ => format!("Chain-{}", chain_id),
    }
}

/// Format address based on chain format requirements
pub fn format_address_for_chain(chain_id: u16, address: &[u8]) -> String {
    match chain_id {
        // Ethereum and EVM chains use hex with 0x prefix
        2 | 4 | 5 | 6 | 10 | 22 | 23 | 24 | 30 => {
            format!("0x{}", hex::encode(address))
        },
        // Solana uses base58
        1 => {
            bs58::encode(address).into_string()
        },
        // Terra/Cosmos use bech32
        3 | 18 | 26 | 28 => {
            // Simplified for now, would need proper bech32 encoding with prefix
            hex::encode(address)
        },
        // Default to hex encoding without prefix
        _ => hex::encode(address),
    }
}

/// Get token metadata for a given token address
pub fn get_token_metadata(token_mint: &str) -> Option<TokenMetadata> {
    // Initialize cache if not already done
    let cache = TOKEN_METADATA_CACHE.get_or_init(|| {
        let mut map = HashMap::new();
        
        // Add well-known Solana tokens
        map.insert(
            "So11111111111111111111111111111111111111112".to_string(),
            TokenMetadata {
                symbol: "SOL".to_string(),
                name: "Solana".to_string(),
                decimals: 9,
            },
        );
        
        map.insert(
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
            TokenMetadata {
                symbol: "USDC".to_string(),
                name: "USD Coin".to_string(),
                decimals: 6,
            },
        );
        
        map.insert(
            "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB".to_string(),
            TokenMetadata {
                symbol: "USDT".to_string(),
                name: "Tether USD".to_string(),
                decimals: 6,
            },
        );

        map
    });
    
    cache.get(token_mint).cloned()
}

/// Extract sequence number from log messages
pub fn extract_sequence_from_logs(log_messages: &[String]) -> u64 {
    for log in log_messages {
        if log.contains("sequence:") {
            if let Some(seq_part) = log.split("sequence:").nth(1) {
                if let Some(seq_str) = seq_part.trim().split_whitespace().next() {
                    if let Ok(seq) = seq_str.parse::<u64>() {
                        return seq;
                    }
                }
            }
        }
    }
    0
}

/// Extract sender address from a transaction
pub fn extract_sender_address(transaction: &substreams_solana::pb::sf::solana::r#type::v1::Transaction) -> String {
    if let Some(message) = transaction.message.as_ref() {
        if !message.account_keys.is_empty() {
            return bs58::encode(&message.account_keys[0]).into_string();
        }
    }
    "unknown".to_string()
}

/// Print detailed block information for debugging
pub fn print_block_details(block: &substreams_solana::pb::sf::solana::r#type::v1::Block) {
    let block_slot = block.slot;
    let block_hash = &block.blockhash;
    let block_timestamp = block.block_time.as_ref().map(|t| t.timestamp).unwrap_or(0);
    let tx_count = block.transactions.len();

    substreams::log::info!("========== BLOCK DETAILS ==========");
    substreams::log::info!("Block: {}", block_slot);
    substreams::log::info!("Timestamp: {}", block_timestamp);
    substreams::log::info!("Block Hash: {}", block_hash);
    substreams::log::info!("Transactions: {}", tx_count);
    substreams::log::info!("==================================");
}

/// Parse instruction data for Wormhole messages
pub fn parse_wormhole_instruction(data: &[u8]) -> Option<(u32, u32, Vec<u8>)> {
    if data.len() < 9 {  // Minimum length for a valid message
        return None;
    }

    // First byte should be 1 for PostMessage
    if data[0] != 1 {
        return None;
    }

    let len = u32::from_le_bytes(data[1..5].try_into().ok()?);
    let payload = data[5..5 + len as usize].to_vec();
    let seq_off = 5 + len as usize;
    let sequence = u32::from_le_bytes(data[seq_off..seq_off + 4].try_into().ok()?);
    let nonce = u32::from_le_bytes(data[seq_off + 4..seq_off + 8].try_into().ok()?);

    Some((sequence, nonce, payload))
} 