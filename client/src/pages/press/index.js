import "./styles.css";
import Pressure from "pressure";

const button = document.getElementById("button");
let socket;
let down = false;

const RETRY_DELAY = 1000;
const JITTER_DELAY = 50;
const FORCE_THRESHOLD = 0.35;

function connect() {
    socket = new WebSocket(`ws://${location.host}/socket`);
    socket.onopen = () => {
        Pressure.set(button, {
            change(force) {
                if (down) {
                    if (force < FORCE_THRESHOLD) {
                        down = false;
                        socket.send(UP);
                        button.className = "";
                    }
                } else {
                    if (force >= FORCE_THRESHOLD) {
                        down = true;
                        socket.send(DOWN);
                        button.className = "down";
                    }
                }
            }
        });
        button.ontouchend = button.ontouchcancel = () => {
            if (down) {
                down = false;
                socket.send(UP);
                button.className = "";
            }
        };
    };

    socket.onclose = e => {
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
