import "./styles.scss";
import SocketManager from "../common/SocketManager.js";
import { KEY_DOWN, KEY_UP } from "../common/CommandCode.js";
import { TAB, Q, W, E, R, CAPS_LOCK, A, S, D, F, SHIFT, Z, X, C } from "../common/Key.js";

const container = document.getElementById("container");
const s = new SocketManager(container);

container.ontouchstart = () => {
    return false;
};

const FORCE_THRESHOLD = 0.15;

function createButton(socket, id, downBuf, upBuf) {
    let down = false;
    let touchId = null;
    const element = document.getElementById(id);

    element.ontouchstart = e => {
        if (!touchId) {
            touchId = e.changedTouches[0].identifier;
        }
        return false;
    };

    element.ontouchend = element.ontouchcancel = e => {
        if (touchId) {
            for (const touch of e.changedTouches) {
                if (touch.identifier === touchId) {
                    touchId = null;
                    if (down) {
                        socket.send(upBuf);
                        down = false;
                        element.classList.remove("down");
                    }
                }
            }
        }
        return false;
    };

    element.ontouchforcechange = e => {
        let force = -1;
        for (const touch of e.changedTouches) {
            if (touch.identifier === touchId) {
                force = touch.force;
                break;
            }
        }
        if (force === -1) return;
        if (down) {
            if (force < FORCE_THRESHOLD) {
                socket.send(upBuf);
                down = false;
                element.classList.remove("down");
            }
        } else {
            if (force >= FORCE_THRESHOLD) {
                socket.send(downBuf);
                down = true;
                element.classList.add("down");
            }
        }
    };
}

createButton(s, "tab", new Uint8Array([KEY_DOWN, TAB]), new Uint8Array([KEY_UP, TAB]));
createButton(s, "q", new Uint8Array([KEY_DOWN, Q]), new Uint8Array([KEY_UP, Q]));
createButton(s, "w", new Uint8Array([KEY_DOWN, W]), new Uint8Array([KEY_UP, W]));
createButton(s, "e", new Uint8Array([KEY_DOWN, E]), new Uint8Array([KEY_UP, E]));
createButton(s, "r", new Uint8Array([KEY_DOWN, R]), new Uint8Array([KEY_UP, R]));

createButton(s, "capslock", new Uint8Array([KEY_DOWN, CAPS_LOCK]), new Uint8Array([KEY_UP, CAPS_LOCK]));
createButton(s, "a", new Uint8Array([KEY_DOWN, A]), new Uint8Array([KEY_UP, A]));
createButton(s, "s", new Uint8Array([KEY_DOWN, S]), new Uint8Array([KEY_UP, S]));
createButton(s, "d", new Uint8Array([KEY_DOWN, D]), new Uint8Array([KEY_UP, D]));
createButton(s, "f", new Uint8Array([KEY_DOWN, F]), new Uint8Array([KEY_UP, F]));

createButton(s, "shift", new Uint8Array([KEY_DOWN, SHIFT]), new Uint8Array([KEY_UP, SHIFT]));
createButton(s, "z", new Uint8Array([KEY_DOWN, Z]), new Uint8Array([KEY_UP, Z]));
createButton(s, "x", new Uint8Array([KEY_DOWN, X]), new Uint8Array([KEY_UP, X]));
createButton(s, "c", new Uint8Array([KEY_DOWN, C]), new Uint8Array([KEY_UP, C]));
