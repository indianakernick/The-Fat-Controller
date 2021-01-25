import "./styles.scss";
import Pressure from "pressure";
import SocketManager from "../common/SocketManager.js";

const button = document.getElementById("button");
const socket = new SocketManager(button);
let down = false;

const FORCE_THRESHOLD = 0.25;

Pressure.set(button, {
    change(force) {
        if (down) {
            if (force < FORCE_THRESHOLD) {
                down = false;
                socket.send(UP);
                button.classList.remove("down");
            }
        } else {
            if (force >= FORCE_THRESHOLD) {
                down = true;
                socket.send(DOWN);
                button.classList.add("down");
            }
        }
    }
});

button.ontouchend = button.ontouchcancel = () => {
    if (down) {
        down = false;
        socket.send(UP);
        button.classList.remove("down");
    }
};
