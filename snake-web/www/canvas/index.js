import { rect, lineX, setupScreen, SPACING } from "./screen.js";
import { BLOCK_PIXELS, PANEL_HEIGHT } from "./constants.js";
import { setupField } from "./field.js";
import { setupPanel } from "./panel.js";

export function setup(width, height) {
  const fieldPixelWidth = (width + 1) * BLOCK_PIXELS;
  const fieldPixelHeight = (height + 1) * BLOCK_PIXELS;
  const innerFieldHeight = fieldPixelHeight + BLOCK_PIXELS;

  const widthPixels = fieldPixelWidth + BLOCK_PIXELS;
  const heightPixels = innerFieldHeight + PANEL_HEIGHT;
  setupScreen(widthPixels, heightPixels, SPACING);

  rect(0, PANEL_HEIGHT, widthPixels, innerFieldHeight);
  const xMax = fieldPixelWidth - 1;
  const yMax = fieldPixelHeight - 1;
  setupField({ xMax, yMax, xOff: 1, yOff: PANEL_HEIGHT + 1 });

  setupPanel(fieldPixelWidth);
  lineX(0, PANEL_HEIGHT - 2, widthPixels);
}
