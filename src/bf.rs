extern crate tty_read;

use std::cmp::{max, min};
use std::u8;

use self::tty_read::TermReader;

/// The size of the memory.
const MEM_SIZE: usize = 30_000;

/// Interpret a Brainfuck program from a string.
/// Return the result string.
pub fn bf(prog: &str) -> String {
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
            b'>' => mem_ptr = min(mem_ptr + 1, MEM_SIZE - 1),
            b'<' => mem_ptr = max(mem_ptr - 1, 0),
            b'+' => mem[mem_ptr] += 1,
            b'-' => mem[mem_ptr] = max(mem[mem_ptr] - 1, 0),
            b'.' => out.push(mem[mem_ptr]),
            b',' => mem[mem_ptr] =
                    TermReader::open_stdin()
                        .expect("failed to open user input reader")
                        .read_byte()
                        .expect("failed to read user input"),
            b'[' => if mem[mem_ptr] == 0 {
                        seek_matching_bracket(&prog, &mut pc);
                    } else {
                        // Remember where the loop starts
                        stack.push(pc);
                    },
            b']' => if mem[mem_ptr] == 0 {
                        stack.pop().expect("Malformed program");
                    } else {
                        // Re-loop
                        pc = *stack.last().expect("Malformed program");
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
#[inline]
fn seek_matching_bracket(prog: &Vec<u8>, pc: &mut usize) {
    // Define the balance
    let mut balance = 1;

    // Seek until we regained zero balance
    while balance > 0 && *pc < prog.len() {
        // Increase the program counter
        *pc += 1;

        // Keep track of the balance
        match prog[*pc] {
            b'[' => balance += 1,
            b']' => balance -= 1,
            _ => {},
        }
    }
}
