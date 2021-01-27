export function createDownButton(socket, id, buf) {
    let count = 0;
    const element = document.getElementById(id);
    element.ontouchstart = () => {
        if (++count === 1) {
            socket.send(buf);
            element.classList.add("down");
        }
        return false;
    };
    element.ontouchend = element.ontouchcancel = () => {
        if (--count === 0) {
            element.classList.remove("down");
        }
        return false;
    };
}

export function createDownUpButton(socket, id, downBuf, upBuf) {
    let count = 0;
    const element = document.getElementById(id);
    element.ontouchstart = () => {
        if (++count === 1) {
            socket.send(downBuf);
            element.classList.add("down");
        }
        return false;
    };
    element.ontouchend = element.ontouchcancel = () => {
        if (--count === 0) {
            socket.send(upBuf);
            element.classList.remove("down");
        }
        return false;
    };
}
