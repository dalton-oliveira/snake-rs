use snake::types::FieldPoint;

pub trait Screen {
    fn setup(&self, width: u16, height: u16);
    fn field_sprite_4x2(&self, sprite: u8, p: &FieldPoint);
    fn field_sprite_2x4(&self, sprite: u8, p: &FieldPoint);
    fn field_sprite_3x3(&self, sprite: u8, p: &FieldPoint);
    fn field_sprite_8x4(&self, sprite: u32, p: &FieldPoint);
    fn panel_sprite_3x5(&self, sprite: u16, px: i16);
    fn panel_sprite_8x4(&self, sprite: u32, x_off_pixels: i16, y_off_pixels: u16);
}
