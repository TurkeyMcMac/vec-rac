use crate::racetrack::Racetrack;
use crate::rng::Rng;
use crate::vector::Vector;
use std::cmp;
use std::convert::TryInto;
use std::iter;

// XXX: Change repeat_array when you change this.
const N_MID_WEIGHTS: usize = 32;

#[derive(Clone, PartialEq, PartialOrd)]
pub struct Brain {
    view_dist: i32,
    mid_weights: [Vec<f32>; N_MID_WEIGHTS],
    out_weights: [[f32; N_MID_WEIGHTS]; 4],
}

impl Brain {
    pub const VERSION: u32 = 1;
    pub fn random(view_dist: i32, rng: &mut Rng) -> Brain {
        let view_dist = pos_round_up_2(i32::abs(view_dist));
        let n_inputs = ((view_dist / 2) * (view_dist / 2) * 4) as usize - 2;
        Brain {
            view_dist,
            mid_weights: repeat_array(|| {
                iter::repeat_with(|| random_f32(rng))
                    .take(n_inputs)
                    .collect()
            }),
            out_weights: [
                repeat_array(|| random_f32(rng)),
                repeat_array(|| random_f32(rng)),
                repeat_array(|| random_f32(rng)),
                repeat_array(|| random_f32(rng)),
            ],
        }
    }

    pub fn mutant(&self, rng: &mut Rng, amount: f64) -> Brain {
        let amount = amount as f32;
        let mut mutant = self.clone();
        for neuron in mutant.mid_weights.iter_mut() {
            for weight in neuron {
                *weight += random_f32(rng) * amount;
            }
        }
        for neuron in mutant.out_weights.iter_mut() {
            for weight in neuron {
                *weight += random_f32(rng) * amount;
            }
        }
        mutant
    }

    pub fn compute_accel(&self, vel: Vector, track: &Racetrack) -> Vector {
        let mut mid_iter = self.mid_weights.iter();
        let mid_out = repeat_array(|| {
            if let Some(neuron) = mid_iter.next() {
                let quarter = (neuron.len() - 2) / 4;
                let mut sum = 0.0;
                let mut i = 0;
                for x in (1..self.view_dist).step_by(2) {
                    for y in (1..self.view_dist).step_by(2) {
                        if let Some(true) = track.get(Vector::new(x, y)) {
                            sum += neuron[i];
                        }
                        if let Some(true) = track.get(Vector::new(-x, y)) {
                            sum += neuron[i + 1 * quarter];
                        }
                        if let Some(true) = track.get(Vector::new(-x, -y)) {
                            sum += neuron[i + 2 * quarter];
                        }
                        if let Some(true) = track.get(Vector::new(x, -y)) {
                            sum += neuron[i + 3 * quarter];
                        }
                        i += 1;
                    }
                }
                sum += vel.x as f32 * neuron[neuron.len() - 2];
                sum += vel.y as f32 * neuron[neuron.len() - 1];
                sum
            } else {
                unreachable!()
            }
        });
        assert!(mid_iter.next().is_none());
        let mut out = vec![
            compute_out(&self.out_weights[0], &mid_out),
            compute_out(&self.out_weights[1], &mid_out),
            compute_out(&self.out_weights[2], &mid_out),
            compute_out(&self.out_weights[3], &mid_out),
        ]
        .into_iter();
        let mut max = out.next().unwrap();
        let mut max_i = 0;
        for (i, choice) in out.enumerate() {
            if choice > max {
                max = choice;
                max_i = i;
            }
        }
        match max_i {
            0 => Vector::new(1, 0),
            1 => Vector::new(0, 1),
            2 => Vector::new(-1, 0),
            3 => Vector::new(0, -1),
            _ => unreachable!(),
        }
    }
}

fn pos_round_up_2(num: i32) -> i32 {
    (num + 1) & !1
}

fn random_f32(rng: &mut Rng) -> f32 {
    rng.forward() as f32 / Rng::RAND_MAX as f32 * 2.0 - 1.0
}

fn repeat_array<T, F: FnMut() -> T>(mut f: F) -> [T; N_MID_WEIGHTS] {
    // TODO: This is dumb.
    [
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
        f(),
    ]
}

fn compute_out(neuron: &[f32; N_MID_WEIGHTS], inputs: &[f32; N_MID_WEIGHTS]) -> f32 {
    neuron.into_iter().zip(inputs).map(|(&w, &i)| w * i).sum()
}
