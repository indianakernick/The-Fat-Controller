import "./styles.scss";
import SocketManager from "../common/SocketManager.js";
import { KEY_CLICK } from "../common/CommandCode.js";
import { LEFT_ARROW, RIGHT_ARROW, HOME, END } from "../common/Key.js";

const container = document.getElementById("container");
const last = document.getElementById("last");
const next = document.getElementById("next");
const prev = document.getElementById("prev");
const first = document.getElementById("first");
const socket = new SocketManager(container);

const lastBuf = new Uint8Array([KEY_CLICK, END]);
const nextBuf = new Uint8Array([KEY_CLICK, RIGHT_ARROW]);
const prevBuf = new Uint8Array([KEY_CLICK, LEFT_ARROW]);
const firstBuf = new Uint8Array([KEY_CLICK, HOME]);

function createButton(element, buffer) {
    element.ontouchstart = () => {
        socket.send(buffer);
        return false;
    };
    element.ontouchend = () => {
        return false;
    };
}

createButton(last, lastBuf);
createButton(next, nextBuf);
createButton(prev, prevBuf);
createButton(first, firstBuf);
