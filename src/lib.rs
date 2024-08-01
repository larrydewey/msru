// SPDX-License-Identifier: Apache-2.0

//! As most of the existing crates require kernel-mode, this provides a
//! Rust-friendly interface for reading and writing to MSRs while in
//! user-space. This does require the `msr` kernel module to be loaded.
//!
//! Currently this crate only supports Linux.

/// A Rust-friendly MSR structure.
pub struct Msr {
    /// A model specific register address we would like to read.
    pub reg: u32,
    pub cpu: u16,
    buffer: u64,
}

impl Msr {
    /// Construct an Msr for a specified register and CPU core.
    pub fn new(reg: u32, cpu: u16) -> Self {
        Self {
            reg,
            cpu,
            buffer: Default::default(),
        }
    }

    /// Returns a u64 value from the bytes buffer.
    pub fn read_value(&mut self) -> u64 {
        self.buffer
    }

    /// Update the byte buffer with the specified value to be written to the
    /// MSR.
    pub fn set_value(&mut self, value: u64) {
        self.buffer = value;
    }
}

pub trait Accessor {
    fn read(&mut self) -> u64;
    fn write(&self);
}

impl Accessor for Msr {
    /// Read the bytes from the MSR at the specified CPU and return the value.
    /// - Expects the a file-handle to have already been opened.
    fn read(&mut self) -> u64 {
        let reg: u32 = self.reg;
        let mut high: u32 = 0u32;
        let mut low: u32 = 0u32;
        unsafe {
            core::arch::asm!("rdmsr", out("eax") low, out("edx") high, in("ecx") reg);
        }
        self.buffer = ((high as u64) << 32) | (low as u64);
        self.read_value()
    }

    /// Write the bytes buffer into the MSR at the specified CPU.
    /// Expects the a file-handle to have already been opened.
    fn write(&self) {
        let high: u32 = self.buffer as u32;
        let low: u32 = (self.buffer >> 32) as u32;
        unsafe {
            core::arch::asm!("wrmsr", in("ecx") self.reg, in("eax") low, in("edx") high);
        }
    }
}
