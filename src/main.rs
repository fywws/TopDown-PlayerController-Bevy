use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)] // Add this line to derive the Component trait for Player
struct Player;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, player_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(64.)),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        ..Default::default()
    })
    .insert(Player);
}

fn player_movement(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        let mut translation = Vec3::default();

        if input.pressed(KeyCode::W) {
            translation.y += 1.0;
        }
        if input.pressed(KeyCode::S) {
            translation.y -= 1.0;
        }
        if input.pressed(KeyCode::A) {
            translation.x -= 1.0;
        }
        if input.pressed(KeyCode::D) {
            translation.x += 1.0;
        }

        transform.translation += translation * 5.0; // Adjust the speed as needed
    }
}
