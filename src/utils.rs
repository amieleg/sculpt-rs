use macroquad::prelude::*;

pub fn triangle_distance_squared(v: Vec3, tri: [Vertex; 3]) -> f32
{
    return (v.distance_squared(tri[0].position) + v.distance_squared(tri[1].position) + v.distance_squared(tri[2].position)) / 3.0;
}

pub fn drop_y_normalize(inp: Vec3) -> Vec3
{
    return vec3(inp.x, 0., inp.z).normalize();
}

