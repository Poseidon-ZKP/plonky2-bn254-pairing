use std::marker::PhantomData;

use ark_bn254::Fq12Config;
use plonky2::{
    field::extension::Extendable, hash::hash_types::RichField,
    plonk::circuit_builder::CircuitBuilder,
};

use crate::{
    fields::{fq12_target::Fq12Target, native::MyFq12},
    pairing::final_exp_native::get_naf,
};

use super::inner_circuit::InnerCircuit;

pub fn pow(a: MyFq12, exp: Vec<u64>) -> MyFq12 {
    let mut res = a.clone();
    let mut is_started = false;
    let naf = get_naf(exp);

    dbg!(naf.len());

    for &z in naf.iter().rev() {
        if is_started {
            res = res * res;
        }

        if z != 0 {
            assert!(z == 1 || z == -1);
            if is_started {
                res = if z == 1 { res * a } else { res / a };
            } else {
                assert_eq!(z, 1);
                is_started = true;
            }
        }
    }
    res
}

struct PowCircuit<F: RichField + Extendable<D>, const D: usize> {
    _maker: PhantomData<F>,
}

struct PowIO<F: RichField + Extendable<D>, const D: usize> {
    res: Fq12Target<F, D>,
    a: Fq12Target<F, D>,
}

impl<F: RichField + Extendable<D>, const D: usize> InnerCircuit for PowCircuit<F, D> {
    type Input = PowIO<F, D>;

    type Output = PowIO<F, D>;

    fn constraint<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
        input: Self::Input,
    ) -> Self::Output {
        todo!()
    }

    fn public_inputs(&self) -> Vec<plonky2::iop::target::Target> {
        todo!()
    }

    fn project_to_input(public_inputs: &[plonky2::iop::target::Target]) -> Self::Input {
        todo!()
    }

    fn project_to_output(public_inputs: &[plonky2::iop::target::Target]) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::pow as pow_native;
    use crate::{
        fields::fq12_target::Fq12Target,
        pairing::{final_exp_native::BN_X, final_exp_target::pow},
    };
    use ark_bn254::Fq12;
    use plonky2::{
        field::goldilocks_field::GoldilocksField,
        iop::witness::PartialWitness,
        plonk::{
            circuit_builder::CircuitBuilder, circuit_data::CircuitConfig,
            config::PoseidonGoldilocksConfig,
        },
    };

    type F = GoldilocksField;
    type C = PoseidonGoldilocksConfig;
    const D: usize = 2;

    #[test]
    fn test_pow_native() {
        let a = Fq12::from(2);
        pow_native(a.into(), vec![BN_X]);
    }

    #[test]
    fn test_pow_target() {
        let a = Fq12::from(2);
        let config = CircuitConfig::standard_ecc_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        let a_t = Fq12Target::constant(&mut builder, a);

        let _r_t = pow(&mut builder, &a_t, vec![BN_X]);

        let pw = PartialWitness::new();
        let data = builder.build::<C>();
        dbg!(data.common.degree_bits());
        let _proof = data.prove(pw);
    }
}
