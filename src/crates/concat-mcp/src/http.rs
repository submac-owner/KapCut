// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 Jareer Abdullah and Concat contributors

//! Streamable HTTP transport for the MCP server.
//!
//! The stdio server is unchanged. This adapter accepts JSON-RPC over HTTP
//! POST at `/mcp`, runs it through the same [`crate::protocol::serve`] handler,
//! and returns the reply as JSON. It adds the three things the remote design
//! asks for: a bearer token, a project-root sandbox, and a per-token tool
//! family allow-list. No key or token is ever written into source.

use std::io::{BufRead, BufReader, Cursor, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

use serde_json::{Value, json};

use crate::backend::{self, Mode};

/// Default address for the HTTP endpoint. The window's own JSON-RPC server
/// owns port 7420, so this transport uses the next port.
pub const DEFAULT_HTTP_ADDRESS: &str = "127.0.0.1:7421";

/// A writer that collects the protocol's output so it can become an HTTP body.
#[derive(Clone)]
struct Shared(Arc<Mutex<Vec<u8>>>);

impl Shared {
    fn new() -> Shared {
        Shared(Arc::new(Mutex::new(Vec::new())))
    }
    fn bytes(&self) -> Vec<u8> {
        self.0.lock().map(|g| g.clone()).unwrap_or_default()
    }
}

impl Write for Shared {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// The security policy read from the environment once at startup.
struct Policy {
    token: Option<String>,
    root: Option<String>,
    families: Vec<String>,
}

impl Policy {
    fn from_env() -> Policy {
        let token = std::env::var("KAPCUT_AGENT_TOKEN")
            .ok()
            .filter(|t| !t.is_empty());
        let root = std::env::var("KAPCUT_PROJECT_ROOT")
            .ok()
            .filter(|t| !t.is_empty());
        let families = std::env::var("KAPCUT_TOOL_FAMILIES")
            .ok()
            .filter(|t| !t.is_empty())
            .map(|t| {
                t.split(',')
                    .map(|f| f.trim().to_ascii_lowercase())
                    .collect()
            })
            .unwrap_or_default();
        Policy {
            token,
            root,
            families,
        }
    }

    fn authorized(&self, header: &str) -> bool {
        match &self.token {
            None => true, // no token configured: local-only development use
            Some(want) => {
                let given = header.strip_prefix("Bearer ").unwrap_or("").trim();
                given == want
            }
        }
    }

    fn path_allowed(&self, path: &str) -> bool {
        let Some(root) = &self.root else {
            return true;
        };
        if path.is_empty() {
            return true;
        }
        let candidate = std::path::Path::new(path);
        if !candidate.is_absolute() {
            return true;
        }
        let base = std::path::Path::new(root);
        candidate.starts_with(base)
    }

    fn tool_allowed(&self, name: &str) -> bool {
        if self.families.is_empty() {
            return true;
        }
        let family = name.split('_').next().unwrap_or("").to_ascii_lowercase();
        self.families.iter().any(|f| f == &family)
    }
}

/// Check a JSON-RPC body against the sandbox rules. Returns a refusal string
/// when something is outside the configured root or family allow-list.
fn refuse_reason(body: &Value, policy: &Policy) -> Option<String> {
    let objects: Vec<&Value> = if let Some(items) = body.as_array() {
        items.iter().collect()
    } else {
        vec![body]
    };
    for obj in objects {
        if obj.get("method").and_then(Value::as_str) == Some("tools/call") {
            if let Some(tool) = obj
                .get("params")
                .and_then(|p| p.get("name"))
                .and_then(Value::as_str)
            {
                if !policy.tool_allowed(tool) {
                    return Some(format!("tool family of '{tool}' is not allowed"));
                }
            }
        }
        if let Some(path) = obj
            .get("params")
            .and_then(|p| p.get("path"))
            .and_then(Value::as_str)
        {
            if !policy.path_allowed(path) {
                return Some("path is outside KAPCUT_PROJECT_ROOT".to_owned());
            }
        }
        if let Some(args) = obj.get("params").and_then(|p| p.get("arguments")) {
            if let Some(path) = args.get("path").and_then(Value::as_str) {
                if !policy.path_allowed(path) {
                    return Some("path is outside KAPCUT_PROJECT_ROOT".to_owned());
                }
            }
        }
    }
    None
}

/// Run one JSON-RPC body through the real protocol handler.
fn dispatch(body: &[u8], mode: Mode) -> Vec<u8> {
    let mut input = body.to_vec();
    if !input.ends_with(b"\n") {
        input.push(b'\n');
    }
    let written = Shared::new();
    match backend::open(mode) {
        Ok(backend) => crate::protocol::serve(Cursor::new(input), written.clone(), backend),
        Err(error) => {
            let reply = json!({ "jsonrpc": "2.0", "id": null,
                "error": { "code": -32000, "message": error } });
            let mut bytes = serde_json::to_vec(&reply).unwrap_or_default();
            bytes.push(b'\n');
            if let Ok(mut buf) = written.0.lock() {
                buf.extend_from_slice(&bytes);
            }
        }
    }
    written.bytes()
}

fn read_request(
    stream: &TcpStream,
) -> std::io::Result<(String, String, Vec<u8>, Vec<(String, String)>)> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    let method = parts.first().unwrap_or(&"").to_string();
    let path = parts.get(1).unwrap_or(&"").to_string();
    let mut headers = Vec::new();
    loop {
        let mut h = String::new();
        if reader.read_line(&mut h)? == 0 {
            break;
        }
        if h == "\r\n" || h == "\n" {
            break;
        }
        if let Some((k, v)) = h.split_once(':') {
            headers.push((k.trim().to_ascii_lowercase(), v.trim().to_string()));
        }
    }
    let len: usize = headers
        .iter()
        .find(|(k, _)| k == "content-length")
        .and_then(|(_, v)| v.parse().ok())
        .unwrap_or(0);
    let mut body = vec![0u8; len];
    if len > 0 {
        reader.read_exact(&mut body)?;
    }
    Ok((method, path, body, headers))
}

fn header<'a>(headers: &'a [(String, String)], key: &str) -> &'a str {
    headers
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.as_str())
        .unwrap_or("")
}

