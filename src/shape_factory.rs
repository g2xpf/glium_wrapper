use crate::types;
use glium::vertex;

pub trait ShapeFactory<'a, U, V>: ShapeFactoryInfo<V>
where
    V: vertex::Vertex,
{
    fn new<'b>(display: &'b glium::Display, draw_parameters: glium::DrawParameters<'a>) -> Self;
    fn spawn(&mut self, value: U);
}

pub trait ShapeFactoryInfo<V: vertex::Vertex> {
    fn vertex_src() -> &'static str;
    fn fragment_src() -> &'static str;
    fn attributes() -> &'static [V];
    fn indices() -> &'static types::Indices;
}
