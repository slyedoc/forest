mod abop_1_9;
mod dragon_curve;
mod fractal_plant;
mod koch_curve;
mod sierpinski;
mod linden_mayer;

use bevy::prelude::*;
pub use linden_mayer::*;

pub use {
    abop_1_9::*,
    dragon_curve::*,
    koch_curve::*,
    sierpinski::*,
    fractal_plant::*,
};

pub fn generate_mesh(t: turtle::Canvas, size: f32) -> Mesh {
    // Get turtle path as lines
    let lines = t.draw_lines(size);

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for (v1, v2) in lines.iter() {
        positions.push([v1.x, v1.y, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([1.0, 1.0]);

        positions.push([v2.x, v2.y, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([1.0, 1.0]);
    }
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::LineList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}
