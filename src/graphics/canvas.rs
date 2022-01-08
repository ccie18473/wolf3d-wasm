use crate::prelude::*;

pub struct Canvas {
    pub width: usize,
    pub height: usize,

    buffer: Vec<u8>,
}

impl Canvas {
    pub fn new(_ctx: &mut Context, width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            buffer: vec![0; width * height * 4],
        }
    }
    pub fn put_pixel(&mut self, x: usize, y: usize, color: RGBColor) {
        let offset = (y * self.width + x) * 4;

        self.buffer[offset] = color.r;
        self.buffer[offset + 1] = color.g;
        self.buffer[offset + 2] = color.b;
        self.buffer[offset + 3] = 255;
    }
    pub fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        let p = ggez::graphics::DrawParam::new().dest(Point2::new(0.0, 0.0));
        let image = ggez::graphics::Image::from_rgba8(
            ctx,
            self.width as u16,
            self.height as u16,
            &self.buffer,
        )?;
        ggez::graphics::draw(ctx, &image, p)?;

        Ok(())
    }
    pub fn draw_fps(&mut self, ctx: &mut Context) -> GameResult {
        let fps = ggez::timer::fps(ctx);
        let fps_display = ggez::graphics::Text::new(format!("FPS: {:.2}", fps));
        let p = cgmath::Point2::new(0.0, 0.0);
        ggez::graphics::draw(ctx, &fps_display, (p,))?;

        let w = self.width;
        let w_display = ggez::graphics::Text::new(format!("Width: {}", w));
        let p = cgmath::Point2::new(0.0, 15.0);
        ggez::graphics::draw(ctx, &w_display, (p,))?;

        let h = self.height;
        let h_display = ggez::graphics::Text::new(format!("Weight: {}", h));
        let p = cgmath::Point2::new(0.0, 30.0);
        ggez::graphics::draw(ctx, &h_display, (p,))?;

        Ok(())
    }
}
