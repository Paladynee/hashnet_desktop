type Vector2Inner = f32;

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct Vector2 {
    pub x: Vector2Inner,
    pub y: Vector2Inner,
}

impl Vector2 {
    pub const fn new(x: Vector2Inner, y: Vector2Inner) -> Self {
        Self { x, y }
    }

    pub const fn set_vec(&mut self, other: &Self) {
        self.x = other.x;
        self.y = other.y;
    }

    pub const fn set(&mut self, x: Vector2Inner, y: Vector2Inner) {
        self.x = x;
        self.y = y;
    }

    pub const fn add_vec(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }

    pub const fn add(&mut self, x: Vector2Inner, y: Vector2Inner) {
        self.x += x;
        self.y += y;
    }

    pub const fn scale(&mut self, factor: Vector2Inner) {
        self.x *= factor;
        self.y *= factor;
    }

    pub const fn scale_vec(&mut self, other: &Self) {
        self.x *= other.x;
        self.y *= other.y;
    }

    pub fn normalize(&mut self) {
        let hypot = self.mag();
        if hypot == 0.0 {
            self.x = 0.0;
            self.y = 0.0;
            return;
        }
        self.x /= hypot;
        self.y /= hypot;
    }

    pub const fn mag_sq(&self) -> Vector2Inner {
        self.x * self.x + self.y * self.y
    }

    pub fn mag(&self) -> Vector2Inner {
        self.mag_sq().sqrt()
    }

    pub fn square(&mut self) {
        let mag = self.mag();
        self.normalize();
        self.scale(mag);
    }

    pub const fn to(&mut self, other: &Self) {
        self.x = other.x - self.x;
        self.y = other.y - self.y;
    }

    pub const fn from(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
    }

    pub const fn neg(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }

    pub fn one_over_d_sq(&mut self, consume_target: &Self, unit_size: &Self) {
        // self.scale_vec(unit_size);
        // let mut tmp = consume_target;
        // tmp.scale_vec(unit_size);
        // self.to(&tmp);

        self.to(consume_target);
        self.scale_vec(unit_size);

        let mag = 1.0 / (self.mag_sq() + 0.1);
        self.normalize();
        self.scale(mag);
    }

    pub fn divide(&self, amount: Vector2Inner) -> Vec<Self> {
        let mut vec_tuple_array = Vec::new();
        let mut unit = self.clone();
        unit.normalize();
        let mag = self.mag();
        unit.scale(mag);

        for i in 0..amount as i32 {
            let mut start = unit.clone();
            start.scale(i as Vector2Inner);

            let mut end = start.clone();
            end.add_vec(&unit);

            vec_tuple_array.push(start);
            vec_tuple_array.push(end);
        }

        vec_tuple_array
    }

    pub fn rotate(&mut self, angle: Vector2Inner) {
        let cos = angle.cos();
        let sin = angle.sin();

        let new_x = self.x.mul_add(cos, -(self.y * sin));
        let new_y = self.x.mul_add(sin, self.y * cos);

        self.x = new_x;
        self.y = new_y;
    }

    pub const fn arr(&self) -> [Vector2Inner; 2] {
        [self.x, self.y]
    }
}
