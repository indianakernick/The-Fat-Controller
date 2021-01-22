import "../../css/controller.css";

const button0 = document.getElementById("button-0");
let socket;

function connect() {
    socket = new WebSocket(`ws://${location.host}/socket`);
    socket.onopen = () => {
        button0.ontouchstart = () => {
            socket.send("down");
            return false;
        };
        button0.ontouchend = () => {
            socket.send("up");
            return false;
        };
    };

    socket.onclose = e => {
        button0.ontouchstart = undefined;
        button0.ontouchend = undefined;
        if (e.code !== 1000) {
            setTimeout(connect, 1000);
        }
    };
}

connect();
