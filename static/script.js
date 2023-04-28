const registerServiceWorker = async () => {
  navigator.serviceWorker.register(
    '/sw.js', { scope: '/' }
  );
};

let auth = document.querySelector('button.auth');
let enterRoom = document.querySelector('button.enter-room');
let pingPublic = document.querySelector('button.ping-public');
let pingPrivate = document.querySelector('button.ping-private');

auth.addEventListener('click', function() {
  let username = document.querySelector('input[name=username]').value;
  const formData = new FormData();
  formData.append("username", username);
  fetch("/authenticate", {
    method: "POST",
    body: formData,
  })
  .then(response => response.text())
  .then(data => { 
    navigator.serviceWorker.ready.then((registration) => {
      registration.active.postMessage(
        {token: data}
      );
    });
  })
});

function connect(eventURI, board) {
  const events = new EventSource(eventURI);

  events.addEventListener("message", (ev) => {
    const msg = JSON.parse(ev.data);
    board.value = `${board.value}\n${ev.data}`;
  });
}

enterRoom.addEventListener('click', function() {
  let room = document.querySelector('input[name=room]').value;
  fetch(`/room/${room}`, {
    method: "GET"
  }).then(data => {
    connect(`/room/${room}/public`, document.querySelector('textarea[name=public-channel]'));
    connect(`/room/${room}/private`, document.querySelector('textarea[name=private-channel]'));
  })
});

pingPublic.addEventListener('click', function() {
  let room = document.querySelector('input[name=room]').value;
  fetch(`/room/${room}/public-ping`, {
    method: "GET"
  });
});

pingPrivate.addEventListener('click', function() {
  let room = document.querySelector('input[name=room]').value;
  fetch(`/room/${room}/private-ping`, {
    method: "GET"
  });
});

registerServiceWorker();
