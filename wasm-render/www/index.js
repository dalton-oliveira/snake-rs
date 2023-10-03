import init, { GameScene } from "./wasm/wasm_render.js";

await init();

const scene = GameScene.new();

const ws = new WebSocket(`ws://${location.host}/game_data`);

document.addEventListener("keydown", (e) => {
  ws.send(e.code);
});

ws.onmessage = async function (msg) {
  const data = new Uint8Array(await msg.data.arrayBuffer());
  if (data[0] === 1) scene.set_data(data.slice(1));
  if (data[0] === 2) scene.snake_id(data.slice(1));
  scene.draw();
};
