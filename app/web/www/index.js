import init, { Universe, FrontKey } from "./wasm/snake_web.js";
import { drawFullGrid } from "./utils.js";

let paused = true;
function pause() {
  paused = !paused;
}

await init();

const universe = Universe.new();

// @todo reverter essa palhaçada pra switch
export const KEY_MAPPINGS = {
  ArrowUp: () => key(FrontKey.Up),
  ArrowRight: () => key(FrontKey.Right),
  ArrowDown: () => key(FrontKey.Down),
  ArrowLeft: () => key(FrontKey.Left),
  KeyI: () => key(FrontKey.Up),
  KeyL: () => key(FrontKey.Right),
  KeyK: () => key(FrontKey.Down),
  KeyJ: () => key(FrontKey.Left),
  KeyG: drawFullGrid,
  KeyP: pause,
  Space: pause,
};

document.addEventListener("keydown", (e) => {
  const func = KEY_MAPPINGS[e.code];
  if (func) {
    func();
    // e.preventDefault();
  }
});

let lastTick = performance.now();

function key(key) {
  universe.key_down(key);
}

function tick() {
  const nextTick = 300 - (performance.now() - lastTick) / 1000;
  if (!paused) {
    universe.tick();
    // drawFullGrid();
  }
  lastTick = performance.now();
  setTimeout(tick, nextTick);
}

// drawFullGrid();
tick();
