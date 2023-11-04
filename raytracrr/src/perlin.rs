use rand::{thread_rng, Rng};

use crate::vec::{Vec3};

const POINT_COUNT: usize = 256;
pub struct Perlin {
    ran_dbl: [f64; POINT_COUNT], // not used
    ran_vec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT]
}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin { 
            ran_dbl: random_double(),
            ran_vec: rand_fill(), 
            perm_x: generate_perlin_permute(), 
            perm_y: generate_perlin_permute(), 
            perm_z: generate_perlin_permute() 
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        const MASK: usize = POINT_COUNT - 1;
        let Vec3{e: [px, py, pz]} = p;

        let u = px - px.floor();
        let v = py - py.floor();
        let w = pz - pz.floor();

        let i = px.floor() as i32;
        let j = py.floor() as i32;
        let k = pz.floor() as i32;

        // let i: usize = ((4 * i) as usize & MASK) as usize;
        // let j: usize = ((4 * j) as usize & MASK) as usize;
        // let k: usize = ((4 * k) as usize & MASK) as usize;

        let mut c = [[[Default::default(); 2]; 2]; 2];

        for di in 0..2 {
            let iterm = (i + di) as usize & MASK;
            let xterm = self.perm_x[iterm];
            let di = di as usize;

            for dj in 0..2 {
                let jterm = (j + dj) as usize & MASK;
                let yterm = self.perm_y[jterm];
                let dj = dj as usize;

                for dk in 0..2 {
                    let kterm = (k + dk) as usize & MASK;
                    let zterm = self.perm_z[kterm];
                    let dk = dk as usize;

                    let index = xterm ^ yterm ^ zterm;
                    c[di][dj][dk] = self.ran_vec[index];
                }
            }
        }

        perlin_interpolation(&c, u, v, w)
    }

    // a composite noise that has multiple summed frequencies. 
    // usually called turbulence, and is a sum of repeated calls to noise:

    // make color proportional to something like a sine function, 
    // and use turbulence to adjust the phase (so it shifts x in sin(x)) which makes the stripes undulate.
    pub fn turb(&self, p: Vec3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

fn generate_perlin_permute() -> [usize; POINT_COUNT] {
    let mut p = [0; POINT_COUNT]; 
    for (i, p) in p.iter_mut().enumerate() {
        *p = i;
    }
    for i in (0..p.len()).rev() {
        let target = thread_rng().gen_range(0..=i);
        p.swap(i, target)
    }
    p
}

fn random_double() -> [f64; POINT_COUNT] {
    let mut rand_dbl = [0.0; POINT_COUNT];
    for i in (0..rand_dbl.len()) {
        rand_dbl[i] = thread_rng().gen_range(0.0..=1.0);
    }
    rand_dbl
}

/**
 * generates a random array of vectors
 */
fn rand_fill() -> [Vec3; POINT_COUNT] {
    let mut rand_vec = [Default::default(); POINT_COUNT];
    for i in &mut rand_vec {
        *i = Vec3::random(-1.0..1.0).normalized();
    }
    rand_vec
}

/**
 * trilinear interpolation
 * TODO: explore
 */
fn perlin_interpolation(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    #![allow(clippy::many_single_char_names)]

    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut accum = 0.0;

    let mut i = 0.0;
    for c in c {
        let iterm = i * uu + (1.0 - i) * (1.0 - uu);
        let iweight = u - i;

        let mut j = 0.0;
        for c in c {
            let jterm = j * vv + (1.0 - j) * (1.0 - vv);
            let jweight = v - j;

            let mut k = 0.0;
            for c in c {
                let kterm = k * ww + (1.0 - k) * (1.0 - ww);
                let kweight = w - k;

                let weight_v = Vec3::new(iweight, jweight, kweight);
                accum += iterm * jterm * kterm * c.dot(weight_v);

                k += 1.0;
            }
            j += 1.0;
        }
        i += 1.0;
    }
    accum
}
