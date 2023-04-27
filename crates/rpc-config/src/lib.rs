use pyo3::prelude::*;
use solders_rpc_account_info_config::RpcAccountInfoConfig;
use solders_rpc_config_no_filter::{
    RpcBlockConfig, RpcBlockProductionConfig, RpcBlockProductionConfigRange,
    RpcBlockSubscribeConfig, RpcBlockSubscribeFilter, RpcBlockSubscribeFilterMentions,
    RpcContextConfig, RpcEpochConfig, RpcGetVoteAccountsConfig, RpcLargestAccountsFilter,
    RpcLeaderScheduleConfig, RpcRequestAirdropConfig, RpcSendTransactionConfig,
    RpcSignatureStatusConfig, RpcSignatureSubscribeConfig, RpcSignaturesForAddressConfig,
    RpcSimulateTransactionAccountsConfig, RpcSimulateTransactionConfig, RpcSupplyConfig,
    RpcTokenAccountsFilterMint, RpcTokenAccountsFilterProgramId, RpcTransactionConfig,
    RpcTransactionLogsConfig, RpcTransactionLogsFilter, RpcTransactionLogsFilterMentions,
};
use solders_rpc_program_accounts_config::RpcProgramAccountsConfig;

pub fn create_config_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let config_mod = PyModule::new(py, "config")?;
    config_mod.add_class::<RpcSignatureStatusConfig>()?;
    config_mod.add_class::<RpcSendTransactionConfig>()?;
    config_mod.add_class::<RpcSimulateTransactionAccountsConfig>()?;
    config_mod.add_class::<RpcSimulateTransactionConfig>()?;
    config_mod.add_class::<RpcRequestAirdropConfig>()?;
    config_mod.add_class::<RpcLeaderScheduleConfig>()?;
    config_mod.add_class::<RpcBlockProductionConfigRange>()?;
    config_mod.add_class::<RpcBlockProductionConfig>()?;
    config_mod.add_class::<RpcGetVoteAccountsConfig>()?;
    config_mod.add_class::<RpcLargestAccountsFilter>()?;
    config_mod.add_class::<RpcSupplyConfig>()?;
    config_mod.add_class::<RpcEpochConfig>()?;
    config_mod.add_class::<RpcAccountInfoConfig>()?;
    config_mod.add_class::<RpcProgramAccountsConfig>()?;
    config_mod.add_class::<RpcTransactionLogsFilter>()?;
    config_mod.add_class::<RpcTransactionLogsFilterMentions>()?;
    config_mod.add_class::<RpcTransactionLogsConfig>()?;
    config_mod.add_class::<RpcTokenAccountsFilterMint>()?;
    config_mod.add_class::<RpcTokenAccountsFilterProgramId>()?;
    config_mod.add_class::<RpcSignatureSubscribeConfig>()?;
    config_mod.add_class::<RpcBlockSubscribeFilter>()?;
    config_mod.add_class::<RpcBlockSubscribeFilterMentions>()?;
    config_mod.add_class::<RpcBlockSubscribeConfig>()?;
    config_mod.add_class::<RpcSignaturesForAddressConfig>()?;
    config_mod.add_class::<RpcBlockConfig>()?;
    config_mod.add_class::<RpcTransactionConfig>()?;
    config_mod.add_class::<RpcContextConfig>()?;
    Ok(config_mod)
}