fn respond(stream: &mut TcpStream, status: &str, body: &[u8], json: bool) -> std::io::Result<()> {
    let kind = if json {
        "application/json"
    } else {
        "text/plain; charset=utf-8"
    };
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {kind}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\n\r\n",
        body.len()
    );
    stream.write_all(head.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()
}

fn json_error(id: Value, message: &str, code: i64) -> Vec<u8> {
    serde_json::to_vec(&json!({ "jsonrpc": "2.0", "id": id,
        "error": { "code": code, "message": message } }))
    .unwrap_or_default()
}

fn handle(mut stream: TcpStream, mode: Mode, policy: &Policy) {
    let (method, path, body, headers) = match read_request(&stream) {
        Ok(v) => v,
        Err(_) => return,
    };
    if !policy.authorized(header(&headers, "authorization")) {
        let _ = respond(
            &mut stream,
            "401 Unauthorized",
            b"bearer token required",
            false,
        );
        return;
    }
    if method == "OPTIONS" {
        let _ = respond(&mut stream, "204 No Content", b"", false);
        return;
    }
    if method == "GET" && path == "/mcp" {
        let info = json!({
            "name": "kapcut", "transport": "streamable-http",
            "endpoint": "/mcp", "methods": ["POST"],
            "token": if policy.token.is_some() { "bearer" } else { "none" },
            "sandbox": { "projectRoot": policy.root, "toolFamilies": policy.families }
        })
        .to_string();
        let _ = respond(&mut stream, "200 OK", info.as_bytes(), true);
        return;
    }
    if method != "POST" || path != "/mcp" {
        let _ = respond(&mut stream, "405 Method Not Allowed", b"POST /mcp", false);
        return;
    }
    let parsed: Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(_) => {
            let out = json_error(json!(null), "invalid JSON body", -32700);
            let _ = respond(&mut stream, "400 Bad Request", &out, true);
            return;
        }
    };
    if let Some(why) = refuse_reason(&parsed, policy) {
        let out = json_error(json!(null), &why, -32000);
        let _ = respond(&mut stream, "403 Forbidden", &out, true);
        return;
    }
    let out = dispatch(&body, mode);
    let _ = respond(&mut stream, "200 OK", &out, true);
}

/// Serve streamable HTTP MCP on `address` until the process ends.
pub fn serve_http(address: &str, mode: Mode) -> Result<(), String> {
    let listener = TcpListener::bind(address).map_err(|e| format!("cannot bind {address}: {e}"))?;
    let policy = Policy::from_env();
    eprintln!(
        "concat mcp: HTTP endpoint at http://{address}/mcp through {} (token: {}, root: {:?})",
        backend_description(mode),
        if policy.token.is_some() {
            "bearer"
        } else {
            "none"
        },
        policy.root
    );
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle(stream, mode, &policy),
            Err(error) => eprintln!("concat mcp: HTTP accept failed: {error}"),
        }
    }
    Ok(())
}

fn backend_description(mode: Mode) -> &'static str {
    match mode {
        Mode::Auto => "auto",
        Mode::Window => "window",
        Mode::Headless => "headless",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy(token: &str, root: &str, families: &str) -> Policy {
        Policy {
            token: if token.is_empty() {
                None
            } else {
                Some(token.to_owned())
            },
            root: if root.is_empty() {
                None
            } else {
                Some(root.to_owned())
            },
            families: families
                .split(',')
                .filter(|f| !f.is_empty())
                .map(|f| f.to_ascii_lowercase())
                .collect(),
        }
    }

    #[test]
    fn bearer_and_allow_lists() {
        let p = policy("secret", "C:/projects", "voice,studio");
        assert!(p.authorized("Bearer secret"));
        assert!(!p.authorized("Bearer wrong"));
        assert!(!p.authorized(""));
        assert!(p.path_allowed("C:/projects/a"));
        assert!(!p.path_allowed("C:/other/a"));
        assert!(p.path_allowed("relative/path"));
        assert!(p.tool_allowed("voice_cut_fillers"));
        assert!(p.tool_allowed("studio_compose"));
        assert!(!p.tool_allowed("clip_add"));
    }

    #[test]
    fn sandbox_checks_json_rpc() {
        let p = policy("", "C:/projects", "voice");
        let ok = json!({ "method": "tools/call", "params": { "name": "voice_transcript_get",
            "arguments": { "path": "C:/projects/demo" } } });
        assert!(refuse_reason(&ok, &p).is_none());
        let bad_path = json!({ "method": "tools/call", "params": { "name": "voice_transcript_get",
            "arguments": { "path": "D:/private" } } });
        assert!(refuse_reason(&bad_path, &p).is_some());
        let bad_tool = json!({ "method": "tools/call", "params": { "name": "clip_add",
            "arguments": { "path": "C:/projects/demo" } } });
        assert!(refuse_reason(&bad_tool, &p).is_some());
    }

    #[test]
    fn dispatch_answers_a_protocol_ping() {
        let out = dispatch(
            br#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#,
            Mode::Headless,
        );
        let text = String::from_utf8_lossy(&out);
        assert!(text.contains("\"result\""), "{text}");
    }
}
