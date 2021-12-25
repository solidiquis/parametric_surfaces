use std::f32::consts::PI;

const PI2: f32 = PI * 2.0;
const STEP: f32 = PI * (5.0 / 180.0);
const COLOR_STEP: f32 = 0.5;
const R1: f32 = 0.5;
const R2: f32 = 0.2;

pub fn compute_vertices() -> (Vec<f32>, Vec<f32>, i32)  {
    let sin = |rads: f32| rads.sin();
    let cos = |rads: f32| rads.cos();

    // Parametric equations:
    let fx = |u: f32, v: f32| (R1 + R2 * cos(v)) * cos(u);
    let fy = |u: f32, v: f32| (R1 + R2 * cos(v)) * sin(u);
    let fz = |v: f32| R2 * sin(v);

    let mut positions = vec![];
    let mut colors = vec![];
    let mut v = 0.0;

    while v <= PI2 {
        let mut u = 0.0;

        while u <= PI2 {
            let x = fx(u, v);
            let y = fy(u, v);
            let z = fz(v);

            positions.push(x);
            positions.push(y);
            positions.push(z);

            colors.push(x + COLOR_STEP);
            colors.push(y + COLOR_STEP);
            colors.push(z + COLOR_STEP);

            u += STEP;
        }
        v += STEP;
    }

    let indices_count = (positions.len() as f32) / 3.0;

    (positions, colors, indices_count as i32)
}
