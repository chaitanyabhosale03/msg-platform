# SecureMsg - Decentralized Encrypted Messaging

Privacy-first messaging platform with end-to-end encryption, decentralized relay architecture, and anonymous identity system.

## Quick Start

```bash
make setup
make docker-up
make relay-run  # Terminal 1
make mobile-run # Terminal 2
```

## Architecture

- **Relay Server**: Rust + Tokio + Axum (WebSocket relay)
- **Mobile Client**: Flutter + Riverpod (encrypted UI)
- **Crypto**: libsodium (X25519, Ed25519, ChaCha20-Poly1305)
- **Storage**: PostgreSQL + SQLite encrypted local DB
- **Network**: WebSocket + gRPC internal services

## Features

- E2E encrypted messaging (Signal Protocol inspired)
- Forward secrecy with Double Ratchet
- Offline message delivery
- Device verification
- Self-destruct messages
- Group messaging
- File sharing (chunked encrypted uploads)
- Metadata minimization
- Zero plaintext on relays

## Development

See Makefile for all commands.
