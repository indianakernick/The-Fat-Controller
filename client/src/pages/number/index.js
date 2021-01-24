import "./styles.scss";
import SocketManager from "../common/SocketManager.js";
import { KEY_CLICK, KEY_DOWN, KEY_UP } from "../common/CommandCode.js";
import {
    N0, N1, N2, N3, N4, N5, N6, N7, N8, N9,
    BACKSPACE, EQUAL, SLASH, ASTERISK, MINUS, PLUS, RETURN, PERIOD,
    SHIFT
} from "../common/Key.js";

const container = document.getElementById("container");
const socket = new SocketManager(container);

function createButton(id, buffer) {
    const element = document.getElementById(id);
    element.ontouchstart = () => {
        socket.send(buffer);
        return false;
    };
    element.ontouchend = () => {
        return false;
    };
}

function createButtonSequence(id, sequence) {
    const element = document.getElementById(id);
    element.ontouchstart = () => {
        for (const buffer of sequence) {
            socket.send(buffer);
        }
        return false;
    };
    element.ontouchend = () => {
        return false;
    };
}

// TODO: Landscape orientation reveals more buttons

createButton("delete", new Uint8Array([KEY_CLICK, BACKSPACE]));
createButton("equal", new Uint8Array([KEY_CLICK, EQUAL]));
createButton("slash", new Uint8Array([KEY_CLICK, SLASH]));
createButton("asterisk", new Uint8Array([KEY_CLICK, ASTERISK]));
createButton("seven", new Uint8Array([KEY_CLICK, N7]));
createButton("eight", new Uint8Array([KEY_CLICK, N8]));
createButton("nine", new Uint8Array([KEY_CLICK, N9]));
createButton("minus", new Uint8Array([KEY_CLICK, MINUS]));
createButton("four", new Uint8Array([KEY_CLICK, N4]));
createButton("five", new Uint8Array([KEY_CLICK, N5]));
createButton("six", new Uint8Array([KEY_CLICK, N6]));
createButtonSequence("plus", [
    new Uint8Array([KEY_DOWN, SHIFT]),
    new Uint8Array([KEY_CLICK, PLUS]),
    new Uint8Array([KEY_UP, SHIFT]),
]);
createButton("one", new Uint8Array([KEY_CLICK, N1]));
createButton("two", new Uint8Array([KEY_CLICK, N2]));
createButton("three", new Uint8Array([KEY_CLICK, N3]));
createButton("return", new Uint8Array([KEY_CLICK, RETURN]));
createButton("zero", new Uint8Array([KEY_CLICK, N0]));
createButton("period", new Uint8Array([KEY_CLICK, PERIOD]));
