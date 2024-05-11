#![allow(dead_code)]

use raylib::prelude::*;
use std::ops::Add;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;

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

    let mut projectiles = Vec::new();

    while !rl.window_should_close() {
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


        if point.x <= 40.0 {
            point.x = 40.0;
        }
        if point.x >= 600.0 {
            point.x = 600.0
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

