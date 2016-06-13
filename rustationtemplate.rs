//! This file is automatically generated using psxunittest:
//! https://github.com/daeken/psxunittest
//!
//! /!\ DO NOT EDIT DIRECTLY /!\

use gpu::{Gpu, VideoClock};
use gpu::renderer::{Renderer, PrimitiveAttributes, Vertex};
use memory::{Interconnect, Addressable};
use memory;
use debugger::DummyDebugger;
use shared::SharedState;
use bios::Bios;

use super::{Cpu, RegisterIndex};

/// Dummy GPU renderer to run the tests
struct DummyRenderer;

impl Renderer for DummyRenderer {
    fn set_draw_offset(&mut self, _: i16, _: i16) {
    }

    fn set_draw_area(&mut self, _: (u16, u16), _: (u16, u16)) {
    }

    fn set_display_mode(&mut self,
                        _: (u16, u16),
                        _: (u16, u16),
                        _: bool) {
    }

    fn push_line(&mut self, _: &PrimitiveAttributes, _: &[Vertex; 2]) {
    }

    fn push_triangle(&mut self, _: &PrimitiveAttributes, _: &[Vertex; 3]) {
    }

    fn push_quad(&mut self, _: &PrimitiveAttributes, _: &[Vertex; 4]) {
    }

    fn fill_rect(&mut self,
                 _: [u8; 3],
                 _: (u16, u16),
                 _: (u16, u16)) {
    }

    fn load_image(&mut self,
                  _: (u16, u16),
                  _: (u16, u16),
                  _: &[u16]) {
    }
}

fn write_blob(cpu: &mut Cpu,
             address: u32,
             blob: &[u32]) {
    let ram = cpu.interconnect_mut().ram_mut();

    for (i, &w) in blob.iter().enumerate() {
        ram.store::<memory::Word>(address + (i * 4) as u32, w);
    }
}

fn write<T: Addressable>(cpu: &mut Cpu,
                         address: u32,
                         v: u32) {
    let ram = cpu.interconnect_mut().ram_mut();

    ram.store::<T>(address, v);
}

fn read<T: Addressable>(cpu: &mut Cpu, address: u32) -> u32 {

    let ram = cpu.interconnect().ram();

    ram.load::<T>(address)
}

$TESTS$

/// Number of CPU cycles after which we consider the test to be a
/// failure
const TIMEOUT: usize = 1_000_000;
