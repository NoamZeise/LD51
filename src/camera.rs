use geometry::*;
use crate::{TextureDraw, GameObject, Colour};
use std::vec::Drain;

pub struct RectDraw {
    pub rect: Rect,
    pub colour: Colour,
}

pub struct Camera {
    rect: Rect,
    window_size: Vec2,
    size_ratio: Vec2,
    draws : Vec<TextureDraw>,
    rects : Vec<RectDraw>,
}

impl Camera {
    pub fn new(rect: Rect, window_size: Vec2) -> Camera {
        let mut cam = Camera {
            rect,
            window_size,
            draws: Vec::new(),
            rects: Vec::new(),
            size_ratio: Vec2::new(0.0, 0.0),
        };
        cam.update_size_ratio();
        cam
    }

    pub fn drain_draws(&mut self) -> Drain<TextureDraw> { 
        self.draws.drain(..)
    }

    pub fn drain_rects(&mut self) -> Drain<RectDraw> { 
        self.rects.drain(..)
    }
    
    pub fn draw(&mut self, game_obj: &GameObject) {
        self.draws.push(
            TextureDraw::new(
                game_obj.texture,
                Rect::new(
                    (game_obj.rect.x - (self.rect.x * game_obj.parallax.x)) / self.size_ratio.x,
                    (game_obj.rect.y - (self.rect.y * game_obj.parallax.y)) / self.size_ratio.y,
                    game_obj.rect.w / self.size_ratio.x,
                    game_obj.rect.h / self.size_ratio.y,
                ),
                game_obj.tex_rect,
                game_obj.colour,
            )
        );
    }

    pub fn draw_rect(&mut self, rect: Rect, colour: Colour) {
        self.rects.push(
            RectDraw {
                rect: Rect::new(
                    (rect.x - self.rect.x) / self.size_ratio.x,
                    (rect.y - self.rect.y) / self.size_ratio.y,
                    rect.w / self.size_ratio.x,
                    rect.h / self.size_ratio.y,
                ),
                colour,
            }
        )
    }

    pub fn get_offset(&self) -> Vec2 {
        return Vec2::new(self.rect.x, self.rect.y);
    }

    pub fn set_offset(&mut self, offset: Vec2) {
        self.rect.x = offset.x;
        self.rect.y = offset.y;
    }

    fn calc_offset(cam: f64, current: f64, min: f64, max: f64) -> f64 {
        if cam > max {
            return (max/2.0) - min;
        }
        let min = min + cam/2.0;
        let max = min + max - cam/1.0;
        if current > min && current < max {
            current
        } else if current < min {
            min 
        } else if current > max {
            max
        } else {
            current
        }
    }

    pub fn centre_on_pos(&mut self, p: Vec2, lim: Rect) {
        let x = Self::calc_offset(
            self.rect.w,
            p.x - (self.rect.w/2.0),
            lim.x - (self.rect.w/2.0),
            lim.w,
        );
        let y = Self::calc_offset(
            self.rect.h,
            p.y - (self.rect.h/2.0),
            lim.y - (self.rect.h/2.0),
            lim.h
        );
        self.rect.x = x;
        self.rect.y = y;
    }

    pub fn get_window_size(&self) -> Vec2 {
        self.window_size
    }

    pub fn set_window_size(&mut self, size: Vec2) {
        self.window_size = size;
        self.update_size_ratio();
    }

    pub fn get_view_size(&self) -> Vec2 {
        Vec2::new(self.rect.w, self.rect.h)
    }
    pub fn set_view_size(&mut self, view: Vec2) {
        self.rect.w = view.x;
        self.rect.h = view.y;
        self.update_size_ratio();
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.rect.w / self.rect.h
    }

    fn update_size_ratio(&mut self) {
        self.size_ratio = Vec2::new(
                self.rect.w / self.window_size.x,
                self.rect.h / self.window_size.y
        );
    }
}
