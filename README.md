<div align="center">

<img width="100%" alt="header" src="https://capsule-render.vercel.app/api?type=waving&height=210&text=Arc%20Testnet%20Bot&fontAlign=50&fontAlignY=36&fontSize=56&desc=Multi-Category%20On-Chain%20Automation%20%7C%20Multi-Wallet%20%7C%20Proxy%20Support&descAlign=50&descAlignY=58"/>

<img alt="typing" src="https://readme-typing-svg.demolab.com?font=Inter&size=18&duration=3000&pause=650&center=true&vCenter=true&width=900&lines=Auto+Watchoor+%7C+Good+Morning+%26+Good+Night;Auto+SuperBridge+USDC+Cross-chain;Auto+ZK+Codex+Say+GM+%26+Deploy+Contract;Auto+OnchainGM+%7C+Mint+Badge+%7C+Deploy;Auto+SwapArc+%7C+Swap+%26+Add+Liquidity;Auto+Axpha+%7C+Swap+USDC+to+EURC+%2F+AD+%2F+CIRCLE;Auto+Curve+Dex+%7C+Swap+%2F+Add+LP+%2F+Stake+Deposit;Auto+Sweet+Haus+%7C+Mint+NFT;Auto+Onmifun+%7C+Swap+%26+Add+LP;Auto+Flow+Three+%7C+Deposit+%26+Withdraw;Auto+Omnihub+%7C+Mint+Shrimp+NFT;Auto+Flow+On+Arc+%7C+Faucet+%2F+Claim+%2F+Add+LP"/>

<p>
  <img alt="rust" src="https://img.shields.io/badge/Rust-2021-f74c00?logo=rust&logoColor=white"/>
  <img alt="platform" src="https://img.shields.io/badge/Platform-Arc%20Testnet-111111"/>
  <img alt="multi-wallet" src="https://img.shields.io/badge/Multi--Wallet-Supported-111111"/>
  <img alt="build" src="https://github.com/Yuurichan-N3/Arc-Testnet/actions/workflows/build.yml/badge.svg"/>
  <img alt="author" src="https://img.shields.io/badge/by-Yuurisandesu-111111"/>
</p>

<p>
  <b>Arc Testnet Bot</b> is a full automation bot for the Arc Testnet network.<br/>
  It handles multiple on-chain activity categories in one cycle: Watchoor interactions, SuperBridge USDC deposits, ZK Codex GM and contract deployments, OnchainGM with badge minting, SwapArc token swaps with liquidity provisioning, Axpha USDC swaps, Curve Dex swaps with LP and staking, Sweet Haus NFT minting, Onmifun swaps and LP, Flow Three deposit and withdraw, Omnihub NFT minting, and Flow On Arc faucet claiming with LP -- all running automatically across multiple wallets.<br/>
  Built and distributed by <b>Yuurisandesu</b>.
</p>

</div>

---

## Table of Contents

