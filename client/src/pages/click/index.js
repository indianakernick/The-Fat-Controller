import "./styles.css";

const button = document.getElementById("button");
let socket;

const RETRY_DELAY = 1000;
const JITTER_DELAY = 50;

function connect() {
    socket = new WebSocket(`ws://${location.host}/socket`);
    socket.onopen = () => {
        button.className = "";
        button.ontouchstart = () => {
            socket.send(DOWN);
            return false;
        };
        button.ontouchend = () => {
            socket.send(UP);
            return false;
        };
    };

    socket.onclose = e => {
        button.className = "offline";
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
