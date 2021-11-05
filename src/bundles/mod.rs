pub mod city;
pub mod procedural;
pub mod space;
pub mod tree;
pub mod spider;

use bevy::{prelude::*, render::mesh::VertexAttributeValues, utils::HashMap};
use bevy_mod_bounding::aabb::Aabb;
use heron::{CollisionShape, RigidBody};
pub use procedural::*;

pub use city::*;
pub use space::*;
use std::fmt::Debug;
pub use tree::*;
pub use spider::*;


pub struct BundlePlugin;
impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TreePlugin)
            .add_plugin(ProceduralPlugin)
            .add_plugin(CityPlugin)
            .add_plugin(SpacePlugin)
            .add_plugin(SpiderPlugin);
            //.add_system(update_bounds_system);
    }
}

trait GltfAssetType {
    fn get_path(&self) -> &str;
}

// Note:  Tried using bevy scene spawner instead of with_children,
// global transforms didn't update correctly still egui edit touch
// a transform then they would appear
//      mut spawner: ResMut<SceneSpawner>,
//      spawner.spawn_as_child(node, e);
fn init_asset_type_system<T: Component + GltfAssetType + Debug>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut T, Option<&Children>, Option<&Name>), Changed<T>>,
) {
    for (e, asset_type, children_option, name_option) in query.iter_mut() {
        // remove children if they exists
        if let Some(children) = children_option {
            for c in children.iter() {
                commands.entity(*c).despawn_recursive();
            }
        }

        // Get the asset path and load it as a child, mark it to get a
        // bounding volume
        let node_path = asset_type.get_path();
        let node: Handle<Scene> = asset_server.load(node_path);
        commands
            .entity(e)
            .with_children(|parent| {
                parent.spawn_scene(node);
            })
            .insert(GenerateBounding);

        // Give assets a default name if they don't already have one
        if name_option.is_none() {
            commands
                .entity(e)
                .insert(Name::new(format!("{:?}", *asset_type)));
        }
    }
}

// Don't delete yet - Used to find aabb of meshes imported works, trying another way right now

#[derive(Component)]
 pub struct GenerateBounding;

 pub fn update_bounds_system(
     mut commands: Commands,
     mut meshes: ResMut<Assets<Mesh>>,
     mut query: QuerySet<(
         QueryState<(Entity, &Children), (With<GenerateBounding>, With<Children>)>,
         QueryState<(
             &GlobalTransform,
             Option<&Children>,
             Option<&Handle<Mesh>>,
         )>,
     )>,
 ) {
     // will become hashmap of entities with GenerateBounding and there Aabb
     let mut volume_results: HashMap<Entity, Aabb> = HashMap::default();

//     // since we can't ask for entire children heirarch, we will have to search for them
//     // we will store the (top level parent the child belongs to,  and itself)
     let mut children_to_process = Vec::<(Entity, Entity)>::default();

//     // build first layer of the entity hierarch
     for (e, children) in query.q0().iter_mut() {
         for c in children.iter() {
             commands.entity(e).remove::<GenerateBounding>();
             children_to_process.push((e, *c));
         }
     }

//     // loop though all children, we will be adding new children to this list as we find them
     while let Some( (parent_entity, child_entity) )= children_to_process.pop() {
         // get the global transform and children of current child
         let (transform, children_option, mesh_option) =
             query.q1().get(child_entity).unwrap();

         // add children to be process
         if let Some(children) = children_option {
             for c in children.iter() {
                 children_to_process.push((parent_entity, *c));
             }
         }
        
         // if we have a mesh, figure out its bounds and add it the results results
         if let Some(mesh_handle) = mesh_option {

//             // NOTE: This logic is from bevy_mod_bounding
             let transform_matrix = Transform {
                 translation: Vec3::ZERO,
                 rotation: transform.rotation,
                 scale: transform.scale,
             }
             .compute_matrix();

             if let Some(mesh) = meshes.get(mesh_handle) {
                 let vertices: Vec<Vec3> = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
                     None => panic!("Mesh does not contain vertex positions"),
                     Some(vertex_values) => match &vertex_values {
                         VertexAttributeValues::Float32x3(positions) => positions
                             .iter()
                             .map(|coordinates| {
                                 transform_matrix.transform_point3(Vec3::from(*coordinates))
                             })
                             .collect(),
                         _ => panic!("Unexpected vertex types in ATTRIBUTE_POSITION"),
                     },
                 };
                 let bound = Aabb::compute_aabb(&vertices);
                 if let Some(exists) = volume_results.get_mut(&parent_entity) {
                     // TODO: make and add fn for this
                     let min = bound.minimums().min(exists.minimums());
                     let max = bound.maximums().max(exists.maximums());
                     exists.minimums = min;
                     exists.maximums = max;
                 } else {
                     volume_results.insert(parent_entity, bound.clone());
                 }
             }
         }
     }

    // Add all our found volumes
    for (root, value) in volume_results.iter() {
        //let bound = BoundingVolume::<Aabb::Aabb>::new(mesh, transform);
        commands
            .entity(*root)
            .insert(RigidBody::Static)
            .insert(CollisionShape::Cuboid {
                half_extends: value.maximums,
                border_radius: None,
            });

    }
}
