import "./styles.scss";
import SocketManager from "../common/SocketManager.js";

const button = document.getElementById("button");
const socket = new SocketManager(button);

button.ontouchstart = () => {
    socket.send(DOWN);
    return false;
};

button.ontouchend = () => {
    socket.send(UP);
    return false;
};
