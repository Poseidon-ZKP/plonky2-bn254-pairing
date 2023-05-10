use plonky2::{
    field::extension::Extendable, hash::hash_types::RichField, iop::target::Target,
    plonk::circuit_builder::CircuitBuilder,
};

pub trait InnerCircuit {
    type Input;
    type Output;

    fn constraint<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        input: Self::Input,
    ) -> Self::Output;

    fn public_inputs(&self) -> Vec<Target>;

    fn project_to_input(public_inputs: &[Target]) -> Self::Input;

    fn project_to_output(public_inputs: &[Target]) -> Self::Output;
}
