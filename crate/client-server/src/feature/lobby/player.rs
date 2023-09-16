use crate::extend_commands;
use crate::feature::multiplayer::{
  Player, PlayerInput,
};
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_xpbd_3d::prelude::*;
use renet::ClientId;

pub const PLAYER_MOVE_SPEED: f32 = 0.07;

pub const PLAYER_SIZE: f32 = 1.0;
pub const PLAYER_SPAWN_POINT: Vec3 = Vec3::new(0., 10., 0.);

pub struct PlayerPlugins;

impl Plugin for PlayerPlugins {
  fn build(&self, app: &mut App) {
    app.add_systems(FixedUpdate, (move_players_system, player_respawn));
  }
}

fn move_players_system(
  mut query: Query<(&mut LinearVelocity, &PlayerInput)>, /* , time: Res<Time> */
) {
  for (mut linear_velocity, input) in query.iter_mut() {
    let x = (input.right as i8 - input.left as i8) as f32;
    let y = (input.down as i8 - input.up as i8) as f32;

    // never use delta time in fixed update !!!
    linear_velocity.x += x * PLAYER_MOVE_SPEED; // * time.delta().as_secs_f32();
    linear_velocity.z += y * PLAYER_MOVE_SPEED; // * time.delta().as_secs_f32();
  }
}

fn player_respawn(
  _commands: Commands,
  mut query: Query<(&mut Position, &mut LinearVelocity, &Player)>,
) {
  for (mut position, mut linear_velocity, _player) in query.iter_mut() {
    if position.y < -5. {
      position.x = PLAYER_SPAWN_POINT.x;
      position.y = PLAYER_SPAWN_POINT.y;
      position.z = PLAYER_SPAWN_POINT.z;

      linear_velocity.z = 0.;
      linear_velocity.y = 0.;
      linear_velocity.x = 0.;
    }
  }
}

extend_commands!(
  spawn_server_side_player(client_id: ClientId),
  |world: &mut World, entity_id: Entity, client_id: ClientId| {
    world
     .entity_mut(entity_id)
     .insert((
       RigidBody::Dynamic,
       Position(PLAYER_SPAWN_POINT),
       Collider::cuboid(PLAYER_SIZE, PLAYER_SIZE, PLAYER_SIZE),
     ))
     .insert(PlayerInput::default())
     .insert(Player { id: client_id });
  }
);

extend_commands!(
  spawn_client_side_player(),
  |world: &mut World, entity_id: Entity| {
    let mesh = world.resource_mut::<Assets<Mesh>>().add(Mesh::from(shape::Cube { size: PLAYER_SIZE }));
    let material = world.resource_mut::<Assets<StandardMaterial>>().add(Color::rgb(0.8, 0.7, 0.6).into());

    world
      .entity_mut(entity_id)
      .insert(PbrBundle {
        mesh: mesh,
        material: material,            
        transform: Transform::from_translation(PLAYER_SPAWN_POINT),
        ..Default::default()
      });
  }
);
