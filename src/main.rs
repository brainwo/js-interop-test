use crate::interop::perform_demo;
use macroquad::prelude::*;

mod interop;

const GAME_WIDHT: f32 = 600.;
const BUTTON_RADIUS: f32 = 64.;

#[derive(PartialEq)]
enum Action {
    MoveLeft,
    MoveRight,
}

struct Control {
    pressed_action: Option<Action>,
}

impl Control {
    fn init() -> Self {
        Control {
            pressed_action: None,
        }
    }

    fn draw_button(_action: Action, pos: Vec2, is_pressed: bool) {
        draw_circle(
            pos.x + BUTTON_RADIUS,
            pos.y - BUTTON_RADIUS,
            BUTTON_RADIUS,
            match is_pressed {
                true => BLACK,
                false => GRAY,
            },
        )
    }

    fn draw(&self, left_button: Vec2, right_button: Vec2) {
        Control::draw_button(
            Action::MoveLeft,
            left_button,
            self.is_action_down(Action::MoveLeft),
        );

        Control::draw_button(
            Action::MoveRight,
            right_button,
            self.is_action_down(Action::MoveRight),
        )
    }

    fn get_button_clicked(pos: Vec2) -> bool {
        is_mouse_button_down(MouseButton::Left)
            && Circle {
                x: pos.x + BUTTON_RADIUS,
                y: pos.y - BUTTON_RADIUS,
                r: BUTTON_RADIUS,
            }
            .contains(&vec2(mouse_position().0, mouse_position().1))
    }

    fn update(&mut self, relative_x: f32) {
        let left_button = vec2(relative_x + 12., screen_height() - 24.);
        let right_button = vec2(relative_x + 300., screen_height() - 24.);
        if Control::get_button_clicked(left_button) {
            self.pressed_action = Some(Action::MoveLeft)
        } else if Control::get_button_clicked(right_button) {
            self.pressed_action = Some(Action::MoveRight)
        } else {
            self.pressed_action = None
        }
        self.draw(left_button, right_button);
    }

    fn is_action_down(&self, action: Action) -> bool {
        self.pressed_action == Some(action)
    }
}

struct Player {
    x: i32,
}

impl Player {
    fn init() -> Self {
        Player { x: 0 }
    }

    fn draw(&self) {
        draw_rectangle(((self.x as f32) / 100.) - 0.05, -0.2, 0.1, 0.1, BLACK);
    }

    fn update(&self) {
        self.draw();
    }

    fn move_player(&mut self, amount: i32) {
        self.x = (self.x + amount).clamp(-100, 100)
    }
}

#[macroquad::main("Typing")]
async fn main() {
    let mut control = Control::init();
    let mut player = Player::init();

    loop {
        let relative_x = (screen_width() - GAME_WIDHT) / 2.;

        draw_checkerboard();

        draw_rectangle(
            (screen_width() - GAME_WIDHT) / 2.,
            0.,
            GAME_WIDHT,
            screen_height(),
            LIGHTGRAY,
        );

        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });

        player.update();

        set_default_camera();

        control.update(relative_x);

        if control.is_action_down(Action::MoveLeft) {
            player.move_player(-1);
            unsafe {
                perform_demo();
            }
        }
        if control.is_action_down(Action::MoveRight) {
            player.move_player(1);
        }

        next_frame().await
    }
}

pub fn draw_checkerboard() {
    for i in 0..=(screen_width() / 20.) as u32 {
        for j in 0..=(screen_height() / 20.) as u32 {
            draw_rectangle(
                i as f32 * 20. - 10.,
                j as f32 * 20. - 10.,
                20.,
                20.,
                match (i + j) % 2 {
                    0 => Color::from_rgba(43, 46, 51, 255),
                    _ => Color::from_rgba(59, 62, 67, 255),
                },
            )
        }
    }
}
