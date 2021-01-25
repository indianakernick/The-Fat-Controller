const RETRY_DELAY = 1000;
const JITTER_DELAY = 50;
const JITTER_BUF = new ArrayBuffer(0);

export default class SocketManager {
    constructor(element) {
        this.element = element;
        this.socket = null;
        this.jitterId = -1;
        this.connect();
    }

    connect() {
        this.socket = new WebSocket(`ws://${location.host}/socket`);
        this.socket.onopen = () => {
            this.element.classList.remove("offline");
            this.jitterId = setInterval(() => {
                this.socket.send(JITTER_BUF);
            }, JITTER_DELAY);
        };
        this.socket.onclose = e => {
            this.element.classList.add("offline");
            clearInterval(this.jitterId);
            if (e.code !== 1000) {
                setTimeout(() => this.connect(), RETRY_DELAY);
            }
        };
    }

    send(data) {
        this.socket.send(data);
    }
}
