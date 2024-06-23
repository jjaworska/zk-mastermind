use ark_bls12_381::Fr;
use ark_groth16::Groth16;
use ark_std::rand::SeedableRng;
use ark_std::{iterable::Iterable, rand::rngs::StdRng};
use ark_groth16::r1cs_to_qap::LibsnarkReduction;
use ark_snark::SNARK;
use ark_ff::{Fp, MontBackend};
use ark_bls12_381::FrConfig;
use ark_std::Zero;
use ark_std::One;

use ark_bls12_381::Config;
use ark_ec::bls12::Bls12;
use ark_groth16::VerifyingKey;

use crate::guess_circuit::GuessCircuit;

use super::crypto::Code;
use super::code_circuit::CodeDeclarationCircuit;


type Curve = ark_bls12_381::Bls12_381;
type CircuitField = Fr;

struct PublicInput(Vec<Fp<MontBackend<FrConfig, 4>, 4>>);

impl From<[u8; 32]> for PublicInput {
    fn from(value: [u8; 32]) -> Self {
        let size: usize = value.len();
        let mut input = vec![CircuitField::zero(); 8 * size];
        for i in 0..32 {
            for j in 0..8 {
                if value[i] >> j & 1 == 1 {
                    input[i * 8 + j] = CircuitField::one();
                }
            }
        }
        PublicInput(input)
    }
}

pub struct PublicGuessInput(Vec<Fp<MontBackend<FrConfig, 4>, 4>>);

impl From<([u8; 32], u8, u8)> for PublicGuessInput {
    fn from(value: ([u8; 32], u8, u8)) -> Self {
        let size: usize = value.0.len();
        let mut input = vec![CircuitField::zero(); 8 * size];
        for i in 0..32 {
            for j in 0..8 {
                if value.0[i] >> j & 1 == 1 {
                    input[i * 8 + j] = CircuitField::one();
                }
            }
        }
        input.push(value.1.into());
        input.push(value.2.into());
        PublicGuessInput(input)
    }
}


pub struct Proof{
    proof: ark_groth16::Proof<ark_ec::bls12::Bls12<ark_bls12_381::Config>>,
    vk: VerifyingKey<Bls12<Config>>,
}

pub fn prove(code:Code, salt: [u8; 32], hash: [u8; 32]) -> Proof {
    let circuit = CodeDeclarationCircuit{code, salt, hash};

    let mut rng = StdRng::seed_from_u64(1);
    let (pk, vk) =
        Groth16::<Curve>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();
    let proof = Groth16::<_, LibsnarkReduction>::prove(&pk, circuit.clone(), &mut rng).unwrap();
    Proof{proof, vk}
}

pub fn verify(hash: [u8; 32], proof: Proof) -> bool{
    let input = PublicInput::from(hash);
    Groth16::<_, LibsnarkReduction>::verify(&proof.vk, &input.0, &proof.proof).unwrap()
}

pub fn prove_guess(code:Code, guess:Code, salt: [u8; 32], hash: [u8; 32], correct: u8, common:u8) -> Proof {
    let circuit = GuessCircuit{code, guess, salt, hash, correct, common};

    let mut rng = StdRng::seed_from_u64(1);
    let (pk, vk) =
        Groth16::<Curve>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();
    let proof = Groth16::<_, LibsnarkReduction>::prove(&pk, circuit.clone(), &mut rng).unwrap();
    Proof{proof, vk}
}

pub fn verify_guess(hash: [u8; 32], correct: u8, common: u8, proof: Proof) -> bool{
    let input = PublicGuessInput::from((hash, correct, common));
    Groth16::<_, LibsnarkReduction>::verify(&proof.vk, &input.0, &proof.proof).unwrap()
}

/*
impl CodeProof {
    fn create(&mut self, pk: ProvingKey<Bls12<Config>>, circuit: CodeDeclarationCircuit, mut rng: StdRng) {
        self.proof = Groth16::<_, LibsnarkReduction>::prove(&pk, circuit.clone(), &mut rng).unwrap()
    }

    fn verify(&self, vk: VerifyingKey<Bls12<Config>>, input: PublicInput) -> bool {
        Groth16::<_, LibsnarkReduction>::verify(&vk, &input.0, &self.proof.clone()).unwrap()
    }

}
pub fn check_procedure() {
  let code: Code = Code{colors: [1, 2, 3, 4]};
  let circuit: CodeDeclarationCircuit = CodeDeclarationCircuit::from (code);


  let mut rng = StdRng::seed_from_u64(1);
  let (pk, vk) =
      Groth16::<Curve>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

  let mut proof = CodeProof{proof: None};  
  let mut t = circuit.hash;
  //t[0] = t[0]+1;
  let input = PublicInput::from(t);
  proof.create(pk, circuit, rng);


  let valid_proof = proof.verify(vk, input);
  assert!(valid_proof);
}*/
