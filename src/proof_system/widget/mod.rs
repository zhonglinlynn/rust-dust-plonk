// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

pub mod arithmetic;
pub mod ecc;
pub mod logic;
pub mod permutation;
pub mod range;

use crate::commitment_scheme::kzg10::Commitment;
use crate::error::Error;
use crate::fft::{EvaluationDomain, Evaluations, Polynomial};
use crate::transcript::TranscriptProtocol;
use dusk_bls12_381::BlsScalar;
use dusk_bytes::{DeserializableSlice, Serializable};
use merlin::Transcript;

/// PLONK circuit verification key
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct VerifierKey {
    /// Circuit size (not padded to a power of two).
    pub(crate) n: usize,
    /// VerifierKey for arithmetic gates
    pub(crate) arithmetic: arithmetic::VerifierKey,
    /// VerifierKey for logic gates
    pub(crate) logic: logic::VerifierKey,
    /// VerifierKey for range gates
    pub(crate) range: range::VerifierKey,
    /// VerifierKey for fixed base curve addition gates
    pub(crate) fixed_base: ecc::scalar_mul::fixed_base::VerifierKey,
    /// VerifierKey for variable base curve addition gates
    pub(crate) variable_base: ecc::curve_addition::VerifierKey,
    /// VerifierKey for permutation checks
    pub(crate) permutation: permutation::VerifierKey,
}

impl Serializable<{ 15 * Commitment::SIZE + u64::SIZE }> for VerifierKey {
    type Error = dusk_bytes::Error;

    #[allow(unused_must_use)]
    fn to_bytes(&self) -> [u8; Self::SIZE] {
        use dusk_bytes::Write;
        let mut buff = [0u8; Self::SIZE];
        let mut writer = &mut buff[..];

        writer.write(&(self.n as u64).to_bytes());
        writer.write(&self.arithmetic.q_m.to_bytes());
        writer.write(&self.arithmetic.q_l.to_bytes());
        writer.write(&self.arithmetic.q_r.to_bytes());
        writer.write(&self.arithmetic.q_o.to_bytes());
        writer.write(&self.arithmetic.q_4.to_bytes());
        writer.write(&self.arithmetic.q_c.to_bytes());
        writer.write(&self.arithmetic.q_arith.to_bytes());
        writer.write(&self.logic.q_logic.to_bytes());
        writer.write(&self.range.q_range.to_bytes());
        writer.write(&self.fixed_base.q_fixed_group_add.to_bytes());
        writer.write(&self.variable_base.q_variable_group_add.to_bytes());
        writer.write(&self.permutation.left_sigma.to_bytes());
        writer.write(&self.permutation.right_sigma.to_bytes());
        writer.write(&self.permutation.out_sigma.to_bytes());
        writer.write(&self.permutation.fourth_sigma.to_bytes());

        buff
    }

    fn from_bytes(buf: &[u8; Self::SIZE]) -> Result<VerifierKey, Self::Error> {
        let mut buffer = &buf[..];

        Ok(Self::from_polynomial_commitments(
            u64::from_reader(&mut buffer)? as usize,
            Commitment::from_reader(&mut buffer)?,
            Commitment::from_reader(&mut buffer)?,
            Commitment::from_reader(&mut buffer)?,
            Commitment::from_reader(&mut buffer)?,
            Commitment::from_reader(&mut buffer)?,
            Commitment::from_reader(&mut buffer)?,
            Commitment::from_reader(&mut buffer)?,
            Commitment::from_reader(&mut buffer)?,
            Commitment::from_reader(&mut buffer)?,
            Commitment::from_reader(&mut buffer)?,
            Commitment::from_reader(&mut buffer)?,
            Commitment::from_reader(&mut buffer)?,
            Commitment::from_reader(&mut buffer)?,
            Commitment::from_reader(&mut buffer)?,
            Commitment::from_reader(&mut buffer)?,
        ))
    }
}

