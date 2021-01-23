import "./styles.css";

const pad = document.getElementById("pad");
let socket;
let touch = null;
let startTouch = null;

const moveView = new Uint8Array(5);
moveView[0] = 1;
const clickView = new Uint8Array([4, 0]);

const RETRY_DELAY = 1000;
const JITTER_DELAY = 50;
const MOVE_SCALE = 1.8;

function copyTouch({ identifier, clientX, clientY }) {
    return { identifier, clientX, clientY };
}

function updateMouse(changeX, changeY) {
    changeX *= MOVE_SCALE;
    changeY *= MOVE_SCALE;
    moveView[1] = changeX & 0xFF;
    moveView[2] = (changeX >> 8) & 0xFF;
    moveView[3] = changeY & 0xFF;
    moveView[4] = (changeY >> 8) & 0xFF;
    socket.send(moveView);
}

function connect() {
    socket = new WebSocket(`ws://${location.host}/socket`);
    socket.onopen = () => {
        pad.className = "";
        pad.ontouchstart = e => {
            if (touch === null) {
                touch = copyTouch(e.changedTouches[0]);
                startTouch = copyTouch(e.changedTouches[0]);
            }
            return false;
        };
        pad.ontouchmove = e => {
            if (touch !== null) {
                for (const t of e.changedTouches) {
                    if (t.identifier === touch.identifier) {
                        updateMouse(t.clientX - touch.clientX, t.clientY - touch.clientY);
                        touch.clientX = t.clientX;
                        touch.clientY = t.clientY;
                        break;
                    }
                }
            }
            return false;
        };
        pad.ontouchend = e => {
            if (touch !== null) {
                for (const t of e.changedTouches) {
                    if (t.identifier === touch.identifier) {
                        updateMouse(t.clientX - touch.clientX, t.clientY - touch.clientY);
                        if (t.clientX === startTouch.clientX && t.clientY === startTouch.clientY) {
                            socket.send(clickView);
                        }
                        touch = null;
                        startTouch = null;
                        break;
                    }
                }
            }
            return false;
        };
        pad.ontouchcancel = e => {
            if (touch !== null) {
                for (const t of e.changedTouches) {
                    if (t.identifier === touch.identifier) {
                        touch = null;
                        startTouch = null;
                        break;
                    }
                }
            }
            return false;
        };
    };

    socket.onclose = e => {
        pad.className = "offline";
        pad.ontouchstart = undefined;
        pad.ontouchend = undefined;
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
