# Uniswap v3 Swap Tracker (ARB, BASE, ETH)
A Rust-based terminal dApp that leverages WebSockets to listen to on-chain Uniswap V3 swap events in real time across the Ethereum, Arbitrum, and Base networks.

### Features
- Real-time monitoring of Uniswap V3 swap events across Ethereum, Arbitrum, and Base blockchain networks using WebSockets
- Token price retrieval via Alchemy’s Token API
- Interactive ratatui terminal UI with multi-pane layout:
    - Pools panel: Displays pool info, sorted by swap count
    - Prices panel: Shows live USD prices for tracked tokens
    - Swap history panel: Lists recent swaps for the selected pool
- Multi-Stream Architecture: Concurrently consumes event streams from multiple networks using `futures::stream::select_all`


### Tools/Technologies
- **Rust**
- **WebSockets** via Alchemy
- **Alloy** for Ethereum log decoding
- **tokio** for async runtime
- **mpsc** for thread-safe communication between backend and UI
- **futures** for stream merging
- **serde** + **reqwest** for HTTP JSON calls
- **Terminal UI** with ratatui



### High-Level Architecture

<img src="sp_sys.png">

### Terminal UI

<img src="with_prices.png">

### Installation

#### Prerequisites
- Rust and Cargo (1.70.0 or newer)
- Alchemy API key

#### Step 1: Clone the Repository
```bash
git clone https://github.com/yourusername/uniswap_tracker_tui.git
cd uniswap_tracker_tui
```

#### Step 2: Set Up Environment Variables
Create a `.env` file in the root directory with your Alchemy API key:
```bash
ALCHEMY_API_KEY=your_alchemy_api_key_here
```

#### Step 3: Build the dApp
```bash
cargo build
```
The compiled binary will be available at `target/release/uniswap_tracker_tui`.

#### Step 4: Run the dApp
```bash
cargo run
```

#### Usage
- Navigate between pools using the up/down arrow keys
- Traverse 10 pools at a time with PgUp/PgDn keys
- Press `p` to pause the terminal ui (backend/data still updated)
- Press `t` to toggle between current token prices and swap history for the selected pool
- Press `q` to quit the application

#### Troubleshooting
- If you encounter connection issues, ensure your Alchemy API key is valid and has access to the required networks
- For "Backend error: invalid type" messages, check that your token price API responses match the expected format
- If the terminal UI appears distorted, ensure your terminal supports TUI applications and try resizing your terminal window

### License
This project is licensed under the MIT License.