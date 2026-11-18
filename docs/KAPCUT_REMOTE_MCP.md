# KapCut remote MCP and agent control

KapCut's MCP server currently speaks **stdio** (like Concat's `concat --mcp`).
This document is the design for the remote transport that lets an agent on
another machine drive a project.

## Goals

- Local agent (chat panel) and remote agent both drive the same project model.
- Remote agent can call KapCut tools but cannot access arbitrary host files.
- Per-agent authentication and tool allow-lists.

## Transport

Streamable HTTP MCP at `/mcp`, with:

```json
{
  "transport": "streamable-http",
  "url": "http://<host>:7420/mcp",
  "auth": { "type": "bearer", "tokenEnv": "KAPCUT_AGENT_TOKEN" }
}
```

## Sandbox rules

- Project paths are restricted to a configured root (`KAPCUT_PROJECT_ROOT`).
- Tool families can be allowed per token via `KAPCUT_TOOL_FAMILIES`
  (comma-separated, same idea as REA's `REA_MCP_TOOL_FAMILIES`).
- Shell, arbitrary file reads, and network egress are not exposed as tools.
- Every destructive call is recorded in the project's evidence log.

## Status

This is the documented target; the HTTP transport itself is a Phase E code
item and is not yet enabled. Until then, connect a remote agent by tunneling
the stdio MCP over SSH, or run OpenCode with its own remote client on the host.
