use bevy::prelude::*;
pub mod misc;
use misc::*;
use rand::{thread_rng, Rng};
use crate::*;

const K_SHELL_MAX:u8 = 2;
const L_SHELL_MAX:u8 = 8;
const M_SHELL_MAX:u8 = 18;
const N_SHELL_MAX:u8 = 32;
const O_SHELL_MAX:u8 = 50;
const P_SHELL_MAX:u8 = 72;
const Q_SHELL_MAX:u8 = 98;

const K_SHELL_RADIUS:f32 = 30.;
const L_SHELL_RADIUS:f32 = 40.0;
const M_SHELL_RADIUS:f32 = 50.0;
const N_SHELL_RADIUS:f32 = 60.0;
const O_SHELL_RADIUS:f32 = 70.0;
const P_SHELL_RADIUS:f32 = 80.0;
const Q_SHELL_RADIUS:f32 = 90.0;


const PROTON_RADIUS:f32 =2.0;
const NEUTRON_RADIUS:f32 = 2.0;
const ELECTRON_RADIUS:f32 = 1.5;

// Define a pub struct for a proton
#[derive(Component)]
pub struct Proton;

// Define a pub struct for an electron
#[derive(Component)]
pub struct Electron(pub u8,pub u8); // shell number

// Define a pub struct for a neutron
#[derive(Component)]
pub struct Neutron;

#[derive(Component)]
pub struct AtomObject(pub AtomType);

#[derive(Clone)]
pub struct Atom {
    pub atom_type: AtomType,
    pub atomic_number: u32,
    pub protons: u32,
    pub electrons: u32,
    pub neutrons: u32,
}


fn assign_electron_to_shell(electron_number: u8) -> u8 {
    if electron_number <= K_SHELL_MAX {
        1 // Electron is in the K shell
    } else if electron_number <= (L_SHELL_MAX+K_SHELL_MAX) {
        2 // Electron is in the L shell
    } else if electron_number <= (M_SHELL_MAX +L_SHELL_MAX+K_SHELL_MAX){
        3 // Electron is in the M shell
    } else if electron_number <= (N_SHELL_MAX + M_SHELL_MAX+L_SHELL_MAX+K_SHELL_MAX) {
        4 // Electron is in the N shell
    } else if electron_number <= (O_SHELL_MAX + N_SHELL_MAX+M_SHELL_MAX+L_SHELL_MAX+K_SHELL_MAX) {
        5 // Electron is in the O shell
    } else if electron_number <= (P_SHELL_MAX + O_SHELL_MAX + N_SHELL_MAX+M_SHELL_MAX+L_SHELL_MAX+K_SHELL_MAX ){
        6 // Electron is in the P shell
    } else if electron_number as u32 <= (Q_SHELL_MAX as u32 +P_SHELL_MAX as u32+ O_SHELL_MAX as u32+ N_SHELL_MAX as u32+M_SHELL_MAX as u32+L_SHELL_MAX as u32+K_SHELL_MAX as u32){
        7 // Electron is in the Q shell
    } else {
        // Handle cases where electron number exceeds Q_SHELL_MAX (if necessary)
        0 // Electron number is invalid
    }
}

pub fn spawn_atom(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    selected_atom: Res<SelectedAtom>
) {
    let atom = selected_atom.0.clone(); 

    // spawn neutrons
    for _ in 0i32..atom.neutrons as i32{
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(shape::UVSphere {
                    radius:NEUTRON_RADIUS,
                    ..default()
                }.into()),
                material:materials.add(Color::YELLOW.into()),
                transform:Transform::from_translation(random_position_within_nucleus(atom.protons.clone())),
                ..default()
            },
            Neutron,AtomObject(atom.atom_type.clone())
        ));
    }
    for _ in 0i32..atom.protons as i32{
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(shape::UVSphere {
                    radius:PROTON_RADIUS,
                    ..default()
                }.into()),
                material:materials.add(Color::BLUE.into()),
                transform:Transform::from_translation(random_position_within_nucleus(atom.protons.clone())),
                ..default()
            },
            Proton,AtomObject(atom.atom_type.clone())
        ));
    }

    for i in 1i32..atom.electrons as i32 + 1{
        let shell_no = assign_electron_to_shell(i.try_into().unwrap());
        commands.spawn((
            PbrBundle {
                mesh:meshes.add(shape::UVSphere {
                    radius: ELECTRON_RADIUS,
                    ..default()
                }.into()),
                material:materials.add(Color::RED.into()),
                transform: Transform::from_translation(Vec3::new(0.,0.,0.)),
                ..default()
            }, 
            Electron(shell_no,i.try_into().unwrap()),AtomObject(atom.atom_type.clone())
        ));
    }


}

