use macroquad::prelude::*;
use crate::utils::*;

#[macro_export]
macro_rules! tri_uv {
    (
        $a:expr, ($ua:expr, $va:expr),
        $b:expr, ($ub:expr, $vb:expr),
        $c:expr, ($uc:expr, $vc:expr)
    ) => {
        crate::Poly::Triangle {
            ix: [$a, $b, $c],
            uvs: [
                vec2($ua, $va),
                vec2($ub, $vb),
                vec2($uc, $vc),
            ],
            normal: Vec3::ZERO, // or whatever default you want
        }
    };
}

#[macro_export]
macro_rules! quad_uv {
    (
        $a:expr, ($ua:expr, $va:expr),
        $b:expr, ($ub:expr, $vb:expr),
        $c:expr, ($uc:expr, $vc:expr),
        $d:expr, ($ud:expr, $vd:expr)
    ) => {
        crate::Poly::Quad {
            ix: [$a, $b, $c, $d],
            uvs: [
                vec2($ua, $va),
                vec2($ub, $vb),
                vec2($uc, $vc),
                vec2($ud, $vd)
            ],
            normal: Vec3::ZERO, // or whatever default you want
        }
    };
}

#[macro_export]
macro_rules! quad_uv2 {
    (
        $a:expr, ($ua:expr, $va:expr),
        $b:expr, ($ub:expr, $vb:expr),
        $c:expr, ($uc:expr, $vc:expr),
        $d:expr, ($ud:expr, $vd:expr)
    ) => {
        crate::Poly::Quad {
            ix: [$a, $b, $c, $d],
            uvs: [
                Model::to_uv($ua, $va),
                Model::to_uv($ub, $vb),
                Model::to_uv($uc, $vc),
                Model::to_uv($ud, $vd)
            ],
            normal: Vec3::ZERO, // or whatever default you want
        }
    };
}

#[macro_export]
macro_rules! tri {
    ($a:expr, $b:expr, $c:expr) => {
        $crate::Poly::Triangle { 
            ix: [$a, $b, $c], 
            uvs: [Vec2::ZERO, Vec2::ZERO, Vec2::ZERO],
            normal: Vec3::ZERO 
        }
    };
}

#[macro_export]
macro_rules! quad {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $crate::Poly::Quad { 
            ix: [$a, $b, $c, $d], 
            uvs: [Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO],
            normal: Vec3::ZERO 
        }
    };
}

#[derive(Clone, Copy)]
pub enum Poly
{
    Triangle {
        ix: [u16; 3],
        uvs: [Vec2; 3],
        normal: Vec3
    },
    Quad {
        ix: [u16; 4],
        uvs: [Vec2; 4],
        normal: Vec3
    }
}

impl Poly
{
    pub fn to_vec(&self) -> Vec<u16>
    {
        match self
        {
            Poly::Triangle { ix, uvs: _, normal: _ } => ix.to_vec(),
            Poly::Quad { ix, uvs: _, normal: _ } => ix.to_vec(),
        }
    }

    pub fn to_tris(&self) -> Vec<Poly>
    {
        match self
        {
            Poly::Triangle { ..} => vec![*self],
            Poly::Quad { ix, uvs, normal} => vec![ 
                Poly::Triangle{ix: [ix[0], ix[1], ix[2]], uvs: [uvs[0], uvs[1], uvs[2]], normal: *normal},
                Poly::Triangle{ix: [ix[2], ix[3], ix[0]], uvs: [uvs[2], uvs[3], uvs[0]], normal: *normal}
            ],
        }
    }

    pub fn add_to_indeces(&self, num: u16) -> Poly
    {
        match self
        {
            Poly::Triangle {ix, uvs, normal} => Poly::Triangle {
                ix: [ix[0] + num, ix[1] + num, ix[2] + num], 
                uvs: *uvs,
                normal: *normal
            },
            Poly::Quad {ix, uvs, normal} => Poly::Quad {
                ix: [ix[0] + num, ix[1] + num, ix[2] + num, ix[3] + num], 
                uvs: *uvs,
                normal: *normal
            }
        }
    }
}

pub struct Model
{
    pub vertices: Vec<Vec3>,
    pub polys: Vec<Poly>,
    pub texture: Option<Texture2D>,
}

