use noise::{NoiseFn, Seedable};
use rand::RngCore;


fn main() {
    let make_noise = || { noise::OpenSimplex::new().set_seed(rand::thread_rng().next_u32()) };
    let noises = [make_noise(), make_noise(),make_noise(), make_noise(),make_noise(), make_noise(),];

    let f = |i| { i as f64 / 10.0 };
    for y in (-40..=40).map(f).rev() {
        for x in (-40..=40).map(f) {
            let (mut max, mut sec, mut id) = (noises[0].get([x, y]), std::f64::NEG_INFINITY, 0);
            for i in 1..noises.len() {
                let val = noises[i].get([x, y]);
                if val > max {
                    id = i;
                    sec = max;
                    max = val;
                } else if val > sec {
                    sec = val;
                }
            }
            let biome = if max > sec {
                [':','-','|','+','X','@'][id]
//                ['*','#'][id]
            } else {
                ' '
            };
            print!("{} ", biome);
        }
        println!();
    }
}