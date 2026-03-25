use macroquad::prelude::*;
use crate::{material_atlas::MaterialAtlas, utils::*};

#[macro_export]
macro_rules! tri {
    ($a:expr, $b:expr, $c:expr) => {
        Poly { 
            ixs: vec![$a, $b, $c], 
            uvs: vec![Vec2::ZERO, Vec2::ZERO, Vec2::ZERO],
            normal: Vec3::ZERO,
            material: None,
        }
    };
}

#[macro_export]
macro_rules! tri_uv {
    (
        $a:expr, ($ua:expr, $va:expr),
        $b:expr, ($ub:expr, $vb:expr),
        $c:expr, ($uc:expr, $vc:expr)
    ) => {
        Poly {
            ixs: vec![$a, $b, $c],
            uvs: vec![
                vec2($ua, $va),
                vec2($ub, $vb),
                vec2($uc, $vc),
            ],
            normal: Vec3::ZERO,
            material: None, 
        }
    };
}

#[macro_export]
macro_rules! quad {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        Poly { 
            ixs: vec![$a, $b, $c, $d], 
            uvs: vec![Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO],
            normal: Vec3::ZERO,
            material: None
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
        Poly {
            ixs: vec![$a, $b, $c, $d],
            uvs: vec![
                vec2($ua, $va),
                vec2($ub, $vb),
                vec2($uc, $vc),
                vec2($ud, $vd)
            ],
            normal: Vec3::ZERO,
            material: None
        }
    };
}

#[macro_export]
macro_rules! quad_uv2 {
    (
        $a:expr, ($ua:expr, $va:expr),
        $b:expr, ($ub:expr, $vb:expr),
        $c:expr, ($uc:expr, $vc:expr),
        $d:expr, ($ud:expr, $vd:expr),
        ($w:expr, $h:expr)
    ) => {
        Poly {
            ixs: vec![$a, $b, $c, $d],
            uvs: vec![
                vec2($ua/$w, $va/$h),
                vec2($ub/$w, $vb/$h),
                vec2($uc/$w, $vc/$h),
                vec2($ud/$w, $vd/$h)
            ],
            normal: Vec3::ZERO,
        }
    };
}

pub struct Tri
{
    pub ixs: [u16; 3],
    pub uvs: [Vec2; 3],
    pub normal: Vec3,
    pub material: Option<u16>,
}

#[derive(Clone)]
pub struct Poly
{
    pub ixs: Vec<u16>,
    pub uvs: Vec<Vec2>,
    pub normal: Vec3,
    pub material: Option<u16>
}

impl Poly
{
    pub fn to_tris(&self) -> Vec<Tri>
    {
        if self.ixs.len() == 3
        {
            return vec!
            [
                Tri
                {
                    ixs: [self.ixs[0], self.ixs[1], self.ixs[2]],
                    uvs: [self.uvs[0], self.uvs[1], self.uvs[2]],
                    normal: self.normal,
                    material: self.material,
                }
            ]
        }
        else if self.ixs.len() == 4
        {
            return vec!
            [
                Tri
                {
                    ixs: [self.ixs[0], self.ixs[1], self.ixs[2]],
                    uvs: [self.uvs[0], self.uvs[1], self.uvs[2]],
                    normal: self.normal,
                    material: self.material
                },
                Tri
                {
                    ixs: [self.ixs[0], self.ixs[2], self.ixs[3]],
                    uvs: [self.uvs[0], self.uvs[2], self.uvs[3]],
                    normal: self.normal,
                    material: self.material
                }
            ]
        }
        else if self.ixs.len() >= 5
        {
            let len = self.ixs.len();
            let mut tris = vec!
            [
                Tri
                {
                    ixs: [self.ixs[0], self.ixs[1], self.ixs[2]],
                    uvs: [self.uvs[0], self.uvs[1], self.uvs[2]],
                    normal: self.normal,
                    material: self.material
                },
                Tri
                {
                    ixs: [self.ixs[0], self.ixs[len-2], self.ixs[len-1]],
                    uvs: [self.uvs[0], self.uvs[len-2], self.uvs[len-1]],
                    normal: self.normal,
                    material: self.material
                }
            ];

            for i in 2..(len-2)
            {
                tris.push
                (
                    Tri
                    {
                        ixs: [self.ixs[0], self.ixs[i], self.ixs[i+1]],
                        uvs: [self.uvs[0], self.uvs[i+1], self.uvs[i+1]],
                        normal: self.normal,
                        material: self.material
                    }
                );
            }

            return tris;
        }

        return vec![];
    }

