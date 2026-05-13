<div align="center">

<img width="100%" alt="header" src="https://capsule-render.vercel.app/api?type=waving&height=210&text=Arc%20Testnet%20Bot&fontAlign=50&fontAlignY=36&fontSize=56&desc=Watchoor%20%7C%20SuperBridge%20%7C%20ZK%20Codex%20%7C%20OnchainGM%20%7C%20SwapArc&descAlign=50&descAlignY=58"/>

<img alt="typing" src="https://readme-typing-svg.demolab.com?font=Inter&size=18&duration=3000&pause=650&center=true&vCenter=true&width=900&lines=Auto+Watchoor+%7C+Good+Morning+%26+Good+Night;Auto+SuperBridge+USDC+Cross-chain;Auto+ZK+Codex+Say+GM+%26+Deploy+Contract;Auto+OnchainGM+%7C+Mint+Badge+%7C+Deploy;Auto+SwapArc+%7C+Swap+%26+Add+Liquidity"/>

<p>
  <img alt="rust" src="https://img.shields.io/badge/Rust-2021-f74c00?logo=rust&logoColor=white"/>
  <img alt="platform" src="https://img.shields.io/badge/Platform-Arc%20Testnet-111111"/>
  <img alt="multi-wallet" src="https://img.shields.io/badge/Multi--Wallet-Supported-111111"/>
  <img alt="author" src="https://img.shields.io/badge/by-Yuurisandesu-111111"/>
</p>

<p>
  <b>Arc Testnet Bot</b> is a full automation bot for the Arc Testnet network.<br/>
  It handles multiple on-chain activity categories in one cycle: Watchoor interactions, SuperBridge USDC deposits, ZK Codex GM & contract deployments, OnchainGM with badge minting, and SwapArc token swaps with liquidity provisioning all running automatically across multiple wallets.<br/>
  Built and distributed by <b>Yuurisandesu</b>.
</p>

</div>

---

## ⚙️ Requirements

- [Rust](https://rustup.rs/) `1.70+` (includes `cargo`)
- Git

---

## 🚀 Installation

**Clone the repository:**

```bash
git clone https://github.com/Yuurichan-N3/Arc-Testnet.git
cd Arc-Testnet
```

**Build the binary:**

```bash
cargo build --release
```

---

## 🔧 Configuration

### 1. Private Keys (.env)

Copy `.env.example` to `.env` and fill in your wallet private keys, one per line:

```env
PRIVATEKEY_1=your_private_key_here
PRIVATEKEY_2=your_private_key_here
```

> Add as many `PRIVATEKEY_N` entries as wallets you want to run. All wallets are processed sequentially per cycle.

### 2. Proxy (proxy.txt)

Fill `proxy.txt` with proxies, one per line (optional leave empty to run without proxy):

```
host:port
host:port:user:pass
http://user:pass@host:port
socks5://user:pass@host:port
```

### 3. Bot Settings (config.json)

`config.json` is **auto-generated on first run** with all defaults filled in. Edit it before re-running to customize which categories are enabled, how many times each action runs per cycle, and whether the bot loops continuously. Set `loop_cycle.enabled` to `true` and adjust `sleep_seconds` to keep the bot running indefinitely between cycles.

---

## ▶️ Running the Bot

**Linux / macOS:**

```bash
./target/release/arc-bot
```

**Windows:**

```bash
.\target\release\arc-bot.exe
```

---

## ✨ Features

### 👁️ Watchoor
Per cycle, the bot sends a series of on-chain interactions to the Watchoor contract: **Good Morning** and **Good Night** transactions, followed by deploying an **NFT contract**, an **ERC20 contract**, and a **Counter contract**. The number of full Watchoor rounds is controlled by `times` in config.

### 🌉 SuperBridge
The bot automatically approves and submits a USDC cross-chain burn deposit via the SuperBridge Token Messenger contract on Arc Testnet, bridging to the destination domain. Configurable `times` per cycle.

### 🧩 ZK Codex
The bot sends a **Say GM** message on-chain (auto-skipped if already done today via preflight check), then deploys a contract through the ZK Codex deploy contract. Each action is individually toggleable.

### 🌅 OnchainGM
The bot submits a **GM Onchain** transaction, followed by **Mint Badge** and a **Deploy** transaction through the OnchainGM contracts. If GM is detected as already completed via preflight check, badge and deploy are automatically skipped for that cycle.

### 🔄 SwapArc
The bot executes **token swaps** (USDC → SWPRC and EURC → SWPRC) and **adds liquidity** to three pools (USDC/EURC, USDC/SWPRC, EURC/SWPRC) on SwapArc DEX. Each swap and LP action includes automatic token approval before execution.

### 👛 Multi Wallet
All wallets defined in `.env` are processed sequentially within every cycle. Each wallet gets its own RPC connection and the wallet index is shown in logs for easy tracking.

### 🔁 Loop Cycle
Set `loop_cycle.enabled` to `true` in `config.json` to keep the bot running indefinitely. After all wallets complete a cycle, a live countdown timer shows the remaining wait before the next cycle begins.

### 🌐 Proxy Support
Proxies are loaded from `proxy.txt` and rotated across wallets and RPC connections. Supports HTTP and SOCKS5 with or without authentication. Running without proxies is fully supported.

---

## 🗂️ File Structure

```text
Arc-Testnet/
├── src/
│   ├── main.rs                      # Entry point, main cycle loop
│   ├── config.rs                    # Config loading and structs
│   ├── constants.rs                 # RPC URL, chain ID, contract addresses
│   ├── wallet.rs                    # Private key loading from .env
│   ├── rpc.rs                       # RPC client, tx execution, preflight checks
│   ├── proxy.rs                     # Proxy pool loading and rotation
│   ├── logger.rs                    # Colored log output
│   ├── calldata.rs                  # Shared calldata utilities
│   ├── banner.rs                    # ASCII banner on startup
│   └── categories/
│       ├── mod.rs
│       ├── watchoor/                # Good Morning, Good Night, Deploy NFT/ERC20/Counter
│       ├── super_bridge/            # USDC cross-chain burn deposit
│       ├── zkcodex/                 # Say GM, Deploy Contract
│       ├── onchaingm/               # GM Onchain, Mint Badge, Deploy
│       └── swaparc/                 # Swap tokens, Add liquidity
├── assets/
│   └── standard.flf                # ASCII font for banner
├── Cargo.toml                       # Project manifest and dependencies
├── Cargo.lock
├── config.json                      # Auto-generated on first run
├── proxy.txt                        # Proxy list
└── .env.example                     # Private key template
```

---

## ⚠️ Disclaimer

This tool is built for educational and technical exploration purposes. Use it wisely and at your own responsibility.

---

<div align="center">
<img width="100%" alt="footer" src="https://capsule-render.vercel.app/api?type=waving&height=120&section=footer"/>
</div>