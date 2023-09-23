import { SPACING, PIXEL_SIZE } from "./render.js";

const canvas = document.getElementById("snake-canvas");
const ctx = canvas.getContext("2d");

export const p = (v) => v * (PIXEL_SIZE + SPACING);

const drawPixelGrid = (width, height, stroke, off_x, off_y) => {
  drawGrid(p(width), p(height), stroke, p(off_x), p(off_y));
};

export function drawGrid(width, height, stroke, off_x, off_y) {
  ctx.strokeStyle = stroke; // Set the grid line color
  ctx.lineWidth = SPACING;

  // Draw vertical grid lines
  for (let x = off_x - SPACING; x < canvas.width; x += width) {
    ctx.beginPath();
    ctx.moveTo(x, off_y);
    ctx.lineTo(x, canvas.height - off_y);
    ctx.stroke();
  }

  // Draw horizontal grid lines
  for (let y = off_y - SPACING; y < canvas.height; y += height) {
    ctx.beginPath();
    ctx.moveTo(off_x, y);
    ctx.lineTo(canvas.width - off_x + SPACING, y);
    ctx.stroke();
  }
}

export function drawFullGrid() {
  drawPixelGrid(1, 1, "lightgrey", 0, 0);
  // drawPixelGrid(2, 2, "green", 2, 2);
  // drawPixelGrid(2, 4, "purple", 2, 1);
  // drawPixelGrid(4, 2, "red", 3, 2);
}
