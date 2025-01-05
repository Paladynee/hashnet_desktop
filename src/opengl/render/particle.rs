use boxarray::BoxedArray;
use voxell_rng::rng::XorShift128;

use crate::vec2::Vector2;

pub struct RenderData<const LEN: usize> {
    data: BoxedArray<LEN, Particle>,
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Particle {
    pos: Vector2,
    vel: Vector2,
    acc: Vector2,
}

impl<const LEN: usize> RenderData<LEN> {
    
    pub fn new(rng: &mut XorShift128) -> Self {
        let mut data: Vec<Particle> = Vec::with_capacity(LEN);

        for _ in 0..LEN {
            data.push(Particle {
                pos: Vector2::new(rng.next_f32(), rng.next_f32()),
                vel: Vector2::new(rng.next_f32(), rng.next_f32()),
                acc: Vector2::new(rng.next_f32(), rng.next_f32()),
            });
        }

        let bs = data.into_boxed_slice();
        let data = BoxedArray::try_from_boxed_slice(bs).unwrap();
        Self { data }
    }

    
    pub fn data(&self) -> &BoxedArray<LEN, Particle> {
        &self.data
    }

    
    pub fn data_mut(&mut self) -> &mut BoxedArray<LEN, Particle> {
        &mut self.data
    }
}
