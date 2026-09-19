# BeckN Public Beta Launch Checklist

## Infrastructure

- [x] PostgreSQL database with PostGIS
- [x] Database migrations written and tested
- [x] Docker image builds successfully
- [x] Docker Compose includes postgres + app
- [x] Health check endpoint (`/api/health`)
- [x] API key authentication (X-API-Key header)
- [x] Rate limiting plug added
- [x] SSL/TLS support configured

## API Server

- [x] BeckN server starts via `mix run --no-halt`
- [x] Bandit HTTP server configuration
- [x] JSON request body parser
- [x] API key gate plug (public endpoints exempt)
- [x] Rate limit plug
- [x] 404 catch-all route
- [x] OpenAPI spec served at `/openapi.yaml`
- [x] All routes compile without warnings

## Core Domain

- [x] BeckN domain with all resources (Order, Item, Provider, Fulfillment, BAP, BPP, Subscription)
- [x] GeoDNS proximity search (earth_distance)
- [x] GBP account management and sync
- [x] Company multi-tenant support
- [x] API key lifecycle (create, verify, revoke, rotate)

## Protocols

- [x] A2A: Agent cards, tasks, artifacts, messages
- [x] MCP: Tools, prompts, server resources, clients
- [x] ANP: Announcements, verifications, witnesses
- [x] ACP: Tokens, issuers, presentations, access policies
- [x] GeoDNS ↔ A2A discovery integration

## SDKs

- [x] TypeScript SDK with 14 passing tests
- [x] Python SDK
- [x] Rust SDK with 22 passing tests
- [x] OpenAPI specification updated

## Testing

- [x] 89 Elixir tests passing
- [x] TypeScript SDK tests passing
- [x] Rust SDK tests passing
- [x] Tests run with `--max-cases=1` (avoids flakiness)
- [x] `mix compile --warnings-as-errors` passes

## Documentation

- [x] README.md (installation + basic usage)
- [x] docs/ (quickstart, API reference, auth, geodns, gbp, docker, protocols, sdks)
- [x] TypeScript SDK README
- [x] Python SDK README
- [x] Rust SDK README

## Legal

- [x] Terms of Service
- [x] Privacy Policy

## CI/CD

- [x] GitHub Actions CI workflow
- [x] Release workflow

## Pre-Launch

- [ ] Deploy to staging environment
- [ ] Smoke test all endpoints
- [ ] Verify Docker build & deploy
- [ ] Update DNS for api.beckn.network
- [ ] Set PROD environment variables
- [ ] Run database migrations in production
- [ ] Configure monitoring/alerting
- [ ] Announce beta to community

## Post-Launch

- [ ] Monitor error rates
- [ ] Track API usage metrics
- [ ] Collect beta feedback
- [ ] Plan next iteration
