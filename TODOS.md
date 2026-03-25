# TODOS

## Auth

### Add rate limiting to OTP endpoints

**What:** Implement rate limiting on `POST /auth/login` (max OTP requests per email per hour) and `POST /auth/verify` (max attempts per OTP).

**Why:** Without rate limiting, a 6-digit OTP has only 1M combinations — brute-forceable. Also prevents email-sending abuse where an attacker triggers thousands of OTP emails to a victim's inbox.

**Context:** Deferred during initial implementation to keep the first pass simple. Implementation approach: store `attempts` counter on the OTP document and count recent OTP documents per email. Reject with 429 when limits exceeded. Consider: 5 OTP requests/email/hour, 5 verify attempts/OTP.

**Effort:** S
**Priority:** P1
**Depends on:** None

### Upgrade OTP storage from plaintext to hashed (argon2)

**What:** Hash OTP codes with argon2 before storing in MongoDB. Verify by hashing the submitted code and comparing.

**Why:** Defense-in-depth. If the database is breached, plaintext OTP codes could be used to hijack active login sessions (within the 5-min TTL window). Hashing makes stolen OTP documents useless.

**Context:** Currently storing OTP codes as plaintext strings. The `argon2` crate is the standard choice in Rust. Change is localized to OTP storage and verification logic — no API changes needed.

**Effort:** S
**Priority:** P2
**Depends on:** None

## Infrastructure

### Extract shared types to a common crate

**What:** Create a shared Rust crate (e.g., `chat_types`) containing `User`, `Message`, and other structs used across the HTTP, WebSocket, and TUI repos.

**Why:** The design doc identifies that User and Message types are needed by all three repos. Without a shared crate, type definitions will drift across repos, causing serialization bugs.

**Context:** Currently types are defined locally in `src/models/`. When the TUI and WebSocket repos start consuming the same MongoDB data, extract types into a workspace crate or a separate git repo with a git dependency. Start extraction when the second repo needs the types — not before.

**Effort:** M
**Priority:** P3
**Depends on:** TUI or WebSocket repo needing shared types

## Completed
