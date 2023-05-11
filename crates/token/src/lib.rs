use std::collections::HashMap;

use pyo3::prelude::*;
pub mod state;
pub mod associated;

pub fn create_token_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let token_mod = PyModule::new(py, "token")?;
    let state_mod = state::create_state_mod(py)?;
    let associated_mod = associated::create_associated_mod(py)?;
    let submodules = [
        state_mod,
        associated_mod,
    ];
    let modules: HashMap<String, &PyModule> = submodules
        .iter()
        .map(|x| (format!("solders.token.{}", x.name().unwrap()), *x))
        .collect();
    let sys_modules = py.import("sys")?.getattr("modules")?;
    sys_modules.call_method1("update", (modules,))?;
    for submod in submodules {
        token_mod.add_submodule(submod)?;
    }
    Ok(token_mod)
}
