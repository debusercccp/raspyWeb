# raspyWeb

A multithreaded HTTPS web server written in Rust, running on a Raspberry Pi 5.  
Built starting from the [The Rust Programming Language](https://doc.rust-lang.org/book/) book (Chapter 20) and evolved into a production-ready setup with Axum + Caddy.

Live at: **https://raspyrust.duckdns.org**

---

## Stack

| Layer | Technology |
|---|---|
| Web framework | [Axum](https://github.com/tokio-rs/axum) |
| Async runtime | [Tokio](https://tokio.rs) |
| TLS / reverse proxy | [Caddy](https://caddyserver.com) |
| DNS | [DuckDNS](https://duckdns.org) |
| Hardware | Raspberry Pi 5 |
| OS | Debian Trixie ARM |

---

## Features

- Async HTTP server via Axum + Tokio
- HTTPS via Caddy (auto Let's Encrypt)
- Service Worker — offline page when the Pi is off
- Terminal-style frontend (HTML/CSS/JS, no frameworks)

---

## Project structure

```
axum-hello/
├── Cargo.toml
├── hello.html      # main page
├── 404.html        # not found page
├── sw.js           # service worker (offline support)
└── src/
    └── main.rs     # Axum server
```

---

## Routes

| Route | Description |
|---|---|
| `GET /` | Serves `hello.html` |
| `GET /sleep` | Waits 5s then serves `hello.html` (async, non-blocking) |
| `GET /sw.js` | Serves the service worker |
| anything else | 404 page |

---

## Run locally

```bash
cargo run
# server starts on http://127.0.0.1:7878
```

---

## Deploy (Raspberry Pi)

**1. Build on the Pi:**
```bash
cargo build --release
./target/release/axum-hello
```

**2. Caddy config (`/etc/caddy/Caddyfile`):**
```
raspyrust.duckdns.org {
    reverse_proxy localhost:7878
}
```

**3. Start Caddy:**
```bash
sudo systemctl enable --now caddy
```

**4. Keep DuckDNS updated (cron every 5 min):**
```bash
*/5 * * * * ~/duckdns/duck.sh >/dev/null 2>&1
```

---

## Security

- SSH password auth disabled (key only)
- `ufw` firewall — only ports 80, 443, 2222 open
- `fail2ban` monitoring SSH
- No exposed database or internal ports

---

## Origin

Started as the book exercise from *The Rust Programming Language* Ch. 20 — a manual thread pool HTTP server — then rewritten with Axum for async handling and deployed on a real Pi with HTTPS.
