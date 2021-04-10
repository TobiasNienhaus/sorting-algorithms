
pub fn base256(input: &[u64]) -> Vec<u64> {
    let mut output = vec![0; input.len()];
    let mut buffer = Vec::from(input);
    let mut count = [0usize; 256];
    for bit in (0..64usize).step_by(8) {
        count.iter_mut().for_each(|v| *v = 0);

        for num in buffer.iter() {
            count[((num >> bit) & 255u64) as usize] += 1;
        }

        for i in 1..count.len() {
            count[i] += count[i - 1];
        }

        for num in buffer.iter().rev() {
            let idx = ((num >> bit) & 255u64) as usize;
            count[idx] -= 1;
            output[count[idx]] = *num;
        }
        // Swap buffer and output
        // Should work, because we have an even number of iterations
        std::mem::swap(&mut output, &mut buffer);
    }
    output
}

pub fn base2(input: &[u64]) -> Vec<u64> {
    let mut output = vec![0; input.len()];
    let mut buffer = Vec::from(input);
    for bit in 0..64usize {
        let mut count = [0usize; 2];

        for num in buffer.iter() {
            count[((num >> bit) & 1) as usize] += 1;
        }

        for i in 1..count.len() {
            count[i] += count[i - 1];
        }

        for num in buffer.iter().rev() {
            let idx = ((num >> bit) & 1) as usize;
            count[idx] -= 1;
            output[count[idx]] = *num;
        }
        // Swap buffer and output
        // Should work, because we have an even number of iterations
        std::mem::swap(&mut output, &mut buffer);
    }
    output
}
