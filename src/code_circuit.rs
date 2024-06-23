use ark_bls12_381::Fr;
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::uint8::UInt8;
use ark_relations::ns;
use ark_relations::r1cs::{ConstraintSystemRef, Result};
use ark_std::iterable::Iterable;
use std::cmp::Ordering;
use sha2::{Digest, Sha256};
use ark_r1cs_std::eq::EqGadget;

use super::crypto::{Code, CODE_LENGTH, COLOR_NUMBER, compute_hash};

type CircuitField = Fr;

#[derive(Clone, Debug)]
pub struct CodeDeclarationCircuit {
    pub code: Code,
    pub salt: [u8; 32],
    pub hash: [u8; 32],
}

impl From<Code> for CodeDeclarationCircuit {
    fn from(code: Code) -> Self {
        let salt = rand::random::<[u8; 32]>();

        let mut hasher = Sha256::new();

        code
            .colors
            .iter()
            .for_each(|col| hasher.update([col]));
        hasher.update(salt);

        let hash_result = hasher.finalize();

        CodeDeclarationCircuit {
            code,
            salt,
            hash: hash_result.into(),
        }
    }
}

impl ark_relations::r1cs::ConstraintSynthesizer<CircuitField> for CodeDeclarationCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<CircuitField>) -> Result<()> {
        
        let max_var: FpVar<CircuitField> = FpVar::new_constant(ns!(cs, "constant"), CircuitField::from(COLOR_NUMBER as u8)).unwrap();

        let code_place_vars: [FpVar<CircuitField>; CODE_LENGTH] = self
            .code
            .colors
            .map(|col| FpVar::new_witness(ns!(cs, "col"), || Ok(CircuitField::from(col))).unwrap());


        let salt_vars: [UInt8<CircuitField>; 32] = self
            .salt
            .map(|bit| UInt8::new_witness(ns!(cs, "salt"), || Ok(bit)).unwrap());

        let hash_vars: [UInt8<CircuitField>; 32] = self
            .hash
            .map(|bit| UInt8::new_input(ns!(cs, "hash"), || Ok(bit)).unwrap());


        let digest_var = compute_hash(&code_place_vars, &salt_vars)?;

        hash_vars
            .iter()
            .zip(digest_var.0)
            .for_each(|(h1, h2)| h1.enforce_equal(&h2).unwrap());

        code_place_vars.iter().for_each(|col| {
            FpVar::enforce_cmp(col, &FpVar::zero(), Ordering::Greater, true).unwrap();
            FpVar::enforce_cmp(col, &max_var, Ordering::Less, false).unwrap();
        });


        Ok(())
    }
}
