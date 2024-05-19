#![allow(dead_code)]

use raylib::prelude::*;
use std::ops::Add;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;

struct Timer {
    start_time: f64,
    life_time: f64,
}

impl Timer {
    fn timer_end(&mut self, rl: &mut RaylibHandle) -> bool {
        return rl.get_time() - self.start_time >= self.life_time;
    }
    fn get_elapsed(&mut self, rl: &mut RaylibHandle) -> f64 {
        return rl.get_time() - self.start_time;
    }
    fn reset(&mut self, rl: &mut RaylibHandle) {
        self.start_time += self.get_elapsed(rl);
    }
}

struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn move_x(&mut self, amount: f32) {
        self.x += amount;
    }
    fn move_y(&mut self, amount: f32) {
        self.y += amount;
    }
}

fn draw_line(origin: Vector2, size: f32, points: &[Vector2], d: &mut RaylibDrawHandle) {
    let apply = |mut i: Vector2| {
        i.scale(size);
        return i.add(origin);
    };

    for i in 0..points.len() {
        d.draw_line_ex(
            apply(points[i]),
            apply(points[(i + 1) % points.len()]),
            2.5,
            Color::WHITE,
        )
    }
}

fn input(projectiles: &mut Vec<Point>, point: &mut Point, rl: &mut RaylibHandle) {
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        point.move_x(5.0)
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        point.move_x(-5.0)
    }

    if rl.is_key_released(KeyboardKey::KEY_UP) {
        let i: Point = Point {
            x: point.x,
            y: point.y - 15.0,
        };
        projectiles.push(i);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Space Invaders")
        .build();

    rl.set_target_fps(60);

    let mut point: Point = Point {
        x: (WINDOW_WIDTH / 2) as f32,
        y: 420.0,
    };

    let mut projectiles: Vec<Point> = Vec::new();
    let mut aliens: Vec<Point> = Vec::new();

    let mut timer: Timer = Timer {
        start_time: rl.get_time(),
        life_time: 4.0,
    };

    let mut move_timer: Timer = Timer {
        start_time: rl.get_time(),
        life_time: 1.0,
    };

    let mut position: u8 = 0;
    let mut last_position: f32 = 0.0;
    let mut row: f32 = 40.0;

    while !rl.window_should_close() {
        input(&mut projectiles, &mut point, &mut rl);

        if point.x <= 40.0 {
            point.x = 40.0;
        }
        if point.x >= 600.0 {
            point.x = 600.0
        }


        if timer.timer_end(&mut rl) {
            timer.reset(&mut rl);
            println!("ALIEN ADDED");

            if !aliens.is_empty() {
                let i : Point = Point { x: last_position - 60.0, y: row};
                last_position = i.x;
                if i.x <= 60.0 {
                    row += 60.0;
                    last_position = 650.0
                }
                aliens.push(i);
                
            } else {
                let i: Point = Point { x: 590.0, y: 40.0 };
                last_position = i.x;
                aliens.push(i);
            }
        }

        if  move_timer.timer_end(&mut rl) && !aliens.is_empty(){
            let mut direction: f32 = 0.0;
            if position == 0 {
                direction = -10.0;
            } else if position == 1 {
                direction = 10.0;
            }

            for i in aliens.iter_mut() {
                i.x += direction;
            }

            if position == 0 {
                position = 1;
            } else {
                position = 0;
            }

            move_timer.reset(&mut rl)
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        if !projectiles.is_empty() {
            for x in projectiles.iter_mut() {
                x.move_y(-10.0);
                if x.y <= 15.0 {
                    projectiles.remove(0);
                    break;
                }
                draw_line(
                    rvec2(x.x, x.y),
                    5.0,
                    &[rvec2(0.0, 1.0), rvec2(0.0, -1.0)],
                    &mut d,
                )
            }
        }

        if !aliens.is_empty() {
            for i in aliens.iter_mut() {
                draw_line(
                    rvec2(i.x, i.y),
                    30.0,
                    &[rvec2(-0.5, 0.5), rvec2(0.5, 0.5), rvec2(0.0, -0.5)],
                    &mut d,
                )
            }
        }


        draw_line(
            rvec2(point.x, point.y),
            20.0,
            &[
                rvec2(-1.0, -0.5),
                rvec2(-1.0, 0.5),
                rvec2(1.0, 0.5),
                rvec2(1.0, -0.5),
            ],
            &mut d,
        );
    }
}
