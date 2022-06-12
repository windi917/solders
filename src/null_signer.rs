use pyo3::{prelude::*, types::PyBytes};
use solana_sdk::signer::{null_signer::NullSigner as NullSignerOriginal, Signer as SignerTrait};
use solders_macros::{pyhash, richcmp_signer};

use crate::{
    impl_display, impl_signer_hash, CommonMethods, Pubkey, PyBytesGeneral, PyFromBytesGeneral,
    PyHash, RichcmpSigner, Signature, SignerTraitWrapper, ToSignerOriginal,
};

#[derive(Clone, Debug, Default, PartialEq)]
#[pyclass(module = "solders.null_signer", subclass)]
/// A signer implementation that always produces :meth:`solders.signature.Signature.default()`.
/// Used as a placeholder for absentee signers whose 'Pubkey` is required to construct
/// the transaction.
///
/// Args:
///     pubkey (Pubkey): The pubkey of the signer.
///
pub struct NullSigner(pub NullSignerOriginal);

#[pyhash]
#[richcmp_signer]
#[pymethods]
impl NullSigner {
    #[new]
    pub fn new(pubkey: &Pubkey) -> Self {
        NullSignerOriginal::new(pubkey.as_ref()).into()
    }

    #[pyo3(name = "pubkey")]
    /// Return the pubkey of the signer.
    ///
    /// Returns:
    ///     Pubkey: The signer's pubkey.
    ///
    pub fn py_pubkey(&self) -> Pubkey {
        self.pubkey().into()
    }

    #[pyo3(name = "sign_message")]
    /// Simply returns :meth:`solders.signature.Signature.default()`.
    ///
    /// Returns:
    ///     Signature: The default signature.
    ///
    pub fn py_sign_message(&self, message: &[u8]) -> Signature {
        self.try_sign_message(message).unwrap().into()
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default null signer.
    ///
    /// Returns:
    ///     NullSigner: The default null signer.
    ///
    pub fn new_default() -> Self {
        Self::default()
    }

    fn __repr__(&self) -> String {
        self.pyrepr()
    }

    fn __str__(&self) -> String {
        self.pystr()
    }

    fn __bytes__<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        self.pybytes(py)
    }

    #[staticmethod]
    /// Deserialize a serialized ``NullSigner`` object.
    ///
    /// Args:
    ///     data (bytes): The serialized ``NullSigner``.
    ///
    /// Returns:
    ///     NullSigner: The deserialized ``NullSigner``.
    fn from_bytes(data: [u8; Pubkey::LENGTH]) -> PyResult<Self> {
        Self::py_from_bytes(&data)
    }
}

impl_display!(NullSigner);
impl_signer_hash!(NullSigner);
impl PyHash for NullSigner {}

impl PyBytesGeneral for NullSigner {
    fn pybytes_general<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        self.py_pubkey().pybytes(py)
    }
}

impl PyFromBytesGeneral for NullSigner {
    fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self> {
        Ok(Self::new(&Pubkey::from_bytes(raw)?))
    }
}

impl CommonMethods for NullSigner {}

impl From<NullSignerOriginal> for NullSigner {
    fn from(signer: NullSignerOriginal) -> Self {
        Self(signer)
    }
}

impl ToSignerOriginal for NullSigner {
    fn to_inner(&self) -> Box<dyn SignerTrait> {
        Box::new(self.0.clone())
    }
}

impl SignerTraitWrapper for NullSigner {}

impl RichcmpSigner for NullSigner {}