pub fn draw_shell(gizmos:&mut Gizmos,r:f32) {
    gizmos.circle(Vec3::ZERO, Vec3::Y,r, Color::BLACK);
}

pub fn draw_shells(mut gizmos:Gizmos,atom:Res<SelectedAtom>) {
    let i = atom.0.electrons;
    if i > (P_SHELL_MAX+O_SHELL_MAX).into() {
        draw_shell(&mut gizmos,Q_SHELL_RADIUS);
    }
    if i > (O_SHELL_MAX+N_SHELL_MAX).into() {
        draw_shell(&mut gizmos,P_SHELL_RADIUS);
    } if i > (N_SHELL_MAX+M_SHELL_MAX).into() {
        draw_shell(&mut gizmos,O_SHELL_RADIUS);
    } if i >(M_SHELL_MAX+L_SHELL_MAX).into() {
        draw_shell(&mut gizmos,N_SHELL_RADIUS);
    } if i > (L_SHELL_MAX+K_SHELL_MAX).into() {
        draw_shell(&mut gizmos,M_SHELL_RADIUS);
    } if i > K_SHELL_MAX.into() {
        draw_shell(&mut gizmos,L_SHELL_RADIUS);
    }

    draw_shell(&mut gizmos,K_SHELL_RADIUS); 

}

pub fn update_electrons(mut electron_q:Query<(&mut Transform,&Electron)>,time: Res<Time>) {
    for (mut transform,electron) in electron_q.iter_mut() {
                let t = time.elapsed_seconds(); 
        let r = match electron.0 {
            1 => K_SHELL_RADIUS,
            2 => L_SHELL_RADIUS,
            3 => M_SHELL_RADIUS,
            4 => N_SHELL_RADIUS,
            5 => O_SHELL_RADIUS,
            6 => P_SHELL_RADIUS,
            7 => Q_SHELL_RADIUS,
            _ => Q_SHELL_RADIUS
        };
        let separation_distance = r/2.0* electron.0.pow(2) as f32;
        //let angular_offset = 2.0 * std::f32::consts::PI;
        // let electron_angle = angular_offset * electron.1 as f32;
        let x = r * (t+separation_distance*electron.1 as f32).sin();// + separation_distance * electron_angle.cos();
        let z = r * (t+separation_distance*electron.1 as f32).cos();// + separation_distance * electron_angle.sin();

        // Update the translation for each electron
        transform.translation.x = x;
        transform.translation.z = z;
    } 
}




pub fn random_position_within_nucleus(proton_num:u32) -> Vec3 {

    let mut rng = thread_rng();
    // Generate a random position within the nucleus sphere
    // You can use spherical coordinates to distribute protons and neutrons evenly within the nucleus.
    // Calculate random theta and phi angles, and then convert to Cartesian coordinates.
    let theta = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
    let phi = rng.gen_range(0.0..std::f32::consts::PI);
    let radius:f32 =  {
        if proton_num < 1 {
            0.
        }else if proton_num < 3{
            1.
        }else if proton_num < 11{
            2.
        }else if proton_num < 19 {
            2.5
        } else if proton_num <  37 {
            3.

        }else if proton_num < 46 {
            3.5
        }
        else if proton_num < 55{
            4.
        }else if proton_num < 87 {
            5.
        }
        else {
            10.
        }
    }; // Adjust the radius as needed
    Vec3::new(
        radius * phi.sin() * theta.cos(),
        radius * phi.sin() * theta.sin(),
        radius * phi.cos(),
    )
}
