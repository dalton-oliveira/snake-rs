import { PANEL_BLOCK_WIDTH, PANEL_BLOCK_HEIGHT } from "./constants.js";
import { clearRect, drawSprite } from "./screen.js";

export function drawSprite3x5(sprite, xBlock) {
  const x0 = panelBlockToPixel(xBlock);
  clearRect(x0, 1, PANEL_BLOCK_WIDTH, PANEL_BLOCK_HEIGHT);
  drawSprite(sprite, x0, 1, PANEL_BLOCK_WIDTH, PANEL_BLOCK_HEIGHT);
}

export function drawPanelSprite8x4(sprite, xOffPixels, yOffPixels) {
  const x0 = xOffPixels >= 0 ? xOffPixels : xMax + xOffPixels;
  clearRect(x0, yOffPixels, 8, 4);
  drawSprite(sprite, x0, yOffPixels, 8, 4);
}

export const panelBlockToPixel = (x) => {
  if (x >= 0) return x * (PANEL_BLOCK_WIDTH + 1) + 1;
  return xMax + x * (PANEL_BLOCK_WIDTH + 1) + 3;
};

let xMax;
/**
 *
 * @param {Object} panelParams
 * @param {Object} panelParams.sprite
 * @param {Object} panelParams.sprite.width
 */
export function setupPanel(width) {
  xMax = width - 1;
}
