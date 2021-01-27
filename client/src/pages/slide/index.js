import "./styles.scss";
import SocketManager from "../common/SocketManager.js";
import { KEY_CLICK } from "../common/CommandCode.js";
import { LEFT_ARROW, RIGHT_ARROW, HOME, END } from "../common/Key.js";
import { createDownButton } from "../common/createButton.js";

const container = document.getElementById("container");
const s = new SocketManager(container);

container.ontouchstart = () => {
    return false;
};

createDownButton(s, "last", new Uint8Array([KEY_CLICK, END]));
createDownButton(s, "next", new Uint8Array([KEY_CLICK, RIGHT_ARROW]));
createDownButton(s, "prev", new Uint8Array([KEY_CLICK, LEFT_ARROW]));
createDownButton(s, "first", new Uint8Array([KEY_CLICK, HOME]));
