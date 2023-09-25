import { PANEL_BLOCK_WIDTH, PANEL_BLOCK_HEIGHT } from "./constants.js";
import { clearRect, drawSprite } from "./screen.js";

export function drawSprite3x5(sprite, xBlock) {
  const x0 = panelBlockToPixel(xBlock);
  clearRect(x0, 1, PANEL_BLOCK_WIDTH, PANEL_BLOCK_HEIGHT);
  drawSprite(sprite, x0, 1, PANEL_BLOCK_WIDTH, PANEL_BLOCK_HEIGHT);
}

export const panelBlockToPixel = (x) => x * (PANEL_BLOCK_WIDTH + 1) + 1;

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
