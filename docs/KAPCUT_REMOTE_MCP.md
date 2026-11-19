# KapCut remote MCP and agent control

KapCut ships two transports for the same MCP server and tool set:

| Transport | Address | Use |
|---|---|---|
| stdio | process pipes | `concat --mcp`, local agent |
| streamable HTTP | `http://127.0.0.1:7421/mcp` | remote agent, `concat-mcp-http` |

The window's own JSON-RPC server keeps `127.0.0.1:7420`; the HTTP endpoint uses
the next port so both can run at once. Both routes call the same
`protocol::serve` handler, so a tool behaves identically over either transport.

## Running the HTTP endpoint

```
concat-mcp-http [auto|window|headless]
```

- Address: `KAPCUT_MCP_HTTP_ADDRESS` (default `127.0.0.1:7421`)
- Mode: argument or `CONCAT_MCP_MODE` (auto, window, headless)

`GET /mcp` returns endpoint metadata. `POST /mcp` takes one JSON-RPC object
(or a batch array) and returns the JSON-RPC reply. `OPTIONS` is answered for
browser preflight.

## Security (read from the environment, never from source)

- `KAPCUT_AGENT_TOKEN` — when set, every request must carry
  `Authorization: Bearer <token>`. Unauthorized requests get 401.
- `KAPCUT_PROJECT_ROOT` — absolute `path` fields must be inside this root;
  otherwise the call is refused with 403. Relative paths are allowed.
- `KAPCUT_TOOL_FAMILIES` — comma-separated tool-family prefixes (for example
  `voice,studio,curves`). A `tools/call` outside these families is refused.
- Shell, arbitrary file reads and network egress are not exposed as tools.
- No key or token is stored in code or in the project.

## Example client config

```json
{
  "transport": "streamable-http",
  "url": "http://127.0.0.1:7421/mcp",
  "auth": { "type": "bearer", "tokenEnv": "KAPCUT_AGENT_TOKEN" }
}
```

## Status

Implemented in source (`concat-mcp/src/http.rs`, `bin/concat-mcp-http.rs`) and
verified by Windows CI. Like the rest of the studio, it is source-only until a
future build is installed; OpenCode's local window-mode connection at `7420`
already works with the installed app.