    pub fn add_to_indeces(&self, num: u16) -> Poly
    {
        let mut p = self.clone();
        for ix in &mut p.ixs
        {
            *ix += num;
        }
        return p;
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
            vec3(-0.5, 0.0,  -0.5),
            vec3(0.5, 0.0,  -0.5),
            vec3(0.5, 0.0, 0.5),
            vec3(-0.5, 0.0, 0.5),
        ];

        let polys = vec![
            quad_uv![0, (0.,0.), 1, (1., 0.), 2, (1., 1.), 3, (0., 1.)]
        ];

        Model
        {
            vertices: vertices,
            polys: polys,
            texture: texture,
        }
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
            //quad_uv![0, (0., 2./3.), 1, (1./4.,2./3.), 2, (1./4.,1./3.), 3, (0.,1./3.)],
            quad_uv![0, (0., 0.), 1, (0.,1.), 2, (1.,1.), 3, (1.,0.)],
            // Bottom
            quad_uv![0, (1./4.,1.), 1, (1./4.,2./3.), 5, (1./2.,2./3.), 4, (1./2.,1.)],
            // Right
            quad_uv![1, (1./4.,2./3.), 2, (1./4.,1./3.), 6, (1./2.,1./3.), 5, (1./2.,2./3.)],
            // Top
            quad_uv![2, (1./4.,1./3.), 3, (1./4.,0.), 7, (1./2.,0.), 6, (1./2.,1./3.)],
            // Left
            quad_uv![3, (1.,1./3.), 0, (1.,2./3.), 4, (3./4.,2./3.), 7, (3./4.,1./3.)],
            // Back
            quad_uv![4, (3./4.,2./3.), 5, (1./2.,2./3.), 6, (1./2.,1./3.), 7, (3./4.,1./3.)]
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

    pub fn poly_as_vertices(&self, poly: &Poly) -> Vec<Vertex>
    {
        return poly.ixs.iter().enumerate().map(|(i,ix)| Vertex::new2(self.vertices[*ix as usize], poly.uvs[i], WHITE)).collect();
    }

    pub fn tri_as_vertices(&self, tri: &Tri) -> Vec<Vertex>
    {
        return vec!
        [
            Vertex::new2(self.vertices[tri.ixs[0] as usize], tri.uvs[0], WHITE),
            Vertex::new2(self.vertices[tri.ixs[1] as usize], tri.uvs[1], WHITE),
            Vertex::new2(self.vertices[tri.ixs[2] as usize], tri.uvs[2], WHITE)
        ]
    }

    pub fn poly_as_tris_vertices(&self, poly: &Poly) -> Vec<Vertex>
    { 
        return poly.to_tris().iter().map(|tri| self.tri_as_vertices(tri)).flat_map(|v| v).collect();
    }

    fn adjust_polys(&mut self, index: u16)
    {
        for poly in &mut self.polys
        {
            for i in &mut poly.ixs
            {
                if *i > index 
                {
                    *i -= 1;
                }
            }
        }
    }
    

    fn vertex_has_poly(&self, index: u16) -> bool
    {
        for poly in &self.polys
        {
            if poly.ixs.contains(&index)
            {
                return true;
            }
        }
        return false;
    }

    fn delete_poly_private(&mut self, index: u16, to_delete: &mut Vec<u16>)
    {
        let deleted = self.polys.remove(index as usize);

        for i in deleted.ixs
        {
            if !to_delete.contains(&i)
            {
                if !self.vertex_has_poly(i)
                {
                    to_delete.push(i)
                }
            }
        }
    }

    fn delete_and_adjust_vertices(&mut self, to_delete: &mut Vec<u16>)
    {
        to_delete.sort();

        let mut extra = 0;
        for index in to_delete.clone()
        {
            self.adjust_polys(index - extra);
            extra += 1;
        }

        to_delete.reverse();
        for index in to_delete.clone()
        {
            self.vertices.remove(index as usize);
        }
    }

    pub fn delete_poly(&mut self, index: u16)
    {
        let mut to_delete = vec![];
        self.delete_poly_private(index, &mut to_delete);

        self.delete_and_adjust_vertices(&mut to_delete);
    }

    pub fn delete_vertex(&mut self, index: u16)
    {
        let mut to_delete = vec![];

        to_delete.push(index);

        let mut i = 0;
        while i < self.polys.len()
        {
            let poly = &self.polys[i];
            if poly.ixs.contains(&index)
            {
                self.delete_poly_private(i as u16, &mut to_delete);
            }
            else 
            {
                i += 1;
            }
        }

        self.delete_and_adjust_vertices(&mut to_delete);
    }

