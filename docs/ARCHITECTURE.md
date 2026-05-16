# Architecture

This document describes the internal execution flow, module responsibilities, and transaction lifecycle of Arc Testnet Bot.

---

## Execution Flow

```text
[ Start ]
    |
    v
[ Print Banner ]
    |
    v
[ Load config.json ]  ----fail---->  [ Auto-generate defaults and exit ]
    |
    v
[ Load proxy.txt ]  ---->  ProxyPool (rotatable, optional)
    |
    v
[ Load .env ]  ----fail---->  [ Exit: no wallets found ]
    |
    v
[ Parse PRIVATEKEY_1..N ]  ---->  Vec<WalletEntry> (sorted by index)
    |
    v
[ Cycle Loop ] (cycle_no increments each round)
    |
    v
    +------ For Each Wallet (sequential) ------+
    |                                          |
    |   [ connect_with_retry ]                 |
    |       |-- Rotate proxy on each attempt   |
    |       |-- Up to 6 attempts               |
    |       |-- Exponential backoff (2s..10s)  |
    |       |-- Validate via get_block_number  |
    |                                          |
    |   [ Fetch Nonce ]                        |
    |       |-- get_transaction_count          |
    |       |-- Skip wallet on failure         |
    |                                          |
    |   [ Run Enabled Categories ] (in order)  |
    |       |                                  |
    |       |-- Watchoor                       |
    |       |-- SuperBridge                    |
    |       |-- ZK Codex                       |
    |       |-- OnchainGM                      |
    |       |-- SwapArc                        |
    |       |-- Axpha                          |
    |       |-- Curve Dex                      |
    |       |-- Sweet Haus                     |
    |       |-- Onmifun                        |
    |       |-- Flow Three                     |
    |       |-- Omnihub                        |
    |       +-- Flow On Arc                    |
    |                                          |
    |   [ Category failure is non-fatal ]      |
    |   [ Nonce is passed and returned ]       |
    +-------------------------------------------+
    |
    v
[ Cycle complete ]
    |
    +-- loop_cycle.enabled = false  ---->  [ Exit ]
    |
    +-- loop_cycle.enabled = true   ---->  [ Countdown Timer ]
                                                |
                                                v
                                         [ Next Cycle ]
```

---

## Module Responsibilities

### main.rs
Entry point. Owns the top-level cycle loop, wallet iteration, and category dispatch. Each category is called in a fixed order. A category failure logs the error and continues to the next one without stopping the wallet cycle. Nonce is threaded through every category call and returned as the updated value.

### config.rs
Loads and deserializes `config.json` from the binary's directory or the working directory. If the file does not exist, a default config is written to disk. Each category has its own config struct with an `enabled` flag and per-action run counts.

### wallet.rs
Reads `.env` from the binary's directory or falls back to the working directory. Scans all environment variables matching `PRIVATEKEY_N`, normalizes the hex format (strips `0x` prefix, validates 64-char hex), parses into `LocalWallet`, and returns a sorted `Vec<WalletEntry>`.

### proxy.rs
Loads `proxy.txt` at startup. Supports `host:port`, `host:port:user:pass`, `http://`, and `socks5://` formats. Exposes a thread-safe `ProxyPool` with round-robin rotation. If the pool is empty, all connections go direct without proxy.

### rpc.rs
Owns the `Client` type alias (`Arc<SignerMiddleware<Provider<Http>, LocalWallet>>`). Handles:
- `connect_with_retry` -- builds an HTTP provider (optionally through proxy), validates connectivity, retries up to 6 times with exponential backoff
- `execute_tx` -- estimates gas, detects EIP-1559 support from the latest block, builds and signs the transaction, broadcasts it, and polls for the receipt
- `silent_approve` -- sends an ERC-20 `approve(spender, MAX)` call using `execute_tx`
- `preflight_check` -- dry-runs a transaction via `eth_call` before broadcasting, used to skip already-completed daily actions

### calldata.rs (shared)
Contains selector helpers used across categories. `sel4(hex)` decodes a 4-byte function selector and returns it as a `Vec<u8>` prefix for manual ABI encoding.

