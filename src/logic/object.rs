use std::fmt;
use glam::Mat4;

pub trait GameObject : fmt::Debug {
    fn ref_transform(&self) -> &Mat4;
    fn mut_transform(&mut self) -> &mut Mat4;
}
