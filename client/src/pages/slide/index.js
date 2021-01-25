import "./styles.scss";
import SocketManager from "../common/SocketManager.js";
import { KEY_CLICK } from "../common/CommandCode.js";
import { LEFT_ARROW, RIGHT_ARROW, HOME, END } from "../common/Key.js";
import createButton from "../common/createButton.js";

const s = new SocketManager(document.getElementById("container"));

createButton(s, "last", [new Uint8Array([KEY_CLICK, END])]);
createButton(s, "next", [new Uint8Array([KEY_CLICK, RIGHT_ARROW])]);
createButton(s, "prev", [new Uint8Array([KEY_CLICK, LEFT_ARROW])]);
createButton(s, "first", [new Uint8Array([KEY_CLICK, HOME])]);
