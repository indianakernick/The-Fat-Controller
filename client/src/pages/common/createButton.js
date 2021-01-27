export function createDownButtonElem(socket, element, buf) {
    let count = 0;
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

export function createDownButton(socket, id, buf) {
    createDownButtonElem(socket, document.getElementById(id), buf);
}

export function createDownUpCustomButtonElem(element, down, up) {
    let count = 0;
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

export function createDownUpCustomButton(id, down, up) {
    createDownUpCustomButtonElem(document.getElementById(id), down, up);
}
