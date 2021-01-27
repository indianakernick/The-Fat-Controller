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

export function createDownUpCustomButton(socket, id, down, up) {
    let count = 0;
    const element = document.getElementById(id);
    element.ontouchstart = () => {
        if (++count === 1) {
            down();
            element.classList.add("down");
        }
        return false;
    };
    element.ontouchend = element.ontouchcancel = () => {
        if (--count === 0) {
            up();
            element.classList.remove("down");
        }
        return false;
    };
}
