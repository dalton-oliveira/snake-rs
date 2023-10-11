import init, { GameScene } from "./wasm/wasm_render.js";

const DIRECTION = {
  LEFT: new Uint8Array([0]),
  UP: new Uint8Array([1]),
  RIGHT: new Uint8Array([2]),
  DOWN: new Uint8Array([3]),
};

await init();

const scene = GameScene.new();

const ws = new WebSocket(`ws://${location.host}/game_data`);

ws.addEventListener("message", async function (msg) {
  const data = new Uint8Array(await msg.data.arrayBuffer());
  if (data[0] === 1) scene.set_data(data.slice(1));
  if (data[0] === 2) scene.snake_id(data.slice(1));
  scene.draw();
});

function toDirection(type) {
  switch (type) {
    case "panleft":
    case "ArrowLeft":
    case "KeyJ":
    case "KeyA":
      return ws.send(DIRECTION.LEFT);
    case "panup":
    case "ArrowUp":
    case "KeyI":
    case "KeyW":
      return ws.send(DIRECTION.UP);
    case "panright":
    case "ArrowRight":
    case "KeyL":
    case "KeyD":
      return ws.send(DIRECTION.RIGHT);
    case "pandown":
    case "ArrowDown":
    case "KeyK":
    case "KeyS":
      return ws.send(DIRECTION.DOWN);
  }
}

const hammertime = new Hammer(document, {});
hammertime.get("pan").set({ direction: Hammer.DIRECTION_ALL });
hammertime.on("panleft panright panup pandown", function (ev) {
  toDirection(ev.type);
  // const threads = [];
});

document.addEventListener("keydown", (e) => {
  toDirection(e.code);
});
