let token;

self.addEventListener('activate', function(event) {
    return self.clients.claim();
});

self.addEventListener('message', function(event) {
    token = event.data.token;
});

self.addEventListener('fetch', event => {
    if (token) {
        event.respondWith(customHeaderRequestFetch(event))
    }
})

function customHeaderRequestFetch(event) {
    const headers = new Headers(event.request.headers);
    headers.set('Authorization', `Bearer ${token}`);
    const newRequest = new Request(event.request, {
        headers: headers
    })
    return fetch(newRequest)
}