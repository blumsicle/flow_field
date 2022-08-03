use nannou::prelude::*;

#[derive(Default)]
pub struct Particle {
    pub position: Vec2,
    pub prev_position: Vec2,
    pub velocity: Vec2,
    pub acceration: Vec2,
    pub topspeed: f32,
    pub hue: f32,
}

impl Particle {
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceration += force;
    }

    pub fn update(&mut self) {
        self.prev_position = self.position;
        self.hue += 0.005;
        if self.hue > 1.0 {
            self.hue = 0.0;
        } else if self.hue < 0.0 {
            self.hue = 1.0;
        }

        self.velocity += self.acceration;
        self.velocity = self.velocity.clamp_length_max(self.topspeed);
        self.position += self.velocity;
        self.acceration *= 0.0;
    }

    pub fn check_edges(&mut self, bounds: Rect) {
        if self.position.x < bounds.left() {
            self.position.x = bounds.right();
            self.prev_position.x = self.position.x;
        } else if self.position.x > bounds.right() {
            self.position.x = bounds.left();
            self.prev_position.x = self.position.x;
        }

        if self.position.y < bounds.bottom() {
            self.position.y = bounds.top();
            self.prev_position.y = self.position.y;
        } else if self.position.y > bounds.top() {
            self.position.y = bounds.bottom();
            self.prev_position.y = self.position.y;
        }
    }

    pub fn draw(&self, draw: &Draw) {
        draw.line()
            .points(self.prev_position, self.position)
            .weight(1.0)
            .color(hsva(self.hue, 0.6, 0.3, 0.4));
    }
}
