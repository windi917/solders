use std::str::FromStr;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use solana_rpc_client_api::config as rpc_config;
use solana_sdk::commitment_config::CommitmentLevel as CommitmentLevelOriginal;
use solders_account_decoder::UiAccountEncoding;
use solders_commitment_config::CommitmentLevel;
use solders_hash::Hash as SolderHash;
use solders_macros::{common_methods, richcmp_eq_only, EnumIntoPy};
use solders_pubkey::Pubkey;
use solders_signature::Signature;
use solders_traits_core::{
    impl_display, py_from_bytes_general_via_cbor, pybytes_general_via_cbor, RichcmpEqualityOnly,
};

use solana_transaction_status::UiTransactionEncoding as UiTransactionEncodingOriginal;
use solders_rpc_config_macros::{
    pyclass_boilerplate, pyclass_boilerplate_with_default, rpc_config_impls,
};
use solders_rpc_config_no_rpc_api::{
    RpcBlockSubscribeFilter, RpcBlockSubscribeFilterMentions, RpcTokenAccountsFilterMint,
    RpcTokenAccountsFilterProgramId, RpcTransactionLogsFilter, RpcTransactionLogsFilterMentions,
};
use solders_transaction_status_enums::{TransactionDetails, UiTransactionEncoding};

pyclass_boilerplate!(
/// Configuration object for ``getSignatureStatuses``.
///
/// Args:
///     search_transaction_history:  If True, a Solana node will search its ledger cache for any signatures not found in the recent status cache
    => RpcSignatureStatusConfig
);

#[richcmp_eq_only]
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

pyclass_boilerplate_with_default!(
    /// Configuration object for ``sendTransaction``.
    ///
    /// Args:
    ///     skip_preflight (bool):  If true, skip the preflight transaction checks.
    ///     preflight_commitment (Optional[CommitmentLevel]): Commitment level to use for preflight checks.
    ///     max_retries: (Optional[int]): Maximum number of times for the RPC node to retry sending
    ///         the transaction to the leader. If this parameter not provided, the RPC node will
    ///         retry the transaction until it is finalized or until the blockhash expires.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    ///
    => RpcSendTransactionConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSendTransactionConfig {
    #[new]
    #[pyo3(signature = (skip_preflight=false, preflight_commitment=None, max_retries=None, min_context_slot=None))]
    pub fn new(
        skip_preflight: bool,
        preflight_commitment: Option<CommitmentLevel>,
        max_retries: Option<usize>,
        min_context_slot: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcSendTransactionConfig {
            skip_preflight,
            preflight_commitment: preflight_commitment.map(CommitmentLevelOriginal::from),
            encoding: Some(UiTransactionEncodingOriginal::Base64),
            max_retries,
            min_context_slot,
        })
    }

    /// bool:  If true, skip the preflight transaction checks.
    #[getter]
    pub fn skip_preflight(&self) -> bool {
        self.0.skip_preflight
    }

    /// Optional[CommitmentLevel]: Commitment level to use for preflight checks.
    #[getter]
    pub fn preflight_commitment(&self) -> Option<CommitmentLevel> {
        self.0.preflight_commitment.map(|p| p.into())
    }

    /// UiTransactionEncoding: Encoding used for the transaction data.
    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(Into::into)
    }

    /// Optional[int]: Maximum number of times for the RPC node to retry sending the transaction to the leader.
    #[getter]
    pub fn max_retries(&self) -> Option<usize> {
        self.0.max_retries
    }

    /// Optional[int]: The minimum slot that the request can be evaluated at.
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

