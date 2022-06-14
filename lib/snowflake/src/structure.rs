use contracts::requires;
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct Empty;
#[derive(Debug)]
pub struct Bits {
    bits: u8,
}

/// This trait is sealed and cannot be implemented for types outside this crate.
pub trait BitsState: private::Sealed {}

impl BitsState for Empty {}
impl BitsState for Bits {}

#[derive(Debug, Default)]
pub struct Never;
#[derive(Debug)]
pub struct Epoch {
    epoch: Instant,
}

/// This trait is sealed and cannot be implemented for types outside this crate.
pub trait EpochState: private::Sealed {}

impl EpochState for Never {}
impl EpochState for Epoch {}

mod private {
    use super::{Bits, Empty, Epoch, Never};

    // this is the "sealed traits" pattern: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
    // this is mostly just to prevent consumers of the API from attempting to form a
    // `IdStructBuilder` with types which implement our typestate traits, but dont fulfill the needs
    // of the builder. Our builder doesnt know about those impls.
    pub trait Sealed {}
    impl Sealed for Empty {}
    impl Sealed for Bits {}
    impl Sealed for Never {}
    impl Sealed for Epoch {}
}

#[derive(Default)]
pub struct IdStructBuilder<Ts: BitsState, Id: BitsState, Seq: BitsState, Ep: EpochState> {
    ts_bits: Ts,
    id_bits: Id,
    seq_bits: Seq,
    epoch: Ep,
}

impl<Ts, Id, Seq, Ep> IdStructBuilder<Ts, Id, Seq, Ep>
    where Ts: BitsState,
          Id: BitsState,
          Seq: BitsState,
          Ep: EpochState {

    pub fn timestamp_bits(self, bits: u8) -> IdStructBuilder<Bits, Id, Seq, Ep> {
        IdStructBuilder::<> {
            ts_bits: Bits{bits},
            id_bits: self.id_bits,
            seq_bits: self.seq_bits,
            epoch: self.epoch,
        }
    }

    pub fn gen_id_bits(self, bits: u8) -> IdStructBuilder<Ts, Bits, Seq, Ep> {
        IdStructBuilder::<> {
            ts_bits: self.ts_bits,
            id_bits: Bits{bits},
            seq_bits: self.seq_bits,
            epoch: self.epoch,
        }
    }

    pub fn sequence_bits(self, bits: u8) -> IdStructBuilder<Ts, Id, Bits, Ep> {
        IdStructBuilder::<> {
            ts_bits: self.ts_bits,
            id_bits: self.id_bits,
            seq_bits: Bits{bits},
            epoch: self.epoch,
        }
    }

    pub fn epoch(self, epoch: Instant) -> IdStructBuilder<Ts, Id, Seq, Epoch> {
        IdStructBuilder::<> {
            ts_bits: self.ts_bits,
            id_bits: self.id_bits,
            seq_bits: self.seq_bits,
            epoch: Epoch{epoch},
        }
    }
}

impl IdStructBuilder<Bits, Bits, Bits, Epoch> {
    #[requires(self.ts_bits.bits + self.id_bits.bits + self.seq_bits.bits == 64)]
    #[requires(self.epoch.epoch <= Instant::now())]
    pub fn create(self) -> IdStructure {
        IdStructure {
            timestamp_bits: self.ts_bits.bits,
            gen_id_bits: self.id_bits.bits,
            sequence_bits: self.seq_bits.bits,
            epoch: self.epoch.epoch,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct IdStructure {
    timestamp_bits: u8,
    gen_id_bits: u8,
    sequence_bits: u8,
    epoch: Instant,
}

impl IdStructure {
    pub fn builder() -> IdStructBuilder<Empty, Empty, Empty, Never> {
        IdStructBuilder::<>::default()
    }

    pub fn get_time_mask(&self) -> u64 {
        (1u64 << self.timestamp_bits) - 1
    }

    pub fn get_gen_id_mask(&self) -> u64 {
        (1u64 << self.gen_id_bits) - 1
    }

    pub fn get_sequence_mask(&self) -> u64 {
        (1u64 << self.sequence_bits) - 1
    }

    pub fn get_time_shift(&self) -> u64 {
        (self.gen_id_bits + self.sequence_bits) as u64
    }

    pub fn get_gen_shift(&self) -> u64 {
        self.sequence_bits as u64
    }

    pub fn get_ticks(&self) -> u64 {
        self.epoch.elapsed().as_millis() as u64
    }

    pub fn get_elapsed(&self) -> Duration {
        self.epoch.elapsed()
    }
}
