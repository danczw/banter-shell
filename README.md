<div align="center">

# Generative Terminal Companion - GTC

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
![Rust](https://github.com/danczw/gtc-cli/actions/workflows/rust-ci.yml/badge.svg)

**A cli tool written in rust designed to facilitate seamless text-based conversations with ChatGPT.**

</div>

------------

<br>

Welcome to Generative Terminal Companion, short GTC, a dynamic and user-friendly CLI tool developed in the Rust programming language. This tool allows to engage with one of the most hyped AI models to date – ChatGPT!

Leveraging the speed of Rust, GTC provides a robust and fast interface to interact with ChatGPT. It's designed with ease of use in mind.

<br>

------------

<br>

### 💬 Intelligent Context Management

Keeping in mind the conversational nature of ChatGPT, GTC automatically saves the last six messages of your conversation. This allows the AI to take into account previous messages when formulating its responses, providing an impressively coherent and engaging chat experience.

### 🔑 Important Note Regarding API Keys

To utilize GTC, an API Key is needed. **Please note that the current version of GTC stores the API key in plain text** within the file `~/.gtc`. As this approach may present potential security risks, we're advising all users to secure their API key properly and to be aware of where and how it’s stored. We're already working on enhanced security features for future releases, so hang tight for those updates! Until then, happy chatting with Generative Terminal Companion!

<br>

------------

<br>

## 🛠️ Development

### Build and Run

Clone the repository and run the following command to build the project:

```bash
cargo build
```

To prompt the Generative Terminal Companion, use the following command:

```bash
cargo run -- -m "<message>"
```

### Testing

To run the tests, use the following command:

```bash
cargo test -- --test-threads 1
```