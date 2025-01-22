# UnifiedStreams
This branch contains a powerful program for seamlessly integrating Substreams, RPC calls, and API calls into your workflows. It simplifies real-time data processing, enhances cross-protocol communication, and streamlines API interactions. Designed with a focus on performance, reliability, and scalability, this program is perfect for building robust, multi-protocol solutions.
To use the portable library please refer to branch "main".


## Prerequisites

1. **Operating System**  
   - Linux, macOS, or Windows (WSL recommended on Windows).

2. **Rust & Cargo**  
   - Make sure they are installed and added to your system PATH.
   
3. **OpenSSL**  
   - Must be installed and available to pkg-config.

4. **JWT Token**  
   - If using Substreams, you may need a JWT token (e.g., `SF_API_TOKEN`) for authenticated calls.


## Installation & Setup

Follow the steps below for a clean installation and configuration.

### 1. Clone the Repository

```bash
https://github.com/SentinelStrategies/UnifiedStreams.git 
```


# 1. Setup a substreams JWT TOKEN
You can set a JWT token as an environment variable to avoid specifying it for each session. If you only need it for your current session, simply run the following export command

```bash
      nano ~/.bashrc
      export SF_API_TOKEN="jwt_token_here"
      source ~/.bashrc
```

# 2. Install Rust and Cargo 
After installing the Rust compiler (rustc) and the Rust package manager (cargo), verify their versions by running:

```bash
      rustc --version
      cargo --version
```

# 3. Install OpenSSL and Related Dependencies
You need to install OpenSSL (and pkg-config) so that Rust can link the necessary libraries during compilation:

```bash 
      pkg-config --libs --cflags openssl
```

# 4. Usage

   Usage1: cargo run api_call {api-url} [optional-headers]
**Example:**
```bash 
cargo run api_call https://httpbin.org/get
```

   Usage2: cargo run rpc_call {rpc-url} {method} {params} {id}
**Example:**
```bash
cargo run rpc_call https://mainnet.infura.io/v3/e2df7af7a38e41d9a7334ce930e566c9 eth_getBalance '["0x3843889b7356e89e63581A594ad826B1F1C445f5", "latest"]' 1 
```

   Usage3: cargo run substreams_call {stream endpoint} {spkg} {module} {start}:{stop}
**Example:**
```bash
cargo run substreams_call https://mainnet.eth.streamingfast.io:443 https://github.com/streamingfast/substreams-uniswap-v3/releases/download/v0.2.10/substreams-uniswap-v3-v0.2.10.spkg uni_v0_2_9:map_tokens_whitelist_pools 21333100:21335101
```
