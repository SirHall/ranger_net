use super::matchbox::{Config, InputWord};
use crate::prog::component::camera::MainCamera;
use bevy::{prelude::*, utils::hashbrown::HashMap, window::PrimaryWindow};
use bevy_ggrs::{LocalInputs, LocalPlayers};
use bytemuck::cast;
use saturating_cast::SaturatingCast;

pub const INPUT_UP: InputWord = 1 << 0;
pub const INPUT_DOWN: InputWord = 1 << 1;
pub const INPUT_LEFT: InputWord = 1 << 2;
pub const INPUT_RIGHT: InputWord = 1 << 3;
pub const INPUT_PRIMARY: InputWord = 1 << 4;
pub const INPUT_SECONDARY: InputWord = 1 << 5;
pub const INPUT_ALT: InputWord = 1 << 6;
pub const INPUT_RELOAD: InputWord = 1 << 7;
pub const INPUT_ROTATION: InputWord = 0x00FFFF00;
// pub const INPUT_RUN
// pub const INPUT_CROUCH // Sneak

pub const CAMERA_CRUSH_FACTOR: f32 = 0.5;

pub fn read_local_inputs(
    mut commands: Commands,
    local_players: Res<LocalPlayers>,

    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();

    let mut local_inputs: HashMap<usize, u32> = HashMap::new();

    for handle in &local_players.0 {
        let mut input: InputWord = 0;

        if keys.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
            input |= INPUT_UP;
        }
        if keys.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
            input |= INPUT_DOWN;
        }
        if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
            input |= INPUT_LEFT
        }
        if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
            input |= INPUT_RIGHT;
        }
        if mouse_buttons.pressed(MouseButton::Left) {
            input |= INPUT_PRIMARY;
        }
        if mouse_buttons.pressed(MouseButton::Right) {
            input |= INPUT_SECONDARY;
        }
        if keys.pressed(KeyCode::Space) {
            input |= INPUT_ALT;
        }
        if keys.pressed(KeyCode::KeyR) {
            input |= INPUT_RELOAD;
        }

        // Mouse direction
        // Copied from: https://bevy-cheatbook.github.io/cookbook/cursor2world.html
        if let Ok(wnd) = q_windows.get_single() {
            // check if the cursor is inside the window and get its position
            if let Some(screen_pos) = wnd.cursor_position() {
                // get the size of the window
                let window_size = Vec2::new(wnd.width(), wnd.height());

                // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
                let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

                // matrix for undoing the projection and camera transform
                let ndc_to_world = camera_transform.compute_matrix() * camera.clip_from_view().inverse();

                // use it to convert ndc to world-space coordinates
                let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();

                let relative_to_camera_pos = world_pos - camera_transform.translation().truncate();
                let crushed_relative_to_camera_pos = (relative_to_camera_pos * CAMERA_CRUSH_FACTOR).as_i16vec2();

                let crushed_x = cast::<i8, u8>(crushed_relative_to_camera_pos.x.saturating_cast::<i8>()) as InputWord;
                let crushed_y = cast::<i8, u8>(crushed_relative_to_camera_pos.y.saturating_cast::<i8>()) as InputWord;

                input |= (crushed_x << 8) | (crushed_y << 16);
            }
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<Config>(local_inputs));
}
