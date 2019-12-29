use std::io;

fn compute(orig_tokens: &Vec<u32>) -> Vec<u32> {
    let mut tokens = orig_tokens.to_vec();
    let mut current_pos = 0;

    loop {
        let opcode = tokens[current_pos];
        let result = match opcode {
            99 => break,
            1 => tokens[tokens[current_pos + 1] as usize] + tokens[tokens[current_pos + 2] as usize],
            2 => tokens[tokens[current_pos + 1] as usize] * tokens[tokens[current_pos + 2] as usize],
            _ => panic!("Received an unknown opcode: {}", opcode)
        };
        // Could not do tokens[tokens[current_pos + 3] as usize] here due to mixing mutable and immutable 
        //  borrows in a single statement
        let update_pos = tokens[current_pos + 3] as usize;
        tokens[update_pos] = result;
        current_pos += 4;
    }

    tokens
}

fn main() -> io::Result<()> {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("could not read line from stdin");
    let mut tokens: Vec<u32> = String::from(input_line.trim()).split(',')
        .map(|t| t.parse::<u32>().expect("could not parse token as u32"))
        .collect();

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            tokens[1] = noun;
            tokens[2] = verb;
            let new_tokens = compute(&tokens);
            if new_tokens[0] == 19690720 {
                println!("Noun: {}\nVerb: {}\nPuzzle Answer: {}", noun, verb, 100 * noun + verb);
                break 'outer;
            }
        }
    }

    Ok(())
}
