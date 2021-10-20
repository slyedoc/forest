use bevy::math::*;
use bevy::render::mesh::Indices;
use bevy::{math::vec3, prelude::*, render::pipeline::PrimitiveTopology, utils::HashMap};
use bevy_inspector_egui::Inspectable;
use std::f32::consts::*;

#[derive(Inspectable, Component, Debug, Copy, Clone)]
pub struct Tree {
    debug_mesh: bool,

    // Tree
    seed: usize,
    #[inspectable(min = 1, max = 7)]
    levels: usize,
    #[inspectable(min = 0.0, max = 1.0)]
    twig_scale: f32,

    // Branching
    #[inspectable(min = 0.1, max = 1.0)]
    inital_branch_length: f32,
    #[inspectable(min = 0.5, max = 1.0)]
    length_falloff_factor: f32,
    #[inspectable(min = 0.1, max = 1.5)]
    length_falloff_power: f32,
    #[inspectable(min = 0.0, max = 1.0)]
    clump_max: f32,
    #[inspectable(min = 0.0, max = 1.0)]
    clump_min: f32,
    #[inspectable(min = 2.0, max = 4.0)]
    branch_factor: f32,
    #[inspectable(min = -1.0, max = 1.0)]
    drop_amount: f32,
    #[inspectable(min = -0.5, max = 1.0)]
    grow_amount: f32,
    #[inspectable(min = -1.0, max = 1.0)]
    sweep_amount: f32,

    // Trunk
    #[inspectable(min = 0.05, max = 1.0)]
    max_radius: f32,
    #[inspectable(min = 0.05, max = 1.0)]
    climb_rate: f32,
    #[inspectable(min = 0.0, max = 0.5)]
    trunk_kink: f32,
    #[inspectable(min = 0, max = 35)]
    tree_steps: usize,
    #[inspectable(min = 0.7, max = 1.0)]
    taper_rate: f32,
    #[inspectable(min = 0.5, max = 0.8)]
    radius_falloff_rate: f32,
    #[inspectable(min = 0, max = 10)]
    twist_rate: usize,
    #[inspectable(min = 0.1, max = 10.0)]
    trunk_length: f32,

    segments: usize,
    v_multiplier: f32,
    rseed: usize,
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            debug_mesh: false,
            clump_max: 0.8,
            clump_min: 0.5,
            length_falloff_factor: 0.85,
            length_falloff_power: 1.0,
            branch_factor: 2.0,
            radius_falloff_rate: 0.6,
            climb_rate: 1.5,
            trunk_kink: 0.00,
            max_radius: 0.25,
            tree_steps: 2,
            taper_rate: 0.95,
            twist_rate: 13,
            segments: 6,
            levels: 3,
            sweep_amount: 0.0,
            inital_branch_length: 0.85,
            trunk_length: 2.5,
            drop_amount: 0.0,
            grow_amount: 0.0,
            v_multiplier: 0.2,
            twig_scale: 2.0,
            seed: 10,
            rseed: 10,
        }
    }
}

impl Tree {
    #[allow(dead_code)]
    fn random(&mut self, a: usize) -> f32 {
        let mut b = a as f32;
        if b != 0.0 {
            b = self.rseed as f32;
            self.rseed += 1;
        }
        (b + b * b).cos().abs()
    }
}

impl From<Tree> for Mesh {
    fn from(data: Tree) -> Self {
        let mut tree = TreeMesh::new(data);

        if data.debug_mesh {
            return tree.debug_mesh();
        }
        tree.mesh()
    }
}

// Currently using usize table

#[derive(Default)]
pub struct TreeMesh {
    pub branches: BranchTable,
    pub properties: Tree,
    pub root: usize,

    verts: Vec<Vec3>,
    faces: Vec<[usize; 3]>,
    normals: Vec<Vec3>,
    uvs: Vec<[f32; 2]>,
    verts_twig: Vec<Vec3>,
    normals_twig: Vec<Vec3>,
    faces_twig: Vec<[usize; 3]>,
    uvs_twig: Vec<[usize; 2]>,
}

