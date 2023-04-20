use crate::prelude::*;

#[derive(Clone)]
pub struct Value {
    id                              : Uuid,
}

impl Pattern for Value {

    fn new() -> Self {

        Self {
            id                      : Uuid::new_v4(),
        }
    }

    fn id(&self) -> Uuid { self.id }
    fn name(&self) -> String {"Value".to_string()}

    fn get_color(&self, uv: Vec2f) -> u8 {

        let x = uv * 8.0;
        let i = floor(x);
        let f = frac(x);

	    let a = self.hash21(i);
        let b = self.hash21(i + vec2f(1.0, 0.0));
        let c = self.hash21(i + vec2f(0.0, 1.0));
        let d = self.hash21(i + vec2f(1.0, 1.0));

        let u = vec2f( f.x * f.x * (3.0 - 2.0 * f.x), f.y * f.y * (3.0 - 2.0 * f.y));

        let xx = self.mix(&a, &b, &u.x);
        let yy = self.mix(&c, &d, &u.x);

        let m = self.mix(&xx, &yy, &u.y);

        if m < 0.5 {
            0
        } else {
            1
        }

        //(m.clamp(0.0, 1.0), self.hash21(glm::floor(&u)))
    }

    fn create(&self) -> Box::<dyn Pattern> {
        Box::new(Value::new())
    }
}