struct Vm {
    // Condition codes
    z: u8 ,
    s: u8,
    p: u8,
    cy: u8,
    ac: u8,
    pad: u8,
    // State
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    int_enable: u8,
    memory: Vec<u8>,
}