pyclass_boilerplate_with_default!(
    /// Accounts configuration for ``simulateTransaction``.
    ///
    /// Args:
    ///     encoding (Optional[UiAccountEncoding]): Encoding for returned Account data
    ///     addresses (Sequence[Pubkey]): An array of accounts to return.
    => RpcSimulateTransactionAccountsConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSimulateTransactionAccountsConfig {
    #[new]
    pub fn new(addresses: Vec<Pubkey>, encoding: Option<UiAccountEncoding>) -> Self {
        Self(rpc_config::RpcSimulateTransactionAccountsConfig {
            encoding: encoding.map(Into::into),
            addresses: addresses.iter().map(|a| a.to_string()).collect(),
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
    pub fn addresses(&self) -> Vec<Pubkey> {
        self.0
            .addresses
            .clone()
            .iter()
            .map(|s| Pubkey::from_str(s).unwrap())
            .collect()
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiAccountEncoding> {
        self.0.encoding.map(Into::into)
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``simulateTransaction``.
    ///
    /// Args:
    ///     sig_verify (bool): If True the transaction signatures will be verified
    ///         (conflicts with ``replace_recent_blockhash``).
    ///     replace_recent_blockhash (bool): If True the transaction recent blockhash
    ///         will be replaced with the most recent blockhash
    ///         (conflicts with ``sig_verify``).
    ///     commitment (Optional[CommitmentLevel]): Commitment level at which to simulate the transaction.
    ///     accounts (Optional[RpcSimulateTransactionAccountsConfig]): Accounts configuration object.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    => RpcSimulateTransactionConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSimulateTransactionConfig {
    #[new]
    #[pyo3(signature = (sig_verify=false, replace_recent_blockhash=false, commitment=None, accounts=None, min_context_slot=None))]
    fn new(
        sig_verify: bool,
        replace_recent_blockhash: bool,
        commitment: Option<CommitmentLevel>,
        accounts: Option<RpcSimulateTransactionAccountsConfig>,
        min_context_slot: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcSimulateTransactionConfig {
            sig_verify,
            replace_recent_blockhash,
            commitment: commitment.map(|c| c.into()),
            encoding: Some(UiTransactionEncodingOriginal::Base64),
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
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(Into::into)
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

pyclass_boilerplate_with_default!(
    /// Configuration object for ``requestAirdrop``.
    /// 
    /// Args:
    ///     recent_blockhash (Optional[Hash]): The ID of a recent ledger entry.
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    /// 
=> RpcRequestAirdropConfig);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcRequestAirdropConfig {
    #[new]
    pub fn new(recent_blockhash: Option<SolderHash>, commitment: Option<CommitmentLevel>) -> Self {
        Self(rpc_config::RpcRequestAirdropConfig {
            recent_blockhash: recent_blockhash.map(|h| h.to_string()),
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
    pub fn recent_blockhash(&self) -> Option<SolderHash> {
        self.0
            .recent_blockhash
            .clone()
            .map(|s| SolderHash::from_str(&s).unwrap())
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.0.commitment.map(|c| c.into())
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getLeaderSchedule``.
    /// 
    /// Args:
    ///     identity (Optional[Pubkey]): Validator identity.
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    /// 
=> RpcLeaderScheduleConfig);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcLeaderScheduleConfig {
    #[new]
    pub fn new(identity: Option<&Pubkey>, commitment: Option<CommitmentLevel>) -> Self {
        Self(rpc_config::RpcLeaderScheduleConfig {
            identity: identity.map(|p| p.to_string()),
            commitment: commitment.map(|c| c.into()),
        })
    }

    #[getter]
    pub fn identity(&self) -> Option<Pubkey> {
        self.0
            .identity
            .clone()
            .map(|s| Pubkey::from_str(&s).unwrap())
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
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

#[richcmp_eq_only]
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
///     identity (Optional[Pubkey]): Validator identity.
///     range (Optional[RpcBlockProductionConfigRange]): Slot range to query. Current epoch if ``None``.
///     commitment (Optional[CommitmentLevel]): Bank state to query.
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

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcBlockProductionConfig {
    #[new]
    pub fn new(
        identity: Option<&Pubkey>,
        range: Option<RpcBlockProductionConfigRange>,
        commitment: Option<CommitmentLevel>,
    ) -> Self {
        Self(rpc_config::RpcBlockProductionConfig {
            identity: identity.map(|p| p.to_string()),
            range: range.map(|r| r.into()),
            commitment: commitment.map(|c| c.into()),
        })
    }

    #[getter]
    pub fn identity(&self) -> Option<Pubkey> {
        self.0
            .identity
            .clone()
            .map(|s| Pubkey::from_str(&s).unwrap())
    }

    #[getter]
    pub fn range(&self) -> Option<RpcBlockProductionConfigRange> {
        self.0.range.clone().map(|r| r.into())
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
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
    ///     vote_pubkey (Optional[Pubkey]): Validator vote address.
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    ///     keep_unstaked_delinquents (Optional[bool]): Do not filter out delinquent validators with no stake.
    ///     delinquent_slot_distance (Optional[int]): Specify the number of slots behind the tip that a validator
    ///         must fall to be considered delinquent.
    ///         NOTE: For the sake of consistency between ecosystem products, it is not recommended that
    ///         this argument be specified.
    /// 
    => RpcGetVoteAccountsConfig);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcGetVoteAccountsConfig {
    #[new]
    pub fn new(
        vote_pubkey: Option<&Pubkey>,
        commitment: Option<CommitmentLevel>,
        keep_unstaked_delinquents: Option<bool>,
        delinquent_slot_distance: Option<u64>,
    ) -> Self {
        Self(rpc_config::RpcGetVoteAccountsConfig {
            vote_pubkey: vote_pubkey.map(|p| p.to_string()),
            commitment: commitment.map(|c| c.into()),
            keep_unstaked_delinquents,
            delinquent_slot_distance,
        })
    }

    #[getter]
    pub fn vote_pubkey(&self) -> Option<Pubkey> {
        self.0
            .vote_pubkey
            .clone()
            .map(|s| Pubkey::from_str(&s).unwrap())
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
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
#[pyclass(module = "solders.rpc.config")]
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
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    ///     filter (Optional[RpcLargestAccountsFilter]): Filter results by account type.
    ///
    => RpcLargestAccountsConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcLargestAccountsConfig {
    #[new]
    pub fn new(
        commitment: Option<CommitmentLevel>,
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
    pub fn commitment(&self) -> Option<CommitmentLevel> {
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
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    ///     exclude_non_circulating_accounts_list (bool): Exclude non circulating accounts list from response.
    ///
    => RpcSupplyConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSupplyConfig {
    #[new]
    pub fn new(
        exclude_non_circulating_accounts_list: bool,
        commitment: Option<CommitmentLevel>,
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
    pub fn exclude_non_circulating_accounts_list(&self) -> bool {
        self.0.exclude_non_circulating_accounts_list
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.0.commitment.map(|c| c.into())
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object containing epoch information.
    ///
    /// Args:
    ///     epoch (Optional[int]): Epoch is a unit of time a given leader schedule is honored, some number of Slots.
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    ///
    => RpcEpochConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcEpochConfig {
    #[new]
    pub fn new(
        epoch: Option<u64>,
        commitment: Option<CommitmentLevel>,
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
    ///     RpcEpochConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
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

#[derive(FromPyObject, Clone, PartialEq, Eq, Serialize, Deserialize, Debug, EnumIntoPy)]
pub enum TransactionLogsFilterWrapper {
    Plain(RpcTransactionLogsFilter),
    Mentions(RpcTransactionLogsFilterMentions),
}

impl Default for TransactionLogsFilterWrapper {
    fn default() -> Self {
        Self::Plain(RpcTransactionLogsFilter::All)
    }
}

impl From<TransactionLogsFilterWrapper> for rpc_config::RpcTransactionLogsFilter {
    fn from(w: TransactionLogsFilterWrapper) -> Self {
        match w {
            TransactionLogsFilterWrapper::Plain(f) => match f {
                RpcTransactionLogsFilter::All => rpc_config::RpcTransactionLogsFilter::All,
                RpcTransactionLogsFilter::AllWithVotes => {
                    rpc_config::RpcTransactionLogsFilter::AllWithVotes
                }
            },
            TransactionLogsFilterWrapper::Mentions(m) => {
                rpc_config::RpcTransactionLogsFilter::Mentions(m.0)
            }
        }
    }
}

impl From<rpc_config::RpcTransactionLogsFilter> for TransactionLogsFilterWrapper {
    fn from(f: rpc_config::RpcTransactionLogsFilter) -> Self {
        match f {
            rpc_config::RpcTransactionLogsFilter::All => Self::Plain(RpcTransactionLogsFilter::All),
            rpc_config::RpcTransactionLogsFilter::AllWithVotes => {
                Self::Plain(RpcTransactionLogsFilter::AllWithVotes)
            }
            rpc_config::RpcTransactionLogsFilter::Mentions(v) => {
                Self::Mentions(RpcTransactionLogsFilterMentions(v))
            }
        }
    }
}

pyclass_boilerplate!(
    /// Configuration object for ``logsSubscribe``.
    ///
    /// Args:
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    ///
    => RpcTransactionLogsConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcTransactionLogsConfig {
    #[new]
    pub fn new(commitment: Option<CommitmentLevel>) -> Self {
        Self(rpc_config::RpcTransactionLogsConfig {
            commitment: commitment.map(|c| c.into()),
        })
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.0.commitment.map(|c| c.into())
    }
}

#[derive(FromPyObject, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIntoPy)]
pub enum RpcTokenAccountsFilterWrapper {
    Mint(RpcTokenAccountsFilterMint),
    ProgramId(RpcTokenAccountsFilterProgramId),
}

impl From<RpcTokenAccountsFilterWrapper> for rpc_config::RpcTokenAccountsFilter {
    fn from(w: RpcTokenAccountsFilterWrapper) -> Self {
        match w {
            RpcTokenAccountsFilterWrapper::Mint(m) => Self::Mint(m.0.to_string()),
            RpcTokenAccountsFilterWrapper::ProgramId(p) => Self::ProgramId(p.0.to_string()),
        }
    }
}

impl From<rpc_config::RpcTokenAccountsFilter> for RpcTokenAccountsFilterWrapper {
    fn from(f: rpc_config::RpcTokenAccountsFilter) -> Self {
        match f {
            rpc_config::RpcTokenAccountsFilter::Mint(s) => {
                Self::Mint(RpcTokenAccountsFilterMint(Pubkey::from_str(&s).unwrap()))
            }
            rpc_config::RpcTokenAccountsFilter::ProgramId(s) => Self::ProgramId(
                RpcTokenAccountsFilterProgramId(Pubkey::from_str(&s).unwrap()),
            ),
        }
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``signatureSubscribe``.
    ///
    /// Args:
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    ///     enable_received_notification (Optional[bool]): Enable received notification.
    => RpcSignatureSubscribeConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSignatureSubscribeConfig {
    #[new]
    fn new(
        commitment: Option<CommitmentLevel>,
        enable_received_notification: Option<bool>,
    ) -> Self {
        rpc_config::RpcSignatureSubscribeConfig {
            commitment: commitment.map(|c| c.into()),
            enable_received_notification,
        }
        .into()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn enable_received_notification(&self) -> Option<bool> {
        self.0.enable_received_notification
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSignatureSubscribeConfig: The default instance.
    ///
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }
}

#[derive(FromPyObject, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, EnumIntoPy)]
pub enum RpcBlockSubscribeFilterWrapper {
    All(RpcBlockSubscribeFilter),
    MentionsAccountOrProgram(RpcBlockSubscribeFilterMentions),
}

impl Default for RpcBlockSubscribeFilterWrapper {
    fn default() -> Self {
        Self::All(RpcBlockSubscribeFilter::All)
    }
}

impl From<RpcBlockSubscribeFilterWrapper> for rpc_config::RpcBlockSubscribeFilter {
    fn from(w: RpcBlockSubscribeFilterWrapper) -> Self {
        match w {
            RpcBlockSubscribeFilterWrapper::All(_) => Self::All,
            RpcBlockSubscribeFilterWrapper::MentionsAccountOrProgram(p) => {
                Self::MentionsAccountOrProgram(p.0)
            }
        }
    }
}

impl From<rpc_config::RpcBlockSubscribeFilter> for RpcBlockSubscribeFilterWrapper {
    fn from(f: rpc_config::RpcBlockSubscribeFilter) -> Self {
        match f {
            rpc_config::RpcBlockSubscribeFilter::All => Self::All(RpcBlockSubscribeFilter::All),
            rpc_config::RpcBlockSubscribeFilter::MentionsAccountOrProgram(p) => {
                Self::MentionsAccountOrProgram(RpcBlockSubscribeFilterMentions(p))
            }
        }
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``blockSubscribe``.
    ///
    /// Args:
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    ///     encoding (Optional[UiTransactionEncoding]): Encoding used for the transaction data.
    ///     transaction_details (Optional[TransactionDetails]): Level of transaction detail to return.
    ///     show_rewards (Optional[bool]): Whether to populate the ``rewards`` array.
    ///     max_supported_transaction_version (Optional[int]): Set the max transaction version to return in responses.
    ///
    => RpcBlockSubscribeConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcBlockSubscribeConfig {
    #[new]
    fn new(
        commitment: Option<CommitmentLevel>,
        encoding: Option<UiTransactionEncoding>,
        transaction_details: Option<TransactionDetails>,
        show_rewards: Option<bool>,
        max_supported_transaction_version: Option<u8>,
    ) -> Self {
        rpc_config::RpcBlockSubscribeConfig {
            commitment: commitment.map(|c| c.into()),
            encoding: encoding.map(Into::into),
            transaction_details: transaction_details.map(Into::into),
            show_rewards,
            max_supported_transaction_version,
        }
        .into()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(Into::into)
    }

    #[getter]
    pub fn transaction_details(&self) -> Option<TransactionDetails> {
        self.0.transaction_details.map(Into::into)
    }

    #[getter]
    pub fn show_rewards(&self) -> Option<bool> {
        self.0.show_rewards
    }

    #[getter]
    pub fn max_supported_transaction_version(&self) -> Option<u8> {
        self.0.max_supported_transaction_version
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcBlockSubscribeConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getSignaturesForAddress``.
    ///
    /// Args:
    ///     before (Optional[Signature]): Start searching backwards from this transaction signature.
    ///     until (Optional[Signature]): Search until this transaction signature.
    ///     limit (Optional[int]): Maximum transaction signatures to return (between 1 and 1,000, default: 1,000).
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    ///
    => RpcSignaturesForAddressConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcSignaturesForAddressConfig {
    #[new]
    fn new(
        before: Option<&Signature>,
        until: Option<&Signature>,
        limit: Option<usize>,
        commitment: Option<CommitmentLevel>,
        min_context_slot: Option<u64>,
    ) -> Self {
        rpc_config::RpcSignaturesForAddressConfig {
            before: before.map(|sig| sig.to_string()),
            until: until.map(|sig| sig.to_string()),
            limit,
            commitment: commitment.map(|c| c.into()),
            min_context_slot,
        }
        .into()
    }

    #[getter]
    pub fn before(&self) -> Option<Signature> {
        self.0
            .before
            .clone()
            .map(|s| Signature::from_str(&s).unwrap())
    }

    #[getter]
    pub fn until(&self) -> Option<Signature> {
        self.0
            .until
            .clone()
            .map(|s| Signature::from_str(&s).unwrap())
    }

    #[getter]
    pub fn limit(&self) -> Option<usize> {
        self.0.limit
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn min_context_slot(&self) -> Option<u64> {
        self.0.min_context_slot
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcSignaturesForAddressConfig: The default instance.
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getBlock``.
    ///
    /// Args:
    ///     encoding (Optional[UiTransactionEncoding]): Encoding used for the transaction data.
    ///     transaction_details (Optional[TransactionDetails]): Level of transaction detail to return.
    ///     rewards (Optional[bool]): Whether to populate the ``rewards`` array.
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    ///     max_supported_transaction_version (Optional[int]): Set the max transaction version to return in responses.
    ///
    => RpcBlockConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcBlockConfig {
    #[new]
    pub fn new(
        encoding: Option<UiTransactionEncoding>,
        transaction_details: Option<TransactionDetails>,
        rewards: Option<bool>,
        commitment: Option<CommitmentLevel>,
        max_supported_transaction_version: Option<u8>,
    ) -> Self {
        rpc_config::RpcBlockConfig {
            encoding: encoding.map(Into::into),
            transaction_details: transaction_details.map(Into::into),
            rewards,
            commitment: commitment.map(|c| c.into()),
            max_supported_transaction_version,
        }
        .into()
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(Into::into)
    }

    #[getter]
    pub fn transaction_details(&self) -> Option<TransactionDetails> {
        self.0.transaction_details.map(Into::into)
    }

    #[getter]
    pub fn rewards(&self) -> Option<bool> {
        self.0.rewards
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn max_supported_transaction_version(&self) -> Option<u8> {
        self.0.max_supported_transaction_version
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcBlockConfig: The default instance.
    ///
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    /// Create a new instance for only showing rewards.
    #[staticmethod]
    pub fn rewards_only() -> Self {
        rpc_config::RpcBlockConfig::rewards_only().into()
    }

    /// Create a new instance for only showing rewards, with a specified commitment level.
    #[staticmethod]
    pub fn rewards_with_commitment(commitment: Option<CommitmentLevel>) -> Self {
        rpc_config::RpcBlockConfig::rewards_with_commitment(commitment.map(|c| c.into())).into()
    }
}

pyclass_boilerplate_with_default!(
    /// Configuration object for ``getTransaction``.
    ///
    /// Args:
    ///     encoding (Optional[UiTransactionEncoding]): Encoding used for the transaction data.
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    ///     max_supported_transaction_version (Optional[int]): Set the max transaction version to return in responses.
    ///
    => RpcTransactionConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcTransactionConfig {
    #[new]
    pub fn new(
        encoding: Option<UiTransactionEncoding>,
        commitment: Option<CommitmentLevel>,
        max_supported_transaction_version: Option<u8>,
    ) -> Self {
        rpc_config::RpcTransactionConfig {
            encoding: encoding.map(Into::into),
            commitment: commitment.map(|c| c.into()),
            max_supported_transaction_version,
        }
        .into()
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcTransactionConfig: The default instance.
    ///
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn encoding(&self) -> Option<UiTransactionEncoding> {
        self.0.encoding.map(Into::into)
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn max_supported_transaction_version(&self) -> Option<u8> {
        self.0.max_supported_transaction_version
    }
}

pyclass_boilerplate_with_default!(
    /// General context configuration.
    ///
    /// Args:
    ///     commitment (Optional[CommitmentLevel]): Bank state to query.
    ///     min_context_slot (Optional[int]): The minimum slot that the request can be evaluated at.
    => RpcContextConfig
);

#[richcmp_eq_only]
#[common_methods]
#[pymethods]
impl RpcContextConfig {
    #[new]
    pub fn new(commitment: Option<CommitmentLevel>, min_context_slot: Option<u64>) -> Self {
        rpc_config::RpcContextConfig {
            commitment: commitment.map(|c| c.into()),
            min_context_slot,
        }
        .into()
    }

    /// Create a new default instance of this class.
    ///
    /// Returns:
    ///     RpcContextConfig: The default instance.
    ///
    #[staticmethod]
    #[pyo3(name = "default")]
    pub fn new_default() -> Self {
        Self::default()
    }

    #[getter]
    pub fn commitment(&self) -> Option<CommitmentLevel> {
        self.0.commitment.map(|c| c.into())
    }

    #[getter]
    pub fn min_context_slot(&self) -> Option<u64> {
        self.0.min_context_slot
    }
}
