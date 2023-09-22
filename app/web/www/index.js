import init, { Universe, FrontKey } from "./wasm/snake_web.js";
await init();

const universe = Universe.new();
// universe.tick();

// console.log(universe);
// universe.tick();
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
      // universe.key_down(FrontKey.Left);
      e.preventDefault();
      break;
  }
});

const canvas = document.getElementById("snake-canvas");
const ctx = canvas.getContext("2d");

const pixelSize = 12.0; // Adjust the grid size as needed

function drawGrid(width, height, stroke, off_x, off_y) {
  ctx.strokeStyle = stroke; // Set the grid line color
  ctx.lineWidth = 1;

  // Draw vertical grid lines
  for (let x = off_x; x < canvas.width; x += width) {
    ctx.beginPath();
    ctx.moveTo(x, off_y);
    ctx.lineTo(x, canvas.height - off_y + 4.5);
    ctx.stroke();
  }

  // Draw horizontal grid lines
  for (let y = off_y; y < canvas.height; y += height) {
    ctx.beginPath();
    ctx.moveTo(off_x, y);
    ctx.lineTo(canvas.width - off_x + 4.5, y);
    ctx.stroke();
  }
}

function drawFullGrid() {
  drawGrid(pixelSize, pixelSize, "lightgrey", 0, 0);
  // drawGrid(pixelSize * 2, pixelSize * 2, "green", pixelSize * 2, pixelSize * 2);
  // drawGrid(gridWidth, gridHeight, "purple", pixelSize * 2, pixelSize * 1);
  // drawGrid(gridHeight, gridWidth, "red", pixelSize * 1, pixelSize * 2);
}

function tick() {
  if (!paused) {
    universe.tick();
    drawFullGrid();
  }
  setTimeout(tick, 1000);
}
universe.draw();
drawFullGrid();
paused = true;
tick();
// universe.tick();
