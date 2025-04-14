use crate::cpu::CPU;

mod bus;
mod cpu;

fn main() {
    let mut cpu = CPU::new();

    // Test LDI
    /*cpu.data_bus_mut().write(0b00101110);
    cpu.clock_cycle();
    cpu.data_bus_mut().write(0b11111111);
    cpu.clock_cycle();
    cpu.data_bus_mut().write(0b00000000);
    cpu.clock_cycle();*/

    // Test LD HL, SP+e
    cpu.data_bus_mut().write(0b11111000);
    cpu.clock_cycle();
    cpu.data_bus_mut().write(0xCC);
    cpu.clock_cycle();
    cpu.data_bus_mut().write(0x00);
    cpu.clock_cycle();
    cpu.data_bus_mut().write(0x00);
    cpu.clock_cycle();

    println!("{}", cpu.register_file());
}
