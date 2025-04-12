use crate::cpu::CPU;

mod bus;
mod cpu;

fn main() {
    let mut cpu = CPU::new();
    cpu.fetch();
    println!("Hello, world!");
}
