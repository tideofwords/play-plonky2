use anyhow::Result;
use plonky2::field::types::Field;
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};


#[cfg(test)]
mod tests {
    use anyhow::Result;
    use plonky2::field::types::Field;
    use plonky2::iop::target::Target;
    use plonky2::iop::witness::{PartialWitness, WitnessWrite};
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};


    #[test]
    fn test_poly() -> Result<()> {
        // RUST_LOG=debug cargo run --example poly
        // env_logger::init();
        
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
    
        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);
    
        const deg: usize = 2;
    
        let coeffs: Vec<Target> = std::iter::repeat_with(|| builder.add_virtual_target())
            .take(deg + 1)
            .collect();
    
        let x: Target = builder.add_virtual_target();
    
        // evaluate the polynomial
        let temp: Target = builder.zero();
        
        for i in 0..(deg + 1) {
            let temp = builder.mul_add(temp, x, coeffs[deg - i]);
        }
    
        // constrain value is 0
        let zero = builder.zero();
        builder.connect(temp, zero);
    
        // public values: coeffs and x
        builder.register_public_inputs(&coeffs);
        builder.register_public_input(x);
    
        // assign values
        let mut pw: PartialWitness<F> = PartialWitness::new();
        pw.set_target(x, F::from_canonical_u64(5));
        let coeff_values: Vec<F> = vec![-30, 1, 1]
            .iter()
            .map(|x| F::from_noncanonical_i64(*x))
            .collect();
        coeffs.iter().zip(coeff_values.iter())
            .for_each(|(targ, val)| 
            pw.set_target(*targ, *val).unwrap());
    
        // build the circuit and proof, and verify the proof
        let data = builder.build::<C>();
        let proof = data.prove(pw)?;
        data.verify(proof)
    }
}

