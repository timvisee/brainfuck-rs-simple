/// The size of the memory.
const MEM_SIZE: usize = 30_000;

/// Interpret a Brainfuck program from a string.
/// Return the result string.
fn bf(prog: &str) -> String {
    // Allocate application memory
    let mut mem = [0u8; MEM_SIZE];

    // Define a memory pointer and program counter
    let mut mem_ptr = 0;
    let mut pc = 0;

    // Get program bytes and define a stack for othe program counter in loops
    let prog: Vec<u8> = prog.bytes().collect();
    let mut stack = vec![];

    // Output
    let mut out: Vec<u8> = vec![];

    // Execute
    while pc < prog.len() {
        match prog[pc] {
            b'>' => mem_ptr += 1,
            b'<' => mem_ptr -= 1,
            b'+' => mem[mem_ptr] += 1,
            b'-' => mem[mem_ptr] -= 1,
            b'.' => out.push(mem[mem_ptr]),
            b',' => panic!("Not yet implemented!"),
            b'[' => if mem[mem_ptr] == 0 {
                        seek_matching_bracket(&prog, &mut pc);
                    } else {
                        // Remember the beginning of the loop for jumping
                        stack.push(pc);
                    },
            b']' => if mem[mem_ptr] == 0 {
                        stack.pop();
                    } else {
                        // Jump to the beginning of the loop
                        pc = stack.pop().expect("Malformed program") - 1;
                    },
            _ => {},
        }

        // Increase the program counter
        pc += 1;
    }

    // Parse and output the string
    String::from_utf8(out).unwrap()
}

/// Seek the program counter until a matching bracket for the current one.
fn seek_matching_bracket(prog: &Vec<u8>, pc: &mut usize) {
    // Define the balance, skip one position
    let mut balance = 1;
    *pc += 1;

    // Seek until we regained zero balance
    while balance > 0 {
        // Keep track of the balance
        match prog[*pc] {
            b'[' => balance += 1,
            b']' => balance -= 1,
            _ => {},
        }

        // Increase the program counter
        *pc += 1;
    }
}



#[test]
fn test_hello_world() {
    assert_eq!(
        bf("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."),
        "Hello World!\n",
    );
}
