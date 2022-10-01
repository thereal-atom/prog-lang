mod virtual_computer;
use virtual_computer::Bit::*;

fn main() {
    let my_v_computer = virtual_computer::VirtualComputer::new();

    let result = my_v_computer.cpu.alu.process(
        [Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, One, Zero],
        [Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero, One, One],
        Zero,
    );
}
