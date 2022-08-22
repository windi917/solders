from __future__ import annotations
from typing import Sequence, List, Optional, TypeVar, Union, Tuple, Dict
from solders.hash import Hash
from solders.account import Account, AccountJSON
from solders.transaction_status import (
    EncodedTransactionWithStatusMeta,
    Reward,
)
from solders.signature import Signature
from solders.pubkey import Pubkey
from solders.epoch_schedule import EpochSchedule
from solders.rpc.errors import RpcCustomError
from solders.transaction import VersionedTransaction
from solders.transaction_status import TransactionErrorType, TransactionReturnData

class RpcResponseContext:
    slot: int
    api_version: Optional[str]
    def __init__(self, slot: int, api_version: Optional[str] = None) -> None: ...

class RpcError:
    code: int
    message: str
    data: Optional[RpcCustomError]
    def __init__(
        self, code: int, message: str, data: Optional[RpcCustomError] = None
    ) -> None: ...

T = TypeVar("T")
Resp = Union[RpcError, T]

class GetAccountInfoResp:
    context: RpcResponseContext
    value: Optional[Account]
    def __init__(
        self, value: Optional[Account], context: RpcResponseContext
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetAccountInfoResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetAccountInfoResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetAccountInfoJsonParsedResp:
    context: RpcResponseContext
    value: Optional[AccountJSON]
    def __init__(
        self, value: Optional[AccountJSON], context: RpcResponseContext
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetAccountInfoJsonParsedResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetAccountInfoJsonParsedResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetBalanceResp:
    context: RpcResponseContext
    value: int
    def __init__(self, value: int, context: RpcResponseContext) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetBalanceResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetBalanceResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetBlockCommitmentResp:
    commitment: Optional[List[int]]
    total_stake: int
    def __init__(
        self, commitment: Optional[Sequence[int]], total_stake: int
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetBlockCommitmentResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetBlockCommitmentResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetBlockHeightResp:
    def __init__(self, height: int) -> None: ...
    @property
    def height(self) -> int: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetBlockHeightResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetBlockHeightResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class RpcBlockProductionRange:
    def __init__(
        self,
        first_slot: int,
        last_slot: int,
    ) -> None: ...
    @property
    def first_slot(self) -> int: ...
    @property
    def last_slot(self) -> int: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> RpcBlockProductionRange: ...
    @staticmethod
    def from_bytes(data: bytes) -> RpcBlockProductionRange: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class RpcBlockProduction:
    def __init__(
        self,
        by_identity: Dict[Pubkey, Tuple[int, int]],
        range: RpcBlockProductionRange,
    ) -> None: ...
    @property
    def by_identity(self) -> Dict[Pubkey, Tuple[int, int]]: ...
    @property
    def range(self) -> RpcBlockProductionRange: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> RpcBlockProduction: ...
    @staticmethod
    def from_bytes(data: bytes) -> RpcBlockProduction: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetBlockProductionResp:
    value: RpcBlockProduction
    context: RpcResponseContext
    def __init__(
        self, value: RpcBlockProduction, context: RpcResponseContext
    ) -> None: ...
    @property
    def height(self) -> int: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetBlockProductionResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetBlockProductionResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetBlockResp:
    previous_blockhash: Hash
    blockhash: Hash
    parent_slot: int
    transactions: Optional[List[EncodedTransactionWithStatusMeta]]
    signatures: Optional[List[Signature]]
    rewards: Optional[List[Reward]]
    block_time: Optional[int]
    block_height: Optional[int]
    def __init__(
        self,
        previous_blockhash: Hash,
        blockhash: Hash,
        parent_slot: int,
        transactions: Optional[Sequence[EncodedTransactionWithStatusMeta]] = None,
        signatures: Optional[Sequence[Signature]] = None,
        rewards: Optional[Sequence[Reward]] = None,
        block_time: Optional[int] = None,
        block_height: Optional[int] = None,
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetBlockResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetBlockResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetBlocksResp:
    def __init__(self, blocks: List[int]) -> None: ...
    @property
    def blocks(self) -> List[int]: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetBlocksResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetBlocksResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetBlockTimeResp:
    def __init__(self, time: Optional[int] = None) -> None: ...
    @property
    def time(self) -> Optional[int]: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetBlockTimeResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetBlockTimeResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class RpcContactInfo:
    pubkey: Pubkey
    gossip: Optional[str]
    tpu: Optional[str]
    rpc: Optional[str]
    version: Optional[str]
    feature_set: Optional[int]
    shred_version: Optional[int]
    def __init__(
        self,
        pubkey: Pubkey,
        gossip: Optional[str] = None,
        tpu: Optional[str] = None,
        rpc: Optional[str] = None,
        version: Optional[str] = None,
        feature_set: Optional[int] = None,
        shred_version: Optional[int] = None,
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> RpcContactInfo: ...
    @staticmethod
    def from_bytes(data: bytes) -> RpcContactInfo: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetClusterNodesResp:
    def __init__(self, nodes: Sequence[RpcContactInfo]) -> None: ...
    @property
    def nodes(self) -> List[RpcContactInfo]: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetClusterNodesResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetClusterNodesResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class EpochInfo:
    epoch: int
    slot_index: int
    slots_in_epoch: int
    absolute_slot: int
    block_height: int
    transaction_count: Optional[int]
    def __init__(
        self,
        epoch: int,
        slot_index: int,
        slots_in_epoch: int,
        absolute_slot: int,
        block_height: int,
        transaction_count: Optional[int] = None,
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> EpochInfo: ...
    @staticmethod
    def from_bytes(data: bytes) -> EpochInfo: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetEpochInfoResp:
    def __init__(self, info: EpochInfo) -> None: ...
    @property
    def info(self) -> EpochInfo: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetEpochInfoResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetEpochInfoResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetEpochScheduleResp:
    def __init__(self, schedule: EpochSchedule) -> None: ...
    @property
    def schedule(self) -> EpochSchedule: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetEpochScheduleResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetEpochScheduleResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetFeeForMessageResp:
    context: RpcResponseContext
    value: Optional[int]
    def __init__(self, value: Optional[int], context: RpcResponseContext) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetFeeForMessageResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetFeeForMessageResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetFirstAvailableBlockResp:
    def __init__(self, slot: int) -> None: ...
    @property
    def slot(self) -> int: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetFirstAvailableBlockResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetFirstAvailableBlockResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetGenesisHashResp:
    def __init__(self, value: Hash) -> None: ...
    @property
    def value(self) -> Hash: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetGenesisHashResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetGenesisHashResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetHealthResp:
    def __init__(self, health: str) -> None: ...
    @property
    def health(self) -> str: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetHealthResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetHealthResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class RpcSimulateTransactionResult:
    err: Optional[TransactionErrorType]
    logs: Optional[List[str]]
    accounts: Optional[List[Optional[Account]]]
    units_consumed: Optional[int]
    return_data: Optional[TransactionReturnData]
    def __init__(
        self,
        err: Optional[TransactionErrorType] = None,
        logs: Optional[Sequence[str]] = None,
        accounts: Optional[Sequence[Optional[Account]]] = None,
        units_consumed: Optional[int] = None,
        return_data: Optional[TransactionReturnData] = None,
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> RpcSimulateTransactionResult: ...
    @staticmethod
    def from_bytes(data: bytes) -> RpcSimulateTransactionResult: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class RpcSnapshotSlotInfo:
    def __init__(self, full: int, incremental: Optional[int] = None) -> None: ...
    @property
    def full(self) -> int: ...
    @property
    def incremental(self) -> Optional[int]: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> RpcSnapshotSlotInfo: ...
    @staticmethod
    def from_bytes(data: bytes) -> RpcSnapshotSlotInfo: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetHighestSnapshotSlotResp:
    def __init__(self, info: RpcSnapshotSlotInfo) -> None: ...
    @property
    def info(self) -> RpcSnapshotSlotInfo: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetHighestSnapshotSlotResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetHighestSnapshotSlotResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class RpcIdentity:
    identity: Pubkey
    def __init__(self, identity: Pubkey) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> RpcIdentity: ...
    @staticmethod
    def from_bytes(data: bytes) -> RpcIdentity: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetIdentityResp:
    def __init__(self, value: RpcIdentity) -> None: ...
    @property
    def value(self) -> RpcIdentity: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetIdentityResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetIdentityResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class RpcInflationGovernor:
    def __init__(
        self,
        initial: float,
        terminal: float,
        taper: float,
        foundation: float,
        foundation_term: float,
    ) -> None: ...
    @staticmethod
    def from_json(raw: str) -> RpcInflationGovernor: ...
    @staticmethod
    def from_bytes(data: bytes) -> RpcInflationGovernor: ...
    @property
    def initial(self) -> float: ...
    @property
    def terminal(self) -> float: ...
    @property
    def taper(self) -> float: ...
    @property
    def foundation(self) -> float: ...
    @property
    def foundation_term(self) -> float: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetInflationGovernorResp:
    def __init__(self, governor: RpcInflationGovernor) -> None: ...
    @property
    def governor(self) -> RpcInflationGovernor: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetInflationGovernorResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetInflationGovernorResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class RpcInflationRate:
    def __init__(
        self, total: float, validator: float, foundation: float, epoch: int
    ) -> None: ...
    @staticmethod
    def from_json(raw: str) -> RpcInflationRate: ...
    @staticmethod
    def from_bytes(data: bytes) -> RpcInflationRate: ...
    @property
    def total(self) -> float: ...
    @property
    def validator(self) -> float: ...
    @property
    def foundation(self) -> float: ...
    @property
    def epoch(self) -> int: ...
    @property
    def foundation_term(self) -> float: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetInflationRateResp:
    def __init__(self, rate: RpcInflationRate) -> None: ...
    @property
    def rate(self) -> RpcInflationRate: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetInflationRateResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetInflationRateResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class RpcInflationReward:
    def __init__(
        self,
        epoch: int,
        effective_slot: int,
        amount: int,
        post_balance: int,
        commission: Optional[int] = None,
    ) -> None: ...
    @staticmethod
    def from_json(raw: str) -> RpcInflationReward: ...
    @staticmethod
    def from_bytes(data: bytes) -> RpcInflationReward: ...
    @property
    def epoch(self) -> int: ...
    @property
    def effective_slot(self) -> int: ...
    @property
    def amount(self) -> int: ...
    @property
    def post_balance(self) -> int: ...
    @property
    def commission(self) -> Optional[int]: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetInflationRewardResp:
    def __init__(self, rewards: Sequence[Optional[RpcInflationReward]]) -> None: ...
    @property
    def rewards(self) -> Sequence[Optional[RpcInflationReward]]: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetInflationRewardResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetInflationRewardResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class RpcAccountBalance:
    def __init__(
        self,
        address: Pubkey,
        lamports: int,
    ) -> None: ...
    @staticmethod
    def from_json(raw: str) -> RpcAccountBalance: ...
    @staticmethod
    def from_bytes(data: bytes) -> RpcAccountBalance: ...
    @property
    def address(self) -> Pubkey: ...
    @property
    def lamports(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetLargestAccountsResp:
    context: RpcResponseContext
    value: List[RpcAccountBalance]
    def __init__(
        self, value: Sequence[RpcAccountBalance], context: RpcResponseContext
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetLargestAccountsResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetLargestAccountsResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class RpcBlockhash:
    blockhash: Hash
    last_valid_block_height: int
    def __init__(self, blockhash: Hash, last_valid_block_height: int) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> RpcBlockhash: ...
    @staticmethod
    def from_bytes(data: bytes) -> RpcBlockhash: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetLatestBlockhashResp:
    context: RpcResponseContext
    value: RpcBlockhash
    def __init__(self, value: RpcBlockhash, context: RpcResponseContext) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetLatestBlockhashResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetLatestBlockhashResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetLeaderScheduleResp:
    def __init__(
        self, schedule: Optional[Dict[Pubkey, Sequence[int]]] = None
    ) -> None: ...
    @property
    def schedule(self) -> Optional[Dict[Pubkey, List[int]]]: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetLatestBlockhashResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetLatestBlockhashResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetMaxRetransmitSlotResp:
    def __init__(self, slot: int) -> None: ...
    @property
    def slot(self) -> int: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetMaxRetransmitSlotResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetMaxRetransmitSlotResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetMaxShredInsertSlotResp:
    def __init__(self, slot: int) -> None: ...
    @property
    def slot(self) -> int: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetMaxShredInsertSlotResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetMaxShredInsertSlotResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetMinimumBalanceForRentExemption:
    def __init__(self, slot: int) -> None: ...
    @property
    def slot(self) -> int: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetMinimumBalanceForRentExemption]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetMinimumBalanceForRentExemption: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetMultipleAccountsResp:
    context: RpcResponseContext
    value: List[Optional[Account]]
    def __init__(
        self, value: Sequence[Optional[Account]], context: RpcResponseContext
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetAccountInfoResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetAccountInfoResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class RpcKeyedAccount:
    pubkey: Pubkey
    account: Account
    def __init__(self, pubkey: Pubkey, account: Account) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> RpcKeyedAccount: ...
    @staticmethod
    def from_bytes(data: bytes) -> RpcKeyedAccount: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetProgramAccountsWithContextResp:
    context: RpcResponseContext
    value: List[RpcKeyedAccount]
    def __init__(
        self, value: Sequence[RpcKeyedAccount], context: RpcResponseContext
    ) -> None: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetProgramAccountsWithContextResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetProgramAccountsWithContextResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...

class GetProgramAccountsWithoutContextResp:
    def __init__(self, accounts: Sequence[RpcKeyedAccount]) -> None: ...
    @property
    def accounts(self) -> List[RpcKeyedAccount]: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> Resp[GetProgramAccountsWithoutContextResp]: ...
    @staticmethod
    def from_bytes(data: bytes) -> GetProgramAccountsWithoutContextResp: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __hash__(self) -> int: ...
