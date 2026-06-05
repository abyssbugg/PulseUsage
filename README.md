# Track all your AI coding subscriptions in one place

See your usage at a glance from your menu bar. No digging through dashboards.

![OpenUsage Screenshot](screenshot.png)

## Download

[**Releases will be published here**](https://github.com/abyssbugg/usage-desktop/releases) when this independent build channel is ready.

Auto-updates are disabled for now while the independent release channel is being configured.

## What It Does

OpenUsage lives in your menu bar and shows you how much of your AI coding subscriptions you've used. Progress bars, badges, and clear labels. No mental math required.

- **One glance.** All your AI tools, one panel.
- **Always up-to-date.** Refreshes automatically on a schedule you pick.
- **Global shortcut.** Toggle the panel from anywhere with a customizable keyboard shortcut.
- **Lightweight.** Opens instantly, stays out of your way.
- **Plugin-based.** New providers get added without updating the whole app.
- **[Local HTTP API](docs/local-http-api.md).** Other apps can read your usage data from `127.0.0.1:6736`.
- **[Proxy support](docs/proxy.md).** Route provider HTTP requests through a SOCKS5 or HTTP proxy.

## Supported Providers

- [**Amp**](docs/providers/amp.md) / free tier, bonus, credits
- [**Antigravity**](docs/providers/antigravity.md) / all models
- [**Claude**](docs/providers/claude.md) / session, weekly, extra usage, local token usage (ccusage)
- [**Codex**](docs/providers/codex.md) / session, weekly, reviews, credits
- [**Copilot**](docs/providers/copilot.md) / premium, chat, completions
- [**Cursor**](docs/providers/cursor.md) / credits, total usage, auto usage, API usage, on-demand, CLI auth
- [**Factory / Droid**](docs/providers/factory.md) / standard, premium tokens
- [**Grok**](docs/providers/grok.md) / credits used, plan, pay-as-you-go cap
- [**JetBrains AI Assistant**](docs/providers/jetbrains-ai-assistant.md) / quota, remaining
- [**Kiro**](docs/providers/kiro.md) / credits, bonus credits, overages
- [**Kimi Code**](docs/providers/kimi.md) / session, weekly
- [**MiniMax**](docs/providers/minimax.md) / coding plan session
- [**OpenCode Go**](docs/providers/opencode-go.md) / 5h, weekly, monthly spend limits
- [**Perplexity**](docs/providers/perplexity.md) / requests, searches, research
- [**Devin**](docs/providers/devin.md) / weekly quota, extra usage
- [**Synthetic**](docs/providers/synthetic.md) / demo provider metrics
- [**Z.ai**](docs/providers/zai.md) / session, weekly, web searches

Community contributions welcome.

Want a provider that's not listed? [Open an issue.](https://github.com/abyssbugg/usage-desktop/issues/new)

## Independent Project

This repository is the independent development home for this app.

The original OpenUsage project remains available as an upstream reference. Useful upstream fixes can be reviewed and brought into this repo deliberately.

Plugins are currently bundled while the app stays focused on a small internal workflow.

### How to Contribute

- **Add a provider.** Each one is just a plugin. See the [Plugin API](docs/plugins/api.md).
- **Fix a bug.** PRs welcome. Provide before/after screenshots.
- **Request a feature.** [Open an issue](https://github.com/abyssbugg/usage-desktop/issues/new) and make your case.

Keep it simple. No feature creep, no AI-generated commit messages, test your changes.

## Built Entirely with AI

Not a single line of code in this project was read or written by hand. 100% AI-generated, AI-reviewed, AI-shipped — using [Cursor](https://cursor.com), [Claude Code](https://docs.anthropic.com/en/docs/claude-code), and [Codex CLI](https://github.com/openai/codex).

OpenUsage is a real-world example of what I teach in the [AI Builder's Blueprint](https://itsbyrob.in/EBDqgJ6) — a proven process for building and shipping software with AI, no coding background required.

## Sponsors

This independent repository does not currently use sponsorship links.

<!-- Add sponsor logos here -->

## Credits

Inspired by [CodexBar](https://github.com/steipete/CodexBar) by [@steipete](https://github.com/steipete). Same idea, very different approach.

## License

[MIT](LICENSE)

---

<details>
<summary><strong>Build from source</strong></summary>

> **Warning**: The `main` branch may not be stable. It is merged directly without staging, so users are advised to use tagged versions for stable builds. Tagged versions are fully tested while `main` may contain unreleased features.

### Stack

...
