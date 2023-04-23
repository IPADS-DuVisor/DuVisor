use crate::csrc;
use crate::csrs;
use crate::vcpu::utils::*;
use core::arch::asm;
use std::sync::atomic::{AtomicU16, Ordering};

#[allow(unused)]
pub struct VirtualInterrupt {
    /*
     * UVTIMER (#16) is not controlled by vcpu.virq field
     * FIXME: use a bit array
     */
    irq_pending: AtomicU16,
}

impl VirtualInterrupt {
    pub fn new() -> Self {
        VirtualInterrupt {
            irq_pending: AtomicU16::new(0),
        }
    }

    pub fn set_pending_irq(&self, irq: u64) {
        if irq >= 16 {
            panic!("set_pending_irq: irq {} out of range", irq);
        }
        self.irq_pending.fetch_or(1 << irq, Ordering::SeqCst);
    }

    pub fn unset_pending_irq(&self, irq: u64) {
        if irq >= 16 {
            panic!("set_pending_irq: irq {} out of range", irq);
        }
        self.irq_pending.fetch_and(!(1 << irq), Ordering::SeqCst);
    }

    pub fn flush_pending_irq(&self) {
        /* Leave IRQ_U_SOFT for hardware UIPI */
        let pending = self.irq_pending.load(Ordering::SeqCst);
        for i in 1..9 {
            if (pending & (1 << i)) != 0 {
                unsafe {
                    csrs!(HUVIP, 1 << i);
                }
            } else {
                unsafe {
                    csrc!(HUVIP, 1 << i);
                }
            }
        }
    }
}
