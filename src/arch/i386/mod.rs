
use arch::{Architecture, ArchitectureState};

mod gdt;
mod idt;

pub struct State {
    gdt: gdt::Gdt,
    idt: idt::Idt,
}

// External variable in assembly code (not actually a function)
extern { fn tls_emul_segment(); }

impl State {
  pub fn new() -> State {
    State{gdt: gdt::Gdt::new(), idt: idt::Idt::new()}
  }
}

impl Architecture for ArchitectureState {
    fn initialise(&mut self) -> bool {
        self.state.gdt.entry(0, 0, 0, 0, 0); // 0x00 - NULL
        self.state.gdt.entry(1, 0, 0xFFFFFFFF, 0x98, 0xCF); // 0x08 - Kernel Code
        self.state.gdt.entry(2, 0, 0xFFFFFFFF, 0x92, 0xCF); // 0x10 - Kernel Data
        self.state.gdt.entry(3, 0, 0xFFFFFFFF, 0xF8, 0xCF); // 0x18 - User Code
        self.state.gdt.entry(4, 0, 0xFFFFFFFF, 0xF2, 0xCF); // 0x20 - User Data
        self.state.gdt.entry(5, tls_emul_segment as uint, 0xFFFFFFFF, 0x92, 0xCF); // 0x28 - TLS emulation (for stack switching support)
        self.state.gdt.load(0x08, 0x10, 0x28);

        self.state.idt.init();

        true
    }

    fn register_trap(&mut self, which: uint, handler: extern "Rust" fn(uint)) {
        self.state.idt.register(which, handler)
    }

    fn get_interrupts(&self) -> bool {
        // TODO: write
        false
    }

    fn set_interrupts(&mut self, state: bool) {
        if state == true {
            unsafe { asm!("sti") }
        } else {
            unsafe { asm!("cli") }
        }
    }

    fn wait_for_event(&self) {
        unsafe { asm!("sti; hlt") }
    }
}