impl TreeMesh {
    pub fn new(data: Tree) -> TreeMesh {
        // this has table is how we i
        let mut branches = BranchTable::default();
        let root = branches.add(Branch {
            head: vec3(0.0, data.trunk_length, 0.0),
            tail: Vec3::ZERO,
            parent: None,
            length: data.inital_branch_length,
            trunk: true,
            ..Default::default()
        });
        let mut tree = TreeMesh {
            branches,
            properties: data,
            root,
            ..Default::default()
        };

        // Grow
        // let grow = vec![root];
        // let mut iter_count = 0;
        // while !grow.is_empty() {
        //     for branch in grow.iter() {
        //         // what to do

        //     }
        //     iter_count += 1;
        // }
        tree.split_branch(
            tree.root,
            data.levels,
            data.tree_steps,
            &mut data.clone(),
            1,
            1,
        );
        tree.create_forks(tree.root, data.max_radius);
        tree.create_twigs(tree.root);
        tree.do_faces(tree.root);
        tree.calc_normals();

        tree
    }

    pub fn mesh(&mut self) -> Mesh {
        let positions: Vec<[f32; 3]> = self.verts.iter().map(|v| [v.x, v.y, v.z]).collect();
        let normals: Vec<[f32; 3]> = self.normals.iter().map(|v| [v.x, v.y, v.z]).collect();
        let uvs: Vec<[f32; 2]> = self.uvs.iter().map(|v| [v[0], v[1]]).collect();
        let mut indices: Vec<u32> = Vec::new();
        for face in self.faces.iter() {
            indices.push(face[0] as u32);
            indices.push(face[1] as u32);
            indices.push(face[2] as u32);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));

