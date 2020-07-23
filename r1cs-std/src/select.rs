use crate::prelude::*;
use algebra::Field;
use r1cs_core::{ConstraintSystem, SynthesisError};

/// If condition is `true`, return `true_value`; else, select `false_value`.
pub trait CondSelectGadget<ConstraintF: Field>
where
    Self: Sized,
{
    fn conditionally_select<CS: ConstraintSystem<ConstraintF>>(
        cs: CS,
        cond: &Boolean,
        true_value: &Self,
        false_value: &Self,
    ) -> Result<Self, SynthesisError>;

    fn cost() -> usize;
}

impl<ConstraintF: Field, A: CondSelectGadget<ConstraintF> + Default + Clone> CondSelectGadget<ConstraintF>
for [A; 32]
{
    fn conditionally_select<CS: ConstraintSystem<ConstraintF>>(mut cs: CS, cond: &Boolean, true_value: &[A; 32], false_value: &[A; 32]) -> Result<Self, SynthesisError> {
        let mut arr = <[A; 32]>::default();
        arr.clone_from_slice(
            &true_value.iter().zip(&false_value[..]).enumerate()
                .map(|(i, (t, f))| A::conditionally_select(&mut cs.ns(|| format!("cond_select_array_{}", i)), cond, t, f))
                    .collect::<Result<Vec<A>, SynthesisError>>()?[..32]
        );
        Ok(arr)
    }

    fn cost() -> usize {
        32 * <A as CondSelectGadget<ConstraintF>>::cost()
    }
}



/// Uses two bits to perform a lookup into a table
pub trait TwoBitLookupGadget<ConstraintF: Field>
where
    Self: Sized,
{
    type TableConstant;
    fn two_bit_lookup<CS: ConstraintSystem<ConstraintF>>(
        cs: CS,
        bits: &[Boolean],
        constants: &[Self::TableConstant],
    ) -> Result<Self, SynthesisError>;

    fn cost() -> usize;
}

/// Uses three bits to perform a lookup into a table, where the last bit
/// performs negation
pub trait ThreeBitCondNegLookupGadget<ConstraintF: Field>
where
    Self: Sized,
{
    type TableConstant;
    fn three_bit_cond_neg_lookup<CS: ConstraintSystem<ConstraintF>>(
        cs: CS,
        bits: &[Boolean],
        b0b1: &Boolean,
        constants: &[Self::TableConstant],
    ) -> Result<Self, SynthesisError>;

    fn cost() -> usize;
}
