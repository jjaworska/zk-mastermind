use ark_crypto_primitives::crh::sha256::constraints::{DigestVar, Sha256Gadget};
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::uint8::UInt8;
use ark_relations::r1cs::Result;
use ark_r1cs_std::ToBytesGadget;
use ark_bls12_381::Fr;

type CircuitField = Fr;

pub const COLOR_NUMBER: usize = 8;
pub const CODE_LENGTH: usize = 4;

#[derive(Clone, Debug)]
pub struct Code {
    pub colors: [u8; CODE_LENGTH],
}

pub fn cast_fp_var_to_uint8(var: &FpVar<CircuitField>) -> Result<UInt8<CircuitField>> {
    let bytes = FpVar::to_bytes(var)?;
    Ok(bytes[0].clone())
}

pub fn compute_hash(
    code_place_vars: &[FpVar<CircuitField>; CODE_LENGTH],
    salt_vars: &[UInt8<CircuitField>; 32],
) -> Result<DigestVar<CircuitField>> {
    let mut hash_gadget: Sha256Gadget<CircuitField> = Sha256Gadget::default();

    code_place_vars.iter().for_each(|col| {
        hash_gadget
            .update(&[cast_fp_var_to_uint8(&col).unwrap()])
            .unwrap()
    });
    hash_gadget.update(salt_vars)?;
    hash_gadget.finalize()
}