# dev-works-logger — GitHub Contributor & Git Activity Logger Bot

[![CI](https://github.com/maemreyo/dev-works-logger/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/maemreyo/dev-works-logger/actions/workflows/ci.yml)
[![Rust](https://github.com/maemreyo/dev-works-logger/actions/workflows/rust.yml/badge.svg)](https://github.com/maemreyo/dev-works-logger/actions/workflows/rust.yml)

**dev-works-logger** is an open-source Rust bot that collects Git and GitHub activity data, tracks repository contributors, and automates weekly or monthly reports. It uses GitHub GraphQL to analyze commits and active repos, generates shareable images, and can post summaries to Twitter and Discord—ideal for team dashboards, contributor recognition, and dev activity logging.

---

## Project description (short)

A Rust-based automation bot that fetches GitHub organization statistics via GraphQL, identifies top contributors by PR/commit activity, generates achievement images, and posts results to Twitter and Discord on a configurable schedule (cron).

---

## Topics & keywords

- **GitHub bot** · **Git statistics** · **contributor tracking** · **Rust automation** · **cron job** · **GitHub GraphQL** · **Twitter bot** · **Discord integration** · **dev activity logger** · **open source**

---

## Author & contact

| | |
|---|---|
| **Author** | KuchikiRenji |
| **Email** | [KuchikiRenji@outlook.com](mailto:KuchikiRenji@outlook.com) |
| **Discord** | `kuchiki_renji` |

For questions, contributions, or collaboration, reach out via email or Discord.

---

## Features

- **Scheduled runs** — Runs every week or month via cron.
- **GitHub GraphQL** — Fetches repos and commit/PR data for an organization.
- **Contributor insights** — Identifies top contributors (e.g. most PRs, commits).
- **Image generation** — Creates images for organization + contributor + achievement text.
- **Twitter & Discord** — Posts generated content to Twitter and Discord webhook.
- **Docker** — Containerized for easy deployment.
- **Rust** — Async runtime (Tokio), type-safe, minimal dependencies.

---

## Environment variables

Create a `.env` file (see `.env.example`) with:

| Variable | Description |
|----------|-------------|
| `GITHUB_ORG_NAME` | GitHub organization name to analyze |
| `GITHUB_TOKEN` | GitHub personal access token (GraphQL) |
| `TWITTER_TOKEN` | Twitter API credentials / token |
| `DISCORD_HOOK` | Discord webhook URL for posting |

---

## Image & post format

Generated posts follow:

- Organization logo/avatar
- Organization name on the image
- Contributor avatar
- Contributor achievement text on the image
- Tweet body: contributor summary, commits, changed files, hashtags
- Optional: most issues, most reviews, etc.

---

## How to run

### Prerequisites

- [Rust](https://www.rust-lang.org/) (edition 2021)
- `.env` with the variables above

### Install & setup

```bash
make -f Makefile
```

This configures Git hooks and installs [cocogitto](https://docs.cocogitto.io/) for commit standardization.

### Run locally

```bash
cargo run
```

The app starts the cron scheduler and runs the configured jobs (GitHub fetch, image generation, Twitter/Discord posting).

### Docker

See `scripts/docker/` and `Dockerfile` for building and running the bot in a container.

---

## Project structure (overview)

- `src/main.rs` — Entry point, cron scheduler.
- `src/modules/` — Git/GraphQL, Twitter, cron, mail, gql_client, etc.
- `src/utils/` — Init, validation.
- `scripts/` — Docker and local run scripts.

---

## Roadmap & status

- [x] Env management (dotenv), GitHub GraphQL client, Git hooks, cron, Docker, Twitter auth & tweet, Discord webhook.
- [ ] Tweet length check, email (SMTP), CLI for config, Discord bot, optional Actix web & MongoDB.

See the full checklist and history in the repository.

---

## License

See [LICENSE](LICENSE).

---

## SEO note for maintainers

This README is written so that search engines and GitHub can index: **GitHub bot**, **Git statistics**, **contributor tracking**, **Rust**, **cron**, **GraphQL**, **Twitter bot**, **Discord**. The first heading and paragraph double as a concise project description for search snippets.