impl Model
{
    pub fn new(texture: Option<Texture2D>) -> Model
    {
        let vertices = vec![
            // Front face
            vec3(0.0, 0.0,  0.0),
            vec3(1.0, 0.0,  0.0),
            vec3(1.0, 0.0, 1.0),
        ];

        let triangles = vec![
            // Front face
            tri_uv![0, (0.,0.), 1, (1., 0.), 2, (1., 1.)]
        ];

        Model
        {
            vertices: vertices,
            polys: triangles,
            texture: texture,
        }
    }

    pub fn to_uv(x: u32, y:u32) -> Vec2
    {
        vec2(x as f32 / 64. ,y as f32/ 48.)
    }

    pub fn gen_cube_model(texture: Option<Texture2D>) -> Model
    {
        let vertices = vec![
            // Front face
            vec3(-0.5, -0.5,  0.5),
            vec3( 0.5, -0.5,  0.5),
            vec3( 0.5,  0.5,  0.5),
            vec3(-0.5,  0.5,  0.5),
            // Back face
            vec3(-0.5, -0.5, -0.5),
            vec3( 0.5, -0.5, -0.5),
            vec3( 0.5,  0.5, -0.5),
            vec3(-0.5,  0.5, -0.5),
        ];

        let polys = vec![
            // Front
            quad_uv2![0, (0,32), 1, (16,32), 2, (16,16), 3, (0,16)],
            // Bottom
            quad_uv2![0, (16,48), 1, (16,32), 5, (32,32), 4, (32,48)],
            // Right
            quad_uv2![1, (16,32), 2, (16,16), 6, (32,16), 5, (32,32)],
            // Top
            quad_uv2![2, (16,16), 3, (16,0), 7, (32,0), 6, (32,16)],
            // Left
            quad_uv2![3, (64,16), 0, (64,32), 4, (48,32), 7, (48,16)],
            // Back
            quad_uv2![4, (48,32), 5, (32,32), 6, (32,16), 7, (48,16)]
        ];

        Model
        {
            vertices: vertices,
            polys: polys,
            texture: texture,
        }
    }

    pub fn gen_mesh(&self) -> Mesh
    {
        let mut vertexes: Vec<Vertex> = vec![];

        for poly in & self.polys
        {
            vertexes.append(&mut self.poly_as_tris_vertices(poly))
        }

        let indexes = (0..(vertexes.len() as u16)).collect();

        return Mesh {
            vertices: vertexes,
            indices: indexes,
            texture: self.texture.clone(),
        };
    }

    pub fn poly_as_vertices(&self, poly: Poly) -> Vec<Vertex>
    {
        match poly
        {
            Poly::Triangle{ ix, uvs, ..} => 
                [
                    Vertex::new2(self.vertices[ix[0] as usize], uvs[0],WHITE), 
                    Vertex::new2(self.vertices[ix[1] as usize], uvs[1], WHITE), 
                    Vertex::new2(self.vertices[ix[2] as usize], uvs[2], WHITE),
                ].to_vec(),
            Poly::Quad{ ix, uvs, ..} => 
                [
                    Vertex::new2(self.vertices[ix[0] as usize], uvs[0],WHITE), 
                    Vertex::new2(self.vertices[ix[1] as usize], uvs[1], WHITE), 
                    Vertex::new2(self.vertices[ix[2] as usize], uvs[2], WHITE),
                    Vertex::new2(self.vertices[ix[3] as usize], uvs[3], WHITE),
                ].to_vec(),
        }
    }

    pub fn poly_as_tris_vertices(&self, poly: &Poly) -> Vec<Vertex>
    {
        match poly
        {
            Poly::Triangle{ ix, uvs, ..} => 
                [
                    Vertex::new2(self.vertices[ix[0] as usize], uvs[0],WHITE), 
                    Vertex::new2(self.vertices[ix[1] as usize], uvs[1], WHITE), 
                    Vertex::new2(self.vertices[ix[2] as usize], uvs[2], WHITE),
                ].to_vec(),
            Poly::Quad{ ix, uvs, ..} => 
                [
                    Vertex::new2(self.vertices[ix[0] as usize], uvs[0],WHITE), 
                    Vertex::new2(self.vertices[ix[1] as usize], uvs[1], WHITE), 
                    Vertex::new2(self.vertices[ix[2] as usize], uvs[2], WHITE),
                    Vertex::new2(self.vertices[ix[2] as usize], uvs[2],WHITE), 
                    Vertex::new2(self.vertices[ix[3] as usize], uvs[3], WHITE), 
                    Vertex::new2(self.vertices[ix[0] as usize], uvs[0], WHITE),
                ].to_vec(),
        }
    }

