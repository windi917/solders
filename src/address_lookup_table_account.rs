use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_sdk::{
    address_lookup_table_account::AddressLookupTableAccount as AddressLookupTableAccountOriginal,
    pubkey::Pubkey as PubkeyOriginal,
};
use solders_macros::{common_methods, richcmp_eq_only};

use crate::{
    impl_display, pubkey::Pubkey, py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    CommonMethods, RichcmpEqualityOnly,
};

#[derive(Serialize, Deserialize)]
#[serde(remote = "AddressLookupTableAccountOriginal")]
struct AddressLookupTableAccountOriginalDef {
    key: PubkeyOriginal,
    addresses: Vec<PubkeyOriginal>,
}

/// The definition of address lookup table accounts as used by ``MessageV0``.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[pyclass(module = "solders.address_lookup_table_account", subclass)]
pub struct AddressLookupTableAccount(
    #[serde(with = "AddressLookupTableAccountOriginalDef")] AddressLookupTableAccountOriginal,
);

impl_display!(AddressLookupTableAccount);
pybytes_general_via_bincode!(AddressLookupTableAccount);
py_from_bytes_general_via_bincode!(AddressLookupTableAccount);

impl CommonMethods<'_> for AddressLookupTableAccount {}
impl RichcmpEqualityOnly for AddressLookupTableAccount {}

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl AddressLookupTableAccount {
    #[new]
    pub fn new(key: Pubkey, addresses: Vec<Pubkey>) -> Self {
        AddressLookupTableAccountOriginal {
            key: key.into(),
            addresses: addresses.into_iter().map(|a| a.into()).collect(),
        }
        .into()
    }

    #[getter]
    pub fn key(&self) -> Pubkey {
        self.0.key.into()
    }

    #[getter]
    pub fn addresses(&self) -> Vec<Pubkey> {
        self.0.addresses.into_iter().map(|a| a.into()).collect()
    }
}

impl From<AddressLookupTableAccountOriginal> for AddressLookupTableAccount {
    fn from(a: AddressLookupTableAccountOriginal) -> Self {
        Self(a)
    }
}

impl From<AddressLookupTableAccount> for AddressLookupTableAccountOriginal {
    fn from(a: AddressLookupTableAccount) -> Self {
        a.0
    }
}