        mesh
    }

    pub fn debug_mesh(&mut self) -> Mesh {
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        for (_key, b) in self.branches.table.iter_mut() {
            positions.push(b.tail.into());
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([1.0, 1.0]);

            positions.push(b.head.into());
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([1.0, 1.0]);
        }
        let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::LineList);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh
    }

    pub fn calc_normals(&mut self) {
        let mut all_normals = Vec::<Vec<Vec3>>::new();
        for _ in self.verts.iter() {
            all_normals.push(Vec::new());
        }

        for i in 0..self.faces.len() {
            let face = self.faces[i];
            let norm = (self.verts[face[1]] - self.verts[face[2]])
                .cross(self.verts[face[1]] - self.verts[face[0]])
                .normalize();
            all_normals[face[0]].push(norm);
            all_normals[face[1]].push(norm);
            all_normals[face[2]].push(norm);
        }
        for (i, ns) in all_normals.iter().enumerate() {
            let mut total = Vec3::ZERO;
            for (j, v) in ns.iter().enumerate() {
                total += all_normals[i][j] * (1.0 / v.y);
            }
            self.normals.push(total);
        }
    }

    pub fn get_branch(&mut self, id: usize) -> &mut Branch {
        self.branches.get(id)
    }

    pub fn do_faces(&mut self, branch_id: usize) {
        let segments = self.properties.segments;
        let child0 = self.get_branch(branch_id).child0.unwrap();
        let child1 = self.get_branch(branch_id).child1.unwrap();

        if self.get_branch(branch_id).parent.is_none() {
            for _i in 0..self.verts.len() {
                // should be push?
                self.uvs.push([0.0, 0.0]);
            }

            let tangent = ((self.get_branch(child0).head - self.get_branch(branch_id).head)
                .cross(self.get_branch(child1).head - self.get_branch(branch_id).head))
            .normalize();
            let normal = self.get_branch(branch_id).head.normalize();
            let mut angle = tangent.dot(vec3(-1.0, 0.0, 0.0)).acos();
            if vec3(-1.0, 0.0, 0.0).cross(tangent).dot(normal) > 0.0 {
                angle = 2.0 * PI - angle;
            }
            let seg_offset = (angle / PI / 2.0 * segments as f32).round() as usize;
            for i in 0..segments {
                let v1 = self.get_branch(branch_id).ring0[i];
                let v2 = self.get_branch(branch_id).root[(i + seg_offset + 1) % segments];
                let v3 = self.get_branch(branch_id).root[(i + seg_offset) % segments];
                let v4 = self.get_branch(branch_id).ring0[(i + 1) % segments];

                self.faces.push([v1, v4, v3]);
                self.faces.push([v4, v2, v3]);
                self.uvs[(i + seg_offset) % segments] =
                    [(i as f32 / segments as f32 - 0.5).abs() * 2.0, 0.0];

                let ring2 = self.get_branch(branch_id).ring2[i];
                let seg = self.get_branch(branch_id).root[(i + seg_offset) % segments];
                let len =
                    (self.verts[v1] - self.verts[seg]).length() * self.properties.v_multiplier;
                self.uvs[v1] = [(i as f32 / segments as f32 - 0.5).abs() * 2.0, len];
                self.uvs[ring2] = [(i as f32 / segments as f32 - 0.5).abs() * 2.0, len];
            }
        }

        if !self.get_branch(child0).ring0.is_empty() {
            let mut seg_offset0: usize = 0;
            let mut seg_offset1: usize = 0;
            let mut match0: f32 = 0.0;
            let mut match1: f32 = 0.0;

            let ring0 = self.get_branch(branch_id).ring1[0];
            let ring2 = self.get_branch(branch_id).ring2[0];
            let head = self.get_branch(branch_id).head;
            let mut v1 = (self.verts[ring0] - head).normalize();
            let mut v2 = (self.verts[ring2] - head).normalize();

            v1 = scale_in_direction(v1, (self.get_branch(child0).head - head).normalize(), 0.0);
            v2 = scale_in_direction(v2, (self.get_branch(child1).head - head).normalize(), 0.0);

            for i in 0..segments {
                let child_ring0 = self.get_branch(child0).ring0[i];
                let mut d = (self.verts[child_ring0] - self.get_branch(child0).head).normalize();
                let mut l = d.dot(v1);
                if seg_offset0 == 0 || l > match0 {
                    match0 = l;
                    seg_offset0 = segments - i;
                }
                d = (self.verts[child_ring0] - self.get_branch(child1).head).normalize();
                l = d.dot(v2);
                if seg_offset1 == 0 || l > match1 {
                    match1 = l;
                    seg_offset1 = segments - i;
                }
            }

            let uv_scale = self.properties.max_radius / self.get_branch(branch_id).radius;

            for i in 0..segments {
                let v1_c0 = self.get_branch(child0).ring0[i];
                let v2_c0 = self.get_branch(branch_id).ring1[(i + seg_offset0 + 1) % segments];
                let v3_c0 = self.get_branch(branch_id).ring1[(i + seg_offset0) % segments];
                let v4_c0 = self.get_branch(child0).ring0[(i + 1) % segments];

                self.faces.push([v1_c0, v4_c0, v3_c0]);
                self.faces.push([v4_c0, v2_c0, v3_c0]);
                let v1_c1 = self.get_branch(child1).ring0[i];
                let v2_c1 = self.get_branch(branch_id).ring2[(i + seg_offset1 + 1) % segments];
                let v3_c1 = self.get_branch(branch_id).ring2[(i + seg_offset1) % segments];
                let v4_c1 = self.get_branch(child1).ring0[(i + 1) % segments];

                self.faces.push([v1_c1, v2_c1, v3_c1]);
                self.faces.push([v1_c1, v4_c1, v2_c1]);

                let b_ring1 = self.get_branch(branch_id).ring1[(i + seg_offset0) % segments];
                let b_ring1_base =
                    self.get_branch(branch_id).ring1[(i + seg_offset0 - 1) % segments];
                let len1 = (self.verts[v1_c0] - self.verts[b_ring1]).length() * uv_scale;
                let uv1 = self.uvs[b_ring1_base];

                self.uvs[v1_c0] = [uv1[0], uv1[1] + len1 * self.properties.v_multiplier];
                let c0_end = self.get_branch(child0).ring2[i];
                let c1_end = self.get_branch(child1).ring2[i];
                self.uvs[c0_end] = [uv1[0], uv1[1] + len1 * self.properties.v_multiplier];

                let b_ring2 = self.get_branch(branch_id).ring2[(i + seg_offset1) % segments];
                let b_ring2_off =
                    self.get_branch(branch_id).ring2[(i + seg_offset1 - 1) % segments];
                let len2 = (self.verts[v1_c0] - self.verts[b_ring2]).length() * uv_scale;
                let uv2 = self.uvs[b_ring2_off];

                self.uvs[v1_c0] = [uv2[0], uv2[1] + len2 * self.properties.v_multiplier];
                self.uvs[c1_end] = [uv2[0], uv2[1] + len2 * self.properties.v_multiplier];
            }

            self.do_faces(child0);
            self.do_faces(child1);
        } else {
            for i in 0..segments {
                let c0_end = self.get_branch(child0).end;
                let c1_end = self.get_branch(child1).end;
                let face = [
                    self.get_branch(child0).end,
                    self.get_branch(branch_id).ring1[(i + 1) % segments],
                    self.get_branch(branch_id).ring1[i],
                ];
                self.faces.push(face);
                let face1 = [
                    self.get_branch(child1).end,
                    self.get_branch(branch_id).ring2[(i + 1) % segments],
                    self.get_branch(branch_id).ring2[i],
                ];
                self.faces.push(face1);

                let b_r1 = self.get_branch(branch_id).ring1[i];
                let len = (self.verts[c0_end] - self.verts[b_r1]).length();
                self.uvs[c0_end] = [
                    (i as f32 / segments as f32 - 1.0 - 0.5).abs() * 2.0,
                    len * self.properties.v_multiplier,
                ];
                let b_r2 = self.get_branch(branch_id).ring2[i];
                let len = (self.verts[c1_end] - self.verts[b_r2]).length();
                self.uvs[c1_end] = [
                    (i as f32 / segments as f32 - 0.5).abs() * 2.0,
                    len * self.properties.v_multiplier,
                ];
            }
        }
    }

    pub fn create_twigs(&mut self, branch_id: usize) {
        if self.get_branch(branch_id).child0.is_none() {
            let head = self.get_branch(branch_id).head;
            let length = self.get_branch(branch_id).length;
            let parent = self.get_branch(branch_id).parent.unwrap();
            let parent_head = self.get_branch(parent).head;
            let parent_c0 = self.get_branch(parent).child0.unwrap();
            let parent_c1 = self.get_branch(parent).child1.unwrap();

            let tangent = (self.get_branch(parent_c0).head - parent_head)
                .cross(self.get_branch(parent_c1).head - parent_head)
                .normalize();
            let binormal = (head - parent_head).normalize();

            let vert1 = self.verts_twig.len();
            self.verts_twig.push(
                (head + (tangent * self.properties.twig_scale))
                    + (binormal * self.properties.twig_scale * 2.0 - length),
            );
            let vert2 = self.verts_twig.len();
            self.verts_twig.push(
                (head + (tangent * -self.properties.twig_scale))
                    + (binormal * self.properties.twig_scale * 2.0 - length),
            );
            let vert3 = self.verts_twig.len();
            self.verts_twig
                .push((head + (tangent * -self.properties.twig_scale)) + (binormal * -length));
            let vert4 = self.verts_twig.len();
            self.verts_twig
                .push((head + (tangent * self.properties.twig_scale)) + (binormal * -length));

            let vert8 = self.verts_twig.len();
            self.verts_twig.push(
                (head + (tangent * self.properties.twig_scale))
                    + (binormal * self.properties.twig_scale * 2.0 - length),
            );
            let vert7 = self.verts_twig.len();
            self.verts_twig.push(
                (head + (tangent * -self.properties.twig_scale))
                    + (binormal * self.properties.twig_scale * 2.0 - length),
            );
            let vert6 = self.verts_twig.len();
            self.verts_twig
                .push((head + (tangent * -self.properties.twig_scale)) + (binormal * -length));
            let vert5 = self.verts_twig.len();
            self.verts_twig
                .push((head + (tangent * self.properties.twig_scale)) + (binormal * -length));

            self.faces_twig.push([vert1, vert2, vert3]);
            self.faces_twig.push([vert4, vert1, vert3]);

            self.faces_twig.push([vert6, vert7, vert8]);
            self.faces_twig.push([vert6, vert8, vert5]);

            let normal = ((self.verts_twig[vert1] - self.verts_twig[vert3])
                .cross(self.verts_twig[vert2] - self.verts_twig[vert3]))
            .normalize();
            let normal2 = ((self.verts_twig[vert7] - self.verts_twig[vert6])
                .cross(self.verts_twig[vert8] - self.verts_twig[vert6]))
            .normalize();

            self.normals_twig.push(normal);
            self.normals_twig.push(normal);
            self.normals_twig.push(normal);
            self.normals_twig.push(normal);

            self.normals_twig.push(normal2);
            self.normals_twig.push(normal2);
            self.normals_twig.push(normal2);
            self.normals_twig.push(normal2);

            self.uvs_twig.push([0, 1]);
            self.uvs_twig.push([1, 1]);
            self.uvs_twig.push([1, 0]);
            self.uvs_twig.push([0, 0]);

            self.uvs_twig.push([0, 1]);
            self.uvs_twig.push([1, 1]);
            self.uvs_twig.push([1, 0]);
            self.uvs_twig.push([0, 0]);
        } else {
            let c0 = self.get_branch(branch_id).child0.unwrap();
            let c1 = self.get_branch(branch_id).child1.unwrap();
            self.create_twigs(c0);
            self.create_twigs(c1);
        }
    }

    pub fn create_forks(&mut self, branch_id: usize, radius: f32) {
        let head = self.get_branch(branch_id).head;

        self.get_branch(branch_id).radius = radius;
        if radius > self.get_branch(branch_id).length {
            self.get_branch(branch_id).radius = self.get_branch(branch_id).length;
        }

        let segments = self.properties.segments;

        let segment_angle = PI * 2.0 / segments as f32;

        if self.get_branch(branch_id).parent.is_none() {
            //create the root of the tree
            let axis = vec3(0.0, 1.0, 0.0);
            for i in 0..segments {
                let vec = vec_axis_angle(vec3(-1.0, 0.0, 0.0), axis, -segment_angle * i as f32);
                let len = self.verts.len();
                self.get_branch(branch_id).root.push(len);
                self.verts
                    .push(vec * (radius / self.properties.radius_falloff_rate));
            }
        }

        //cross the branches to get the left
        //add the branches to get the up
        if self.get_branch(branch_id).child0.is_some() {
            let child0 = self.get_branch(branch_id).child0.unwrap();
            let child1 = self.get_branch(branch_id).child1.unwrap();

            let axis = match self.get_branch(child0).parent {
                Some(parent) => (head - self.get_branch(parent).head).normalize(),
                None => head.normalize(),
            };

            let axis1 = (head - self.get_branch(child0).head).normalize();
            let axis2 = (head - self.get_branch(child1).head).normalize();
            let tangent = axis1.cross(axis2).normalize();
            self.get_branch(branch_id).tangent = tangent;

            let axis3 = (tangent.cross(((axis1 * -1.0) + (axis2 * -1.0)).normalize())).normalize();
            let dir = vec3(axis2[0], 0.0, axis2[2]);
            let centerloc = head + (dir * -self.properties.max_radius / 2.0);

            let mut scale = self.properties.radius_falloff_rate;

            if (self.get_branch(branch_id).child0.is_some() && self.get_branch(child0).trunk)
                || self.get_branch(branch_id).trunk
            {
                scale = 1.0 / self.properties.taper_rate;
            }

            //main segment ring
            let linch0 = self.verts.len();
            self.get_branch(branch_id).ring0.push(linch0);
            self.get_branch(branch_id).ring2.push(linch0);
            self.verts.push(centerloc + (tangent * (radius * scale)));

            let start = self.verts.len() - 1;
            let d1 = vec_axis_angle(tangent, axis2, 1.57);
            let d2 = tangent.cross(axis).normalize();
            let s = 1.0 / d1.dot(d2);

            for i in 1..(segments / 2) {
                let mut vec = vec_axis_angle(tangent, axis2, segment_angle * i as f32);
                self.get_branch(branch_id).ring0.push(start + i);
                self.get_branch(branch_id).ring2.push(start + i);
                vec = scale_in_direction(vec, d2, s);
                self.verts.push(centerloc + (vec * (radius * scale)));
            }
            let linch1 = self.verts.len();
            self.get_branch(branch_id).ring0.push(linch1);
            self.get_branch(branch_id).ring1.push(linch1);
            self.verts.push(centerloc + (tangent * (-radius * scale)));
            for i in (segments / 2 + 1)..segments {
                let vec = vec_axis_angle(tangent, axis1, segment_angle * i as f32);
                let len = self.verts.len();
                self.get_branch(branch_id).ring0.push(len);
                let len = self.verts.len();
                self.get_branch(branch_id).ring1.push(len);
                self.verts.push(centerloc + (vec * (radius * scale)));
            }
            self.get_branch(branch_id).ring1.push(linch0);
            self.get_branch(branch_id).ring2.push(linch1);
            let start = self.verts.len() - 1;
            for i in 1..(segments / 2) {
                let vec = vec_axis_angle(tangent, axis3, segment_angle * i as f32);
                self.get_branch(branch_id).ring1.push(start + i);
                self.get_branch(branch_id)
                    .ring2
                    .push(start + (segments / 2 - i));
                let v = vec * (radius * scale);
                self.verts.push(centerloc + v);
            }

            //TODO: was not used
            //child radius is related to the brans direction and the length of the branch
            //let length0 = (branch.head - self.get_branch(branch.child0.unwrap()).head).length();
            //let length1 = (branch.head - self.get_branch(branch.child1.unwrap()).head).length();

            let mut radius0 = 1.0 * radius * self.properties.radius_falloff_rate;
            let radius1 = 1.0 * radius * self.properties.radius_falloff_rate;
            if self.get_branch(child0).trunk {
                radius0 = radius * self.properties.taper_rate;
            }
            self.create_forks(child0, radius0);
            self.create_forks(child1, radius1);
        } else {
            //add points for the ends of braches
            self.get_branch(branch_id).end = self.verts.len();
            //branch.head=addVec(branch.head,scaleVec([self.properties.xBias,self.properties.yBias,self.properties.zBias],branch.length*3));
            self.verts.push(head);
        }
    }

    // doesn't use self
    pub fn mirror_branch(&self, vec: Vec3, norm: Vec3, properties: &Tree) -> Vec3 {
        let v = norm.cross(vec.cross(norm));
        let s = properties.branch_factor * v.dot(vec);
        vec3(vec.x - v.x * s, vec.y - v.y * s, vec.z - v.z * s)
    }

    pub fn split_branch(
        &mut self,
        branch_id: usize,
        level: usize,
        steps: usize,
        properties: &mut Tree,
        l1: usize,
        l2: usize,
    ) {
        let r_level = properties.levels - level;
        let po = match self.get_branch(branch_id).parent {
            Some(b) => self.get_branch(b).head,
            None => Vec3::ZERO,
        };
        let so = self.get_branch(branch_id).head;
        let dir = (so - po).normalize();

        let normal = dir.cross(dir.zxy());
        let tangent = dir.cross(normal);
        let r = properties.random(r_level * 10 + l1 * 5 + l2 + properties.seed);
        //let r2 = properties.random(r_level * 10 + l1 * 5 + l2 + 1 + properties.seed);
        let clumpmax = properties.clump_max;
        let clumpmin = properties.clump_min;

        let mut adj = (normal * r) + (tangent * (1.0 - r));
        if r > 0.5 {
            adj *= -1.0;
        };

        let clump = (clumpmax - clumpmin) * r + clumpmin;
        let mut newdir = (adj * (1.0 - clump)) + (dir * clump).normalize();

        let mut newdir2 = self.mirror_branch(newdir, dir, properties);
        if r > 0.5 {
            std::mem::swap(&mut newdir, &mut newdir2);
        }

        if steps > 0 {
            let angle = steps as f32 / properties.tree_steps as f32
                * 2.0
                * PI
                * properties.twist_rate as f32;
            newdir2 = vec3(angle.sin(), r, angle.cos()).normalize();
        }

        let grow_amount = level as f32 * level as f32
            / (properties.levels as f32 * properties.levels as f32)
            * properties.grow_amount;
        let drop_amount = r_level as f32 * properties.drop_amount;
        let sweep_amount = r_level as f32 * properties.sweep_amount;
        newdir = (newdir + vec3(sweep_amount, drop_amount + grow_amount, 0.0)).normalize();
        newdir2 = (newdir2 + vec3(sweep_amount, drop_amount + grow_amount, 0.0)).normalize();

        let head0 = so + (newdir * self.get_branch(branch_id).length);
        let head1 = so + (newdir2 * self.get_branch(branch_id).length);
        let b0 = Branch {
            head: head0,
            tail: self.get_branch(branch_id).head,
            parent: Some(branch_id),
            length: self
                .get_branch(branch_id)
                .length
                .powf(properties.length_falloff_power)
                * properties.length_falloff_factor,
            trunk: false,
            ..Default::default()
        };
        let c0 = self.branches.add(b0);

        let b1 = Branch {
            head: head1,
            tail: self.get_branch(branch_id).head,
            parent: Some(branch_id),
            length: self
                .get_branch(branch_id)
                .length
                .powf(properties.length_falloff_power)
                * properties.length_falloff_factor,
            ..Default::default()
        };

        let c1 = self.branches.add(b1);

        if level > 0 {
            if steps > 0 {
                self.get_branch(c1).head = self.get_branch(branch_id).head
                    + vec3(
                        (r - 0.5) * 2.0 * properties.trunk_kink,
                        properties.climb_rate,
                        (r - 0.5) * 2.0 * properties.trunk_kink,
                    );
                self.get_branch(c1).trunk = true;
                self.get_branch(c1).length =
                    self.get_branch(branch_id).length * properties.taper_rate;
                self.split_branch(c1, level, steps - 1, properties, l1 + 1, l2);
            } else {
                self.split_branch(c0, level - 1, 0, properties, l1 + 1, l2);
            }
            self.split_branch(c1, level - 1, 0, properties, l1, l2 + 1);
        }

        self.get_branch(branch_id).child0 = Some(c0);

        let len = self
            .get_branch(branch_id)
            .length
            .powf(properties.length_falloff_power)
            * properties.length_falloff_factor;

        let b = Branch {
            head: head1,
            tail: self.get_branch(branch_id).head,
            parent: Some(branch_id),
            length: len,
            ..Default::default()
        };
        let new_child = self.branches.add(b);

        self.get_branch(branch_id).child1 = Some(new_child);
    }
}

