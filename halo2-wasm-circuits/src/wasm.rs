use std::io::BufReader;

use halo2_proofs::pasta::EqAffine;
use halo2_proofs::plonk::{keygen_pk, keygen_vk};
use halo2_proofs::poly::commitment::Params;
use halo2_proofs::{
    circuit::Value,
    pasta::Fp,
    plonk::create_proof,
    transcript::{Blake2bWrite, Challenge255},
};

use rand_core::OsRng;

use js_sys::Uint8Array;

use wasm_bindgen::prelude::*;

use crate::fibonacci::FiboCircuit;

#[wasm_bindgen]
pub fn prove_fib(e1: u64, e2: u64, params_ser: JsValue) {
    // create circuit
    let elem_1 = Value::known(Fp::from(e1));
    let elem_2 = Value::known(Fp::from(e2));
    let circuit = FiboCircuit::<Fp> { elem_1, elem_2 };

    // create witness (instance). in this case, proof is trivial, so witness is empty
    let instance = [];

    // create params
    let params_vec = Uint8Array::new(&params_ser).to_vec();
    let params = Params::<EqAffine>::read(&mut BufReader::new(&params_vec[..])).unwrap();

    // create pk + vk (using params and empty circuit)
    let empty_circuit = FiboCircuit::<Fp> {
        elem_1: Value::unknown(),
        elem_2: Value::unknown(),
    };
    let vk = keygen_vk(&params, &empty_circuit).expect("keygen_vk should not fail");
    let pk = keygen_pk(&params, vk.clone(), &empty_circuit).expect("keygen_pk should not fail");

    // create proof
    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
    create_proof(
        &params,
        &pk,
        &[circuit.clone()],
        &[&instance],
        OsRng,
        &mut transcript,
    )
    .expect("create_proof should not fail");

    let proof: Vec<u8> = transcript.finalize();
    JsValue::from_serde(&proof).unwrap();
}