### logger.rs
Colored terminal output. `lg` (green) for success, `ly` (yellow) for info and warnings, `lr` (red) for errors. Includes a `sanitize` function that strips private key material from error strings before logging.

### constants.rs
Stores the RPC URL, chain ID, and any shared on-chain constants referenced by multiple modules.

### banner.rs
Prints the ASCII art banner on startup using the `standard.flf` font from the `assets/` directory.

---

## Transaction Lifecycle

```text
Category calls execute_tx(client, nonce, to, data, value, gas_fallback, label)
    |
    v
[ estimate_gas ]
    |-- eth_estimateGas via client
    |-- falls back to gas_fallback if estimation fails
    |
    v
[ detect_fee_type ]
    |-- reads latest block base_fee_per_gas
    |-- EIP-1559: max_fee = base_fee * 2, priority_fee = 1 gwei
    |-- Legacy:   gas_price = current_gas_price * 1.1
    |
    v
[ build TypedTransaction ]
    |-- EIP-1559: Eip1559TransactionRequest
    |-- Legacy:   TransactionRequest
    |-- nonce, to, data, value, gas, fee params attached
    |
    v
[ sign and broadcast ]
    |-- client.send_transaction
    |
    v
[ poll for receipt ]
    |-- polls every RECEIPT_POLL_SEC seconds
    |-- timeout after RECEIPT_TIMEOUT_SEC seconds
    |-- logs tx hash on success
    |
    v
[ return nonce + 1 ]
```

---

## Category Internal Pattern

Every category follows the same structure:

```text
run(client, nonce, cfg) -> Result<U256>
    |
    v
[ Log category started ]
    |
    v
[ Preflight check ] (if applicable)
    |-- skip if already done today (ZK Codex Say GM, OnchainGM, Flow On Arc Faucet)
    |
    v
[ silent_approve ] (if token spending is required)
    |-- only approves if allowance is not already sufficient
    |
    v
[ execute_tx ] (one or more, depending on config times)
    |-- nonce increments after each successful tx
    |-- failure logs the error and breaks or continues depending on category
    |
    v
[ Log category completed ]
    |
    v
[ return updated nonce ]
```

---

## Nonce Management

Nonce is fetched once per wallet via `get_transaction_count` at the start of the wallet cycle. It is then passed into each category and returned incremented by 1 after every successful `execute_tx` call. For categories with internal async gaps (Curve Dex, Onmifun), a `refresh_nonce` call via `get_transaction_count` is made mid-category to re-sync with the chain in case of any discrepancy.

---

## Proxy Rotation

```text
ProxyPool holds Vec<ProxyEntry> with an atomic index
    |
    +-- connect_with_retry calls proxy_pool.rotate() on each failed attempt
    +-- proxy_pool.current_url() returns the active proxy URL
    +-- proxy_pool.current_label() returns a display label for logging
    +-- If pool is empty, build_provider connects directly without proxy
```

---

## Config Structure

```text
config.json
    |
    +-- watchoor          { enabled, times }
    +-- super_bridge      { enabled, times }
    +-- zkcodex           { enabled, say_gm: { enabled }, deploy_contract: { enabled, times } }
    +-- onchaingm         { enabled, gm_onchain: { enabled, times }, mint_badge: { enabled, times }, deploy: { enabled, times } }
    +-- swaparc           { enabled, swaps: { usdc_to_swprc, eurc_to_swprc }, add_lp: { usdc_eurc, usdc_swprc, eurc_swprc } }
    +-- axpha             { enabled, swaps: { usdc_to_eurc, usdc_to_ad, usdc_to_circle } }
    +-- curvedex          { enabled, slippage_bps, swaps: { usdc_to_wusdc, wusdc_to_wbtc, wusdc_to_art }, add_lp: { usdc_eurc }, stake_deposit: { usdc_eurc } }
    +-- sweethaus         { enabled, times }
    +-- onmifun           { enabled, swap: { enabled }, add_lp: { enabled } }
    +-- flowthree         { enabled, times }
    +-- omnihub           { enabled, times }
    +-- flowonarc         { enabled }
    +-- loop_cycle        { enabled, sleep_seconds }
```