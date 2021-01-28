import "./styles.scss";
import SocketManager from "../common/SocketManager.js";
import {
    MOUSE_MOVE_RELATIVE,
    MOUSE_SCROLL_X,
    MOUSE_SCROLL_Y,
    MOUSE_DOWN,
    MOUSE_UP
} from "../common/CommandCode.js";
import { LEFT } from "../common/MouseButton.js";

const moveBuf = new Uint8Array([MOUSE_MOVE_RELATIVE, 0, 0, 0, 0]);
const scrollXBuf = new Uint8Array([MOUSE_SCROLL_X, 0, 0]);
const scrollYBuf = new Uint8Array([MOUSE_SCROLL_Y, 0, 0]);
const scrollXYBuf = new Uint8Array([MOUSE_SCROLL_X, 0, 0, MOUSE_SCROLL_Y, 0, 0]);
const downBuf = new Uint8Array([MOUSE_DOWN, LEFT]);
const upBuf = new Uint8Array([MOUSE_UP, LEFT]);

const MOVE_SCALE = 2.0;
const SCROLL_SCALE = 1.8;
const FORCE_THRESHOLD = 0.3;

function copyInt16(buffer, index, integer) {
    buffer[index] = (integer >> 8) & 0xFF
    buffer[index + 1] = integer & 0xFF;
}

function mouseMove(changeX, changeY) {
    changeX = Math.round(changeX * MOVE_SCALE);
    changeY = Math.round(changeY * MOVE_SCALE);
    copyInt16(moveBuf, 1, changeX);
    copyInt16(moveBuf, 3, changeY);
    socket.send(moveBuf);
}

function scaleNonZero(num, scale) {
    if (num > 0) {
        return Math.round(Math.max(1, num * scale));
    } else if (num < 0) {
        return Math.round(Math.min(-1, num * scale));
    } else {
        return 0;
    }
}

function mouseScroll(changeX, changeY) {
    if (changeX) {
        changeX = scaleNonZero(changeX, SCROLL_SCALE);
    }
    if (changeY) {
        changeY = scaleNonZero(changeY, SCROLL_SCALE);
    }

    if (changeX && changeY) {
        copyInt16(scrollXYBuf, 1, changeX);
        copyInt16(scrollXYBuf, 4, changeY);
        socket.send(scrollXYBuf);
    } else if (changeX) {
        copyInt16(scrollXBuf, 1, changeX);
        socket.send(scrollXBuf);
    } else if (changeY) {
        copyInt16(scrollYBuf, 1, changeY);
        socket.send(scrollYBuf);
    }
}

class TouchHandler {
    constructor() {
        this.touches = [];
        this.down = false;
    }

    static copyTouch(touch) {
        return {
            id: touch.identifier,
            x: touch.clientX,
            y: touch.clientY,
            force: touch.force
        };
    }

    static mouseMove(from, to) {
        mouseMove(to.clientX - from.x, to.clientY - from.y);
    }

    static updatePos(from, to) {
        from.x = to.clientX;
        from.y = to.clientY;
    }

    mouseUp() {
        socket.send(upBuf);
        this.down = false;
        pad.classList.remove("down");
    }

    mouseDown() {
        socket.send(downBuf);
        this.down = true;
        pad.classList.add("down");
    }

    findIndex(id) {
        return this.touches.findIndex(touch => {
            return touch.id === id;
        });
    }

    find(id) {
        return this.touches.find(touch => {
            return touch.id === id;
        });
    }

    getAvgPos() {
        const pos = {x: 0, y: 0};
        for (const touch of this.touches) {
            pos.x += touch.x;
            pos.y += touch.y;
        }
        pos.x /= this.touches.length;
        pos.y /= this.touches.length;
        return pos;
    }

    start(e) {
        for (const touch of e.changedTouches) {
            this.touches.push(TouchHandler.copyTouch(touch));
        }
    }

    move(e) {
        if (this.touches.length === 1) {
            TouchHandler.mouseMove(this.touches[0], e.changedTouches[0]);
            TouchHandler.updatePos(this.touches[0], e.changedTouches[0]);
        } else if (this.touches.length === 2) {
            const posBefore = this.getAvgPos();
            for (const touch of e.changedTouches) {
                TouchHandler.updatePos(this.find(touch.identifier), touch);
            }
            const posAfter = this.getAvgPos();
            mouseScroll(posAfter.x - posBefore.x, posAfter.y - posBefore.y);
        } else {
            for (const touch of e.changedTouches) {
                TouchHandler.updatePos(this.find(touch.identifier), touch);
            }
        }
    }

    end(e) {
        if (this.touches.length === 1) {
            TouchHandler.mouseMove(this.touches[0], e.changedTouches[0]);
            this.touches.splice(0, 1);
            if (this.down) {
                this.mouseUp();
            }
        } else {
            this.cancel(e);
        }
    }

    cancel(e) {
        for (const touch of e.changedTouches) {
            this.touches.splice(this.findIndex(touch.identifier), 1);
        }
        if (this.down) {
            this.mouseUp();
        }
    }

    forceChange(e) {
        if (this.touches.length === 1) {
            const force = e.changedTouches[0].force;
            this.touches[0].force = force;
            if (this.down) {
                if (force < FORCE_THRESHOLD) {
                    this.mouseUp();
                }
            } else {
                if (force >= FORCE_THRESHOLD) {
                    this.mouseDown();
                }
            }
        }
    }
}

const pad = document.getElementById("button");
const socket = new SocketManager(pad);
const touchHandler = new TouchHandler();

pad.ontouchstart = e => {
    touchHandler.start(e);
    return false;
};

pad.ontouchmove = e => {
    touchHandler.move(e);
    return false;
};

pad.ontouchend = e => {
    touchHandler.end(e);
    return false;
};

pad.ontouchcancel = e => {
    touchHandler.cancel(e);
    return false;
};

pad.ontouchforcechange = e => {
    touchHandler.forceChange(e);
    return false;
};
