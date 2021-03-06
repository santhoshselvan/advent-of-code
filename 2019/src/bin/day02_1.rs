

pub fn main() {
    //....................../´¯/)
    //....................,/¯../
    //.................../..../
    //............./´¯/'...'/´¯¯`·¸
    //........../'/.../..../......./¨¯\
    //........('(...´...´.... ¯~/'...')
    //.........\.................'...../
    //..........''...\.......... _.·´
    //............\..............(
    //..............\.............\...
    //................To Rust.........
    //...for its annoying restrictions and being right...

    // Rust: Even though borrowing errors may be frustrating at times,
    // remember that it’s the Rust compiler pointing out a potential
    // bug early (at compile time rather than at runtime) and
    // showing you exactly where the problem is.
    // Then you don’t have to track down why your data isn’t what you thought it was.

    let input = include_str!("../../data/two.data");
    let codes = intcode(input, true);

    println!("Output of Intcode: {}", codes.join(","))
}

fn intcode(input: &str, restore_gravity: bool) -> Vec<String> {
    let mut memory: Vec<String> = input.split(',').map(|s| s.to_string()).collect();
    if restore_gravity {
        memory[1] = String::from("12");
        memory[2] = String::from("2");
    }
    let mut current_opcode_position = 0;
    while (current_opcode_position < memory.len() - 1)
        && memory[current_opcode_position] != String::from("99") {
        let current_opcode = &memory[current_opcode_position];
        let input_position_1 = memory[current_opcode_position + 1].parse::<usize>().unwrap();
        let input_position_2 = memory[current_opcode_position + 2].parse::<usize>().unwrap();
        let output_position = memory[current_opcode_position + 3].parse::<usize>().unwrap();

        let input1 = memory[input_position_1].parse::<i32>().unwrap();
        let input2 = memory[input_position_2].parse::<i32>().unwrap();
        match current_opcode.as_str() {
            "1" => {
                let x = (input1 + input2).to_string();
                memory[output_position] = x
            }
            "2" => {
                let x1 = (input1 * input2).to_string();
                memory[output_position] = x1
            }
            _ => {}
        }

        current_opcode_position += 4;
    }
    memory
}

#[cfg(test)]
mod tests {
    use crate::intcode;

    #[test]
    fn it_works() {
        assert_eq!(intcode(String::from("2,4,3,4,33").as_str(), false).join(","), "2,4,3,4,132");
        assert_eq!(intcode(String::from("1,0,0,0,99").as_str(), false).join(","), "2,0,0,0,99");
        assert_eq!(intcode(String::from("2,3,0,3,99").as_str(), false).join(","), "2,3,0,6,99");
        assert_eq!(intcode(String::from("2,4,4,5,99,0").as_str(), false).join(","), "2,4,4,5,99,9801");
        assert_eq!(intcode(String::from("1,1,1,4,99,5,6,0,99").as_str(), false).join(","), "30,1,1,4,2,5,6,0,99");
    }
}