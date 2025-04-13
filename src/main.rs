use crate::cpu::CPU;

mod bus;
mod cpu;

fn main() {
    let mut cpu = CPU::new();
    cpu.data_bus_mut().write(0b00110110);
    cpu.clock_cycle();
    cpu.data_bus_mut().write(0b11111111);
    cpu.clock_cycle();
    cpu.data_bus_mut().write(0b00000000);
    cpu.clock_cycle();
    println!("{}", cpu.register_file());
}
