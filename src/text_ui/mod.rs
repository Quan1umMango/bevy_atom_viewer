use bevy::prelude::*;
use crate::*;


#[derive(Component)]
pub struct AtomNameText;

pub fn spawn_text(mut commands:Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Atom Name: ",
                TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                ..default()
            }),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Px(500.),
            ..default()
        }), 
        AtomNameText,
    ));
}

pub fn update_text( mut ev_change:EventReader<ChangeSelectedAtom>, selected_atom:Res<SelectedAtom>,mut atom_name_text_q: Query< &mut Text, With<AtomNameText> >) {
    for _ in ev_change.iter() {
        if let Some(mut atom_name_text) = atom_name_text_q.get_single_mut().ok() {
            let name = selected_atom.0.atom_type.to_string();
            atom_name_text.sections[1].value = format!("{name}");
        }
    }
}