    pub fn get_closest_poly(&self, loc: Vec3) -> Option<Poly>
    {
        self.polys
            .iter()
            .map(|poly| (*poly, triangle_distance_squared(loc, self.poly_as_vertices(*poly))))
            .min_by(|(_, dist_a), (_, dist_b)| dist_a.partial_cmp(dist_b).unwrap())
            .map(|(tri, _)| tri)
    }

    pub fn ray_tri_intersect(&self, poly: Poly, loc: Vec3, dir: Vec3) -> Option<f32> // needs fixing cause its no longer tri!!
    {
        match poly
        {
            Poly::Quad{..} => 
            {
                let tris = poly.to_tris();
                if let Some(t) = self.ray_tri_intersect(tris[0], loc, dir)
                {
                    return Some(t);
                }
                if let Some(t) = self.ray_tri_intersect(tris[1], loc, dir)
                {
                    return Some(t)
                }
                return None;
            }
            Poly::Triangle{..} => 
            {
                // gpt
                let tri = self.poly_as_vertices(poly);
                let normal = (tri[1].position - tri[0].position).cross(tri[2].position - tri[0].position);
                let t = (normal.dot(tri[0].position - loc)) / (normal.dot(dir));
                let p_int = (t * dir) + loc;
                let c0 = (tri[1].position - tri[0].position).cross(p_int - tri[0].position);
                let c1 = (tri[2].position - tri[1].position).cross(p_int - tri[1].position);
                let c2 = (tri[0].position - tri[2].position).cross(p_int - tri[2].position);
                // gpt
                if (normal.dot(c0) >= 0.0 && normal.dot(c1) >= 0.0 && normal.dot(c2) >= 0.0)
                {
                    return Some(t);
                }
                else 
                {      
                    return None;
                }
            }
        }
    }

    pub fn send_ray(&self, loc: Vec3, dir: Vec3) -> Option<(Vec3, Poly)>
    {
        let max_t = 100000.;
        let mut lowest_t: f32 = max_t; // max value
        let mut target_poly = tri![0,0,0];

        for poly in self.polys.clone()
        {
            if let Some(t) = self.ray_tri_intersect(poly, loc, dir)
            {
                if t < lowest_t // -t because ray_tri_intersect returns negative t usually
                {
                    lowest_t = t;
                    target_poly = poly;
                }
            }
        }

        if lowest_t != max_t
        {
            return Some(((lowest_t * dir) + loc, target_poly));
        }
        else 
        {
            return None;    
        }
    }

    pub fn send_ray_vertex(&self, loc: Vec3, dir: Vec3, angle_range: f32) -> Option<u16>
    {
        let mut closest_distance: f32 = 50000.; // max value
        let mut target: Option<u16> = None;

        let mut i: u16 = 0;
        for v in self.vertices.clone()
        {
            let dir_to_vec = (v - loc).normalize();
            let angle = dir.angle_between(dir_to_vec);

            if angle < angle_range
            {
                let distance = loc.distance(v);
                if distance < closest_distance
                {
                    closest_distance = distance;
                    target = Some(i);
                }
            }

            i += 1;
        }

        return target;
    }

    pub fn calc_uv(&self, poly: Poly, loc: Vec3) -> Option<Vec2>
    {
        let vertices = self.poly_as_vertices(poly);

        let a = vertices[0].position;
        let b = vertices[1].position - a;
        let c = vertices[2].position - a;

        let target = loc - a;

        // gpt
        // Precompute dot products
        let b2 = b.dot(b);
        let bc = b.dot(c);
        let c2 = c.dot(c);
        let tb = target.dot(b);
        let tc = target.dot(c);

        // Determinant of the 2×2 system
        let det = b2 * c2 - bc * bc;
        if det.abs() < 1e-6 {
            return None; // B and C are collinear
        }

        // Solve for α and β
        let alpha = (tb * c2 - tc * bc) / det;
        let beta  = (tc * b2 - tb * bc) / det;
        // gpt

        let uv_ab = vertices[1].uv - vertices[0].uv;
        let uv_ac = vertices[2].uv - vertices[0].uv;

        let result_uv = vertices[0].uv + alpha * uv_ab + beta * uv_ac;

        return Some(result_uv);
    }
}



