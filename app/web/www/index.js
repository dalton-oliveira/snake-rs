import init, { Universe, FrontKey } from "./wasm/snake_web.js";
import { drawFullGrid } from "./utils.js";

await init();

const universe = Universe.new();
let paused = false;

document.addEventListener("keydown", (e) => {
  switch (e.code) {
    case "ArrowUp":
      universe.key_down(FrontKey.Up);
      e.preventDefault();
      break;
    case "ArrowRight":
      universe.key_down(FrontKey.Right);
      e.preventDefault();
      break;
    case "ArrowDown":
      universe.key_down(FrontKey.Down);
      e.preventDefault();
      break;
    case "ArrowLeft":
      universe.key_down(FrontKey.Left);
      e.preventDefault();
      break;
    case "KeyG":
      drawFullGrid();
      e.preventDefault();
      break;
    case "KeyP":
    case "Space":
      paused = !paused;
      e.preventDefault();
      break;
  }
});

function tick() {
  if (!paused) {
    universe.tick();
    drawFullGrid();
  }
  setTimeout(tick, 1000);
}

drawFullGrid();
paused = true;
tick();