// TODO: Sure this is some build in fn
fn vec_axis_angle(vec: Vec3, axis: Vec3, angle: f32) -> Vec3 {
    //v cos(T) + (axis x v) * sin(T) + axis*(axis . v)(1-cos(T)
    let cosr = angle.cos();
    let sinr = angle.sin();
    ((vec * cosr) + axis.cross(vec) * sinr) + (axis * axis.dot(vec) * (1.0 - cosr))
}

fn scale_in_direction(vector: Vec3, direction: Vec3, scale: f32) -> Vec3 {
    let current_mag = vector.dot(direction);
    let change = direction * (current_mag * scale - current_mag);
    vector + change
}

#[derive(Default)]
pub struct BranchTable {
    add_index: usize,
    pub table: HashMap<usize, Branch>,
}

impl BranchTable {
    pub fn add(&mut self, b: Branch) -> usize {
        let result = self.add_index;
        self.table.insert(self.add_index, b);
        self.add_index += 1;
        result
    }

    pub fn get(&mut self, key: usize) -> &mut Branch {
        self.table.get_mut(&key).expect("Branch Key not found")
    }
}

#[derive(Debug)]
pub struct Branch {
    pub head: Vec3,
    pub tail: Vec3,

    pub parent: Option<usize>,
    pub child0: Option<usize>,
    pub child1: Option<usize>,
    pub length: f32,
    pub radius: f32,
    pub trunk: bool,
    pub root: Vec<usize>,
    pub tangent: Vec3,
    pub ring0: Vec<usize>,
    pub ring1: Vec<usize>,
    pub ring2: Vec<usize>,
    pub end: usize,
}

impl Default for Branch {
    fn default() -> Self {
        Self {
            head: Vec3::Y,
            tail: Vec3::ZERO,
            parent: None,
            child0: None,
            child1: None,
            length: 1.0,
            radius: 0.0,
            trunk: false,
            root: Vec::new(),
            tangent: Vec3::ZERO,
            ring0: Vec::new(),
            ring1: Vec::new(),
            ring2: Vec::new(),
            end: 0,
        }
    }
}
