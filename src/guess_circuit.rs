use ark_bls12_381::Fr;
use ark_ff::Fp;
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::select::CondSelectGadget;
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
pub struct GuessCircuit {
    pub code: Code,
    pub guess: Code,
    pub salt: [u8; 32],
    pub hash: [u8; 32],
    pub correct: u8,
    pub common: u8,
}

impl From<(Code, Code, u8, u8)> for GuessCircuit {
    fn from(code_guess: (Code, Code, u8, u8)) -> Self {
        let salt = rand::random::<[u8; 32]>();

        let mut hasher = Sha256::new();

        code_guess.0
            .colors
            .iter()
            .for_each(|col| hasher.update([col]));
        hasher.update(salt);

        let hash_result = hasher.finalize();

        let code = code_guess.0;
        let guess = code_guess.1;

        GuessCircuit {
            code,
            guess,
            salt,
            hash: hash_result.into(),
            correct: code_guess.2, 
            common: code_guess.3,
        }
    }
}

impl ark_relations::r1cs::ConstraintSynthesizer<CircuitField> for GuessCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<CircuitField>) -> Result<()> {
        //creating variables

        let code_vars: [FpVar<CircuitField>; CODE_LENGTH] = self
            .code
            .colors
            .map(|col| FpVar::new_witness(ns!(cs, "code"), || Ok(CircuitField::from(col))).unwrap());

        let guess_vars: [FpVar<CircuitField>; CODE_LENGTH] = self
            .guess
            .colors
            .map(|col| FpVar::new_witness(ns!(cs, "guess"), || Ok(CircuitField::from(col))).unwrap());


        let salt_vars: [UInt8<CircuitField>; 32] = self
            .salt
            .map(|bit| UInt8::new_witness(ns!(cs, "salt"), || Ok(bit)).unwrap());

        let hash_vars: [UInt8<CircuitField>; 32] = self
            .hash
            .map(|bit| UInt8::new_input(ns!(cs, "hash"), || Ok(bit)).unwrap());

        //computing hash
        let digest_var = compute_hash(&code_vars, &salt_vars)?;

        hash_vars
            .iter()
            .zip(digest_var.0)
            .for_each(|(h1, h2)| h1.enforce_equal(&h2).unwrap());

        //chcecking correct
        let correct_var = FpVar::new_input(ns!(cs, "correct"), || Ok(CircuitField::from(self.correct)))?;
        let common_var = FpVar::new_input(ns!(cs, "common"), || Ok(CircuitField::from(self.common)))?;

        let mut counter: FpVar<Fp<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4>> = FpVar::new_witness(ns!(cs, "counter"), || Ok(CircuitField::from(0)))?;

        code_vars.iter().zip(guess_vars.clone()).for_each(|(code, guess)| {
            let is_equal = FpVar::is_eq(
                &code,
                &guess,
            ).unwrap();
            counter = FpVar::conditionally_select(&is_equal, &(&counter+&FpVar::one()), &counter).unwrap();
        });
        
        correct_var.enforce_equal(&counter).unwrap();

        //checking common
        let constants: Vec<FpVar<CircuitField>> = (0..COLOR_NUMBER)
            .map(|number| {
                FpVar::new_constant(ns!(cs, "constant"), CircuitField::from(number as u8)).unwrap()
            })
            .collect();


        let mut guess_sum_vars: [FpVar<CircuitField>; COLOR_NUMBER] =  core::array::from_fn(|_| FpVar::zero());
        
        for (i, sum) in constants.iter().zip(guess_sum_vars.iter_mut()) {
            guess_vars.iter().for_each(|col| {
                let is_equal = FpVar::is_eq(
                    &col,
                    &i,
                ).unwrap();
                *sum = FpVar::conditionally_select(&is_equal, &(&*sum+&FpVar::one()), &sum).unwrap();
            });

        }

        let mut code_sum_vars: [FpVar<CircuitField>; COLOR_NUMBER] =  core::array::from_fn(|_| FpVar::zero());
        
        for (i, sum) in constants.iter().zip(code_sum_vars.iter_mut()) {
            code_vars.iter().for_each(|col| {
                let is_equal = FpVar::is_eq(
                    &col,
                    &i,
                ).unwrap();
                *sum = FpVar::conditionally_select(&is_equal, &(&*sum+&FpVar::one()), &sum).unwrap();
            });
        }
        
        let mut counter2: FpVar<Fp<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4>> = FpVar::new_witness(ns!(cs, "counter"), || Ok(CircuitField::from(0)))?;

        code_sum_vars.iter().zip(guess_sum_vars).for_each(|(code, guess)| {
            let is_equal = FpVar::is_cmp(code, &guess, Ordering::Less, true).unwrap();
            counter2 = FpVar::conditionally_select(&is_equal, &(&counter2+code), &(&counter2+guess)).unwrap();
        });
        
        common_var.enforce_equal(&counter2).unwrap();

        Ok(())
    }
}