impl VerifierKey {
    /// Constructs a VerifierKey from the widget VerifierKey's that are
    /// constructed based on the selector polynomial commitments and the
    /// sigma polynomial commitments.
    pub(crate) fn from_polynomial_commitments(
        n: usize,
        q_m: Commitment,
        q_l: Commitment,
        q_r: Commitment,
        q_o: Commitment,
        q_4: Commitment,
        q_c: Commitment,
        q_arith: Commitment,
        q_logic: Commitment,
        q_range: Commitment,
        q_fixed_group_add: Commitment,
        q_variable_group_add: Commitment,
        left_sigma: Commitment,
        right_sigma: Commitment,
        out_sigma: Commitment,
        fourth_sigma: Commitment,
    ) -> VerifierKey {
        let arithmetic = arithmetic::VerifierKey {
            q_m,
            q_l,
            q_r,
            q_o,
            q_4,
            q_c,
            q_arith,
        };
        let logic = logic::VerifierKey { q_c, q_logic };
        let range = range::VerifierKey { q_range };
        let fixed_base = ecc::scalar_mul::fixed_base::VerifierKey {
            q_fixed_group_add,
            q_l,
            q_r,
        };

        let variable_base = ecc::curve_addition::VerifierKey {
            q_variable_group_add,
        };

        let permutation = permutation::VerifierKey {
            left_sigma,
            right_sigma,
            out_sigma,
            fourth_sigma,
        };

        VerifierKey {
            n,
            arithmetic,
            logic,
            range,
            variable_base,
            fixed_base,
            permutation,
        }
    }

    /// Adds the circuit description to the transcript
    pub(crate) fn seed_transcript(&self, transcript: &mut Transcript) {
        transcript.append_commitment(b"q_m", &self.arithmetic.q_m);
        transcript.append_commitment(b"q_l", &self.arithmetic.q_l);
        transcript.append_commitment(b"q_r", &self.arithmetic.q_r);
        transcript.append_commitment(b"q_o", &self.arithmetic.q_o);
        transcript.append_commitment(b"q_c", &self.arithmetic.q_c);
        transcript.append_commitment(b"q_4", &self.arithmetic.q_4);
        transcript.append_commitment(b"q_arith", &self.arithmetic.q_arith);
        transcript.append_commitment(b"q_range", &self.range.q_range);
        transcript.append_commitment(b"q_logic", &self.logic.q_logic);
        transcript.append_commitment(
            b"q_variable_group_add",
            &self.variable_base.q_variable_group_add,
        );
        transcript.append_commitment(
            b"q_fixed_group_add",
            &self.fixed_base.q_fixed_group_add,
        );

        transcript
            .append_commitment(b"left_sigma", &self.permutation.left_sigma);
        transcript
            .append_commitment(b"right_sigma", &self.permutation.right_sigma);
        transcript.append_commitment(b"out_sigma", &self.permutation.out_sigma);
        transcript
            .append_commitment(b"fourth_sigma", &self.permutation.fourth_sigma);

        // Append circuit size to transcript
        transcript.circuit_domain_sep(self.n as u64);
    }
}

/// PLONK circuit proving key
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProverKey {
    /// Circuit size
    pub(crate) n: usize,
    /// ProverKey for arithmetic gate
    pub(crate) arithmetic: arithmetic::ProverKey,
    /// ProverKey for logic gate
    pub(crate) logic: logic::ProverKey,
    /// ProverKey for range gate
    pub(crate) range: range::ProverKey,
    /// ProverKey for fixed base curve addition gates
    pub(crate) fixed_base: ecc::scalar_mul::fixed_base::ProverKey,
    /// ProverKey for permutation checks
    pub(crate) permutation: permutation::ProverKey,
    /// ProverKey for variable base curve addition gates
    pub(crate) variable_base: ecc::curve_addition::ProverKey,
    // Pre-processes the 4n Evaluations for the vanishing polynomial, so they
    // do not need to be computed at the proving stage.
    // Note: With this, we can combine all parts of the quotient polynomial in
    // their evaluation phase and divide by the quotient polynomial without
    // having to perform IFFT
    pub(crate) v_h_coset_4n: Evaluations,
}

