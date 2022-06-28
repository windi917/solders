use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_client::rpc_config;
use solana_sdk::commitment_config::CommitmentLevel as CommitmentLevelOriginal;
use solana_transaction_status::UiTransactionEncoding as UiTransactionEncodingOriginal;
use solders_macros::common_methods;

use crate::{
    account_decoder::UiAccountEncoding,
    commitment_config::{CommitmentConfig, CommitmentLevel},
    impl_display, py_from_bytes_general_via_bincode, pybytes_general_via_bincode,
    transaction_status::UiTransactionEncoding,
    CommonMethods, PyBytesBincode, PyFromBytesBincode, RichcmpEqualityOnly,
};

macro_rules! pyclass_boilerplate {
    ($(#[$attr:meta])* => $name:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[pyclass(module = "solders.rpc.config", subclass)]
        pub struct $name(rpc_config::$name);
    };
}

macro_rules! pyclass_boilerplate_with_default {
    ($(#[$attr:meta])* => $name:ident) => {
        $(#[$attr])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
        #[pyclass(module = "solders.rpc.config", subclass)]
        pub struct $name(rpc_config::$name);
    };
}

macro_rules! rpc_config_impls {
    ($ident:ident) => {
        pybytes_general_via_bincode!($ident);
        py_from_bytes_general_via_bincode!($ident);
        impl_display!($ident);
        impl RichcmpEqualityOnly for $ident {}
        impl CommonMethods<'_> for $ident {}
        impl From<rpc_config::$ident> for $ident {
            fn from(c: rpc_config::$ident) -> Self {
                Self(c)
            }
        }
        impl From<$ident> for rpc_config::$ident {
            fn from(c: $ident) -> Self {
                c.0
            }
        }
    };
}

pyclass_boilerplate!(
/// Configuration object for ``getSignatureStatuses``.
///
/// Args:
///     search_transaction_history:  If True, a Solana node will search its ledger cache for any signatures not found in the recent status cache
    => RpcSignatureStatusConfig
);

#[common_methods]
#[pymethods]
impl RpcSignatureStatusConfig {
    #[new]
    pub fn new(search_transaction_history: bool) -> Self {
        Self(rpc_config::RpcSignatureStatusConfig {
            search_transaction_history,
        })
    }

    #[getter]
    pub fn search_transaction_history(&self) -> bool {
        self.0.search_transaction_history
    }
}

rpc_config_impls!(RpcSignatureStatusConfig);

pyclass_boilerplate_with_default!(
    /// Configuration object for ``sendTransaction``.
    ///
    /// Args:
    ///     skip_preflight (bool):  If true, skip the preflight transaction checks.
    ///     preflight_commitment (Optional[CommitmentLevel]): Commitment level to use for preflight.
    ///     encoding: (Optional[UiTransactionEncoding]): Encoding used for the transaction data.
    ///     max_retries: (Optional[int]): Maximum number of times for the RPC node to retry sending
    ///         the transaction to the leader. If this parameter not provided, the RPC node will
    ///         retry the transaction until it is finalized or until the blockhash expires.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    ///
    => RpcSendTransactionConfig
);

#[common_methods]
#[pymethods]
impl RpcSendTransactionConfig {
    #[new]
    pub fn new(
        skip_preflight: bool,
        preflight_commitment: Option<CommitmentLevel>,
        encoding: Option<UiTransactionEncoding>,
        max_retries: Option<usize>,
        min_context_slot: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcSendTransactionConfig {
            skip_preflight,
            preflight_commitment: preflight_commitment.map(CommitmentLevelOriginal::from),
            encoding: encoding.map(UiTransactionEncodingOriginal::from),
            max_retries,
            min_context_slot,
        })
    }

    #[getter]
    pub fn skip_preflight(&self) -> bool {
        self.0.skip_preflight
    }

    #[getter]
    pub fn preflight_commitment(&self) -> Option<CommitmentLevel> {
        self.0.preflight_commitment.map(|p| p.into())
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(|e| e.into())
    }

    #[getter]
    pub fn max_retries(&self) -> Option<usize> {
        self.0.max_retries
    }

    #[getter]
    pub fn min_context_slot(&self) -> Option<u64> {
        self.0.min_context_slot
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSendTransactionConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }
}

rpc_config_impls!(RpcSendTransactionConfig);

pyclass_boilerplate_with_default!(
    /// Accounts configuration for ``simulateTransaction``.
    ///
    /// Args:
    ///     encoding (Optional[UiAccountEncoding]): Encoding for returned Account data
    ///     addresses (Sequence[str]): An array of accounts to return, as base-58 encoded strings.
    => RpcSimulateTransactionAccountsConfig
);

#[common_methods]
#[pymethods]
impl RpcSimulateTransactionAccountsConfig {
    #[new]
    pub fn new(encoding: Option<UiAccountEncoding>, addresses: Vec<String>) -> Self {
        Self(rpc_config::RpcSimulateTransactionAccountsConfig {
            encoding: encoding.map(|x| x.into()),
            addresses,
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSimulateTransactionAccountsConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiAccountEncoding> {
        self.0.encoding.map(|e| e.into())
    }

    #[getter]
    pub fn addresses(&self) -> Vec<String> {
        self.0.addresses.clone()
    }
}

rpc_config_impls!(RpcSimulateTransactionAccountsConfig);

pyclass_boilerplate_with_default!(
    /// Configuration object for ``simulateTransaction``.
    ///
    /// Args:
    ///     sig_verify (bool): If True the transaction signatures will be verified
    ///         (conflicts with ``replace_recent_blockhash``).
    ///     replace_recent_blockhash (bool): If True the transaction recent blockhash
    ///         will be replaced with the most recent blockhash
    ///         (conflicts with ``sig_verify``).
    ///     commitment (Optional[CommitmentConfig]): Commitment level at which to simulate the transaction.
    ///     encoding (Optional[UiTransactionEncoding]): Encoding used for the transaction data.
    ///     accounts (Optional[RpcSimulateTransactionAccountsConfig]): Accounts configuration object.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    => RpcSimulateTransactionConfig
);

#[common_methods]
#[pymethods]
impl RpcSimulateTransactionConfig {
    #[new]
    fn new(
        sig_verify: bool,
        replace_recent_blockhash: bool,
        commitment: Option<CommitmentConfig>,
        encoding: Option<UiTransactionEncoding>,
        accounts: Option<RpcSimulateTransactionAccountsConfig>,
        min_context_slot: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcSimulateTransactionConfig {
            sig_verify,
            replace_recent_blockhash,
            commitment: commitment.map(|c| c.into()),
            encoding: encoding.map(|e| e.into()),
            accounts: accounts.map(|a| a.into()),
            min_context_slot,
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSimulateTransactionConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn sig_verify(&self) -> bool {
        self.0.sig_verify
    }

    #[getter]
    pub fn replace_recent_blockhash(&self) -> bool {
        self.0.replace_recent_blockhash
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(|e| e.into())
    }

    #[getter]
    pub fn accounts(&self) -> Option<RpcSimulateTransactionAccountsConfig> {
        self.0.accounts.clone().map(|a| a.into())
    }

    #[getter]
    pub fn min_context_slot(&self) -> Option<u64> {
        self.0.min_context_slot
    }
}

rpc_config_impls!(RpcSimulateTransactionConfig);

pyclass_boilerplate_with_default!(
    /// Configuration object for ``requestAirdrop``.
    /// 
    /// Args:
    ///     recent_blockhash (Optional[str]): The ID of a recent ledger entry.
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    /// 
=> RpcRequestAirdropConfig);

rpc_config_impls!(RpcRequestAirdropConfig);

#[common_methods]
#[pymethods]
impl RpcRequestAirdropConfig {
    #[new]
    pub fn new(recent_blockhash: Option<&str>, commitment: Option<CommitmentConfig>) -> Self {
        Self(rpc_config::RpcRequestAirdropConfig {
            recent_blockhash: recent_blockhash.map(String::from),
            commitment: commitment.map(|c| c.into()),
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcRequestAirdropConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn recent_blockhash(&self) -> Option<String> {
        self.0.recent_blockhash.clone()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getLeaderSchedule``.
    /// 
    /// Args:
    ///     identity (Optional[str]): Validator identity, as a base-58 encoded string
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    /// 
=> RpcLeaderScheduleConfig);

rpc_config_impls!(RpcLeaderScheduleConfig);

#[common_methods]
#[pymethods]
impl RpcLeaderScheduleConfig {
    #[new]
    pub fn new(identity: Option<&str>, commitment: Option<CommitmentConfig>) -> Self {
        Self(rpc_config::RpcLeaderScheduleConfig {
            identity: identity.map(String::from),
            commitment: commitment.map(|c| c.into()),
        })
    }

    #[getter]
    pub fn identity(&self) -> Option<String> {
        self.0.identity.clone()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }
}

pyclass_boilerplate_with_default!(
    /// Range object for ``RpcBlockProductionConfig``.
    /// 
    /// Args:
    ///     first_slot (int): First slot in the range
    ///     last_slot (Optional[int]): Last slot in the range.
    /// 
=> RpcBlockProductionConfigRange);

rpc_config_impls!(RpcBlockProductionConfigRange);

#[common_methods]
#[pymethods]
impl RpcBlockProductionConfigRange {
    #[new]
    pub fn new(first_slot: u64, last_slot: Option<u64>) -> Self {
        Self(rpc_config::RpcBlockProductionConfigRange {
            first_slot,
            last_slot,
        })
    }

    #[getter]
    pub fn first_slot(&self) -> u64 {
        self.0.first_slot
    }

    #[getter]
    pub fn last_slot(&self) -> Option<u64> {
        self.0.last_slot
    }
}

/// Configuration object for ``getBlockProduction``.
///
/// Args:
///     identity (Optional[str]): Validator identity, as a base-58 encoded string
///     range (Optional[RpcBlockProductionConfigRange]): Slot range to query. Current epoch if ``None``.
///     commitment (Optional[CommitmentConfig]): Bank state to query.
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[pyclass(module = "solders.rpc.config", subclass)]
pub struct RpcBlockProductionConfig(rpc_config::RpcBlockProductionConfig);

impl PartialEq for RpcBlockProductionConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0.identity == other.0.identity
            && self.0.range == other.0.range
            && self.0.commitment == other.0.commitment
    }
}

rpc_config_impls!(RpcBlockProductionConfig);

#[common_methods]
#[pymethods]
impl RpcBlockProductionConfig {
    #[new]
    pub fn new(
        identity: Option<&str>,
        range: Option<RpcBlockProductionConfigRange>,
        commitment: Option<CommitmentConfig>,
    ) -> Self {
        Self(rpc_config::RpcBlockProductionConfig {
            identity: identity.map(String::from),
            range: range.map(|r| r.into()),
            commitment: commitment.map(|c| c.into()),
        })
    }

    #[getter]
    pub fn identity(&self) -> Option<String> {
        self.0.identity.clone()
    }

    #[getter]
    pub fn range(&self) -> Option<RpcBlockProductionConfigRange> {
        self.0.range.clone().map(|r| r.into())
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcBlockProductionConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getVoteAccounts``.
    /// 
    /// Args:
    ///     vote_pubkey (Optional[str]): Validator vote address, as a base-58 encoded string
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///     keep_unstaked_delinquents (Optional[bool]): Do not filter out delinquent validators with no stake.
    ///     delinquent_slot_distance (Optional[int]): Specify the number of slots behind the tip that a validator
    ///         must fall to be considered delinquent.
    ///         NOTE: For the sake of consistency between ecosystem products, it is not recommended that
    ///         this argument be specified.
    /// 
    => RpcGetVoteAccountsConfig);

rpc_config_impls!(RpcGetVoteAccountsConfig);

#[common_methods]
#[pymethods]
impl RpcGetVoteAccountsConfig {
    #[new]
    pub fn new(
        vote_pubkey: Option<&str>,
        commitment: Option<CommitmentConfig>,
        keep_unstaked_delinquents: Option<bool>,
        delinquent_slot_distance: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcGetVoteAccountsConfig {
            vote_pubkey: vote_pubkey.map(String::from),
            commitment: commitment.map(|c| c.into()),
            keep_unstaked_delinquents,
            delinquent_slot_distance,
        })
    }

    #[getter]
    pub fn vote_pubkey(&self) -> Option<String> {
        self.0.vote_pubkey.clone()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn keep_unstaked_delinquents(&self) -> Option<bool> {
        self.0.keep_unstaked_delinquents
    }

    #[getter]
    pub fn delinquent_slot_distance(&self) -> Option<u64> {
        self.0.delinquent_slot_distance
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcGetVoteAccountsConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }
}

/// Filter for ``getLargestAccounts``.
#[pyclass]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RpcLargestAccountsFilter {
    Circulating,
    NonCirculating,
}

impl From<RpcLargestAccountsFilter> for rpc_config::RpcLargestAccountsFilter {
    fn from(f: RpcLargestAccountsFilter) -> Self {
        match f {
            RpcLargestAccountsFilter::Circulating => {
                rpc_config::RpcLargestAccountsFilter::Circulating
            }
            RpcLargestAccountsFilter::NonCirculating => {
                rpc_config::RpcLargestAccountsFilter::NonCirculating
            }
        }
    }
}

impl From<rpc_config::RpcLargestAccountsFilter> for RpcLargestAccountsFilter {
    fn from(f: rpc_config::RpcLargestAccountsFilter) -> Self {
        match f {
            rpc_config::RpcLargestAccountsFilter::Circulating => {
                RpcLargestAccountsFilter::Circulating
            }
            rpc_config::RpcLargestAccountsFilter::NonCirculating => {
                RpcLargestAccountsFilter::NonCirculating
            }
        }
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getLargestAccounts``.
    ///
    /// Args:
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///     filter (Optional[RpcLargestAccountsFilter]): Filter results by account type.
    ///
    => RpcLargestAccountsConfig
);

rpc_config_impls!(RpcLargestAccountsConfig);

#[common_methods]
#[pymethods]
impl RpcLargestAccountsConfig {
    #[new]
    pub fn new(
        commitment: Option<CommitmentConfig>,
        filter: Option<RpcLargestAccountsFilter>,
    ) -> Self {
        Self(rpc_config::RpcLargestAccountsConfig {
            commitment: commitment.map(|c| c.into()),
            filter: filter.map(|f| f.into()),
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcLargestAccountsConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn filter(&self) -> Option<RpcLargestAccountsFilter> {
        self.0.filter.clone().map(|c| c.into())
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getSupply``.
    ///
    /// Args:
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///     exclude_non_circulating_accounts_list (bool): Exclude non circulating accounts list from response.
    ///
    => RpcSupplyConfig
);

rpc_config_impls!(RpcSupplyConfig);

#[common_methods]
#[pymethods]
impl RpcSupplyConfig {
    #[new]
    pub fn new(
        commitment: Option<CommitmentConfig>,
        exclude_non_circulating_accounts_list: bool,
    ) -> Self {
        Self(rpc_config::RpcSupplyConfig {
            commitment: commitment.map(|c| c.into()),
            exclude_non_circulating_accounts_list,
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSupplyConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn exclude_non_circulating_accounts_list(&self) -> bool {
        self.0.exclude_non_circulating_accounts_list
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for containing epoch information.
    ///
    /// Args:
    ///     epoch (Optional[int]): Epoch is a unit of time a given leader schedule is honored, some number of Slots.
    ///     commitment (Optional[CommitmentConfig]): Bank state to query.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    ///
    => RpcEpochConfig
);

rpc_config_impls!(RpcEpochConfig);

#[common_methods]
#[pymethods]
impl RpcEpochConfig {
    #[new]
    pub fn new(
        epoch: Option<u64>,
        commitment: Option<CommitmentConfig>,
        min_context_slot: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcEpochConfig {
            epoch,
            commitment: commitment.map(|c| c.into()),
            min_context_slot,
        })
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSupplyConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentConfig> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn epoch(&self) -> Option<u64> {
        self.0.epoch
    }

    #[getter]
    pub fn min_context_slot(&self) -> Option<u64> {
        self.0.min_context_slot
    }
}

pub fn create_config_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let config_mod = PyModule::new(py, "config")?;
    config_mod.add_class::<RpcSignatureStatusConfig>()?;
    config_mod.add_class::<RpcSendTransactionConfig>()?;
    config_mod.add_class::<RpcSimulateTransactionAccountsConfig>()?;
    config_mod.add_class::<RpcSimulateTransactionConfig>()?;
    config_mod.add_class::<RpcRequestAirdropConfig>()?;
    Ok(config_mod)
}
