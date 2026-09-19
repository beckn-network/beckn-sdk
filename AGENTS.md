# BeckN SDK Engineering Guide

This document provides instructions for AI coding agents working on the BeckN SDK codebase.

## Project Structure

```
beckn-sdk/
  spec/                       # OpenAPI specification
  typescript/                 # TypeScript/JavaScript SDK
  python/                     # Python SDK
  rust/                       # Rust SDK
  SDK_PLAN.md                 # SDK architecture plan
  POSITIONING.md              # Product positioning
  BETA_CHECKLIST.md           # Beta launch checklist
```

## TypeScript SDK

```bash
cd beckn-sdk/typescript

# Install dependencies
npm install

# Build (TypeScript -> JS)
npm run build

# Type check (no output = pass)
npx tsc --noEmit

# Run tests (tests are compiled JS in dist/tests/)
npm test
```

Key files:
- `src/client.ts` — Main `BeckNClient` class with all API methods
- `src/types.ts` — TypeScript type definitions (60+ interfaces)
- `src/index.ts` — Exports
- `tests/sdk.test.ts` — Test suite (14 tests)

## Python SDK

```bash
cd beckn-sdk/python

# Install (editable, with dev dependencies)
pip install -e ".[dev]"

# Run tests
pytest tests/

# Type check
mypy beckn/

# Lint
ruff check beckn/
```

Key files:
- `beckn/client.py` — Main `BeckNClient` class
- `beckn/types.py` — Type definitions
- `beckn/__init__.py` — Package exports
- `tests/test_sdk.py` — Test suite

## Rust SDK

```bash
cd beckn-sdk/rust

# Build
cargo build

# Run tests
cargo test

# Type check
cargo check

# Lint
cargo clippy -- -W clippy::all
```

Key files:
- `src/client.rs` — Main `BeckNClient` struct
- `src/lib.rs` — Library entry point
- `src/models.rs` — Data models
- `src/types.rs` — Type definitions
- `tests/sdk_test.rs` — Test suite (22 tests)

## OpenAPI Specification

The spec is at `beckn-sdk/spec/openapi.yaml`. After adding new endpoints:
1. Update the spec
2. Regenerate SDK types if needed (manual update for now)

## Publishing

### TypeScript
```bash
npm login
npm publish
```

### Python
```bash
pip install build twine
python -m build
twine upload dist/*
```

### Rust
```bash
cargo login
cargo publish
```

## Adding New SDK Methods

When adding a new API method:

1. Add to TypeScript: `src/client.ts` + `src/types.ts` + `tests/sdk.test.ts`
2. Add to Python: `beckn/client.py` + `beckn/types.py` + `tests/test_sdk.py`
3. Add to Rust: `src/client.rs` + `src/models.rs` + `tests/sdk_test.rs`
4. Update OpenAPI spec: `spec/openapi.yaml`
5. Update docs: `beckn/docs/sdks.md`
