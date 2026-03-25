use macroquad::prelude::*;
use crate::{my_model::Model, player::Player};



pub trait Aligner
{
    fn align(&self, m: &Model, p: &Player) -> Vec3;
}

pub struct GridAligner
{
    gridsize: f32
}

impl Aligner for GridAligner
{
    fn align(&self, _m: &Model, p: &Player) -> Vec3
    {
        let looking_at = p.mi.position + p.mi.front * 2.0;

        return (looking_at / self.gridsize).floor() * self.gridsize;
    }
}

impl GridAligner
{
    pub fn new(size: f32) -> GridAligner
    {
        GridAligner
        {
            gridsize: size
        }
    }    
}

pub struct SimpleAligner
{

}

impl Aligner for SimpleAligner
{
    fn align(&self, _m: &Model, p: &Player) -> Vec3
    {
        return p.mi.position + p.mi.front * 2.0;
    }
}