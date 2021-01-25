export default function(socket, id, buffer) {
    let count = 0;
    const element = document.getElementById(id);
    element.ontouchstart = () => {
        if (++count === 1) {
            socket.send(buffer);
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
