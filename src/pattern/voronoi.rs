use crate::prelude::*;

#[derive(Clone)]
pub struct Voronoi {
    id                              : Uuid,
}

impl Pattern for Voronoi {

    fn new() -> Self {

        Self {
            id                      : Uuid::new_v4(),
        }
    }

    fn id(&self) -> Uuid { self.id }
    fn name(&self) -> String {"Voronoi".to_string()}

    fn get_color(&self, uv: Vec2f) -> u8 {

        /// 2D hash, 2 out
        fn hash22(p: Vec2f ) -> Vec2f {
            let pp = vec2f(dot(p, vec2f(127.1, 311.7)),
                    dot(p, vec2f(269.5,183.3)));
            frac(sin(pp) * vec2f(18.5453, 18.5453 ))
        }

        let x = uv * 8.0;////GF2::new(p.0, p.1);
        let n = floor(x);
        let f = frac(x);

        let mut m = vec3f( 8.0, 8.0, 8.0 );
        for j in -1..=1 {
            for i in -1..=1 {
                let  g = vec2f( i as f32, j as f32 );
                let  o = hash22( n + g );

                let r = g - f + (vec2f(0.5, 0.5) + 0.5 * sin(6.2831*o));
                let d = dot( r, r );
                if d < m.x {
                    m = vec3f( d, o.x, o.y );
                }
            }
        }

        if 0.5 + 0.5 * (m.y + m.z) < 0.5 {
            0
        } else {
            1
        }
    }

    fn create(&self) -> Box::<dyn Pattern> {
        Box::new(Voronoi::new())
    }
}