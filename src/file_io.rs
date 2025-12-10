use std::str::{SplitWhitespace, Split};
use std::{fs::File, path::Path};
use std::fs::{read_to_string, write};

use macroquad::prelude::*;

use crate::my_model::{Model, Poly};
use crate::utils::{fvec2, fvec3};


pub fn load_obj_file(path: &str) -> Result<Model, &'static str>
{
    let mut vertices = vec![];
    let mut uvs = vec![];
    let mut normals = vec![];
    let mut polys = vec![];

    let path = Path::new(path);

    let maybe_as_string = read_to_string(&path);
    let as_string = match maybe_as_string
    {
        Err(_) => return Err("Failed to read file"),
        Ok(s) => s,
    };

    let lines = as_string.lines();

    for line in lines
    {
        let mut parts = line.split_whitespace();
        if let Some(first_part) = parts.next()
        {
            if first_part == "v"
            {
                if let Some(vec) = parse_vec3(&mut parts)
                {
                    vertices.push(vec);
                }
            }
            else if first_part == "vt"
            {
                if let Some(uv) = parse_vec2(&mut parts)
                {
                    uvs.push(uv);
                }
            }
            else if first_part == "vn"
            {
                if let Some(normal) = parse_vec3(&mut parts)
                {
                    normals.push(normal);
                }
            }
            else if first_part == "f"
            {
                let mut poly_ixs = vec![];
                let mut poly_uvs = vec![];
                let mut poly_norm = Vec3::ZERO;

                for part in parts
                {
                    if let Some((ix,uv_ix,n_ix)) = parse_poly(&mut part.split('/'))
                    {
                        poly_ixs.push(ix);
                        poly_uvs.push(uvs[uv_ix as usize]);
                        poly_norm = normals[n_ix as usize];
                    }
                }
                
                if poly_ixs.len() == 3
                {
                    polys.push(Poly::Triangle {
                        ix: poly_ixs[0..3].try_into().unwrap(),
                        uvs: poly_uvs[0..3].try_into().unwrap(),
                        normal: poly_norm
                    })
                }
                if poly_ixs.len() == 4
                {
                    polys.push(Poly::Quad {
                        ix: poly_ixs[0..4].try_into().unwrap(),
                        uvs: poly_uvs[0..4].try_into().unwrap(),
                        normal: poly_norm
                    })
                }
            }
        }
    }

    return Ok(Model{vertices:vertices, polys: polys, texture: None});
}

fn parse_vec3(parts: &mut SplitWhitespace<'_>) -> Option<Vec3>
{
    let x = parts.next()?.parse().ok()?;
    let y = parts.next()?.parse().ok()?;
    let z = parts.next()?.parse().ok()?;

    return Some(vec3(x,y,z));
}

fn parse_vec2(parts: &mut SplitWhitespace<'_>) -> Option<Vec2>
{
    let u = parts.next()?.parse().ok()?;
    let v = parts.next()?.parse().ok()?;

    return Some(vec2(u,v));
}

fn parse_poly(parts: &mut Split<'_, char>) -> Option<(u16,u32,u32)>
{
    let index: u16 = parts.next()?.parse().ok()?;
    let uv_index: u32 = parts.next()?.parse().ok()? ;
    let normal_index: u32 = parts.next()?.parse().ok()?;

    return Some((index-1, uv_index-1, normal_index-1));
}

pub fn write_obj_file(path: &str, model: &Model) -> Result<(), &'static str>
{
    let mut content = String::new();

    for v in &model.vertices
    {
        content.push_str(&format!("v {} \n", fvec3(*v)));
    }

    let mut polys_string = String::new();
    let mut uvs_string = String::new();
    let mut normals_string = String::new();


    let mut uv_index = 1;
    let mut normals_index = 1;
    for poly in &model.polys
    {
        match poly
        {
            Poly::Triangle{ix, uvs, normal} =>
            {
                uvs_string.push_str(&format!("vt {} \nvt {} \nvt {} \n", fvec2(uvs[0]), fvec2(uvs[1]), fvec2(uvs[2])));
                normals_string.push_str(&format!("vn {} \n", fvec3(*normal)));
                polys_string.push_str(&format!("f {}/{}/{} {}/{}/{} {}/{}/{} \n", 
                    ix[0]+1, 
                    uv_index, 
                    normals_index, 
                    ix[1]+1, 
                    uv_index+1,
                    normals_index,
                    ix[2]+1,
                    uv_index+2,
                    normals_index
                ));
                uv_index += 3;
            },
            Poly::Quad{ix, uvs, normal} =>
            {
                uvs_string.push_str(&format!("vt {} \nvt {} \nvt {} \nvt {}\n", fvec2(uvs[0]), fvec2(uvs[1]), fvec2(uvs[2]), fvec2(uvs[3])));
                normals_string.push_str(&format!("vn {} \n", fvec3(*normal)));
                polys_string.push_str(&format!("f {}/{}/{} {}/{}/{} {}/{}/{} {}/{}/{} \n", 
                    ix[0]+1, 
                    uv_index, 
                    normals_index, 
                    ix[1]+1, 
                    uv_index+1,
                    normals_index,
                    ix[2]+1,
                    uv_index+2,
                    normals_index,
                    ix[3]+1,
                    uv_index+3,
                    normals_index
                ));
                uv_index += 4;
            }
        }
        normals_index += 1;
    }

    content.push_str(&uvs_string);
    content.push_str(&normals_string);
    content.push_str(&polys_string);

    write(path, content).map_err(|_| "Failed to write file")?;

    Ok(())
}