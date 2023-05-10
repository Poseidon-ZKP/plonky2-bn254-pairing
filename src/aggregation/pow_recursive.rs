use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{circuit_builder::CircuitBuilder, circuit_data::CircuitConfig},
};

type F = GoldilocksField;
const D: usize = 2;

pub fn pow_gen_proof() {
    let config = CircuitConfig::standard_ecc_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    
}
