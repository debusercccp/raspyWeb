const CACHE = 'raspyrust-v1';

// Risorse da mettere in cache al primo avvio
const PRECACHE = ['/', '/sw.js'];

// Installazione — metti in cache le risorse principali
self.addEventListener('install', e => {
    e.waitUntil(
        caches.open(CACHE).then(cache => cache.addAll(PRECACHE))
    );
    self.skipWaiting();
});

// Attivazione — pulisci cache vecchie
self.addEventListener('activate', e => {
    e.waitUntil(
        caches.keys().then(keys =>
            Promise.all(keys.filter(k => k !== CACHE).map(k => caches.delete(k)))
        )
    );
    self.clients.claim();
});

// Fetch — prova la rete, se fallisce usa la cache
self.addEventListener('fetch', e => {
    e.respondWith(
        fetch(e.request)
            .then(response => {
                // Aggiorna la cache con la risposta fresca
                const clone = response.clone();
                caches.open(CACHE).then(cache => cache.put(e.request, clone));
                return response;
            })
            .catch(() => {
                // Pi offline — restituisci la versione cached
                return caches.match(e.request).then(cached => {
                    if (cached) return cached;
                    // Se non c'è nemmeno la cache, mostra pagina offline inline
                    return new Response(`
                        <!DOCTYPE html>
                        <html lang="it">
                        <head>
                            <meta charset="utf-8">
                            <title>Offline</title>
                            <style>
                                body {
                                    background: #080c0f;
                                    color: #c8d8e0;
                                    font-family: monospace;
                                    display: flex;
                                    flex-direction: column;
                                    align-items: center;
                                    justify-content: center;
                                    height: 100vh;
                                    margin: 0;
                                }
                                h1 { color: #ffb700; font-size: 2rem; margin-bottom: 1rem; }
                                p  { color: #4a6070; }
                                .dot {
                                    width: 12px; height: 12px;
                                    border-radius: 50%;
                                    background: #ff3c5a;
                                    box-shadow: 0 0 10px #ff3c5a;
                                    margin-bottom: 2rem;
                                    animation: pulse 2s ease-in-out infinite;
                                }
                                @keyframes pulse {
                                    0%,100% { opacity:1 } 50% { opacity:0.3 }
                                }
                            </style>
                        </head>
                        <body>
                            <div class="dot"></div>
                            <h1>Pi è offline 😴</h1>
                            <p>Il server non è raggiungibile. Riprova più tardi.</p>
                        </body>
                        </html>
                    `, { headers: { 'Content-Type': 'text/html' } });
                });
            })
    );
});