- [Requirements](#-requirements)
- [Installation](#-installation)
- [Configuration](#-configuration)
- [Running the Bot](#-running-the-bot)
- [Features](#-features)
- [File Structure](#-file-structure)
- [Disclaimer](#-disclaimer)

---

## ⚙️ Requirements

- Rust `1.70+` (includes `cargo`)
- Git

---

## 🚀 Installation

### Install Rust

**Linux / macOS:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**

Download and run the installer from https://rustup.rs, then restart your terminal.

**Termux (Android):**

Termux does not support the Rust toolchain natively at full capacity. The recommended approach is to use a Ubuntu proot environment via `proot-distro`:

```bash
pkg update && pkg install proot-distro
proot-distro install ubuntu
proot-distro login ubuntu
```

Once inside the Ubuntu environment, install dependencies and Rust:

```bash
apt update && apt install -y curl git build-essential pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Clone the Repository

```bash
git clone https://github.com/Yuurichan-N3/Arc-Testnet.git
cd Arc-Testnet
```

---

## 🔧 Configuration

### 1. Private Keys (.env)

Copy `.env.example` to `.env` and fill in your wallet private keys, one per line:

```env
PRIVATEKEY_1=your_private_key_here
PRIVATEKEY_2=your_private_key_here
```

Add as many `PRIVATEKEY_N` entries as wallets you want to run. All wallets are processed sequentially per cycle.

### 2. Proxy (proxy.txt)

Fill `proxy.txt` with proxies, one per line (optional, leave empty to run without proxy):

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

There are several ways to run the bot depending on your preference and platform.

### Using run.sh (Linux / macOS / Termux)

`run.sh` is an all-in-one helper script that handles building and running in a single command. Make it executable first:

```bash
chmod +x run.sh
```

Then choose a mode:

```bash
./run.sh direct    # build and run in foreground (default)
./run.sh nohup     # build and run in background, logs saved to arc-bot.log
./run.sh screen    # build and run in a detached screen session
./run.sh tmux      # build and run in a detached tmux session
./run.sh logs      # tail the log file
./run.sh stop      # stop the running bot process
```

For background modes, attach to the session anytime:

```bash
# screen
screen -r arc-bot

# tmux
tmux attach -t arc-bot
```

### Using make (Linux / macOS)

```bash
make release   # optimized release build
make start     # build release and run immediately
make run       # build debug and run
make check     # check for errors without building
make fmt       # format the code
make clean     # remove all build artifacts
make size      # show release binary size
```

### Manual (all platforms)

Build first:

```bash
cargo build --release
```

Then run:

```bash
# Linux / macOS / Termux
./target/release/arc-bot

# Windows
.\target\release\arc-bot.exe
```

### Download Prebuilt Binary (Linux x86_64)

If you do not want to build from source, a prebuilt Linux binary is automatically compiled on every push to main via GitHub Actions.

Download the latest binary from the Actions page:
https://github.com/Yuurichan-N3/Arc-Testnet/actions/workflows/build.yml

Open the latest successful run, scroll to the Artifacts section, and download `arc-bot-linux-x86_64`. The binary is retained for 7 days per build. After downloading, make it executable and run:

```bash
chmod +x arc-bot
./arc-bot
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
The bot executes token swaps (USDC to SWPRC and EURC to SWPRC) and adds liquidity to three pools (USDC/EURC, USDC/SWPRC, EURC/SWPRC) on SwapArc DEX. Each swap and LP action includes automatic token approval before execution.

### 💱 Axpha
The bot executes swaps on the Axpha DEX, supporting three trading pairs from USDC: USDC to EURC, USDC to AD, and USDC to CIRCLE. Each pair is individually configurable with a run count per cycle. Swap value and deadline are handled automatically per transaction.

### 📈 Curve Dex
The bot interacts with the Curve DEX across three actions. For swaps, it supports USDC to WUSDC, WUSDC to WBTC, and WUSDC to ART -- each with automatic approval, a live on-chain quote, configurable slippage tolerance in basis points, and a preflight simulation before execution. For Add LP, the bot approves both USDC and EURC then adds liquidity to the USDC/EURC pool. For Stake Deposit, the bot approves the LP token and deposits it into the staking contract. Each of the three actions is individually configurable with a run count per cycle.

### 🍬 Sweet Haus
The bot mints an NFT from the Sweet Haus contract by submitting a claim transaction with the wallet address as the receiver. The mint value and calldata are encoded automatically. Configurable run count per cycle.

### 🎯 Onmifun
The bot interacts with the Onmifun DEX router across two actions. For swaps, it buys **ETH to CHNOS** and **ETH to MOG** in sequence. For Add LP, it approves each token, then adds liquidity to the **ETH/CHNOS** and **ETH/MOG** pools. Both actions are individually toggleable via config.

### 🌊 Flow Three
The bot submits a **Deposit** transaction followed immediately by a **Withdraw** transaction to the Flow Three contract. Both actions run as a pair per cycle and are configurable by run count.

### 🦐 Omnihub
The bot mints a **Shrimp NFT** from the Omnihub contract by submitting a mint transaction with a fixed ETH value. Configurable run count per cycle.

### 🌿 Flow On Arc
The bot runs a full Flow On Arc cycle: it first attempts a **Faucet Claim** (auto-skipped via preflight if already claimed this period), then approves USDC, CAT, DARC, and PANDA tokens, and adds liquidity to three pools: **USDC/CAT**, **USDC/DARC**, and **USDC/PANDA**.

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
├── .github/
│   └── workflows/
│       └── build.yml                # CI: auto build release binary on push
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
│       ├── swaparc/                 # Swap tokens, Add liquidity
│       ├── axpha/                   # Swap USDC to EURC / AD / CIRCLE
│       ├── curvedex/                # Swap, Add LP USDC/EURC, Stake Deposit
│       ├── sweethaus/               # Mint NFT via claim transaction
│       ├── onmifun/                 # Swap ETH to CHNOS/MOG, Add LP ETH/CHNOS and ETH/MOG
│       ├── flowthree/               # Deposit and Withdraw
│       ├── omnihub/                 # Mint Shrimp NFT
│       └── flowonarc/               # Faucet Claim, Add LP USDC/CAT, USDC/DARC, USDC/PANDA
├── assets/
│   └── standard.flf                # ASCII font for banner
├── Cargo.toml                       # Project manifest and dependencies
├── Cargo.lock
├── Makefile                         # make targets: build, run, release, start, clean, size
├── run.sh                           # Run helper: direct, nohup, screen, tmux, logs, stop
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