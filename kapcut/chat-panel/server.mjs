// KapCut chat panel bridge -> OpenCode HTTP API (via `opencode api` CLI).
// No npm dependencies. Run: node kapcut/chat-panel/server.mjs
import http from "node:http";
import fs from "node:fs";
import path from "node:path";
import { execFile } from "node:child_process";
import { promisify } from "node:util";

const exec = promisify(execFile);
const ROOT = path.dirname(new URL(import.meta.url).pathname).replace(/^\/(\w:)/, "$1");
const PORT = Number(process.env.KAPCUT_CHAT_PORT || 7421);
const OPENCODE = process.env.OPENCODE_CLI ||
  "C:\\Users\\Admin\\AppData\\Roaming\\ai.opencode.desktop\\cli\\2.2.6-x64\\opencode-cli.exe";

async function api(method, endpoint, data) {
  const args = ["api", method, endpoint];
  if (data !== undefined) args.push("--data", JSON.stringify(data));
  const { stdout } = await exec(OPENCODE, args, { maxBuffer: 16 * 1024 * 1024 });
  return JSON.parse(stdout || "null");
}

const json = (res, code, body) => {
  res.writeHead(code, { "content-type": "application/json; charset=utf-8" });
  res.end(JSON.stringify(body));
};

const server = http.createServer(async (req, res) => {
  const url = new URL(req.url, `http://localhost:${PORT}`);
  try {
    if (req.method === "GET" && (url.pathname === "/" || url.pathname === "/index.html")) {
      const html = fs.readFileSync(path.join(ROOT, "index.html"), "utf8");
      res.writeHead(200, { "content-type": "text/html; charset=utf-8" });
      return res.end(html);
    }
    if (req.method === "GET" && url.pathname === "/api/sessions") {
      return json(res, 200, await api("get", "/api/session"));
    }
    if (req.method === "POST" && url.pathname === "/api/session") {
      const body = await new Promise(r => { let s = ""; req.on("data", c => s += c); req.on("end", () => r(s ? JSON.parse(s) : {})); });
      return json(res, 200, await api("post", "/api/session", body));
    }
    const promptMatch = url.pathname.match(/^\/api\/session\/([^/]+)\/prompt$/);
    if (req.method === "POST" && promptMatch) {
      const body = await new Promise(r => { let s = ""; req.on("data", c => s += c); req.on("end", () => r(JSON.parse(s))); });
      const text = typeof body === "string" ? body : (body.text ?? body.prompt ?? "");
      return json(res, 200, await api("post", `/api/session/${encodeURIComponent(promptMatch[1])}/prompt`, {
        parts: [{ type: "text", text }],
      }));
    }
    const msgMatch = url.pathname.match(/^\/api\/session\/([^/]+)\/messages$/);
    if (req.method === "GET" && msgMatch) {
      return json(res, 200, await api("get", `/api/session/${encodeURIComponent(msgMatch[1])}/message`));
    }
    return json(res, 404, { error: "not found" });
  } catch (err) {
    return json(res, 500, { error: String(err?.stderr || err?.message || err) });
  }
});

server.listen(PORT, () => console.log(`KapCut chat panel: http://localhost:${PORT}`));
