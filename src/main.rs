/*

TODO: Multi-threaded 2D cellular automata particle simulator.
Particle simulator where you can draw various particles.
stone isn't affected by fire, etc.
wood burns, etc.
fire runs out if there's no fuel.
water has gravity and extinquishes fire.
sand which has gravity.
etc

*/

/* Design

Going to need to make a collision system for particles. As there would be such a massive scale of particles
i'll have consider all options carefully to get good performance.

The simulation will be broken up into many chunks.
A chunk is simply a grid of particles of various size that is considered a "processing unit"
Maybe I don't even need chunks? But for cache-efficiency chunks might be a performance improvement.


Maybe they could just be stored in rows of vectors. Since I don't really need to search(use a bmaptree)


With chunks inside chunks.
A chunk is only simulated when active.
If a chunk has an interaction that effects another chunk.
That chunk is activated and set to update in that tick.


Each particle will be simulated independantly from another particle by having a two-phase update loop.
The first phase each "awake" particle records what "interactions" it would do to every single neighbor around it.
The second phase all of the interactions are then also updated independantly for each particle.

Interactions would need to be ordered by priority to decide which interaction(or maybe multiple interactions?) win to decide the final state change of the particle.

A sleeping(not "Awake") particle:
No emitted interactions.
No interactions effecting it.
Is "Deadlocked", meaning it hasn't changed state since the last tick.


Interactions(a particle can have multiple interactions);
Fire -> Sets thing on fire duh
Gravity -> A perpetual force that makes particles go down.
Water -> Takes out fire
Repulsion -> Makes the particle get a acceleration interaction in whatever direction.
Velocity -> Velocity particle is moving in whatever direction.

If a particles emits a Velocity interaction:
That particle is

Also particles should define counter interactions. What a particle does when a given particle interacts with it.

Counter-Interactions for particles(a particle can have at most one counter-interaction per interaction.);
Counter-interactions

Bedrock -> Immune to everything for example.
Fire -> Gets extinguished when affected by water, Gravity makes it fall.
Wood -> Gets set aflame by Fire, which makes it send Fire interactions to other particles.
Sand -> Affected by gravity.
Water -> Affected by gravity.


A potential optimization would be to merge(or cache?) inner particles of the same type.
Like for example, if there are 5000x5000 water particles that are somehow in a neat cube, I shouldn't need to update all of them.
As their interactions should be determinable.

*/

use std::sync::Arc;

use bevy::{
    asset::FileAssetIo,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_egui::{egui, EguiContext, EguiPlugin};

#[derive(Component, Default, Clone)]
struct ParticleMetadetaComponent {
    color: Color,
}

#[derive(Component, Default, Clone)]
pub struct StoneParticleComponent {}

#[derive(Bundle, Clone, Default)]
pub struct ParticleBundle {
    pub meshBundle: MaterialMesh2dBundle,
    pub metadata: ParticleMetadetaComponent,
}


lazy_static! {
    static ref MESH_CACHE: Arc<Dashmap>> = Arc::
}

fn main() {
    App::new()
        .insert_resource(AssetServer::new(FileAssetIo::new("assets", true)))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_system(ui)
        .add_startup_system(setup)
        .run();
}

fn ui(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("flecks").show(egui_context.ctx_mut(), |w| {
        w.label("helllloooooooooooo?");
    });
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.)),
        ..default()
    });

    // // Rectangle
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgb(0.25, 0.25, 0.75),
    //         custom_size: Some(Vec2::new(50.0, 100.0)),
    //         ..default()
    //     },
    //     ..default()
    // });

    // Circle

    // particle pixel.

    //particle

    (0..10000).for_each(|f| {
        commands.spawn(ParticleBundle {

         })
    })

    commands.spawn(ParticleBundle { ..default() });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(1., 4).into()).into(),
        material: materials.add(ColorMaterial::from(Color::BLUE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    });
}
