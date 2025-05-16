# sm_bridge_indexer

A Substreams package that indexes Wormhole cross‑chain messages on Token Bridge and SPL‑Token mint/burn events on Solana, alongside block metadata. Designed as a **reference implementation** for real‑time bridge analytics and dashboards.

> **Initialized** via `substreams init` on the `sol‑minimal` template, then scaled up for maximum complexity and usefulness.

---

## Installation & Quick Start

```bash
# 1. Build your Substreams Wasm
substreams build
```

```bash
# 2. Authenticate (if you haven’t yet)
substreams auth
```

```bash
# 3. Launch the interactive GUI
substreams gui
```

```bash
# 4. (Optional) Publish to Substreams Registry
substreams registry login
substreams registry publish
```

## Modules & Handlers

### `map_filter_wormhole`

- Kind: **map**
- Input: **solana:blocks_without_votes**
- Output: **proto:wormhole.v1.Instructions**

#### Descrription:
- Iterates each transaction in every block
- Filters by Wormhole program IDs:
    - **worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth** (Core)
    - **wormDTUJ6AWPNvk59vGQbDvGJmqbDTdgWgAqcLBCgUb** (Token Bridge)
- Emits raw instruction data, account keys, and program IDs

### `map_decode_wormhole`

- Kind: **map**
- Input: **map_filter_wormhole**
- Output: **proto:wormhole.v1.WormholeMessage**

#### Description:
- Scans the filtered instructions for the first **PostMessage** (instruction tag = 1) from the Core program
- Parses:
    - Payload length (bytes 1–4)
    - Payload bytes
    - Sequence number (8 bytes immediately following the payload)
- Emits a structured message with fields:
    - **emitter** (account that posted the message)
    - **sequence** (Verifiable Action Approval sequence)
    - **payload** (raw payload bytes)

### `map_spl_mint_burn`

- Kind: **map**
- Input: **solana:blocks_without_votes**
- Output: **proto:spl.v1.MintOrBurnEvent**

#### Description:
- Filters instructions by the SPL Token program ID: **TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA**
- Manually decodes MintTo (**7**) and Burn (**8**) instructions:
    - Reads the first byte of **data** to identify the instruction
    - Parses the next 8 bytes as a little‑endian **u64** amount
    - Maps account indices to **mint** and **owner** fields
    - Emits **{ mint, owner, amount, is_mint }**

### `map_my_data`

- Kind: **map**
- Input: **solana:blocks_without_votes**
- Output: **proto:mydata.v1.MyData**

#### Description:
- Captures basic block metadata:
    - **block_hash**, **block_slot**, **block_timestamp**
    - **transactions_len** (number of txns)
    - **instructions_len** (total instructions)


### Complexity of the Substreams
- Starts from **sol-minimal** template but hand‑crafts three extra specialized modules.
- Manual decoding of both Wormhole cross‑chain instructions and SPL Token events—far beyond auto‑generated scaffolding.

### Usage & Consumption
- CLI & GUI ready: **substreams run** or **substreams gui**.
- Outputs Protobuf‑typed streams, ideal for SQL sinks, JSON APIs, dashboards.

### Innovation of the Product/Concept
- Real‑time bridge message indexing on Solana.
- Merges off‑chain Veifiable Action Approvals(VAAs) semantics (sequence, payload) with on‑chain asset movements.

### Clarity in Business Model
- Data service: deliver live bridge analytics to DeFi platforms.
- Compliance tool: audit Veifiable Action Approvals(VAAs) & large token movements.
- SaaS dashboard: tiered API for historical & real‑time queries.

### Real‑World Usefulness & Market Potential
- Bridges underpin multi‑chain liquidity—monitoring them is mission‑critical.
- Provides transparency & risk metrics for treasuries, AMMs, and cross‑chain protocols.

### User Impact
- Empowers developers to plug in a reference indexer.
- Equips analysts/traders with actionable bridge‑flow insights.
- Enhances user trust via transparent, real‑time bridge statistics integrated into Web3 apps.# sm_wormhole_bridgeidx
