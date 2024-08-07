use bevy::app::{App, Plugin, Startup, Update};
use bevy::color::palettes::css::*;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Gizmos, KeyCode, Query, Res};
use bevy::prelude::Time;
use std::f32::*;
use crate::snake_model::*;


pub struct SnakePlugin;

// #[derive(Component)]
// pub struct Food {
//     food_pos: Vec2,
//     direction: f32,
//     radius: f32,
//     color: Srgba,
// }
// #[derive(Component)]
// pub struct Bound {
//     bound_pos: Vec2,
//     radius: f32,
// }

impl Plugin for SnakePlugin {
    fn build (&self, app: &mut App) {
        app.add_systems(Startup, snake_start);
        app.add_systems(Update, snake_update);
        // app.add_systems(Startup, food_start);
        // app.add_systems(Update, draw_food);
        // app.add_systems(Update, bound_start);
        // app.add_systems(Update, draw_bound);
    }
}
//
// fn bound_start(mut commands: Commands) {
//     commands.spawn(Bound{
//         bound_pos: Vec2::new(0.0, 0.0),
//         radius: 500.0,
//     });
// }
// fn draw_bound(
//     mut gizmos: Gizmos,
//     bound_query: Query<&mut Bound>,
// ) {
//     for bound in &bound_query{
//         gizmos.circle_2d(bound.bound_pos, bound.radius, RED);
//     }
// }

fn snake_start (mut commands: Commands) {
    for i in snake_head_new_list() {
        commands.spawn(i);
    }}
//
// fn food_start (mut commands: Commands) {
//     let mut rnd = rand::thread_rng();
//     for _ in 0..3 {
//         let hue: f32 = rnd.gen();
//         let color: Srgba = Color::hsl(hue * 360.0, 0.95, 0.7).to_srgba();
//         commands.spawn(Food {
//             food_pos: new_food_position(),
//             direction: new_food_direction(),
//             radius: 10.0,
//             color: color,
//         });
//     }
// }
//
// fn new_food_position() -> Vec2 {
//     let x = rand::thread_rng().gen_range(-300..=300) as f32;
//     let y = rand::thread_rng().gen_range(-300..=300) as f32;
//     Vec2::new(x, y)
// }
// fn new_food_direction() -> f32 {
//     let num = rand::thread_rng().gen_range(-180..=180) as f32;
//     num
// }
//
// fn food_is_eaten_by_any_snake(food: &Food, snake_query: &mut Query<&mut SnakeHead>) -> bool {
//     for snake in snake_query {
//         if snake_eats_food(&snake, food) {
//             return true;
//         }
//     }
//     return false;
// }
//
// fn draw_food(
//     mut gizmos: Gizmos,
//     bound_query: Query<&mut Bound>,
//     mut food_query: Query<&mut Food>,
//     mut snake_query: Query<&mut SnakeHead>,
// ) {
//     for mut food in &mut food_query {
//         if food_is_eaten_by_any_snake(&food, &mut snake_query) {
//             food.food_pos = new_food_position();
//         }
//         let food_move = {
//             let x = f32::sin(food.direction);
//             let y = f32::cos(food.direction);
//             Vec2::new(x, y)
//         };
//
//         food.food_pos += food_move;
//         gizmos.circle_2d(food.food_pos, food.radius, food.color);
//
//         for bound in &bound_query {
//             let origin = Vec2::new(0.0, 0.0);
//             let distance_from_origin_to_food: f32 = {
//                 let distance_vector = origin - food.food_pos;
//                 ((distance_vector.x * distance_vector.x) + (distance_vector.y * distance_vector.y)).sqrt()
//             };
//             if distance_from_origin_to_food > (bound.radius - food.radius) {
//                 food.direction = new_food_direction()
//             }
//         }
//     }
// }

fn draw_node(gizmos: &mut Gizmos, position: Vec2, radius: f32) {
    gizmos.circle_2d(position, radius, YELLOW);
    gizmos.circle_2d(position, radius / 2.0, YELLOW);
}

fn keyboard_movement_up_down_impure(keyboard_input: &Res<ButtonInput<KeyCode>>) -> SnakeMoveDirection {
    if keyboard_input.pressed(KeyCode::ArrowUp) { SnakeMoveDirection::Forward }
    else if keyboard_input.pressed(KeyCode::ArrowDown) { SnakeMoveDirection::Backward }
    else { SnakeMoveDirection::Stop }
}

fn keyboard_rotation(keyboard_input: &Res<ButtonInput<KeyCode>>, snake: &SnakeHead, time: &Res<Time>) -> f32 {
    let unit: f32 = {
        if keyboard_input.pressed(KeyCode::ArrowRight) { 1.0 }
        else if keyboard_input.pressed(KeyCode::ArrowLeft) { -1.0 }
        else { 0.0 }
    };
    consts::PI / 180.0 * snake.rotation_speed_in_degrees * unit * time.delta_seconds()
}

fn draw_tail(gizmos: &mut Gizmos, radius: f32, snake: &SnakeHead){
    let mut distance = radius * 2.0;
    for i in 1..3 {
        let shift_from_head: Vec2 = {
            let x_tail = f32::sin(snake.head_direction_angle - consts::PI) * distance;
            let y_tail = f32::cos(snake.head_direction_angle - consts::PI) * distance;
            Vec2::new(x_tail, y_tail)
        };

        let tail_pos = snake.head_pos + shift_from_head;
        let tail_radius = radius - (20.0 * i as f32);
        distance += 75.0;
        draw_node(gizmos, tail_pos, tail_radius);
    }
}
// fn snake_eats_food(
//     snake: &SnakeHead,
//     food: &Food
// ) -> bool {
//     let distance_vector = snake.head_pos - food.food_pos;
//     let distance_between = ((distance_vector.x * distance_vector.x) + (distance_vector.y * distance_vector.y)).sqrt();
//     distance_between < food.radius + snake.head_radius
// }
fn snake_update (
    mut gizmos: Gizmos,
    mut snake_query: Query<&mut SnakeHead>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for mut snake in &mut snake_query {
        snake.head_direction_angle += keyboard_rotation(&keyboard_input, &snake, &time) * (snake.movement_speed / 4.0);

        let head_move: Vec2 = {
            let keyboard_up_down_input = keyboard_movement_up_down_impure(&keyboard_input);
            head_move_pure(keyboard_up_down_input, time.delta_seconds(), &snake)
        };

        snake.head_pos += head_move;

        draw_node(&mut gizmos, snake.head_pos, snake.head_radius);

        draw_tail(&mut gizmos, snake.head_radius, &snake);
    }
}