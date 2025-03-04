use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut balls = vec![
        Ball::new(100.0, 100.0, 75.0, RED, 0.7, 0.6, 2.8),
        Ball::new(100.0, 100.0, 45.0, YELLOW, 0.8, 0.8, 1.8),
        Ball::new(200.0, 100.0, 20.0, BLUE, 0.98, 0.98, 0.8),
    ];
    let mut mouse_tregectory: Vec<(f32, f32)> = Vec::new();
    let mut fps = 0;
    let mut update_fps_counter = 0;
    let mut can_update = true;

    loop {
        clear_background(BLACK);
        draw_text(
            &format!("FPS: {}", fps.to_string()),
            10.0,
            20.0,
            32.0,
            WHITE,
        );

        for ball in balls.iter_mut() {
            let grab = ball.grab();
            if grab == 1 {
                mouse_tregectory.push(mouse_position());
                if mouse_tregectory.len() > 20 {
                    mouse_tregectory.remove(0);
                }
            } else if grab == -1 {
                let x_push = mouse_tregectory[0].0 - mouse_tregectory[mouse_tregectory.len() - 1].0;
                let y_push = mouse_tregectory[0].1 - mouse_tregectory[mouse_tregectory.len() - 1].1;

                let x_push = x_push * -1.0;
                let y_push = y_push * -1.0;

                let x_push = x_push / 20.0;
                let y_push = y_push / 20.0;

                let force = (x_push, y_push);

                ball.apply_force(force);
            }

            ball.update();
            ball.draw();
        }

        if can_update {
            fps = get_fps();
            update_fps_counter = 0;
            can_update = false;
        } else {
            update_fps_counter += 1;
            if update_fps_counter > 100 {
                can_update = true;
            }
        }
        next_frame().await
    }
}

struct Ball {
    x: f32,
    y: f32,
    r: f32,
    c: Color,
    y_vel: f32,
    x_vel: f32,
    grabing: bool,
    retention: f32,
    bounce_retention: f32,
    stop_boucing: f32,
    mass: f32,
    force: (f32, f32),
}

impl Ball {
    fn new(
        x: f32,
        y: f32,
        r: f32,
        c: Color,
        retention: f32,
        bounce_retention: f32,
        mass: f32,
    ) -> Self {
        Self {
            x,
            y,
            r,
            c,
            y_vel: 0.0,
            x_vel: 0.0,
            grabing: false,
            retention,
            bounce_retention,
            stop_boucing: 0.05,
            mass,
            force: (0.0, 0.0),
        }
    }

    fn update(&mut self) {
        if self.y_vel.abs() < self.stop_boucing {
            self.y_vel = 0.0;
            self.x_vel = self.x_vel * self.retention;
        }

        let x_acc = self.force.0 / self.mass;
        let y_acc = self.force.1 / self.mass;

        self.y_vel += y_acc;
        self.x_vel += x_acc;

        let gravity = 0.1;
        self.y_vel += gravity;

        self.x += self.x_vel;
        self.y += self.y_vel;

        if self.y + self.r > screen_height() {
            self.y = screen_height() - self.r;
            self.y_vel = self.y_vel * -1.0 * self.bounce_retention;
        }

        if self.x + self.r > screen_width() {
            self.x = screen_width() - self.r;
            self.x_vel = self.x_vel * -1.0 * self.bounce_retention;
        } else if self.x - self.r < 0.0 {
            self.x = self.r;
            self.x_vel = self.x_vel * -1.0 * self.bounce_retention;
        }

        if self.grabing {
            self.x = mouse_position().0;
            self.y = mouse_position().1;
        }

        self.force = (0.0, 0.0);
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.r + 2.0, WHITE);
        draw_circle(self.x, self.y, self.r, self.c);
    }

    fn grab(&mut self) -> i32 {
        let mouse_pos = mouse_position();
        if is_mouse_button_pressed(MouseButton::Left)
            && mouse_pos.0 > self.x - self.r
            && mouse_pos.0 < self.x + self.r
            && mouse_pos.1 > self.y - self.r
            && mouse_pos.1 < self.y + self.r
        {
            self.grabing = true;
        } else if is_mouse_button_released(MouseButton::Left) && self.grabing {
            self.grabing = false;
            self.y_vel = 0.0;
            return -1;
        }

        if self.grabing {
            return 1;
        } else {
            return 0;
        }
    }

    fn apply_force(&mut self, force: (f32, f32)) {
        self.force = force;
    }
}
