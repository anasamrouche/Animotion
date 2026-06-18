pub(crate) mod primitives {
    #[derive(Clone)]
    pub(crate) struct AnimFloat {
        value: f32,
    }

    impl AnimFloat {
        pub fn new(value: f32) -> Self {
            Self { value }
        }
        pub fn value(&self) -> f32 {
            self.value
        }
    }

    #[derive(Clone)]
    pub(crate) struct AnimPosition {
        x: AnimFloat,
        y: AnimFloat,
        z: AnimFloat,
        w: AnimFloat,
    }

    impl AnimPosition {
        pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
            Self {
                x: AnimFloat::new(x),
                y: AnimFloat::new(y),
                z: AnimFloat::new(z),
                w: AnimFloat::new(w),
            }
        }

        pub(crate) fn to_array(&self) -> [f32; 4] {
            [
                self.x.value(),
                self.y.value(),
                self.z.value(),
                self.w.value(),
            ]
        }
    }

    #[derive(Clone)]
    pub(crate) struct AnimColor {
        r: AnimFloat,
        g: AnimFloat,
        b: AnimFloat,
        a: AnimFloat,
    }

    impl AnimColor {
        pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
            Self {
                r: AnimFloat::new(r),
                g: AnimFloat::new(g),
                b: AnimFloat::new(b),
                a: AnimFloat::new(a),
            }
        }

        pub fn from_array(arr: [f32; 4]) -> Self {
            Self::new(arr[0], arr[1], arr[2], arr[3])
        }

        pub(crate) fn to_array(&self) -> [f32; 4] {
            [
                self.r.value(),
                self.g.value(),
                self.b.value(),
                self.a.value(),
            ]
        }
    }
}

pub(crate) mod objects {
    use wgpu::PrimitiveTopology;

    use super::primitives::{AnimColor, AnimFloat, AnimPosition};
    use crate::core::AnimRender;
    use crate::core::Vertex;

    #[derive(Clone)]
    pub(crate) enum AnimObject {
        Line(Line),
    }

    #[derive(Clone)]
    pub(crate) struct Line {
        start_point: AnimPosition,
        end_point: AnimPosition,
        start_color: AnimColor,
        end_color: AnimColor,
        thickness: AnimFloat,
    }

    impl AnimRender for Line {
        const TOPOLOGY: PrimitiveTopology = PrimitiveTopology::LineList;

        fn get_vertices(&self) -> Vec<Vertex> {
            vec![
                Vertex::new(self.start_point.to_array(), self.start_color.to_array()),
                Vertex::new(self.end_point.to_array(), self.end_color.to_array()),
            ]
        }

        fn get_indices(&self) -> Vec<u32> {
            Vec::from([0, 1])
        }
    }

    impl Line {
        pub(crate) fn new(
            start_point: AnimPosition,
            end_point: AnimPosition,
            start_color: AnimColor,
            end_color: AnimColor,
            thickness: AnimFloat,
        ) -> Self {
            Self {
                start_point,
                end_point,
                start_color,
                end_color,
                thickness,
            }
        }
    }

    #[derive(Clone)]
    pub struct Scene {
        pub objects: Vec<AnimObject>,
        pub background_color: AnimColor,
    }

    impl Scene {
        pub fn new(objects: Vec<AnimObject>, background_color: AnimColor) -> Self {
            Self {
                objects,
                background_color,
            }
        }

        pub(crate) fn add_object(&mut self, object: AnimObject) {
            self.objects.push(object);
        }

        pub fn background_color_f64(&self) -> [f64; 4] {
            let color = self.background_color.to_array();
            [
                color[0] as f64,
                color[1] as f64,
                color[2] as f64,
                color[3] as f64,
            ]
        }
    }
}
