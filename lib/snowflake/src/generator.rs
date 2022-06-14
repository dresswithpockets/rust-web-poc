use std::sync::Arc;
use std::hint;
use parking_lot::{Mutex, MutexGuard};

use crate::structure::IdStructure;

#[derive(Debug, Default)]
pub struct Empty;

#[derive(Debug)]
pub struct Structure {
    structure: IdStructure,
}

pub trait StructState: private::Sealed  {}

impl StructState for Empty {}
impl StructState for Structure {}

#[derive(Debug)]
pub struct Id {
    id: u64,
}

pub trait IdState: private::Sealed  {}

impl IdState for Empty {}
impl IdState for Id {}

mod private {
    use super::{Empty, Structure, Id};

    pub trait Sealed {}
    impl Sealed for Empty {}
    impl Sealed for Structure {}
    impl Sealed for Id {}
}

#[derive(Debug, Default)]
pub struct IdGenBuilder<S: StructState, I: IdState> {
    structure: S,
    id: I,
}

impl<S: StructState, I: IdState> IdGenBuilder<S, I> {
    pub fn structure(self, structure: IdStructure) -> IdGenBuilder<Structure, I> {
        IdGenBuilder::<> {
            structure: Structure { structure },
            id: self.id,
        }
    }

    pub fn id(self, id: u64) -> IdGenBuilder<S, Id> {
        IdGenBuilder::<> {
            structure: self.structure,
            id: Id { id },
        }
    }
}

impl IdGenBuilder<Structure, Id> {
    pub fn create(self) -> SafeIdGenerator {
        SafeIdGenerator {
            structure: Arc::new(self.structure.structure),
            state: Arc::new(Mutex::new(SimpleState {
                id: self.id.id,
                sequence: 0,
                last_gen: 0,
            })),
        }
    }
}

pub trait IdGenerator: Send + Sync {
    fn get_id(&self) -> Result<u64, IdErr>;
}

#[derive(Debug)]
pub enum IdErr {
    NonMonotonic
}

#[derive(Debug)]
struct SimpleState {
    id: u64,
    sequence: u64,
    last_gen: u64,
}

#[derive(Debug)]
pub struct SafeIdGenerator {
    structure: Arc<IdStructure>,
    state: Arc<Mutex<SimpleState>>,
}

impl SafeIdGenerator {
    pub fn builder() -> IdGenBuilder<Empty, Empty> {
        IdGenBuilder::<>::default()
    }

    fn get_id_impl(&self, state: &mut MutexGuard<SimpleState>) -> Result<u64, IdErr> {
        let get_ticks_masked = || -> u64 {
            self.structure.get_ticks() & self.structure.get_time_mask()
        };

        let timestamp = get_ticks_masked();
        if timestamp < state.last_gen {
            return Err(IdErr::NonMonotonic);
        }

        // if we're in the same timestamp as the previous ID, we just increment the sequence
        if timestamp == state.last_gen {
            // if we're beyond the maximum amount of sequence that we mask by, then spin wait until
            // we've waited long enough, then retry.
            if state.sequence >= self.structure.get_sequence_mask() {
                while get_ticks_masked() == timestamp {
                    hint::spin_loop()
                }
                return self.get_id_impl(state)
            }
            state.sequence += 1
        } else { // otherwise, reset the sequence and use the new timestamp
            state.sequence = 0;
            state.last_gen = timestamp;
        }

        Ok((timestamp << self.structure.get_time_shift())
            + (state.id << self.structure.get_gen_shift())
            + state.sequence)
    }
}

impl IdGenerator for SafeIdGenerator {
    // todo(ashley) async? maybe via tokio?
    fn get_id(&self) -> Result<u64, IdErr> {
        let mut state = self.state.lock();
        self.get_id_impl(&mut state)
    }
}

pub struct IterMut<'a> {
    generator: &'a mut SafeIdGenerator,
    next: Option<u64>,
}

impl<'a> SafeIdGenerator {
    pub fn iter_mut(&'a mut self) -> IterMut<'a> {
        let next = self.get_id().ok();

        IterMut {
            generator: self,
            next,
        }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|id| {
            self.next = self.generator.get_id().ok();
            id
        })
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;
    use itertools::Itertools;
    use crate::generator::IdGenerator;
    use super::{IdStructure, SafeIdGenerator};

    #[test]
    #[should_panic]
    fn structure_bits_add_to_64() {
        IdStructure::builder()
            .gen_id_bits(0)
            .sequence_bits(0)
            .timestamp_bits(0)
            .epoch(Instant::now())
            .create();
    }

    fn setup_good() -> SafeIdGenerator {
        let id_structure = IdStructure::builder()
            .timestamp_bits(50)
            .gen_id_bits(13)
            .sequence_bits(1)
            .epoch(Instant::now())
            .create();

        SafeIdGenerator::builder()
            .structure(id_structure)
            .id(0)
            .create()
    }

    #[test]
    fn generator_gets_single_id() {
        let id_generator = setup_good();
        let id = id_generator.get_id().unwrap();
        assert!(id > 0)
    }

    #[test]
    fn generator_spin_waits_after_sequence_exhaustion_for_new_id() {
        let gen = setup_good();
        let mut ids = [0u64; 4];
        for i in 0..4 {
            ids[i] = gen.get_id().unwrap();
        }

        let get_ts = |i: u64| {
            (i >> gen.structure.get_time_shift()) & gen.structure.get_time_mask()
        };
        assert!(get_ts(ids[1]) < get_ts(ids[3]));
    }

    #[test]
    fn iterator_gets_multiple_ids() {
        let mut gen = setup_good();
        assert!((0..4).map(|_| gen.iter_mut().next().unwrap()).all_unique());
    }
}