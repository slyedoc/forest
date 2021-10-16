use bevy::{math::*, prelude::*, render::pipeline::PrimitiveTopology};
use bevy_inspector_egui::Inspectable;

use crate::{
    helper::compute_aabb,
    prelude::{convert_vec2, convert_vec3},
};

// DO NOT USE
#[derive(Inspectable, Component, Debug, Copy, Clone)]
pub struct Plane {
    #[inspectable(min = Vec2::ZERO, max = Vec2::new(10.0, 10.0))]
    pub size: Vec2,
    //#[inspectable(min = Vec2::splat(1.0), max = Vec2::new(10.0, 10.0))]
    #[inspectable(ignore)]
    pub divisions: UVec2,
    pub debug: bool,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            divisions: uvec2(1, 1),
            size: vec2(1.0, 1.0),
            debug: false,
        }
    }
}

impl From<Plane> for Mesh {
    fn from(shape: Plane) -> Self {
        // figure out how many
        let triangle_count = (shape.divisions[0] * shape.divisions[1]) as usize * 2;
        let mut vertices: Vec<Vec3> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut uvs: Vec<Vec2> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let step_x = shape.size.x / shape.divisions.x as f32;
        let step_y = shape.size.y / shape.divisions.y as f32;
        for row in 0..shape.divisions.x {
            for column in 0..shape.divisions.y {
                vertices.push(vec3(row as f32, column as f32, 0.0));
            }
        }

        // 0 2 4 5
        // 1 3 5 6
        // build grid of positions
        for x_div in 0..shape.divisions[0] {
            let x_pos = step_x * x_div as f32;
            for y_div in 0..shape.divisions[1] {
                let y_pos = step_y * y_div as f32;
                let pos = [x_pos, y_pos];
                vertices.push(vec3(pos[0], pos[1] + step_y, 0.0));
                vertices.push(vec3(pos[0], pos[1], 0.0));
                vertices.push(vec3(pos[0] + step_x, pos[1], 0.0));
                vertices.push(vec3(pos[0] + step_x, pos[1] + step_y, 0.0));
            }
        }
        // Normals
        for _ in vertices.iter() {
            normals.push(Vec3::Z)
        }

        // build indies
        for i in (0..triangle_count as u32).step_by(2) {
            // 1 3 4 5
            // 0 2 5 6
            indices.push(i);
            indices.push(i + 1);
            indices.push(i + 2);

            indices.push(i + 3);
            //indices.push(i);
        }
        // uvs
        let bounds = compute_aabb(&vertices);
        for pos in vertices.iter() {
            let x = (pos[0] * 0.5 / bounds.maximums.x) + 0.5;
            let y = (pos[1] * 0.5 / bounds.maximums.y) + 0.5;
            uvs.push(vec2(x, y));
        }

        info!("positions: {}", vertices.len());
        info!("normals: {}", normals.len());
        info!("indices: {}", indices.len());

        if shape.debug {
            return debug_mesh(&vertices, triangle_count);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleStrip);
        //mesh.set_indices(Some(Indices::U32(indices)));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, convert_vec3(&vertices));
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, convert_vec3(&normals));
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, convert_vec2(&uvs));
        mesh
    }
}

fn debug_mesh(positions: &[Vec3], triangle_count: usize) -> Mesh {
    let mut new_positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    for i in (0..triangle_count).step_by(2) {
        // 0 3
        // 1 2
        new_positions.push(positions[i]);
        new_positions.push(positions[i + 1]);
        new_positions.push(positions[i + 2]);

        new_positions.push(positions[i]);
        new_positions.push(positions[i + 3]);
        new_positions.push(positions[i + 2]);
    }
    // set a normal and uv for each pos
    for _ in positions.iter() {
        normals.push(Vec3::Z);
        uvs.push([0.5, 0.5]);
    }

    info!("{:?}", positions);
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::LineStrip);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, convert_vec3(positions));
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, convert_vec3(&normals));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}
