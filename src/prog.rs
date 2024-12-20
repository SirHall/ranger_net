pub mod component;
pub mod resource;
pub mod system;

// use bevy_egui::EguiPlugin;
use avian2d::{
    math::Vector,
    position::{PreSolveAccumulatedTranslation, PreSolveRotation, PreviousRotation},
    prelude::*,
    sync::SyncConfig,
};
use bevy::prelude::*;
use bevy_asset_loader::loading_state::{config::ConfigureLoadingState, LoadingState, LoadingStateAppExt};
use bevy_ggrs::prelude::*;
use broad_phase::AabbIntersections;
use component::bullet::Bullet;
use dynamics::solver::{ContactSoftnessCoefficients, SolverConfig};
use resource::{assets::ProgAssets, game_state::GameState};
use system::{
    battle::{
        local::{camera_follow, init_grid_map},
        player::spawn_players,
        sim::{fire_bullets::fire_bullets, move_players::move_players},
    },
    init,
    inputs::read_local_inputs,
    matchbox::{start_matchbox_socket, wait_for_players, Config},
};

#[derive(Debug)]
pub struct Prog;

impl Plugin for Prog {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GgrsPlugin::<Config>::default(),
            //  PhysicsPlugins::new(GgrsSchedule)
        ))
        .init_state::<GameState>()
        // .insert_resource(Gravity(Vector::ZERO))
        .insert_resource(Gravity(Vector::NEG_Y * 9.81 * 0.1))
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .load_collection::<ProgAssets>()
                .continue_to_state(GameState::Connecting),
        )
        .add_systems(OnEnter(GameState::Connecting), (init, start_matchbox_socket))
        .add_systems(OnEnter(GameState::Game), (spawn_players, init_grid_map))
        .add_systems(
            Update,
            (
                (wait_for_players,).run_if(in_state(GameState::Connecting)),
                (camera_follow,).run_if(in_state(GameState::Game)),
            ),
        )
        .add_systems(ReadInputs, (read_local_inputs,))
        .add_systems(GgrsSchedule, (move_players, fire_bullets.after(move_players)))
        .rollback_component_with_clone::<Transform>()
        .rollback_component_with_copy::<Bullet>()
        .rollback_component_with_clone::<Sprite>()
        .rollback_component_with_clone::<GlobalTransform>()
        .rollback_component_with_clone::<Handle<Image>>()
        .rollback_component_with_clone::<Visibility>()
        .rollback_component_with_clone::<InheritedVisibility>()
        .rollback_component_with_clone::<ViewVisibility>()
        .rollback_component_with_copy::<Position>()
        .rollback_component_with_copy::<Rotation>()
        .rollback_component_with_copy::<PreSolveAccumulatedTranslation>()
        .rollback_component_with_copy::<PreSolveRotation>()
        .rollback_component_with_copy::<PreviousRotation>()
        .rollback_resource_with_clone::<PrepareConfig>()
        .rollback_resource_with_reflect::<BroadCollisionPairs>()
        .rollback_component_with_clone::<AabbIntersections>()
        // .rollback_component_with_clone::<AabbIntervals>()
        .rollback_resource_with_clone::<Collisions>()
        .rollback_resource_with_clone::<NarrowPhaseConfig>()
        .rollback_resource_with_reflect::<Gravity>()
        .rollback_resource_with_copy::<SleepingThreshold>()
        .rollback_resource_with_copy::<DeactivationTime>()
        // .rollback_resource_with_reflect::<LastPhysicsTick>()
        .rollback_resource_with_clone::<PhysicsLengthUnit>()
        .rollback_resource_with_clone::<SolverConfig>()
        .rollback_resource_with_copy::<ContactSoftnessCoefficients>()
        .rollback_resource_with_copy::<SubstepCount>()
        .rollback_resource_with_clone::<SpatialQueryPipeline>()
        .rollback_resource_with_clone::<SyncConfig>();
        // ContactConstraints
    }
}
