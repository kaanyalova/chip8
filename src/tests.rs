use super::*;

#[test]

//screen clear
fn test_00E0() {
    let mut c8 = Emulator::init();
    Emulator::execute(&mut c8, 0x00E0)
}

#[test]
fn test_1nnn() {
    let mut c8 = Emulator::init();
    Emulator::execute(&mut c8, 0x103C);
    assert_eq!(c8.program_counter, 60);
    Emulator::execute(&mut c8, 0x12D7);
    assert_eq!(c8.program_counter, 727);
}

#[test]
fn test_2nnn() {
    let mut c8 = Emulator::init();
    Emulator::execute(&mut c8, 0x2C31);
    assert_eq!(c8.program_counter, 3121);
}

#[test]
fn test_3xkk() {
    let mut c8 = Emulator::init();
    c8.registers[5] = 12;
    Emulator::execute(&mut c8, 0x350c); // 3 / 5 / 0, 12
    assert_eq!(c8.program_counter, 2);
}

#[test]
fn test_4xkk() {
    let mut c8 = Emulator::init();
    c8.registers[5] = 113; //not 12
    Emulator::execute(&mut c8, 0x450c);
    assert_eq!(c8.program_counter, 2)
}

fn yet_another_check() {
    println!("Some stuff")
}
