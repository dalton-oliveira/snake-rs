export const SPACING = 1;
export const PIXEL_W = 6;
export const PIXEL_H = 9;

const BG = "rgb(164, 197, 63)";
const PIXEL_COLOR = "rgb(59, 75, 20)";

const canvas = document.getElementById("snake-canvas");
const ctx = canvas.getContext("2d");

export function setupScreen(width, height) {
  canvas.style.backgroundColor = BG;
  canvas.width = width * (PIXEL_W + SPACING) + 1;
  canvas.height = height * (PIXEL_H + SPACING);
  blurryPixels();
}

/**
 *
 * @param {number} sprite unsigned integer of lenght param.widh * param.height
 * @param {number} x0 Pixel start x coordinate
 * @param {number} y0 Pixel start y coordinate
 * @param {number} width Sprite width in pixels
 * @param {number} height Sprite height in pixels
 * @param {(number, number) => [number, number]} translateCallback The translate function
 */
export function drawSprite(sprite, x0, y0, width, height, translateFunc) {
  if (sprite === 0) return; // clear only
  for (let i = 0; i < width * height; i++) {
    if ((sprite & (1 << i)) === 0) continue;
    let x = (i % width) + x0;
    let y = Math.floor(i / width) + y0;
    [x, y] = translateFunc ? translateFunc(x, y) : [x, y];
    pixel(x, y);
  }
}

export function clearRect(x, y, w, h) {
  [w, h] = pixelRectToCanvas(w, h);
  [x, y] = pixelToCanvas(x, y);
  // hack: rect shadows was not being totally cleared
  ctx.clearRect(x - 0.5, y - 0.5, w + 0.5, h + 0.5);
}

export function rect(x, y, width, height) {
  lineX(x, y, width);
  lineX(x, y + height - 1, width);
  lineY(x, y, height);
  lineY(x + width - 1, y, height);
}

export function pixel(x, y) {
  const [x0, y0] = pixelToCanvas(x, y);
  ctx.fillRect(x0, y0, PIXEL_W, PIXEL_H);
}

export function lineX(x0, y, length) {
  for (let x = 0; x < length; x++) {
    pixel(x0 + x, y);
  }
}

export function lineY(x, y0, length) {
  for (let y = 0; y < length; y++) {
    pixel(x, y0 + y);
  }
}

const pixelToCanvas = (x, y) => [
  x * (PIXEL_W + SPACING) + 1,
  y * (PIXEL_H + SPACING),
];

const pixelRectToCanvas = (w, h) => [
  w * (PIXEL_W + SPACING),
  h * (PIXEL_H + SPACING),
];

function blurryPixels() {
  ctx.shadowBlur = 1;
  ctx.shadowColor = PIXEL_COLOR;
  ctx.fillStyle = PIXEL_COLOR;
  ctx.save();
}
