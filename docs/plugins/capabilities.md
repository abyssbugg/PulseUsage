# Host Capabilities Reference

Plugin host API access is governed by **capabilities**. Each plugin declares which host APIs it needs via the `hostCapabilities` array in its manifest. The runtime only injects the requested APIs — this is least-privilege enforcement.

## Schema Versions

| Version | Field | Behavior |
|---|---|---|
| `1` (legacy) | `hostCapabilities` absent | Capabilities are **inferred** from the plugin ID via the v1 compatibility map. |
| `2` (current) | `hostCapabilities: [...]` present | Capabilities are **explicit**. The v1 map is not consulted. |

A schema v2 plugin with `hostCapabilities: []` (empty array) gets no host API access (beyond `log` and `utils`, which are always available). This is distinct from omitting the field, which triggers v1 inference.

## Capability Strings

| String | Grants access to | Notes |
|---|---|---|
| `fsRead` | `ctx.host.fs.exists`, `ctx.host.fs.readText` | Filesystem read |
| `fsWrite` | `ctx.host.fs.writeText` | Filesystem write |
| `fsListDir` | `ctx.host.fs.listDir` | Directory listing |
| `keychainRead` | `ctx.host.keychain.readGenericPassword`, `readGenericPasswordForCurrentUser` | macOS Keychain read |
| `keychainWrite` | `ctx.host.keychain.writeGenericPassword`, `writeGenericPasswordForCurrentUser` | macOS Keychain write |
| `keychainDelete` | `ctx.host.keychain.deleteGenericPassword` | macOS Keychain delete |
| `httpRequest` | `ctx.host.http.request` (and `ctx.util.request` which wraps it) | HTTP requests |
| `httpDangerousLocalhostTls` | `dangerouslyIgnoreTls: true` in `ctx.host.http.request` | Only for `127.0.0.1`/`localhost`/`::1`. Runtime enforces localhost-only. |
| `sqliteQuery` | `ctx.host.sqlite.query` | SQLite read |
| `sqliteExec` | `ctx.host.sqlite.exec` | SQLite write |
| `plistRead` | `ctx.host.plist.read` | macOS plist read (via `plutil`) |
| `ccusageQuery` | `ctx.host.ccusage.query` | ccusage CLI invocation |
| `lsDiscover` | `ctx.host.ls.discover` | Local server discovery |
| `cryptoAes` | `ctx.host.crypto.decryptAes256Gcm`, `encryptAes256Gcm` | AES-256-GCM |
| `cryptoSha` | `ctx.host.crypto.sha256Hex` | SHA-256 |
| `envRead` | `ctx.host.env.get` | Environment variable read (allowlisted) |

**Always available** (not capability-gated): `ctx.host.log.*` and `ctx.util.*`.

## Migration Path (v1 → v2)

### Step 1: Audit
Run this grep against your `plugin.js` to find every `ctx.host.*` API you use:
```sh
grep -oE "ctx\.host\.[a-zA-Z]+\.[a-zA-Z]+" plugin.js | sort -u
grep -oE "ctx\.util\.request" plugin.js
```
Cross-reference each API with the table above to build your capability list. Note that `ctx.util.request` wraps `ctx.host.http.request`, so it requires `httpRequest`.

### Step 2: Declare
Update `plugin.json`:
```json
{
  "schemaVersion": 2,
  "hostCapabilities": ["fsRead", "httpRequest"]
}
```

### Step 3: Verify
Run the provider validator:
```sh
bun run scripts/validate-provider-metadata.mjs --root .
```
The validator warns on unknown capability strings so you catch typos.

### Step 4: Test
Run the plugin and verify it functions correctly. Check the Diagnostics panel — "Capabilities" should read "Explicit (N)".

## Compatibility Behavior

- **Schema v2 with `hostCapabilities`**: explicit declaration is authoritative. The v1 map is not consulted.
- **Schema v1 or v2 with `hostCapabilities` absent**: falls back to v1 inference. A structured warning is logged:
  ```
  Plugin "<id>" is using legacy capability inference (schema v1, N capabilities).
  Please migrate to explicit hostCapabilities in plugin.json.
  ```
- **Unknown plugin ID (no v1 map entry)**: gets zero capabilities (log + utils only). This is the fail-safe for third-party plugins not in the compat map.

## Deprecation Policy

The v1 compatibility map is **part of the platform compatibility contract**. It is not deprecated. It exists to support third-party plugins that predate schema v2. See the [Deprecation Policy](../deprecation/capability-v1-compat.md) for removal criteria.

## Migration Tooling (Future)

A future tool could auto-generate an initial `hostCapabilities` block by statically analyzing a plugin's `ctx.host.*` usage. This is research-only — not yet implemented. Plugin authors must currently audit manually using the grep approach above.