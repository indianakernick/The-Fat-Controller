const RETRY_DELAY = 1000;
const TICK_DELAY = 50;
const TICK_BUF = new ArrayBuffer(0);
const TICK_IDLE_COUNT = 30 * 1000 / TICK_DELAY;

export default class SocketManager {
    constructor(element) {
        this.element = element;
        this.socket = null;
        this.tickId = -1;
        this.tickCount = 0;
        this.connect();
    }

    startTick() {
        this.tickId = setInterval(() => {
            this.socket.send(TICK_BUF);
            if (++this.tickCount > TICK_IDLE_COUNT) {
                clearInterval(this.tickId);
                this.tickId = -1;
            }
        }, TICK_DELAY);
    }

    connect() {
        this.socket = new WebSocket(`ws://${location.host}/socket`);
        this.socket.onopen = () => {
            this.element.classList.remove("offline");
            this.tickCount = 0;
            this.startTick();
        };
        this.socket.onclose = e => {
            this.element.classList.add("offline");
            clearInterval(this.tickId);
            if (e.code !== 1000) {
                setTimeout(() => this.connect(), RETRY_DELAY);
            }
        };
    }

    send(data) {
        this.socket.send(data);
        this.tickCount = 0;
        if (this.tickId === -1) {
            this.startTick();
        }
    }
}
