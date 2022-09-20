#[derive(PartialEq)]
#[derive(Clone, Copy)]
enum Bit {
    Zero = 0,
    One = 1,
}
fn bit_to_int(bit: Bit) -> u8 {
    if bit == One { return 1 }
    0
}
fn bits_to_ints(bits: Vec<Bit>) -> Vec<u8> {
    let mut ints = vec![];

    bits
        .iter()
        .for_each(|bit| ints.push(bit_to_int(*bit)));

    ints
}
fn int_to_bit(int: u8) -> Bit {
    if int == 1 { return One }
    Zero
}
fn ints_to_bits(ints: Vec<u8>) -> Vec<Bit> {
    let mut bits = vec![];

    ints
        .iter()
        .for_each(|int| bits.push(int_to_bit(*int)));

    bits
}
use Bit::*;

//
// Logic Gates
//

fn and(a: Bit, b: Bit) -> Bit {
    if a == One && b == One { return One }
    Zero
}

fn not(a: Bit) -> Bit {
    if a == Zero { return One }
    Zero
}

// Past these there should be no using '&&' or '=='

fn nand(a: Bit, b: Bit) -> Bit {
    not(and(a, b))
}

fn or(a: Bit, b: Bit) -> Bit {
    nand(not(a), not(b))
}

fn xor(a: Bit, b: Bit) -> Bit {
    and(or(a, b), nand(b, a))
}

fn nor(a: Bit, b: Bit) -> Bit {
    not(or(a, b))
}

//
// Components
//

// I am sure there are multiple, probably simple, ways to optimize this and make it look nicer
// but for now I want to spend as little time as possible on it and just get it working
struct AdderResult {
    sum: Bit,
    carry_out: Bit,
}
fn adder(a: Bit, b: Bit, carry_in: Bit) -> AdderResult {
    let d = xor(a, b);
    let e = and(b, a);
    let sum = xor(d, carry_in);
    let g = and(d, carry_in);
    let carry_out = or(g, e);
    
    AdderResult {
        sum,
        carry_out,
    }
}

// so I will have a function like
// fn create_adder(bits, inputs: { a, b, carry_in }) {}

struct FourBitAdderResult {
    sum: [Bit; 4],
    carry_out: Bit,
}
fn four_bit_adder(a: [Bit; 4], b: [Bit; 4], carry_in: Bit) -> FourBitAdderResult {
    let adder_a_result = adder(a[3], b[3], carry_in);
    let adder_b_result = adder(a[2], b[2], adder_a_result.carry_out);
    let adder_c_result = adder(a[1], b[1], adder_b_result.carry_out);
    let adder_d_result = adder(a[0], b[0], adder_c_result.carry_out);

    FourBitAdderResult {
        sum: [
            adder_d_result.sum,
            adder_c_result.sum,
            adder_b_result.sum,
            adder_a_result.sum,
        ],
        carry_out: adder_d_result.carry_out,
    }
}

struct SixteenBitAdderResult {
    sum: [Bit; 16],
    carry_out: Bit,
}
fn sixteen_bit_adder(a: [Bit; BITS], b: [Bit; BITS], carry_in: Bit) -> SixteenBitAdderResult {
    let mut latest_result = adder(a[0], b[0], carry_in);
    let mut sum = [latest_result.sum; BITS];
    
    let mut bit = 1;
    while bit < BITS {
        latest_result = adder(a[bit], b[bit], latest_result.carry_out);
        sum[bit] = latest_result.sum;

        bit += 1;
    };

    SixteenBitAdderResult {
        sum,
        carry_out: latest_result.carry_out,
    }
}

//
// Abstracted ==========================================================================================================================================================
// 

const BITS: usize = 16;
struct ALU {}
struct ALUFlags {
    zero: Bit,
    carry: Bit,
    negative: Bit,
}
struct ALUProcessResult {
    flags: ALUFlags,
    sum: [Bit; BITS],
}
impl ALU {
    fn new() -> ALU {
        println!("{}-Bit ALU Created", BITS);

        ALU {}
    }

    fn process(a: [Bit; BITS], b: [Bit; BITS], subtract: Bit) -> ALUProcessResult {
        ALUProcessResult {
            flags: ALUFlags {
                zero: Zero,
                carry: Zero,
                negative: Zero,
            },
            sum: [Zero; BITS],
        } 
    }
}
struct CPU {
    alu: ALU,
}
struct Memory {
    y: i32,
}
pub struct VirtualComputer {
    cpu: CPU,
    memory: Memory,
}
impl VirtualComputer {
    // Before I get started I want to mention that a lot of what I use here I learnt from
    // ben eater's [8-Bit Breadboard Computer series](https://www.youtube.com/playlist?list=PLowKtXNTBypGqImE405J2565dvjafglHU)
    // and sebastian lague's [Exploring how computers work](https://www.youtube.com/watch?v=QZwneRb-zqA) videos.

    // So a computer consists of a few main parts, which are made of smaller parts until we get to transistors.
    // While I would love to create a virtual computer completely from scratch (literally using logic gate functions like and(a, b)), that's not the aim of this project (maybe in the future).
    // So for now I'll stick to a level of abstraction that is still relatively complex (to give me maximum control) but also quite simple to not waste much time.

    pub fn new() -> VirtualComputer {
        println!("Creating a {}-bit virtual computer...", BITS);

        let mut number_a_bits = [Zero; BITS];
        let mut number_b_bits = [Zero; BITS];

        let mut i = 0;
        while i < BITS {
            number_a_bits[i] = ints_to_bits(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0])[i];
            number_b_bits[i] = ints_to_bits(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1])[i];

            i += 1;
        }
        
        let seventy_three = sixteen_bit_adder(
            number_a_bits,
            number_b_bits,
            Zero,
        );

        let mut bit = 0;
        while bit < BITS {
            print!("{}", bit_to_int(seventy_three.sum[bit]));
            bit += 1;
        };

        print!(" {}\n", bit_to_int(seventy_three.carry_out));

        VirtualComputer {
            cpu: CPU {
                alu: ALU::new(),
            },
            memory: Memory {
                y: 0
            },
        }
    }

    // The few main components of a computer (or at least the ones I want to build) are:

    // Memory (RAM, ROM, Registers, Cache)

    // CPU - Control Unit, ALU, Cache

    // The Arithmetic and Logic Unit (ALU) is where arithmetic and logic operations are carried out (obviously -_-).
    // It normally takes in two (binary) numbers as well as a code to determine the operation (op code) 
    // and outputs a result as well as some flags (info about the operation) such as whether the result was 0 or if it was negative.
    // So a basic ALU can add and subtract numbers which is enough to also perform operations like division, multiplication, exponentiation and more.
    // The ALU consists of an adder as well as some logic gates and other small components used for certain operations like subtraction (check out two's complement) and zero checking.

    // Bus
    // The bus is where all inputs and outputs travel between the CPU and memory
}