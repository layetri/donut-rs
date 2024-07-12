use crate::dsp::buffer::Buffer;

pub struct AddAndDivide {
    // pub hpf: HighPassFilter,
    // pub volumes: HashSet<String, Arc<Parameter>>
}

impl AddAndDivide {
    pub fn new() -> Self {
        Self {
            // hpf: HighPassFilter::new(),
            // volumes: HashSet::<String, Arc<Parameter>>::new()
        }
    }

    pub fn process(&mut self, inputs: &Vec<Buffer>, block_size: usize, div: f32) -> Buffer {
        let mut output = Buffer::new(block_size, "AddAndDivide".to_string());

        let mult = (0.5 + 0.4 * (1.0 / div).sqrt()).min(1.0);

        for i in 0..block_size {
            let mut sum = 0.0;
            for input in inputs {
                sum += input[i];
            }

            sum *= mult * 0.6;

            sum = if sum >= 1.0 {
                0.666_666_7
            } else if sum <= -1.0 {
                -0.666_666_7
            } else {
                sum - (sum.powf(3.0) / 3.0)
            };

            output.write(sum);
            output.tick();
        }

        // self.hpf.process(&mut output);
        output
    }
}