use crate::{sample::Sample, Node, Result};
use std::f64::consts::PI;

/// A chebyshev low pass filter
#[derive(Debug)]
pub struct LowPass {
    m: usize,
    ep: f64,
    a: Vec<f64>,
    d: Vec<[f64; 2]>,
    // Stream, Channel, Order
    w: Vec<Vec<Vec<[f64; 3]>>>,
}

impl LowPass {
    pub fn new(sample_rate: u32, n: usize, epsilon: f64, frequency: f64) -> Result<Self> {
        if n % 2 != 0 {
            return Err("n must be even".into());
        }
        let m = n / 2;
        let mut self_a = vec![0.0f64; m];
        let mut d = vec![[0.0f64; 2]; m];
        let a = (PI * frequency / sample_rate as f64).tan();
        let a_2 = a.powi(2);
        let u = ((1.0 + (1.0 + epsilon.powi(2)).sqrt()) / epsilon).ln();
        let su = (u / n as f64).sinh();
        let cu = (u / n as f64).cosh();
        for ((i, self_a), d) in (0..m).map(|i| i as f64).zip(&mut self_a).zip(&mut d) {
            let bc_base = PI * (2.0 * i + 1.0) / (2.0 * n as f64);
            let b = bc_base.sin() * su;
            let c = bc_base.cos() * cu;
            let c = b.powi(2) + c.powi(2);
            let s = a_2 * c + 2.0 * a * b + 1.0;
            *self_a = a_2 / (4.0 * s);
            d[0] = 2.0 * (1.0 - a_2 * c) / s;
            d[1] = -(a_2 * c - 2.0 * a * b + 1.0) / s;
        }
        let ep = 2.0 / epsilon;
        Ok(Self {
            m,
            ep,
            a: self_a,
            d,
            w: Vec::new(),
        })
    }
}

impl Node for LowPass {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        // First resize w
        self.w.resize_with(inputs.len(), Default::default);
        for (input, w) in inputs.into_iter().zip(&mut self.w) {
            w.resize_with(input.len(), || vec![[0.0; 3]; self.m]);
        }

        for (input, w) in inputs.into_iter().zip(&mut self.w) {
            let mut output = Sample::zeroed(input.len());
            for ((value, out), w) in input.iter().copied().zip(&mut output).zip(w) {
                *out = value;
                for ((w, d), a) in w
                    .into_iter()
                    .zip(self.d.iter().copied())
                    .zip(self.a.iter().copied())
                {
                    w[0] = d
                        .iter()
                        .copied()
                        .zip(w.iter().skip(1).copied())
                        .map(|(d, w)| d * w)
                        .sum::<f64>()
                        + *out;
                    *out = a * (w[0] + 2.0 * w[1] + w[2]);
                    w.rotate_right(1);
                    w[0] = w[1];
                }
            }
            outputs.push(output * self.ep);
        }
        Ok(())
    }
}
