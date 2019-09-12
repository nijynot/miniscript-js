use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

extern crate bitcoin;

use std::str::FromStr;
use bitcoin::Script;
use miniscript::policy::{Concrete, Liftable};
use miniscript::{Miniscript, DummyKey};
use js_sys::Array;

pub mod script_types {
    pub const MINISCRIPT: &'static str = "MINISCRIPT";
    pub const POLICY: &'static str = "POLICY";
}

pub mod public_key_types {
    pub const STRING: &'static str = "STRING";
    pub const PUBLIC_KEY: &'static str = "PUBLIC_KEY";
    pub const DUMMY_KEY: &'static str = "DUMMY";
}

trait MiniscriptBindings {
    fn script_size(&self) -> Option<usize>;
    fn _encode(&self) -> Option<Script>;
    fn max_satisfaction_witness_elements(&self) -> Option<usize>;
    fn max_satisfaction_size(&self, one_cost: usize) -> Option<usize>;
    fn normalized(&self) -> String;
    fn is_trivial(&self) -> bool;
    fn is_unsatisfiable(&self) -> bool;
    fn relative_timelocks(&self) -> Vec<u32>;
    fn at_age(&self, time: u32) -> String;
    fn n_keys(&self) -> usize;
    fn minimum_n_keys(&self) -> usize;
    fn sorted(&self) -> String;
    fn len(&self) -> Option<usize>;
    fn is_empty(&self) -> Option<bool>;
    fn to_bytes(&self) -> Option<Vec<u8>>;
    fn to_p2sh(&self) -> Option<String>;
    fn to_v0_p2wsh(&self) -> Option<String>;
    fn is_p2sh(&self) -> Option<bool>;
    fn is_p2pkh(&self) -> Option<bool>;
    fn is_p2pk(&self) -> Option<bool>;
    fn is_witness_program(&self) -> Option<bool>;
    fn is_v0_p2wsh(&self) -> Option<bool>;
    fn is_v0_p2wpkh(&self) -> Option<bool>;
    fn is_op_return(&self) -> Option<bool>;
    fn is_provably_unspendable(&self) -> Option<bool>;
}