impl ProverKey {
    /// Returns the number of `Polynomial`s contained in a ProverKey.
    const fn num_polys() -> usize {
        15
    }

    /// Returns the number of `Evaluations` contained in a ProverKey.
    const fn num_evals() -> usize {
        17
    }

    /// Serialises a [`ProverKey`] struct into a Vec of bytes.
    #[allow(unused_must_use)]
    pub fn to_var_bytes(&self) -> Vec<u8> {
        use dusk_bytes::Write;
        // Fetch size in bytes of each Polynomial
        let poly_size = self.arithmetic.q_m.0.len() * BlsScalar::SIZE;
        // Fetch size in bytes of each Evaluations
        let evals_size = self.arithmetic.q_m.1.evals.len() * BlsScalar::SIZE
            + EvaluationDomain::SIZE;
        // Create the vec with the capacity counting the 3 u64's plus the 15
        // Polys and the 17 Evaluations.
        let mut bytes = vec![
            0u8;
            (Self::num_polys() * poly_size
                + evals_size * Self::num_evals()
                + 17 * u64::SIZE) as usize
        ];

        let mut writer = &mut bytes[..];
        writer.write(&(self.n as u64).to_bytes());
        // Write Evaluation len in bytes.
        writer.write(&(evals_size as u64).to_bytes());

        // Arithmetic
        writer.write(&(self.arithmetic.q_m.0.len() as u64).to_bytes());
        writer.write(&self.arithmetic.q_m.0.to_var_bytes());
        writer.write(&self.arithmetic.q_m.1.to_var_bytes());

        writer.write(&(self.arithmetic.q_l.0.len() as u64).to_bytes());
        writer.write(&self.arithmetic.q_l.0.to_var_bytes());
        writer.write(&self.arithmetic.q_l.1.to_var_bytes());

        writer.write(&(self.arithmetic.q_r.0.len() as u64).to_bytes());
        writer.write(&self.arithmetic.q_r.0.to_var_bytes());
        writer.write(&self.arithmetic.q_r.1.to_var_bytes());

        writer.write(&(self.arithmetic.q_o.0.len() as u64).to_bytes());
        writer.write(&self.arithmetic.q_o.0.to_var_bytes());
        writer.write(&self.arithmetic.q_o.1.to_var_bytes());

        writer.write(&(self.arithmetic.q_4.0.len() as u64).to_bytes());
        writer.write(&self.arithmetic.q_4.0.to_var_bytes());
        writer.write(&self.arithmetic.q_4.1.to_var_bytes());

        writer.write(&(self.arithmetic.q_c.0.len() as u64).to_bytes());
        writer.write(&self.arithmetic.q_c.0.to_var_bytes());
        writer.write(&self.arithmetic.q_c.1.to_var_bytes());

        writer.write(&(self.arithmetic.q_arith.0.len() as u64).to_bytes());
        writer.write(&self.arithmetic.q_arith.0.to_var_bytes());
        writer.write(&self.arithmetic.q_arith.1.to_var_bytes());

        // Logic
        writer.write(&(self.logic.q_logic.0.len() as u64).to_bytes());
        writer.write(&self.logic.q_logic.0.to_var_bytes());
        writer.write(&self.logic.q_logic.1.to_var_bytes());

        // Range
        writer.write(&(self.range.q_range.0.len() as u64).to_bytes());
        writer.write(&self.range.q_range.0.to_var_bytes());
        writer.write(&self.range.q_range.1.to_var_bytes());

        // Fixed base multiplication
        writer.write(
            &(self.fixed_base.q_fixed_group_add.0.len() as u64).to_bytes(),
        );
        writer.write(&self.fixed_base.q_fixed_group_add.0.to_var_bytes());
        writer.write(&self.fixed_base.q_fixed_group_add.1.to_var_bytes());

        // Variable base addition
        writer.write(
            &(self.variable_base.q_variable_group_add.0.len() as u64)
                .to_bytes(),
        );
        writer.write(&self.variable_base.q_variable_group_add.0.to_var_bytes());
        writer.write(&self.variable_base.q_variable_group_add.1.to_var_bytes());

        // Permutation
        writer.write(&(self.permutation.left_sigma.0.len() as u64).to_bytes());
        writer.write(&self.permutation.left_sigma.0.to_var_bytes());
        writer.write(&self.permutation.left_sigma.1.to_var_bytes());

        writer.write(&(self.permutation.right_sigma.0.len() as u64).to_bytes());
        writer.write(&self.permutation.right_sigma.0.to_var_bytes());
        writer.write(&self.permutation.right_sigma.1.to_var_bytes());

        writer.write(&(self.permutation.out_sigma.0.len() as u64).to_bytes());
        writer.write(&self.permutation.out_sigma.0.to_var_bytes());
        writer.write(&self.permutation.out_sigma.1.to_var_bytes());

        writer
            .write(&(self.permutation.fourth_sigma.0.len() as u64).to_bytes());
        writer.write(&self.permutation.fourth_sigma.0.to_var_bytes());
        writer.write(&self.permutation.fourth_sigma.1.to_var_bytes());

        writer.write(&self.permutation.linear_evaluations.to_var_bytes());

        writer.write(&self.v_h_coset_4n.to_var_bytes());

        bytes
    }

