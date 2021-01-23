import "./styles.css";

const button = document.getElementById("button");
let socket;

const RETRY_DELAY = 1000;
const JITTER_DELAY = 50;

function downUpStart() {
    socket.send("d" + TARGET);
    return false;
}

function downUpEnd() {
    socket.send("u" + TARGET);
    return false;
}

function clickStart() {
    socket.send("c" + TARGET);
    return false;
}

function clickEnd() {
    return false;
}

function connect() {
    socket = new WebSocket(`ws://${location.host}/socket`);
    socket.onopen = () => {
        if (DOWN_UP) {
            button.ontouchstart = downUpStart;
            button.ontouchend = downUpEnd;
        } else {
            button.ontouchstart = clickStart;
            button.ontouchend = clickEnd;
        }
    };

    socket.onclose = e => {
        button.ontouchstart = undefined;
        button.ontouchend = undefined;
        if (e.code !== 1000) {
            setTimeout(connect, RETRY_DELAY);
        }
    };
}

connect();

// This massively reduces jitter
const buf = new ArrayBuffer(0);
setInterval(() => {
    if (socket.readyState === WebSocket.OPEN) {
        socket.send(buf);
    }
}, JITTER_DELAY);
