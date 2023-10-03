import { SPACING, PIXEL_W, PIXEL_H } from "./screen.js";
import {
  PANEL_HEIGHT,
  PIXEL_X_OFFSET,
  PIXEL_Y_OFFSET,
  BLOCK_PIXELS,
} from "./constants.js";

const canvas = document.getElementById("snake-canvas");
const ctx = canvas.getContext("2d");

export const w = (v) => v * (PIXEL_W + SPACING);
export const h = (v) => v * (PIXEL_H + SPACING);

const drawPixelGrid = (width, height, stroke, off_x, off_y) => {
  drawGrid(w(width), h(height), stroke, w(off_x), h(off_y));
};

export function drawGrid(width, height, stroke, off_x, off_y) {
  ctx.save();
  ctx.strokeStyle = stroke; // Set the grid line color
  ctx.lineWidth = SPACING / 2;

  // Draw vertical grid lines
  for (let x = off_x - SPACING; x < canvas.width; x += width) {
    ctx.beginPath();
    ctx.moveTo(x + SPACING, off_y);
    ctx.lineTo(x + SPACING, canvas.height);
    ctx.stroke();
  }

  // Draw horizontal grid lines
  for (let y = off_y - SPACING; y < canvas.height; y += height) {
    ctx.beginPath();
    ctx.moveTo(off_x + SPACING, y);
    ctx.lineTo(canvas.width - off_x + SPACING, y);
    ctx.stroke();
  }
  ctx.restore();
}

export function drawFullGrid() {
  // drawPixelGrid(1, 1, "lightgrey", 0, 0);
  drawPixelGrid(
    BLOCK_PIXELS,
    BLOCK_PIXELS,
    "green",
    PIXEL_X_OFFSET,
    PIXEL_Y_OFFSET + PANEL_HEIGHT
  );
  // drawPixelGrid(
  //   2,
  //   4,
  //   "purple",
  //   PIXEL_X_OFFSET,
  //   PIXEL_Y_OFFSET + PANEL_HEIGHT + 1
  // );
  // drawPixelGrid(4, 2, "red", PIXEL_X_OFFSET + 1, PIXEL_Y_OFFSET + PANEL_HEIGHT);
  // drawPixelGrid(4, 2, "red", 3, 2);
}
