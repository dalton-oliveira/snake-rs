use snake::types::FieldPoint;

use crate::types::Screen;
pub struct LocalScreen {}

impl Screen for LocalScreen {
    fn setup(&self, width: u16, height: u16) {
        unsafe { setup(width, height) };
    }
    fn field_sprite_4x2(&self, sprite: u8, p: &FieldPoint) {
        unsafe { drawSprite4x2(sprite.reverse_bits(), p.x, p.y) };
    }
    fn field_sprite_2x4(&self, sprite: u8, p: &FieldPoint) {
        unsafe { drawSprite2x4(sprite.reverse_bits(), p.x, p.y) };
    }
    fn field_sprite_3x3(&self, sprite: u8, p: &FieldPoint) {
        unsafe { drawSprite3x3(sprite.reverse_bits(), p.x * 2 + 1, p.y * 2 + 1) };
    }
    fn field_sprite_8x4(&self, sprite: u32, p: &FieldPoint) {
        unsafe { drawSprite8x4(sprite.reverse_bits(), p.x * 2 + 1, p.y * 2 + 1) };
    }
    fn panel_sprite_3x5(&self, sprite: u16, px: i16) {
        unsafe { drawPanelSprite3x5(sprite.reverse_bits(), px) };
    }
    fn panel_sprite_8x4(&self, sprite: u32, x_off_pixels: i16, y_off_pixels: u16) {
        unsafe { drawPanelSprite8x4(sprite.reverse_bits(), x_off_pixels, y_off_pixels) };
    }
}

// JavaScript bridges
#[link(wasm_import_module = "/render/field.js")]
extern "C" {
    fn drawSprite4x2(sprite: u8, px: u16, py: u16);
    fn drawSprite2x4(sprite: u8, px: u16, py: u16);
    fn drawSprite3x3(sprite: u8, px: u16, py: u16);
    fn drawSprite8x4(sprite: u32, px: u16, py: u16);
}
#[link(wasm_import_module = "/render/panel.js")]
extern "C" {
    fn drawPanelSprite3x5(sprite: u16, px: i16);
    fn drawPanelSprite8x4(sprite: u32, xOffPixels: i16, yOffPixels: u16);
}
#[link(wasm_import_module = "/render/index.js")]
extern "C" {
    fn setup(width: u16, height: u16);
}
