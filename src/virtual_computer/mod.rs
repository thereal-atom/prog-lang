#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum Bit {
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
fn half_adder(a: Bit, b: Bit) -> AdderResult {
    AdderResult {
        sum: xor(a, b),
        carry_out: and(a, b),
    }
}
fn adder(a: Bit, b: Bit, carry_in: Bit) -> AdderResult {
    let result1 = half_adder(a, b);
    let result2 = half_adder(result1.sum, carry_in);
    
    AdderResult {
        sum: result2.sum,
        carry_out: or(result1.carry_out, result2.carry_out),
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
    let mut sum = [Zero; BITS];
    let mut latest_result = adder(a[15], b[15], carry_in);

    let mut bit = BITS - 2;
    while bit > 0 {
        latest_result = adder(a[bit], b[bit], latest_result.carry_out);
        sum[bit] = latest_result.sum;

        bit -= 1;
    }

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

    pub fn process(a: [Bit; BITS], b: [Bit; BITS], subtract: Bit) -> ALUProcessResult {
        let mut b_bits = b;

        let mut b_bit = 0;
        while b_bit < BITS - 1 {
            b_bits[b_bit] = xor(b_bits[b_bit], subtract);

            b_bit += 1;
        }

        let adder_result = sixteen_bit_adder(a, b_bits, subtract);

        let mut is_zero = Zero;

        // this would normally use a bunch of 'not' and 'and' gates
        // but I'm too lazy for that so I'll just use a loop like this

        let mut sum_bit = 0;
        while sum_bit < BITS - 1 {
            if adder_result.sum[sum_bit] == One {
                is_zero = One;
            }

            sum_bit += 1;
        }

        ALUProcessResult {
            flags: ALUFlags {
                zero: is_zero,
                carry: adder_result.carry_out,
                negative: adder_result.sum[0],
            },
            sum: adder_result.sum,
        } 
    }
}
struct CPU {
    pub alu: ALU,
}
struct Memory {
    y: i32,
}
pub struct VirtualComputer {
    pub cpu: CPU,
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

        // let mut number_a_bits = [Zero; BITS];
        // let mut number_b_bits = [Zero; BITS];

        // let mut i = 0;
        // while i < BITS {
        //     number_a_bits[i] = ints_to_bits(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0])[i];
        //     number_b_bits[i] = ints_to_bits(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1])[i];

        //     i += 1;
        // }
        
        // let seventy_three = sixteen_bit_adder(
        //     [Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, One, Zero, Zero, Zero, Zero, Zero],
        //     [Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, One, Zero, One, Zero, Zero, One],
        //     Zero,
        // );

        // let seventy_three = sixteen_bit_adder(
        //     number_a_bits,
        //     number_b_bits,
        //     Zero,
        // );

        // let sixteen = four_bit_adder(
        //     [One, One, Zero, Zero],
        //     [One, Zero, One, One],
        //     Zero
        // );

        // print!("Result:  ");
        // print!(" {} ", bit_to_int(seventy_three.carry_out));

        // let mut bit = 0;
        // while bit < BITS {
        //     print!("{}", bit_to_int(seventy_three.sum[bit]));
        //     bit += 1;
        // };

        // println!("\nExpected: 0 0000000001001001");

        // test_adders();

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

// Tests ==========================================================================================================================================================

fn test_adders () {
    let sum0 = adder(Zero, Zero, Zero);
    let sum1 = adder(Zero, Zero, One);
    let sum2 = adder(Zero, One, Zero);
    let sum3 = adder(Zero, One, One);
    let sum4 = adder(One, Zero, Zero);
    let sum5 = adder(One, Zero, One);
    let sum6 = adder(One, One, Zero);
    let sum7 = adder(One, One, One);

    println!("Sum | Carry | Expected Sum | Expected Carry | Expected Decimal");
    println!("--------------------------------------------------------------");
    println!("{}   | {}     | 0            | 0              | 0               ", bit_to_int(sum0.sum), bit_to_int(sum0.carry_out));
    println!("{}   | {}     | 1            | 0              | 1               ", bit_to_int(sum1.sum), bit_to_int(sum1.carry_out));
    println!("{}   | {}     | 1            | 0              | 1               ", bit_to_int(sum2.sum), bit_to_int(sum2.carry_out));
    println!("{}   | {}     | 0            | 1              | 2               ", bit_to_int(sum3.sum), bit_to_int(sum3.carry_out));
    println!("{}   | {}     | 1            | 0              | 1               ", bit_to_int(sum4.sum), bit_to_int(sum4.carry_out));
    println!("{}   | {}     | 0            | 1              | 2               ", bit_to_int(sum5.sum), bit_to_int(sum5.carry_out));
    println!("{}   | {}     | 0            | 1              | 2               ", bit_to_int(sum6.sum), bit_to_int(sum6.carry_out));
    println!("{}   | {}     | 1            | 1              | 3               ", bit_to_int(sum7.sum), bit_to_int(sum7.carry_out));
}