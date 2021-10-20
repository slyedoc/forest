use std::f32::consts::*;

use bevy::{
    math::*,
    prelude::*,
    render::{mesh::Indices, pipeline::PrimitiveTopology},
};
use bevy_inspector_egui::Inspectable;

use crate::prelude::{compute_aabb, convert_vec2, convert_vec3};

#[derive(Inspectable, Component, Debug, Copy, Clone)]
pub struct Circle {
    #[inspectable(min = 0.0, max = 360.0, speed = 1.0)]
    angle_in_degrees: f32,
    #[inspectable(min = 0.0, max = 10.0, speed = 0.1)]
    radius: f32,
    triangles_per_rad: usize,
    #[inspectable(min = Vec2::new(0.0, 0.0), max = Vec2::new(1.0, 1.0), speed = 0.1)]
    uv_offset: Vec2,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            angle_in_degrees: 360.0,
            triangles_per_rad: 5,
            radius: 0.5,
            uv_offset: Vec2::new(0.0, 0.0),
        }
    }
}

// http://www.code-spot.co.za/2020/11/04/generating-meshes-procedurally-in-unity/
impl From<Circle> for Mesh {
    fn from(c: Circle) -> Self {
        // figure out how many
        let triangle_count = (2.0 * PI * c.triangles_per_rad as f32).ceil() as u32;
        let sector_angle = c.angle_in_degrees.to_radians();

        let mut positions: Vec<Vec3> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut uvs: Vec<Vec2> = Vec::new();
        let normal: Vec3 = Vec3::Y; // They will all be the same

        // origin
        positions.push(Vec3::ZERO);
        normals.push(normal);
        // go around the the out outside of circle add positions
        for i in 0..triangle_count {
            let theta = i as f32 / triangle_count as f32 * sector_angle;
            let vertex = vec3(theta.cos() * c.radius, theta.sin() * c.radius, 0.0);

            positions.push(vertex);
            normals.push(normal);
        }
        let mut indices = Vec::new();
        for i in 0..triangle_count {
            let index0: u32 = 0;
            let index1: u32 = i + 1;
            let mut index2: u32 = i + 2;

            if i == triangle_count - 1 {
                //special case
                index2 = 1; //second vertex of last triangle is vertex1
            }

            indices.push(index0);
            indices.push(index1);
            indices.push(index2);
        }
        // uvs
        let bounds = compute_aabb(&positions);
        for pos in positions.iter() {
            let x = (pos[0] * 0.5 / bounds.maximums.x) + 0.5 + c.uv_offset.x;
            let y = (pos[1] * 0.5 / bounds.maximums.y) + 0.5 + c.uv_offset.y;
            uvs.push(vec2(x, y));
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, convert_vec3(&positions));
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, convert_vec3(&normals));
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, convert_vec2(&uvs));
        mesh
    }
}
