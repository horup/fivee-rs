use bevy::prelude::*;
use common::CommonAssets;

pub struct PluginAssets;

impl Plugin for PluginAssets {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, startup);
    }
}

fn startup(
    mut ca: ResMut<CommonAssets>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Fonts
    let font = asset_server.load("fonts/helvetica.ttf");
    ca.font_insert("default", font);

    // Textures
    ca.image_insert("cell", asset_server.load("images/cell.png"));
    ca.image_insert("brick", asset_server.load("images/brick.png"));

    // Materials
    let tex = ca.image("cell");
    ca.material_insert(
        "cell",
        materials.add(StandardMaterial {
            base_color_texture: Some(tex),
            ..default()
        }),
    );
    let tex = ca.image("brick");
    ca.material_insert(
        "brick",
        materials.add(StandardMaterial {
            base_color_texture: Some(tex),
            ..default()
        }),
    );
    ca.material_insert(
        "black",
        materials.add(StandardMaterial {
            base_color: Color::BLACK,
            unlit: true,
            ..Default::default()
        }),
    );

    // Meshes
    ca.mesh_insert("tile", asset_server.load("meshes/cell.gltf#Mesh0/Primitive0"));
    ca.mesh_insert("cube", asset_server.load("meshes/cube.gltf#Mesh0/Primitive0"));
    ca.mesh_insert("token", asset_server.load("meshes/token.gltf#Mesh0/Primitive0"));
}
