use anyhow::Result;
use plonky2::field::extension::{Extendable, FieldExtension};
use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::{CircuitConfig, CircuitData, CommonCircuitData, VerifierCircuitData, VerifierOnlyCircuitData};
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::plonk::proof::ProofWithPublicInputs;

use crate::poly::{gen_poly_proof, VerifiablePolyProof};

pub fn build_recursive_proof() -> Option<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    // make the proof whose verification I want to prove
    let vpp: VerifiablePolyProof<F, C, D> = gen_poly_proof::<F, C, D>().unwrap();

    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);
    let mut pw: PartialWitness<F> = PartialWitness::new();

    let proof_target = builder.add_virtual_proof_with_pis(&vpp.verifier_circuit_data.common);
    builder.register_public_inputs(&proof_target.public_inputs);

    pw.set_proof_with_pis_target(&proof_target, &vpp.proof);

    let vd_target = builder.add_virtual_verifier_data(vpp.verifier_circuit_data.common.config.fri_config.cap_height);
    pw.set_verifier_data_target(&vd_target, &vpp.verifier_circuit_data.verifier_only).unwrap();

    builder.verify_proof::<C>(&proof_target, &vd_target, &vpp.verifier_circuit_data.common);

    let data = builder.build::<C>();
    let proof = data.prove(pw).unwrap();

    Some(())
}


#[cfg(test)]
mod tests {
    use super::build_recursive_proof;

    #[test]
    fn test_recursion() -> () {
        build_recursive_proof();

        ()
    }
}