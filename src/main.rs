use crate::cpu::CPU;

mod bus;
mod cpu;

fn main() {
    let mut cpu = CPU::new();

    // Test LD HL, SP+e
    // LD HL, 0b0000000000110011
    cpu.data_bus.write(0b00100001);
    cpu.clock_cycle();
    cpu.data_bus.write(0b00110101);
    cpu.clock_cycle();
    cpu.data_bus.write(0b00000000);
    cpu.clock_cycle();
    cpu.data_bus.write(0b00000000);
    cpu.clock_cycle();

    // LD SP, HL
    cpu.data_bus.write(0b11111001);
    cpu.clock_cycle();
    cpu.data_bus.write(0b00000000);
    cpu.clock_cycle();

    // LD HL, SP+e
    cpu.data_bus.write(0b11111000);
    cpu.clock_cycle();
    cpu.data_bus.write(0xCC);
    cpu.clock_cycle();
    cpu.data_bus.write(0x00);
    cpu.clock_cycle();
    cpu.data_bus.write(0x00);
    cpu.clock_cycle();

    println!("{}", &cpu.register_file);
}
