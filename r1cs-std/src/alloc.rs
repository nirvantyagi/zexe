use crate::Vec;
use algebra::Field;
use core::borrow::Borrow;
use r1cs_core::{ConstraintSystem, SynthesisError};

pub trait AllocGadget<V, ConstraintF: Field>
where
    Self: Sized,
    V: ?Sized,
{
    fn alloc_constant<T, CS: ConstraintSystem<ConstraintF>>(
        cs: CS,
        t: T,
    ) -> Result<Self, SynthesisError>
    where
        T: Borrow<V>;

    fn alloc<F, T, CS: ConstraintSystem<ConstraintF>>(cs: CS, f: F) -> Result<Self, SynthesisError>
    where
        F: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<V>;

    fn alloc_checked<F, T, CS: ConstraintSystem<ConstraintF>>(
        cs: CS,
        f: F,
    ) -> Result<Self, SynthesisError>
    where
        F: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<V>,
    {
        Self::alloc(cs, f)
    }

    fn alloc_input<F, T, CS: ConstraintSystem<ConstraintF>>(
        cs: CS,
        f: F,
    ) -> Result<Self, SynthesisError>
    where
        F: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<V>;

    fn alloc_input_checked<F, T, CS: ConstraintSystem<ConstraintF>>(
        cs: CS,
        f: F,
    ) -> Result<Self, SynthesisError>
    where
        F: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<V>,
    {
        Self::alloc_input(cs, f)
    }
}

impl<I, ConstraintF: Field, A: AllocGadget<I, ConstraintF>> AllocGadget<[I], ConstraintF>
    for Vec<A>
{
    #[inline]
    fn alloc_constant<T, CS: ConstraintSystem<ConstraintF>>(
        mut cs: CS,
        t: T,
    ) -> Result<Self, SynthesisError>
    where
        T: Borrow<[I]>,
    {
        let mut vec = Vec::new();
        for (i, value) in t.borrow().iter().enumerate() {
            vec.push(A::alloc_constant(cs.ns(|| format!("value_{}", i)), value)?);
        }
        Ok(vec)
    }

    fn alloc<F, T, CS: ConstraintSystem<ConstraintF>>(
        mut cs: CS,
        f: F,
    ) -> Result<Self, SynthesisError>
    where
        F: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<[I]>,
    {
        let mut vec = Vec::new();
        for (i, value) in f()?.borrow().iter().enumerate() {
            vec.push(A::alloc(&mut cs.ns(|| format!("value_{}", i)), || {
                Ok(value)
            })?);
        }
        Ok(vec)
    }

    fn alloc_input<F, T, CS: ConstraintSystem<ConstraintF>>(
        mut cs: CS,
        f: F,
    ) -> Result<Self, SynthesisError>
    where
        F: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<[I]>,
    {
        let mut vec = Vec::new();
        for (i, value) in f()?.borrow().iter().enumerate() {
            vec.push(A::alloc_input(
                &mut cs.ns(|| format!("value_{}", i)),
                || Ok(value),
            )?);
        }
        Ok(vec)
    }

    fn alloc_checked<F, T, CS: ConstraintSystem<ConstraintF>>(
        mut cs: CS,
        f: F,
    ) -> Result<Self, SynthesisError>
    where
        F: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<[I]>,
    {
        let mut vec = Vec::new();
        for (i, value) in f()?.borrow().iter().enumerate() {
            vec.push(A::alloc_checked(
                &mut cs.ns(|| format!("value_{}", i)),
                || Ok(value),
            )?);
        }
        Ok(vec)
    }

    fn alloc_input_checked<F, T, CS: ConstraintSystem<ConstraintF>>(
        mut cs: CS,
        f: F,
    ) -> Result<Self, SynthesisError>
    where
        F: FnOnce() -> Result<T, SynthesisError>,
        T: Borrow<[I]>,
    {
        let mut vec = Vec::new();
        for (i, value) in f()?.borrow().iter().enumerate() {
            vec.push(A::alloc_input_checked(
                &mut cs.ns(|| format!("value_{}", i)),
                || Ok(value),
            )?);
        }
        Ok(vec)
    }
}

impl<I, ConstraintF: Field, A: AllocGadget<I, ConstraintF> + Default + Copy> AllocGadget<[I; 32], ConstraintF>
for [A; 32]
{
    #[inline]
    fn alloc_constant<T, CS: ConstraintSystem<ConstraintF>>(
        mut cs: CS,
        t: T,
    ) -> Result<Self, SynthesisError>
        where
            T: Borrow<[I; 32]>,
    {
        let mut arr = <[A; 32]>::default();
        arr.copy_from_slice(
            &t.borrow().iter().enumerate().map(|(i, v)| {
                A::alloc_constant(cs.ns(|| format!("value_{}", i)), v)
            })
                .collect::<Result<Vec<A>, SynthesisError>>()?[..32]
        );
        Ok(arr)
    }

    fn alloc<F, T, CS: ConstraintSystem<ConstraintF>>(
        mut cs: CS,
        f: F,
    ) -> Result<Self, SynthesisError>
        where
            F: FnOnce() -> Result<T, SynthesisError>,
            T: Borrow<[I; 32]>,
    {
        let mut arr = <[A; 32]>::default();
        arr.copy_from_slice(
            &f()?.borrow().iter().enumerate().map(|(i, v)| {
                A::alloc(cs.ns(|| format!("value_{}", i)), || Ok(v) )
            })
                .collect::<Result<Vec<A>, SynthesisError>>()?[..32]
        );
        Ok(arr)
    }

    fn alloc_input<F, T, CS: ConstraintSystem<ConstraintF>>(
        mut cs: CS,
        f: F,
    ) -> Result<Self, SynthesisError>
        where
            F: FnOnce() -> Result<T, SynthesisError>,
            T: Borrow<[I; 32]>,
    {
        let mut arr = <[A; 32]>::default();
        arr.copy_from_slice(
            &f()?.borrow().iter().enumerate().map(|(i, v)| {
                A::alloc_input(cs.ns(|| format!("value_{}", i)), || Ok(v) )
            })
                .collect::<Result<Vec<A>, SynthesisError>>()?[..32]
        );
        Ok(arr)
    }
}
