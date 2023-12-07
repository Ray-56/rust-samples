const socket = new WebSocket('ws://localhost:3000/ws');
let count = 0;

socket.addEventListener('open', function (event) {
    socket.send('Hello Server!');
    setInterval(() => {
        socket.send(`send msg count = ${count++}`)
    }, 2000);
});

socket.addEventListener('message', function (event) {
    console.log('Message from server ', event.data);
});