    /// Deserialises a slice of bytes into a [`ProverKey`].
    pub fn from_slice(bytes: &[u8]) -> Result<ProverKey, Error> {
        let mut buffer = &bytes[..];
        let n = u64::from_reader(&mut buffer)? as usize;
        let evaluations_size = u64::from_reader(&mut buffer)? as usize;
        // let domain = crate::fft::EvaluationDomain::new(4 * size)?;
        // TODO: By creating this we can avoid including the EvaluationDomain
        // inside Evaluations. See: dusk-network/plonk#436

        let poly_from_reader = |buf: &mut &[u8]| -> Result<Polynomial, Error> {
            let serialized_poly_len =
                u64::from_reader(buf)? as usize * BlsScalar::SIZE;
            // If the announced len is zero, simply return an empty poly and
            // leave the buffer intact.
            if serialized_poly_len == 0 {
                return Ok(Polynomial { coeffs: vec![] });
            }
            let (a, b) = buf.split_at(serialized_poly_len);
            let poly = Polynomial::from_slice(a);
            *buf = b;

            poly
        };

        let evals_from_reader =
            |buf: &mut &[u8]| -> Result<Evaluations, Error> {
                let (a, b) = buf.split_at(evaluations_size);
                let eval = Evaluations::from_slice(a);
                *buf = b;

                eval
            };

        let q_m_poly = poly_from_reader(&mut buffer)?;
        let q_m_evals = evals_from_reader(&mut buffer)?;
        let q_m = (q_m_poly, q_m_evals);

        let q_l_poly = poly_from_reader(&mut buffer)?;
        let q_l_evals = evals_from_reader(&mut buffer)?;
        let q_l = (q_l_poly, q_l_evals);

        let q_r_poly = poly_from_reader(&mut buffer)?;
        let q_r_evals = evals_from_reader(&mut buffer)?;
        let q_r = (q_r_poly, q_r_evals);

        let q_o_poly = poly_from_reader(&mut buffer)?;
        let q_o_evals = evals_from_reader(&mut buffer)?;
        let q_o = (q_o_poly, q_o_evals);

        let q_4_poly = poly_from_reader(&mut buffer)?;
        let q_4_evals = evals_from_reader(&mut buffer)?;
        let q_4 = (q_4_poly, q_4_evals);

        let q_c_poly = poly_from_reader(&mut buffer)?;
        let q_c_evals = evals_from_reader(&mut buffer)?;
        let q_c = (q_c_poly, q_c_evals);

        let q_arith_poly = poly_from_reader(&mut buffer)?;
        let q_arith_evals = evals_from_reader(&mut buffer)?;
        let q_arith = (q_arith_poly, q_arith_evals);

        let q_logic_poly = poly_from_reader(&mut buffer)?;
        let q_logic_evals = evals_from_reader(&mut buffer)?;
        let q_logic = (q_logic_poly, q_logic_evals);

        let q_range_poly = poly_from_reader(&mut buffer)?;
        let q_range_evals = evals_from_reader(&mut buffer)?;
        let q_range = (q_range_poly, q_range_evals);

        let q_fixed_group_add_poly = poly_from_reader(&mut buffer)?;
        let q_fixed_group_add_evals = evals_from_reader(&mut buffer)?;
        let q_fixed_group_add =
            (q_fixed_group_add_poly, q_fixed_group_add_evals);

        let q_variable_group_add_poly = poly_from_reader(&mut buffer)?;
        let q_variable_group_add_evals = evals_from_reader(&mut buffer)?;
        let q_variable_group_add =
            (q_variable_group_add_poly, q_variable_group_add_evals);

        let left_sigma_poly = poly_from_reader(&mut buffer)?;
        let left_sigma_evals = evals_from_reader(&mut buffer)?;
        let left_sigma = (left_sigma_poly, left_sigma_evals);

        let right_sigma_poly = poly_from_reader(&mut buffer)?;
        let right_sigma_evals = evals_from_reader(&mut buffer)?;
        let right_sigma = (right_sigma_poly, right_sigma_evals);

        let out_sigma_poly = poly_from_reader(&mut buffer)?;
        let out_sigma_evals = evals_from_reader(&mut buffer)?;
        let out_sigma = (out_sigma_poly, out_sigma_evals);

        let fourth_sigma_poly = poly_from_reader(&mut buffer)?;
        let fourth_sigma_evals = evals_from_reader(&mut buffer)?;
        let fourth_sigma = (fourth_sigma_poly, fourth_sigma_evals);

        let perm_linear_evaluations = evals_from_reader(&mut buffer)?;

        let v_h_coset_4n = evals_from_reader(&mut buffer)?;

        let arithmetic = arithmetic::ProverKey {
            q_m,
            q_l: q_l.clone(),
            q_r: q_r.clone(),
            q_o,
            q_c: q_c.clone(),
            q_4,
            q_arith,
        };

        let logic = logic::ProverKey {
            q_logic,
            q_c: q_c.clone(),
        };

        let range = range::ProverKey { q_range };

        let fixed_base = ecc::scalar_mul::fixed_base::ProverKey {
            q_fixed_group_add,
            q_l,
            q_r,
            q_c,
        };

        let permutation = permutation::ProverKey {
            left_sigma,
            right_sigma,
            out_sigma,
            fourth_sigma,
            linear_evaluations: perm_linear_evaluations,
        };

        let variable_base = ecc::curve_addition::ProverKey {
            q_variable_group_add,
        };

        let prover_key = ProverKey {
            n,
            arithmetic,
            logic,
            range,
            fixed_base,
            variable_base,
            permutation,
            v_h_coset_4n,
        };

        Ok(prover_key)
    }

