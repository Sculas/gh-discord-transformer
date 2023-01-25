# gh-discord-transformer

A small serverless function to transform GitHub sponsorship webhook events into Discord webhook events.
**Requires Vercel.**

## Environment Variables

- `GITHUB_SECRET`: github webhook secret
- `DISCORD_WEBHOOK_URL`: discord webhook url
- `DISCORD_WEBHOOK_TID`: optional discord webhook thread id
