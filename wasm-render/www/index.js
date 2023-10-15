import init, { GameScene } from "./wasm/wasm_render.js";

const COMMAND = {
  LEFT: new Uint8Array([0]),
  UP: new Uint8Array([1]),
  RIGHT: new Uint8Array([2]),
  DOWN: new Uint8Array([3]),
  PONG: (data) => new Uint8Array([5, ...data.slice(1)]),
  QUIT: new Uint8Array([6]),
};

await init();

const scene = GameScene.new();
const protocol = location.protocol.replace("http", "ws");
const ws = new WebSocket(`${protocol}//${location.host}/game_data`);

ws.addEventListener("message", async function (msg) {
  // const now = performance.now();
  const buf = await msg.data.arrayBuffer();
  // console.log(`buf ${performance.now() - now} ms`);
  const data = new Uint8Array(buf);
  if (data[0] === 1) scene.set_data(data.slice(1));
  if (data[0] === 2) scene.snake_id(data.slice(1));
  if (data[0] === 3) {
    ws.send(COMMAND.PONG(data));
    return;
  }
  scene.draw();
});

function toDirection(type) {
  switch (type) {
    case "panleft":
    case "ArrowLeft":
    case "KeyJ":
    case "KeyA":
      return ws.send(COMMAND.LEFT);
    case "panup":
    case "ArrowUp":
    case "KeyI":
    case "KeyW":
      return ws.send(COMMAND.UP);
    case "panright":
    case "ArrowRight":
    case "KeyL":
    case "KeyD":
      return ws.send(COMMAND.RIGHT);
    case "pandown":
    case "ArrowDown":
    case "KeyK":
    case "KeyS":
      return ws.send(COMMAND.DOWN);
    case "KeyQ":
      return ws.send(COMMAND.QUIT);
  }
}

const hammertime = new Hammer(document, {});
hammertime.get("pan").set({ direction: Hammer.DIRECTION_ALL });
hammertime.on("panleft panright panup pandown", function (ev) {
  toDirection(ev.type);
});

document.addEventListener("keydown", (e) => {
  toDirection(e.code);
});
