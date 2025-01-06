# UnifiedStreams (main branch)
This "main" branch contains a powerful program for seamlessly integrating Substreams, RPC calls, and API calls into your workflows. It simplifies real-time data processing, enhances cross-protocol communication, and streamlines API interactions. Designed with a focus on performance, reliability, and scalability, this program is perfect for building robust, multi-protocol solutions.
TO use portable library please refer of other branch "lib".

# 1. Setup a substreams JWT TOKEN
You can set a JWT token as environment varibale so you will not need to set for every session.
To set for current session only only run export commant from below.
         ```bash
               nano ~/.bashrc
               export SF_API_TOKEN="jwt_token_here"
               source ~/.bashrc
         ```
# 2. Install Rust and Cargo 
After installing verify version. Also add in system path.
         ```bash
               rustc --version
               cargo --version
         ```
# 3. Install OpenSSL and Related Dependencies
Install openssl and add it to system path varibales. After installation verify below.
         ```bash 
               pkg-config --libs --cflags openssl
         ```

# 4. Usage
   Usage1: cargo run api_call <api-url> [optional-headers]
      **Example:**
      bash``` 
      cargo run api_call https://httpbin.org/get
      ```

   Usage2: cargo run rpc_call <rpc-url> <method> <params> <id>
      **Example:**
      bash```
      cargo run rpc_call https://mainnet.infura.io/v3/e2df7af7a38e41d9a7334ce930e566c9 eth_getBalance '["0x3843889b7356e89e63581A594ad826B1F1C445f5", "latest"]' 1 
      ```

   Usage3: cargo run substreams_call <stream endpoint> <spkg> <module> <start>:<stop>
      **Example:**
      bash```
      cargo run substreams_call https://mainnet.eth.streamingfast.io:443 https://github.com/streamingfast/substreams-uniswap-v3/releases/download/v0.2.10/substreams-uniswap-v3-v0.2.10.spkg uni_v0_2_9:map_tokens_whitelist_pools 21333100:21335101
      ```