    pub(crate) fn v_h_coset_4n(&self) -> &Evaluations {
        &self.v_h_coset_4n
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fft::{EvaluationDomain, Polynomial};
    use dusk_bls12_381::BlsScalar;

    fn rand_poly_eval(n: usize) -> (Polynomial, Evaluations) {
        let polynomial = Polynomial::rand(n, &mut rand::thread_rng());
        (polynomial, rand_evaluations(n))
    }

    fn rand_evaluations(n: usize) -> Evaluations {
        let domain = EvaluationDomain::new(4 * n).unwrap();
        let values: Vec<_> = (0..4 * n)
            .map(|_| BlsScalar::random(&mut rand::thread_rng()))
            .collect();
        let evaluations = Evaluations::from_vec_and_domain(values, domain);
        evaluations
    }

    #[test]
    fn test_serialise_deserialise_prover_key() {
        let n = 1 << 11;

        let q_m = rand_poly_eval(n);
        let q_l = rand_poly_eval(n);
        let q_r = rand_poly_eval(n);
        let q_o = rand_poly_eval(n);
        let q_c = rand_poly_eval(n);
        let q_4 = rand_poly_eval(n);
        let q_arith = rand_poly_eval(n);

        let q_logic = rand_poly_eval(n);

        let q_range = rand_poly_eval(n);

        let q_fixed_group_add = rand_poly_eval(n);

        let q_variable_group_add = rand_poly_eval(n);

        let left_sigma = rand_poly_eval(n);
        let right_sigma = rand_poly_eval(n);
        let out_sigma = rand_poly_eval(n);
        let fourth_sigma = rand_poly_eval(n);
        let linear_evaluations = rand_evaluations(n);

        let v_h_coset_4n = rand_evaluations(n);

        let arithmetic = arithmetic::ProverKey {
            q_m,
            q_l: q_l.clone(),
            q_r: q_r.clone(),
            q_o,
            q_c: q_c.clone(),
            q_4,
            q_arith,
        };

        let logic = logic::ProverKey {
            q_logic,
            q_c: q_c.clone(),
        };

        let range = range::ProverKey { q_range };

        let fixed_base = ecc::scalar_mul::fixed_base::ProverKey {
            q_fixed_group_add,
            q_l,
            q_r,
            q_c,
        };

        let permutation = permutation::ProverKey {
            left_sigma,
            right_sigma,
            out_sigma,
            fourth_sigma,
            linear_evaluations,
        };

        let variable_base = ecc::curve_addition::ProverKey {
            q_variable_group_add,
        };

        let prover_key = ProverKey {
            n,
            arithmetic,
            logic,
            fixed_base,
            range,
            variable_base,
            permutation,
            v_h_coset_4n,
        };

        let prover_key_bytes = prover_key.to_var_bytes();
        let pk = ProverKey::from_slice(&prover_key_bytes).unwrap();

        assert_eq!(pk, prover_key);
        assert_eq!(pk.to_bytes(), prover_key.to_bytes());
    }

    #[test]
    fn test_serialise_deserialise_verifier_key() {
        use crate::commitment_scheme::kzg10::Commitment;
        use dusk_bls12_381::G1Affine;

        let n = 2usize.pow(5);

        let q_m = Commitment::from_affine(G1Affine::generator());
        let q_l = Commitment::from_affine(G1Affine::generator());
        let q_r = Commitment::from_affine(G1Affine::generator());
        let q_o = Commitment::from_affine(G1Affine::generator());
        let q_c = Commitment::from_affine(G1Affine::generator());
        let q_4 = Commitment::from_affine(G1Affine::generator());
        let q_arith = Commitment::from_affine(G1Affine::generator());

        let q_range = Commitment::from_affine(G1Affine::generator());

        let q_fixed_group_add = Commitment::from_affine(G1Affine::generator());
        let q_variable_group_add =
            Commitment::from_affine(G1Affine::generator());

        let q_logic = Commitment::from_affine(G1Affine::generator());

        let left_sigma = Commitment::from_affine(G1Affine::generator());
        let right_sigma = Commitment::from_affine(G1Affine::generator());
        let out_sigma = Commitment::from_affine(G1Affine::generator());
        let fourth_sigma = Commitment::from_affine(G1Affine::generator());

        let arithmetic = arithmetic::VerifierKey {
            q_m,
            q_l,
            q_r,
            q_o,
            q_c,
            q_4,
            q_arith,
        };

        let logic = logic::VerifierKey { q_logic, q_c };

        let range = range::VerifierKey { q_range };

        let fixed_base = ecc::scalar_mul::fixed_base::VerifierKey {
            q_fixed_group_add,
            q_l,
            q_r,
        };
        let variable_base = ecc::curve_addition::VerifierKey {
            q_variable_group_add,
        };

        let permutation = permutation::VerifierKey {
            left_sigma,
            right_sigma,
            out_sigma,
            fourth_sigma,
        };

        let verifier_key = VerifierKey {
            n,
            arithmetic,
            logic,
            range,
            fixed_base,
            variable_base,
            permutation,
        };

        let verifier_key_bytes = verifier_key.to_bytes();
        let got = VerifierKey::from_bytes(&verifier_key_bytes).unwrap();

        assert_eq!(got, verifier_key);
    }
}
