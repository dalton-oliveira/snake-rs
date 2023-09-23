import init, { Universe, FrontKey } from "./wasm/snake_web.js";
import { drawFullGrid } from "./utils.js";

let paused = true;
function pause() {
  paused = !paused;
}

await init();

const universe = Universe.new();

export const KEY_MAPPINGS = {
  ArrowUp: () => universe.key_down(FrontKey.Up),
  ArrowRight: () => universe.key_down(FrontKey.Right),
  ArrowDown: () => universe.key_down(FrontKey.Down),
  ArrowLeft: () => universe.key_down(FrontKey.Left),
  KeyG: drawFullGrid,
  KeyP: pause,
  Space: pause,
};

document.addEventListener("keydown", (e) => {
  const func = KEY_MAPPINGS[e.code];
  if (func) {
    func();
    e.preventDefault();
  }
});

function tick() {
  if (!paused) {
    universe.tick();
    // drawFullGrid();
  }
  setTimeout(tick, 1000);
}

drawFullGrid();
tick();
