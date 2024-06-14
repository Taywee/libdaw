use crate::{sample::Sample, Node, Result};
use std::f64::consts::PI;

/// A butterworth high pass filter
#[derive(Debug)]
pub struct HighPass {
    // order / 2
    n: usize,
    a: Vec<f64>,
    d: Vec<[f64; 2]>,
    // Stream, Channel, Order
    w: Vec<Vec<Vec<[f64; 3]>>>,
}

impl HighPass {
    pub fn new(sample_rate: u32, order: usize, frequency: f64) -> Result<Self> {
        if order % 2 != 0 {
            return Err("Order must be even".into());
        }
        let n = order / 2;
        let mut self_a = vec![0.0f64; n];
        let mut d = vec![[0.0f64; 2]; n];
        let a = (PI * frequency / sample_rate as f64).tan();
        let a_2 = a.powi(2);
        for ((i, self_a), d) in (0..n).map(|i| i as f64).zip(&mut self_a).zip(&mut d) {
            let r = (PI * (2.0 * i + 1.0) / (4.0 * n as f64)).sin();
            let s = a_2 + 2.0 * a * r + 1.0;
            *self_a = 1.0 / s;
            d[0] = 2.0 * (1.0 - a_2) / s;
            d[1] = -(a_2 - 2.0 * a * r + 1.0) / s;
        }
        Ok(Self {
            n,
            a: self_a,
            d,
            w: Vec::new(),
        })
    }
}

impl Node for HighPass {
    fn process<'a, 'b, 'c>(
        &'a mut self,
        inputs: &'b [Sample],
        outputs: &'c mut Vec<Sample>,
    ) -> Result<()> {
        // First resize w
        self.w.resize_with(inputs.len(), Default::default);
        for (input, w) in inputs.into_iter().zip(&mut self.w) {
            w.resize_with(input.len(), || vec![[0.0; 3]; self.n]);
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
                    *out = a * (w[0] - 2.0 * w[1] + w[2]);
                    w.rotate_right(1);
                    w[0] = w[1];
                }
            }
            outputs.push(output);
        }
        Ok(())
    }
}
