export const SPACING = 1;
export const PIXEL_W = 6;
export const PIXEL_H = 9;

const canvas = document.getElementById("snake-canvas");
const ctx = canvas.getContext("2d");

const UP = 0;
const RIGHT = 1;
const DOWN = 2;
const LEFT = 3;
export const BLOCK_PIXELS = 2;
const BLOCK_W = (PIXEL_W + SPACING) * BLOCK_PIXELS;
const BLOCK_H = (PIXEL_H + SPACING) * BLOCK_PIXELS;
export const PIXEL_X_OFFSET = 1;
export const PIXEL_Y_OFFSET = 1;
const BLOCK_PIXELS_H = { width: 4, height: 2, xOff: 1, yOff: 0 };
const BLOCK_PIXELS_V = { width: 2, height: 4, xOff: 0, yOff: 1 };

const PANEL_BLOCK_HEIGHT = 5;
const PANEL_BLOCK_WIDTH = 3;
// const numberSpacing = 1;
const PANEL_HEIGHT = 3 + PANEL_BLOCK_HEIGHT + SPACING;

const horizontalSprite = (direction) => [UP, DOWN].includes(direction);
/**
 *
 * @param {UP | RIGHT | DOWN | LEFT } direction
 * @returns {{width: number, xOff: number, yOff: number}} bp
 */
const blockPixels = (direction) =>
  horizontalSprite(direction) ? BLOCK_PIXELS_H : BLOCK_PIXELS_V;

/**
 * Transforms a sprite (unsiged 8bits) into canvas 'pixels'. When going to Right or Left,
 * sprites are draw as vertical blocks (width 2 and height 4). Going Up or Down,
 * sprites are draw as horizontal blocks (width 4 and height 2).
 * @param {number} sprite
 * @param {UP | RIGHT | DOWN | LEFT } direction
 * @param {number} x
 * @param {number} y
 */
export const drawSprite = (sprite, direction, x, y) => {
  clearBlock(x, y, direction);
  if (sprite == 0) return;
  const bp = blockPixels(direction);
  const [x0, y0] = blockToPixel(x, y);
  // console.log({ x0, y0 });
  for (let i = 0; i < 8; i++) {
    if ((sprite & (1 << i)) !== 0) {
      let x = i % bp.width;
      let y = Math.floor(i / bp.width);
      pixel(x0 + x - bp.xOff, y0 + y - bp.yOff);
    }
  }
};

export const drawPanelSprite = (x, xOff, sprite) => {
  clearPanelBlock(x);
  const x0 = panelBlockToPixel(x, xOff) + xOff;
  for (let i = 0; i < PANEL_BLOCK_WIDTH * PANEL_BLOCK_HEIGHT; i++) {
    if ((sprite & (1 << i)) !== 0) {
      let x = i % PANEL_BLOCK_WIDTH;
      let y = Math.floor(i / PANEL_BLOCK_WIDTH);
      pixel(x0 + x, y + 1);
    }
  }
};

export const clearPanelBlock = (x) => {
  const [w, h] = pixelRectToCanvas(PANEL_BLOCK_WIDTH, PANEL_BLOCK_HEIGHT);
  x = panelBlockToPixel(x);
  x = pixelToCanvas(x);
  ctx.clearRect(x, 0, w, h);
};

export const panelBlockToPixel = (x, spacing) =>
  x * (PANEL_BLOCK_WIDTH + spacing);

/**
 * Clears sprite blocks
 * @param {number} x
 * @param {number} y
 * @param {UP | RIGHT | DOWN | LEFT} direction
 */
export const clearBlock = (x, y, direction) => {
  const bp = blockPixels(direction);
  const [w, h] = pixelRectToCanvas(bp.width, bp.height);
  [x, y] = blockToPixel(x, y);
  [x, y] = pixelToCanvas(x - bp.xOff, y - bp.yOff);
  ctx.clearRect(x, y, w, h);
};

export const pixel = (x, y) => {
  const [x0, y0] = pixelToCanvas(x, y);
  ctx.fillStyle = "rgb(59, 75, 20)";
  ctx.fillRect(x0, y0, PIXEL_W, PIXEL_H);
};

const blockToPixel = (x, y) => [
  PIXEL_X_OFFSET + x * BLOCK_PIXELS,
  PIXEL_Y_OFFSET + PANEL_HEIGHT + 1 + y * BLOCK_PIXELS,
];

const pixelToCanvas = (x, y) => [
  x * (PIXEL_W + SPACING),
  y * (PIXEL_H + SPACING),
];

const pixelRectToCanvas = (w, h) => [
  w * PIXEL_W + SPACING,
  h * PIXEL_H + SPACING,
];

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

export function rect(x, y, width, height) {
  lineX(x, y, width);
  lineX(x, y + height - 1, width);
  lineY(x, y, height);
  lineY(x + width - 1, y, height);
}

export function setup(width, height) {
  const innerWidth = width * 2 * BLOCK_PIXELS + PIXEL_X_OFFSET * 2;
  const innerHeight = height * 2 * BLOCK_PIXELS + PIXEL_Y_OFFSET * 2 + 1;

  canvas.width = innerWidth * (PIXEL_W + SPACING);
  canvas.height = (innerHeight + PANEL_HEIGHT) * (PIXEL_H + SPACING);
  rect(0, PANEL_HEIGHT, innerWidth, innerHeight);
  lineX(0, PANEL_HEIGHT - 2, innerWidth);
}
