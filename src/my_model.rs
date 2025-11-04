use macroquad::prelude::*;
use crate::utils::*;

#[derive(Copy, Clone)]
pub struct Triangle
{
    pub ix: [u16; 3], // indexes
    pub normal: Vec3,
}

#[macro_export]
macro_rules! tri {
    ($a:expr, $b:expr, $c:expr) => {
        $crate::Triangle { ix: [$a, $b, $c], normal: Vec3::ZERO }
    };
}

impl Triangle
{
    pub fn new(indices: [u16; 3]) -> Triangle
    {
        return Triangle {ix: indices, normal: Vec3::ZERO};
    }

    pub fn new_normal(indices: [u16;3], vertices: [Vertex; 3])
    {
        let normal_undirected = vertices[0].position - vertices[1].position
            .cross(vertices[0].position - vertices[2].position)
            .normalize();
    }

    pub fn send_ray()
    {

    }
}

pub struct Model
{
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub texture: Option<Texture2D>,
}

impl Model
{
    pub fn new(texture: Texture2D) -> Model
    {
                let vertices = vec![
            // Front face
            Vertex::new2(vec3(0.0, 0.0,  0.0), vec2(0.0, 0.0), WHITE),
            Vertex::new2(vec3( 1.0, 0.0,  0.0), vec2(1.0, 0.0), WHITE),
            Vertex::new2(vec3( 0.0,  0.0,  1.0), vec2(0.0, 1.0), WHITE),
        ];

        let triangles = vec![
            // Front face
            tri![0, 1, 2]
        ];

        Model
        {
            vertices: vertices,
            triangles: triangles,
            texture: Some(texture),
        }
    }

    pub fn gen_cube_model(texture: Option<Texture2D>) -> Model
    {
        let vertices = vec![
            // Front face
            Vertex::new2(vec3(-0.5, -0.5,  0.5), vec2(0.0, 0.0), WHITE),
            Vertex::new2(vec3( 0.5, -0.5,  0.5), vec2(1.0, 0.0), WHITE),
            Vertex::new2(vec3( 0.5,  0.5,  0.5), vec2(1.0, 1.0), WHITE),
            Vertex::new2(vec3(-0.5,  0.5,  0.5), vec2(0.0, 1.0), WHITE),
            // Back face
            Vertex::new2(vec3(-0.5, -0.5, -0.5), vec2(1.0, 0.0), WHITE),
            Vertex::new2(vec3( 0.5, -0.5, -0.5), vec2(0.0, 0.0), WHITE),
            Vertex::new2(vec3( 0.5,  0.5, -0.5), vec2(0.0, 1.0), WHITE),
            Vertex::new2(vec3(-0.5,  0.5, -0.5), vec2(1.0, 1.0), WHITE),
        ];

        let triangles = vec![
            // Front face
            tri![0, 1, 2], tri![0, 2, 3],
            // Back face
            tri![4, 6, 5], tri![4, 7, 6],
            // Left face
            tri![4, 5, 1], tri![4, 1, 0],
            // Right face
            tri![3, 2, 6], tri![3, 6, 7],
            // Top face
            tri![1, 5, 6], tri![1, 6, 2],
            // Bottom face
            tri![4, 0, 3], tri![4, 3, 7],
        ];

        Model
        {
            vertices: vertices,
            triangles: triangles,
            texture: texture,
        }
    }

    pub fn gen_mesh(&self) -> Mesh
    {
        return Mesh {
            vertices: self.vertices.clone(),
            indices: self.triangles.iter().flat_map(|tri| tri.ix.to_vec()).collect(),
            texture: self.texture.clone(),
        };
    }

    pub fn tri_as_vertices(&self, tri: Triangle) -> [Vertex; 3]
    {
        return [self.vertices[tri.ix[0] as usize], self.vertices[tri.ix[1] as usize], self.vertices[tri.ix[2] as usize]];
    }

    pub fn get_closest_triangle(&self, loc: Vec3) -> Option<Triangle>
    {
        self.triangles
            .iter()
            .map(|tri| (*tri, triangle_distance_squared(loc, self.tri_as_vertices(*tri))))
            .min_by(|(_, dist_a), (_, dist_b)| dist_a.partial_cmp(dist_b).unwrap())
            .map(|(tri, _)| tri)
    }

    pub fn ray_tri_intersect(&self, tri_indeces: Triangle, loc: Vec3, dir: Vec3) -> Option<f32>
    {
        let tri = self.tri_as_vertices(tri_indeces);
        let normal = (tri[1].position - tri[0].position).cross(tri[2].position - tri[0].position);
        let t = (normal.dot(tri[0].position - loc)) / (normal.dot(dir));
        let p_int = (t * dir) + loc;
        let c0 = (tri[1].position - tri[0].position).cross(p_int - tri[0].position);
        let c1 = (tri[2].position - tri[1].position).cross(p_int - tri[1].position);
        let c2 = (tri[0].position - tri[2].position).cross(p_int - tri[2].position);
        if (normal.dot(c0) >= 0.0 && normal.dot(c1) >= 0.0 && normal.dot(c2) >= 0.0)
        {
            return Some(t);
        }
        else 
        {      
            return None;
        }
    }

    pub fn send_ray(&self, loc: Vec3, dir: Vec3) -> Option<(Vec3, Triangle)>
    {
        let mut lowest_t: f32 = 0.;
        let mut target_tri = tri![0,0,0];

        for tri in self.triangles.clone()
        {
            if let Some(t) = self.ray_tri_intersect(tri, loc, dir)
            {
                if -t < lowest_t // -t because ray_tri_intersect returns negative t usually
                {
                    lowest_t = -t;
                    target_tri = tri;
                }
            }
        }

        if lowest_t != 0.
        {
            return Some(((lowest_t * dir) + loc, target_tri));
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
            let dir_to_vec = (v.position - loc).normalize();
            let angle = dir.angle_between(dir_to_vec);

            if angle < angle_range
            {
                let distance = loc.distance(v.position);
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
}



