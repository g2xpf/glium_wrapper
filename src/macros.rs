#[macro_export]
macro_rules! texture {
    ($display: expr, $image_path: expr, $image_format: expr) => {{
        let image = image::load(
            std::io::Cursor::new(&include_bytes!($image_path)[..]),
            $image_format,
        )
        .unwrap()
        .to_rgba();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        glium::texture::Texture2d::new($display, image).unwrap()
    }};
}

#[macro_export]
macro_rules! implement_shape_factory {
    ( $factory_name:ident ; $vertex:ident, $( $vid:ident ),+ ; $uniform:ty, $( $uid:ident ),+ ) => {
        glium::implement_vertex!($vertex, $( $vid ),+);

        pub struct $factory_name<'a> {
            program: glium::Program,
            vertex_buffer: glium::VertexBuffer<$vertex>,
            index_buffer: glium::IndexBuffer<glium_wrapper::types::Index>,
            draw_parameters: glium::DrawParameters<'a>,
            pub uniform: Vec<$uniform>,
        }

        impl<'a> std::ops::Deref for $factory_name<'a> {
            type Target = [$uniform];

            fn deref(&self) -> &[$uniform] {
                &self.uniform
            }
        }

        impl<'a> std::ops::DerefMut for $factory_name<'a> {
            fn deref_mut(&mut self) -> &mut [$uniform] {
                &mut self.uniform
            }
        }

        impl<'a> glium_wrapper::shape_factory::ShapeFactory<'a, $uniform, $vertex> for $factory_name<'a> {
            fn new<'b>(display: &'b glium::Display, draw_parameters: glium::DrawParameters<'a>) -> Self {
                $factory_name {
                    program: glium::Program::from_source(display, Self::vertex_src(), Self::fragment_src(), None).unwrap(),
                    vertex_buffer: glium::VertexBuffer::new(display, Self::attributes()).unwrap(),
                    index_buffer: glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, Self::indices()).unwrap(),
                    draw_parameters: draw_parameters,
                    uniform: Vec::new(),
                }
            }

            fn spawn(&mut self, value: $uniform) {
                self.uniform.push(value);
            }

            fn draw<T>(&self, surface: &mut T) where T: glium::Surface {
                for s in self.uniform.iter() {
                    surface.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniform!{
                        $(
                            $uid: s.$uid,
                        )+
                    }, &self.draw_parameters).unwrap();
                }
            }
        }
    };
}
