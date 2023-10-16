use bevy::prelude::*;
mod atoms;
use atoms::{*,misc::AtomType};
use bevy_panorbit_camera::*;
use bevy_egui::EguiPlugin;
mod egui_ui;
use egui_ui::*;
mod text_ui;
use text_ui::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::GRAY))
        .insert_resource(SelectedAtom(Atom::helium()))
        .add_event::<ChangeSelectedAtom>()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup,(setup_camera,spawn_atom,spawn_text))
        .add_systems(Update,(update_electrons,draw_shells,despawn_old_atom))
        .add_systems(Update,update_ui)
        .add_systems(Update,update_text)
        .run();
}



pub fn setup_camera(
    mut commands: Commands,
) {

    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
        PanOrbitCamera::default(),
    ));

}


#[derive(Resource)]
pub struct SelectedAtom(pub Atom);

#[derive(Event)]
pub struct ChangeSelectedAtom(pub AtomType); 


fn despawn_old_atom(mut ev_change:EventReader<ChangeSelectedAtom>,
    mut commands:Commands,
    entities_q:Query<(Entity,&AtomObject),With<AtomObject>>,
    new_atom:Res<SelectedAtom>, 
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut new = false;
    for _ in ev_change.iter() {
        for (e,atom_object) in entities_q.iter() {
            if atom_object.0 != new_atom.0.atom_type {
                new = true;
                commands.entity(e).despawn();
            }
        }
    }
    if new {
        spawn_atom(commands,meshes,materials,new_atom); 
    }
}

