import { drawSprite, clearRect, pixel } from "./screen.js";
import { BLOCK_PIXELS } from "./constants.js";

/**
 * Renders a sprite (unsiged 8bits int) into block of 4x2 'pixels'.
 * @param {number} sprite
 * @param {number} fieldX Must be between [0..gameWidth * 2)
 * @param {number} fieldY Must be between [0..gameHeight * 2)
 */
export function drawSprite4x2(sprite, fieldX, fieldY) {
  const [x0, y0] = [fieldX * BLOCK_PIXELS - 1, fieldY * BLOCK_PIXELS];
  clearFieldSprite(x0, y0, 4, 2);
  drawSprite(sprite, x0, y0, 4, 2, translate);
}

/**
 * Transforms a sprite (unsiged 8bits int) into block of 2x4 'pixels'.
 * @param {number} sprite
 * @param {number} fieldX Must be between [0..gameWidth * 2)
 * @param {number} fieldY Must be between [0..gameHeight * 2)
 */
export function drawSprite2x4(sprite, fieldX, fieldY) {
  const [x0, y0] = [fieldX * BLOCK_PIXELS, fieldY * BLOCK_PIXELS - 1];
  clearFieldSprite(x0, y0, 2, 4);
  drawSprite(sprite, x0, y0, 2, 4, translate);
}

export function drawSprite3x3(sprite, fieldX, fieldY) {
  const [x0, y0] = [fieldX * BLOCK_PIXELS - 1, fieldY * BLOCK_PIXELS - 1];
  clearFieldSprite(x0, y0, 3, 3);
  drawSprite(sprite, x0, y0, 3, 3, translate);
}

export function drawSprite8x4(sprite, fieldX, fieldY) {
  const [x0, y0] = [fieldX * BLOCK_PIXELS - 1, fieldY * BLOCK_PIXELS - 1];
  clearFieldSprite(x0, y0, 8, 4);
  drawSprite(sprite, x0, y0, 8, 4, translate);
}

/**
 * Coordinate translation to field area
 * @param {number} x
 * @param {number} y
 */
function translate(x, y) {
  const [rx, ry] = reflectOnBorder(x, y);
  return [rx + xOff, ry + yOff];
}

/**
 * Clears a field sprite. If existing reflection, also clear it up.
 * @param {number} x
 * @param {number} y
 * @param {number} width
 * @param {number} height
 */
function clearFieldSprite(x, y, width, height) {
  clearFieldRectRaw(x, y, width, height);
  const reflected = reflectFieldSprite(x, y, width, height);
  if (reflected) {
    clearFieldRectRaw(reflected[0], reflected[1], width, height);
  }
}

function reflectFieldSprite(x, y, w, h) {
  let [rx, ry] = reflectOnBorder(x, y);
  if (rx != x || ry != y) return [rx, ry];
  if (x + w > xMax) return [1, y];
  if (y + h > yMax) return [x, 1];
}

const clearFieldRectRaw = (x, y, w, h) => clearRect(x + xOff, y + yOff, w, h);

const reflectOnBorder = (x, y) => [reflect(x, xMax), reflect(y, yMax)];

function reflect(n, max) {
  if (n > max) return 1;
  if (n === 0) return max - 1;
  return n;
}

// field params
let xMax;
let yMax;
let xOff;
let yOff;

/**
 *
 * @param params Field setup params
 * @param.xMax Limit on x-axis
 * @param.yMax Limit on y-axis
 * @param.xOff Offset on x due to borders
 * @param.yOff Offset on y due to borders
 */
export function setupField(params) {
  xMax = params.xMax;
  yMax = params.yMax;
  xOff = params.xOff;
  yOff = params.yOff;
}
