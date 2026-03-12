# raspyWeb — Server HTTPS in Rust su Raspberry Pi

Web server asincrono scritto in Rust, in esecuzione su un Raspberry Pi 5.
Partito dall'esercizio del capitolo 20 di *The Rust Programming Language*,
evoluto in un setup completo con Axum + Caddy.

Live: https://raspyrust.duckdns.org

---

## Stack

| Layer | Tecnologia |
|---|---|
| Web framework | Axum |
| Async runtime | Tokio |
| TLS / reverse proxy | Caddy |
| DNS | DuckDNS |

---

## Funzionalità

- Server HTTP asincrono via Axum + Tokio
- HTTPS automatico via Caddy (Let's Encrypt)
- Service Worker — pagina offline quando il Pi è spento
- Frontend in stile terminale (HTML/CSS/JS, nessun framework)

---

## Struttura
```
axum-hello/
├── Cargo.toml
├── hello.html       # pagina principale
├── 404.html         # pagina not found
├── sw.js            # service worker (supporto offline)
└── src/
    └── main.rs      # server Axum
```

---

## Route

| Route | Descrizione |
|---|---|
| GET / | Serve hello.html |
| GET /sleep | Attende 5s poi serve hello.html (asincrono, non bloccante) |
| GET /sw.js | Serve il service worker |
| altro | Pagina 404 |

---

## Avvio locale
```bash
cargo run
# server disponibile su http://127.0.0.1:7878
```

HTTPS è gestito da Caddy sul Raspberry Pi.
In locale il server espone HTTP semplice — va bene per lo sviluppo.

---

## Deploy su Raspberry Pi

Build:
```bash
cargo build --release
./target/release/axum-hello
```

Caddyfile:
```
raspyrust.duckdns.org {
    reverse_proxy localhost:7878
}
```

Avvio Caddy:
```bash
sudo systemctl enable --now caddy
```

Aggiornamento DuckDNS (cron ogni 5 minuti):
```bash
*/5 * * * * ~/duckdns/duck.sh >/dev/null 2>&1
```

---

## Sicurezza

- Autenticazione SSH solo a chiave (password disabilitata)
- Firewall ufw — solo porte 80, 443, 2222 aperte
- fail2ban attivo su SSH
- Nessun database o porta interna esposta