    pub fn get_closest_poly(&self, loc: Vec3) -> Option<Poly>
    {
        self.polys
            .iter()
            .map(|poly| (poly.clone(), poly_distance_squared(loc, self.poly_as_vertices(poly))))
            .min_by(|(_, dist_a), (_, dist_b)| dist_a.partial_cmp(dist_b).unwrap())
            .map(|(tri, _)| tri)
    }

    pub fn ray_tri_intersect(&self, tri: &Tri, loc: Vec3, dir: Vec3) -> Option<f32> // needs fixing cause its no longer tri!!
    {
        // gpt
        let tri = self.tri_as_vertices(tri);
        let normal = (tri[1].position - tri[0].position).cross(tri[2].position - tri[0].position);
        let t = (normal.dot(tri[0].position - loc)) / (normal.dot(dir));
        let p_int = (t * dir) + loc;

        let c0 = (tri[1].position - tri[0].position).cross(p_int - tri[0].position);
        let c1 = (tri[2].position - tri[1].position).cross(p_int - tri[1].position);
        let c2 = (tri[0].position - tri[2].position).cross(p_int - tri[2].position);
        // gpt
        if normal.dot(c0) >= 0.0 && normal.dot(c1) >= 0.0 && normal.dot(c2) >= 0.0
        {
            return Some(t);
        }
        else 
        {      
            return None;
        }
    }

    pub fn ray_intersect(&self, poly: &Poly, loc: Vec3, dir: Vec3) -> Option<f32>
    {
        let tris = poly.to_tris();

        for tri in &tris
        {
            if let Some(t) = self.ray_tri_intersect(tri, loc, dir)
            {
                return Some(t);
            }
        }

        return None;
    }

    pub fn send_ray(&self, loc: Vec3, dir: Vec3) -> Option<(Vec3, Poly, u16)>
    {
        let max_t = 1e6;
        let mut lowest_t = max_t; // max value
        let mut target_poly = tri![0,0,0];
        let mut target_index = 0;

        let mut i = 0;
        for poly in self.polys.clone()
        {
            if let Some(t) = self.ray_intersect(&poly, loc, dir)
            {
                if t < lowest_t && t >= 0.
                {
                    lowest_t = t;
                    target_poly = poly;
                    target_index = i;
                }
            }
            i += 1;
        }

        if lowest_t != max_t
        {
            return Some(((lowest_t * dir) + loc, target_poly, target_index));
        }
        else 
        {
            return None;    
        }
    }

    pub fn send_ray_vertex(&self, loc: Vec3, dir: Vec3, angle_range: f32) -> Option<u16>
    {
        let mut closest_distance = 1e6; // max value
        let mut target = None;

        let mut i= 0;
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

    fn calc_uv_tri(&self, tri: &Tri, loc: Vec3) -> Option<Vec2>
    {
        let vertices = self.tri_as_vertices(tri);

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

    pub fn calc_uv(&self, poly: Poly, loc: Vec3) -> Option<Vec2>
    {
        let tris = poly.to_tris();

        for tri in &tris
        {
            if let Some(vec2) = self.calc_uv_tri(tri, loc)
            {
                return Some(vec2);
            }
        }

        return None;
    }

    pub fn set_uvs(&mut self, mats: &MaterialAtlas)
    {
        for poly in &mut self.polys
        {
            if let Some(material_id) = poly.material
            {
                let uvs = mats.get_uvs(material_id);
                let uv_corners = [uvs.0, vec2(uvs.0.x, uvs.1.y), uvs.1, vec2(uvs.1.x, uvs.0.x)];

                let mut i = 0;
                for uv in &mut poly.uvs
                {
                    uv.x = uv_corners[i].x;
                    uv.y = uv_corners[i].y;
                    i += 1;
                }
            }
            else
            {
                for uv in &mut poly.uvs
                {
                    uv.x = 0.;
                    uv.y = 0.;
                }
            }
        }
    }

    pub fn set_uv(&mut self, index: u16, mats: &MaterialAtlas)
    {
        if let Some(material_id) = self.polys[index as usize].material
        {
            let uvs = mats.get_uvs(material_id);
            let uv_corners = [uvs.0, vec2(uvs.1.x, uvs.0.y), uvs.1, vec2(uvs.0.x, uvs.1.y)];

            let mut i = 0;
            for uv in &mut self.polys[index as usize].uvs
            {
                uv.x = uv_corners[i].x;
                uv.y = uv_corners[i].y;
                i += 1;
            }
        }
        else
        {
            for uv in &mut self.polys[index as usize].uvs
            {
                uv.x = 0.;
                uv.y = 0.;
            }
        }
    }

}