impl MiniscriptBindings for Miniscript<String> {
    fn script_size(&self) -> Option<usize> { None }
    fn _encode(&self) -> Option<Script> { None }
    fn max_satisfaction_witness_elements(&self) -> Option<usize> {
        None
    }
    fn max_satisfaction_size(&self, _one_cost: usize) -> Option<usize> {
        None
    }
    fn normalized(&self) -> String {
        self.lift().normalized().to_string()
    }
    fn is_trivial(&self) -> bool { self.lift().is_trivial() }
    fn is_unsatisfiable(&self) -> bool { self.lift().is_unsatisfiable() }
    fn relative_timelocks(&self) -> Vec<u32> { self.lift().relative_timelocks() }
    fn at_age(&self, time: u32) -> String { self.lift().at_age(time).to_string() }
    fn n_keys(&self) -> usize { self.lift().n_keys() }
    fn minimum_n_keys(&self) -> usize { self.lift().minimum_n_keys() }
    fn sorted(&self) -> String { self.lift().sorted().to_string() }
    fn len(&self) -> Option<usize> { None }
    fn is_empty(&self) -> Option<bool> { None }
    fn to_bytes(&self) -> Option<Vec<u8>> { None }
    fn to_p2sh(&self) -> Option<String> { None }
    fn to_v0_p2wsh(&self) -> Option<String> { None }
    fn is_p2sh(&self) -> Option<bool> { None }
    fn is_p2pkh(&self) -> Option<bool> { None }
    fn is_p2pk(&self) -> Option<bool> { None }
    fn is_witness_program(&self) -> Option<bool> { None }
    fn is_v0_p2wsh(&self) -> Option<bool> { None }
    fn is_v0_p2wpkh(&self) -> Option<bool> { None }
    fn is_op_return(&self) -> Option<bool> { None }
    fn is_provably_unspendable(&self) -> Option<bool> { None }
}
impl MiniscriptBindings for Miniscript<bitcoin::PublicKey> {
    fn script_size(&self) -> Option<usize> {
        Some(self.script_size())
    }
    fn _encode(&self) -> Option<Script> { Some(self.encode()) }
    fn max_satisfaction_witness_elements(&self) -> Option<usize> {
        Some(self.max_satisfaction_witness_elements())
    }
    fn max_satisfaction_size(&self, one_cost: usize) -> Option<usize> {
        Some(self.max_satisfaction_size(one_cost))
    }
    fn normalized(&self) -> String {
        self.lift().normalized().to_string()
    }
    fn is_trivial(&self) -> bool { self.lift().is_trivial() }
    fn is_unsatisfiable(&self) -> bool { self.lift().is_unsatisfiable() }
    fn relative_timelocks(&self) -> Vec<u32> { self.lift().relative_timelocks() }
    fn at_age(&self, time: u32) -> String { self.lift().at_age(time).to_string() }
    fn n_keys(&self) -> usize { self.lift().n_keys() }
    fn minimum_n_keys(&self) -> usize { self.lift().minimum_n_keys() }
    fn sorted(&self) -> String { self.lift().sorted().to_string() }
    fn len(&self) -> Option<usize> { Some(self.encode().len()) }
    fn is_empty(&self) -> Option<bool> { Some(self.encode().is_empty()) }
    fn to_bytes(&self) -> Option<Vec<u8>> { Some(self.encode().to_bytes()) }
    fn to_p2sh(&self) -> Option<String> { Some(self.encode().to_p2sh().asm()) }
    fn to_v0_p2wsh(&self) -> Option<String> { Some(self.encode().to_v0_p2wsh().asm()) }
    fn is_p2sh(&self) -> Option<bool> { Some(self.encode().is_p2sh()) }
    fn is_p2pkh(&self) -> Option<bool> { Some(self.encode().is_p2pkh()) }
    fn is_p2pk(&self) -> Option<bool> { Some(self.encode().is_p2pk()) }
    fn is_witness_program(&self) -> Option<bool> { Some(self.encode().is_witness_program()) }
    fn is_v0_p2wsh(&self) -> Option<bool> { Some(self.encode().is_v0_p2wsh()) }
    fn is_v0_p2wpkh(&self) -> Option<bool> { Some(self.encode().is_v0_p2wpkh()) }
    fn is_op_return(&self) -> Option<bool> { Some(self.encode().is_op_return()) }
    fn is_provably_unspendable(&self) -> Option<bool> {
        Some(self.encode().is_provably_unspendable())
    }
}
impl MiniscriptBindings for Miniscript<DummyKey> {
    fn script_size(&self) -> Option<usize> { None }
    fn _encode(&self) -> Option<Script> { Some(self.encode()) }
    fn max_satisfaction_witness_elements(&self) -> Option<usize> {
        Some(self.max_satisfaction_witness_elements())
    }
    fn max_satisfaction_size(&self, one_cost: usize) -> Option<usize> {
        Some(self.max_satisfaction_size(one_cost))
    }
    fn normalized(&self) -> String {
        self.lift().normalized().to_string()
    }
    fn is_trivial(&self) -> bool { self.lift().is_trivial() }
    fn is_unsatisfiable(&self) -> bool { self.lift().is_unsatisfiable() }
    fn relative_timelocks(&self) -> Vec<u32> { self.lift().relative_timelocks() }
    fn at_age(&self, time: u32) -> String { self.lift().at_age(time).to_string() }
    fn n_keys(&self) -> usize { self.lift().n_keys() }
    fn minimum_n_keys(&self) -> usize { self.lift().minimum_n_keys() }
    fn sorted(&self) -> String { self.lift().sorted().to_string() }
    fn len(&self) -> Option<usize> { Some(self.encode().len()) }
    fn is_empty(&self) -> Option<bool> { Some(self.encode().is_empty()) }
    fn to_bytes(&self) -> Option<Vec<u8>> { Some(self.encode().to_bytes()) }
    fn to_p2sh(&self) -> Option<String> { Some(self.encode().to_p2sh().asm()) }
    fn to_v0_p2wsh(&self) -> Option<String> { Some(self.encode().to_v0_p2wsh().asm()) }
    fn is_p2sh(&self) -> Option<bool> { Some(self.encode().is_p2sh()) }
    fn is_p2pkh(&self) -> Option<bool> { Some(self.encode().is_p2pkh()) }
    fn is_p2pk(&self) -> Option<bool> { Some(self.encode().is_p2pk()) }
    fn is_witness_program(&self) -> Option<bool> { Some(self.encode().is_witness_program()) }
    fn is_v0_p2wsh(&self) -> Option<bool> { Some(self.encode().is_v0_p2wsh()) }
    fn is_v0_p2wpkh(&self) -> Option<bool> { Some(self.encode().is_v0_p2wpkh()) }
    fn is_op_return(&self) -> Option<bool> { Some(self.encode().is_op_return()) }
    fn is_provably_unspendable(&self) -> Option<bool> {
        Some(self.encode().is_provably_unspendable())
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn match_script(script: &str, script_type: &str, pk_type: &str) -> Box<dyn MiniscriptBindings> {
    let attr = (script_type, pk_type);
    let ms = match attr {
        (script_types::POLICY, public_key_types::STRING) => {
            let concrete = Concrete::<String>::from_str(script).expect_throw("Failed to parse Policy string.");
            Box::new(concrete.compile().expect_throw("Failed to compile Miniscript.")) as Box::<dyn MiniscriptBindings>
        },
        (script_types::POLICY, public_key_types::PUBLIC_KEY) => {
            let concrete = Concrete::<bitcoin::PublicKey>::from_str(script).expect_throw("Failed to parse Policy string.");
            Box::new(concrete.compile().expect_throw("Failed to compile Miniscript.")) as Box::<dyn MiniscriptBindings>
        },
        (script_types::POLICY, public_key_types::DUMMY_KEY) => {
            let concrete = Concrete::<DummyKey>::from_str(script).expect_throw("Failed to parse Policy string.");
            Box::new(concrete.compile().expect_throw("Failed to compile Miniscript.")) as Box::<dyn MiniscriptBindings>
        },
        (script_types::MINISCRIPT, public_key_types::STRING) => {
            Box::new(Miniscript::<String>::from_str(script).expect_throw("Failed to parse Miniscript string.")) as Box::<dyn MiniscriptBindings>
        },
        (script_types::MINISCRIPT, public_key_types::PUBLIC_KEY) => {
            Box::new(Miniscript::<bitcoin::PublicKey>::from_str(script).expect_throw("Failed to parse Miniscript string.")) as Box::<dyn MiniscriptBindings>
        },
        (script_types::MINISCRIPT, public_key_types::DUMMY_KEY) => {
            Box::new(Miniscript::<DummyKey>::from_str(script).expect_throw("Failed to parse Miniscript string.")) as Box::<dyn MiniscriptBindings>
        },
        (_, _) => panic!("Failed to parse unknown string.")
    };

    ms
}

#[wasm_bindgen]
pub fn policy_to_miniscript(policy_str: &str) -> String {
    let policy = Concrete::<String>::from_str(policy_str).expect_throw("Failed to parse Policy string.");
    format!("{}", policy.compile().expect_throw("Failed to compile Miniscript."))
}

#[wasm_bindgen]
pub fn script_size(script: &str, script_type: &str, pk_type: &str) -> usize {
    let ms = match_script(script, script_type, pk_type);
    ms.script_size().unwrap()
}

#[wasm_bindgen]
pub fn max_satisfaction_witness_elements(script: &str, script_type: &str, pk_type: &str) -> usize {
    let ms = match_script(script, script_type, pk_type);
    ms.max_satisfaction_witness_elements().unwrap()
}

#[wasm_bindgen]
pub fn to_script(script: &str, script_type: &str, pk_type: &str) -> String {
    let ms = match_script(script, script_type, pk_type);
    format!("{}", ms._encode().unwrap().asm())
}

#[wasm_bindgen]
pub fn normalized(script: &str, script_type: &str, pk_type: &str) -> String {
    let ms = match_script(script, script_type, pk_type);
    ms.normalized()
}

#[wasm_bindgen]
pub fn is_trivial(script: &str, script_type: &str, pk_type: &str) -> bool {
    let ms = match_script(script, script_type, pk_type);
    ms.is_trivial()
}

#[wasm_bindgen]
pub fn is_unsatisfiable(script: &str, script_type: &str, pk_type: &str) -> bool {
    let ms = match_script(script, script_type, pk_type);
    ms.is_unsatisfiable()
}

#[derive(Serialize)]
pub struct Timelocks {
    pub timelocks: Vec<u32>,
}

#[wasm_bindgen]
pub fn relative_timelocks(script: &str, script_type: &str, pk_type: &str) -> JsValue {
    let ms = match_script(script, script_type, pk_type);
    let timelocks = Timelocks {
        timelocks: ms.relative_timelocks(),
    };
    JsValue::from_serde(&timelocks).unwrap()
}

#[wasm_bindgen]
pub fn at_age(script: &str, script_type: &str, pk_type: &str, time: u32) -> String {
    let ms = match_script(script, script_type, pk_type);
    ms.at_age(time)
}

#[wasm_bindgen]
pub fn n_keys(script: &str, script_type: &str, pk_type: &str) -> usize {
    let ms = match_script(script, script_type, pk_type);
    ms.n_keys()
}

#[wasm_bindgen]
pub fn minimum_n_keys(script: &str, script_type: &str, pk_type: &str) -> usize {
    let ms = match_script(script, script_type, pk_type);
    ms.minimum_n_keys()
}

#[wasm_bindgen]
pub fn sorted(script: &str, script_type: &str, pk_type: &str) -> String {
    let ms = match_script(script, script_type, pk_type);
    ms.sorted()
}

#[wasm_bindgen]
pub fn len(script: &str, script_type: &str, pk_type: &str) -> usize {
    let ms = match_script(script, script_type, pk_type);
    ms.len().unwrap()
}

#[wasm_bindgen]
pub fn is_empty(script: &str, script_type: &str, pk_type: &str) -> bool {
    let ms = match_script(script, script_type, pk_type);
    ms.is_empty().unwrap()
}

#[derive(Serialize)]
pub struct Bytes {
    pub bytes: Vec<u8>,
}

#[wasm_bindgen]
pub fn to_bytes(script: &str, script_type: &str, pk_type: &str) -> JsValue {
    let ms = match_script(script, script_type, pk_type);
    let bytes = Bytes {
        bytes: ms.to_bytes().unwrap(),
    };
    JsValue::from_serde(&bytes).unwrap()
}

#[wasm_bindgen]
pub fn to_p2sh(script: &str, script_type: &str, pk_type: &str) -> String {
    let ms = match_script(script, script_type, pk_type);
    ms.to_p2sh().unwrap()
}

#[wasm_bindgen]
pub fn to_v0_p2wsh(script: &str, script_type: &str, pk_type: &str) -> String {
    let ms = match_script(script, script_type, pk_type);
    ms.to_v0_p2wsh().unwrap()
}

#[wasm_bindgen]
pub fn is_p2sh(script: &str, script_type: &str, pk_type: &str) -> bool {
    let ms = match_script(script, script_type, pk_type);
    ms.is_p2sh().unwrap()
}

#[wasm_bindgen]
pub fn is_p2pkh(script: &str, script_type: &str, pk_type: &str) -> bool {
    let ms = match_script(script, script_type, pk_type);
    ms.is_p2pkh().unwrap()
}

#[wasm_bindgen]
pub fn is_p2pk(script: &str, script_type: &str, pk_type: &str) -> bool {
    let ms = match_script(script, script_type, pk_type);
    ms.is_p2pk().unwrap()
}

#[wasm_bindgen]
pub fn is_witness_program(script: &str, script_type: &str, pk_type: &str) -> bool {
    let ms = match_script(script, script_type, pk_type);
    ms.is_witness_program().unwrap()
}

#[wasm_bindgen]
pub fn is_v0_p2wsh(script: &str, script_type: &str, pk_type: &str) -> bool {
    let ms = match_script(script, script_type, pk_type);
    ms.is_v0_p2wsh().unwrap()
}

#[wasm_bindgen]
pub fn is_v0_p2wpkh(script: &str, script_type: &str, pk_type: &str) -> bool {
    let ms = match_script(script, script_type, pk_type);
    ms.is_v0_p2wpkh().unwrap()
}

#[wasm_bindgen]
pub fn is_op_return(script: &str, script_type: &str, pk_type: &str) -> bool {
    let ms = match_script(script, script_type, pk_type);
    ms.is_op_return().unwrap()
}

#[wasm_bindgen]
pub fn is_provably_unspendable(script: &str, script_type: &str, pk_type: &str) -> bool {
    let ms = match_script(script, script_type, pk_type);
    ms.is_provably_unspendable().unwrap()
}
