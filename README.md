## UnifiedStreams (lib branch)
This "lib" brnach contains robust library code for integrating Substreams, RPC calls, and API calls into your applications. It simplifies real-time data processing, cross-protocol communication, and API integrations. Designed with extensibility, performance, and reliability in mind, it’s perfect for building scalable, multi-protocol systems.

## Compiling the Library

1. **Clone the Repository and Checkout the `lib` Branch**  
   ```bash
   git clone https://github.com/YourUsername/YourRepo.git
   cd YourRepo
   git checkout lib

2. **Install Rust and Cargo**
Ensure you have both the Rust compiler (rustc) and Cargo package manager installed and configured in your system PATH.

3. **Install OpenSSL and Related Dependencies**
Make sure OpenSSL and any necessary dependencies are correctly installed and recognized by your system.

4. **Build the Library**

```bash
cargo build --release
```

This will produce optimized binary artifacts, including various library files, in the target/release directory. These files can be integrated into other languages or projects as needed.

## Usage

# 1. Setup a substreams JWT TOKEN
You can set a JWT token as an environment variable to avoid specifying it for each session. If you only need it for your current session, simply run the following export command

```bash
      nano ~/.bashrc
      export SF_API_TOKEN="jwt_token_here"
      source ~/.bashrc
```

# 2. Import library in coresponding language and call corresponding function from library for get results.
      rpc_call_ffi
      api_call_ffi
      substreams_call_ffi