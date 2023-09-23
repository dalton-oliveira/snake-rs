const canvas = document.getElementById("snake-canvas");
const ctx = canvas.getContext("2d");

const [UP, RIGHT, DOWN, LEFT] = [0, 1, 2, 3];
export const PIXEL_SIZE = 10;
export const SPACING = 1;
const PIXEL_W = 10 - SPACING;
const PIXEL_H = 10 - SPACING;
const BLOCK_PIXELS = 2;
const BLOCK_SIZE = (PIXEL_SIZE + SPACING) * BLOCK_PIXELS;
const BLOCK_PIXEL_SIZE = 2;
const PIXEL_X_OFFSET = 2;
const PIXEL_Y_OFFSET = 2;

const horizontal = (direction) => [UP, DOWN].includes(direction);

/**
 * Transforms a sprite (unsiged 8bits) into canvas 'pixels'. When going to Right or Left,
 * sprites are draw as vertical blocks (width 2 and height 4). Going Up or Down,
 * sprites are draw as horizontal blocks (width 4 and height 2).
 * @param {number} sprite
 * @param {UP | DOWN | LEFT | RIGHT} direction
 * @param {number} x
 * @param {number} y
 */
export const drawSprite = (sprite, direction, x, y) => {
  clearBlock(x, y, direction);
  if (sprite == 0) return;
  const [b_width, xOff, yOff] = horizontal(direction) ? [4, 1, 0] : [2, 0, 1];
  const [x0, y0] = blockToPixel(x, y);
  for (let i = 0; i < 8; i++) {
    if ((sprite & (1 << i)) !== 0) {
      let x = i % b_width;
      let y = Math.floor(i / b_width);
      pixel(x0 + x - xOff, y0 + y - yOff);
    }
  }
};

export const clearBlock = (x, y, direction) => {
  let [w, h, xOff, yOff] = horizontal(direction) ? [4, 2, 1, 0] : [2, 4, 0, 1];
  [w, h] = pixelRectToCanvas(w, h);
  [x, y] = blockToPixel(x, y);
  [x, y] = pixelToCanvas(x - xOff, y - yOff);
  ctx.clearRect(x, y, w, h);
};

export const pixel = (x, y) => {
  const [x0, y0] = pixelToCanvas(x, y);
  ctx.fillRect(x0, y0, PIXEL_W, PIXEL_H);
};

const blockToPixel = (x, y) => {
  return [
    PIXEL_X_OFFSET + x * BLOCK_PIXEL_SIZE,
    PIXEL_Y_OFFSET + y * BLOCK_PIXEL_SIZE,
  ];
};

const pixelToCanvas = (x, y) => [
  x * (PIXEL_W + SPACING * 2) - SPACING / 2,
  y * (PIXEL_H + SPACING * 2) - SPACING / 2,
];

const pixelRectToCanvas = (w, h) => [
  w * (PIXEL_W + SPACING),
  h * (PIXEL_H + SPACING),
];

export function setup(width, height) {
  canvas.width = width * BLOCK_SIZE * 2 + PIXEL_SIZE * 2;
  canvas.height = height * BLOCK_SIZE * 2 + PIXEL_SIZE * 2;
}
