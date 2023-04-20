use crate::prelude::*;

#[derive(Clone)]
pub struct Brick {
    id                              : Uuid,

    ratio                           : f32,
    brick                           : f32,
    cell                            : f32,
    gap                             : f32,
    bevel                           : f32,
    round                           : f32,
}

impl Pattern for Brick {

    fn new() -> Self {

        Self {
            id                      : Uuid::new_v4(),

            ratio                   : 2.0,
            brick                   : 1.0,
            cell                    : 16.0,
            gap                     : 0.08,
            bevel                   : 0.07,
            round                   : 0.25,
        }
    }

    fn id(&self) -> Uuid { self.id }
    fn name(&self) -> String {"Brick".to_string()}

    fn get_color(&self, uv: Vec2f) -> u8 {

        let mut u = uv / 2.0 + vec2f(10000.0, 10000.0);

        let bevel = vec2f(self.bevel, self.bevel);
        let gap = vec2f(self.gap, self.gap);
        let round: f32 = self.round;

        let w = vec2f(self.ratio,1.0);
        u = u * vec2f(self.cell, self.cell) / w;

        if self.brick == 1.0 {
            u.x += 0.5 * u.y.floor() % 2.0;
        }

        let t = frac(u) - vec2f(1.0, 1.0) / 2.0;
        let s = w * t;

        let a = w / 2.0 - gap - abs(s);
        let b = a * vec2f(2.0, 2.0) / bevel;
        let mut m = b.x.min(b.y);
        if a.x < round && a.y < round {
           m = (round - length(vec2f(round, round) - a)) * 2.0 / dot(bevel,normalize(vec2f(round, round) - a));
        }

        if m < 0.5 {
            0
        } else {
            1
        }

        //(m.clamp(0.0, 1.0), self.hash21(glm::floor(&u)))
    }

    fn create(&self) -> Box::<dyn Pattern> {
        Box::new(Brick::new())
    }
}