from typing import List, Optional, Sequence, Tuple

from solders.account import Account
from solders.clock import Clock
from solders.commitment_config import CommitmentLevel
from solders.hash import Hash
from solders.keypair import Keypair
from solders.message import Message
from solders.pubkey import Pubkey
from solders.rent import Rent
from solders.signature import Signature
from solders.transaction import VersionedTransaction
from solders.transaction_status import (
    TransactionErrorType,
    TransactionReturnData,
    TransactionStatus,
)

class BanksClient:
    async def get_account(
        self, address: Pubkey, commitment: Optional[CommitmentLevel] = None
    ) -> Account: ...
    async def get_balance(
        self,
        address: Pubkey,
        commitment: Optional[CommitmentLevel] = None,
    ) -> int: ...
    async def get_block_height(
        self, commitment: Optional[CommitmentLevel] = None
    ) -> int: ...
    async def get_clock(self) -> Clock: ...
    async def get_fee_for_message(
        self,
        message: Message,
        commitment: Optional[CommitmentLevel] = None,
    ) -> Optional[int]: ...
    async def get_latest_blockhash(
        self, commitment: Optional[CommitmentLevel] = None
    ) -> Tuple[Hash, int]: ...
    async def get_rent(self) -> Rent: ...
    async def get_slot(self, commitment: Optional[CommitmentLevel] = None) -> int: ...
    async def get_transaction_status(
        self, signature: Signature
    ) -> Optional[TransactionStatus]: ...
    async def get_transaction_statuses(
        self, signatures: Sequence[Signature]
    ) -> List[Optional[Signature]]: ...
    async def process_transaction(
        self,
        transaction: VersionedTransaction,
        commitment: Optional[CommitmentLevel] = None,
    ) -> None: ...
    async def process_transaction_with_metadata(
        self, Transaction: VersionedTransaction
    ) -> BanksTransactionResultWithMeta: ...
    async def process_transaction_with_preflight(
        self,
        transaction: VersionedTransaction,
        commitment: Optional[CommitmentLevel] = None,
    ) -> None: ...
    async def send_transaction(self, transaction: VersionedTransaction) -> None: ...
    async def simulate_transaction(
        self,
        transaction: VersionedTransaction,
        commitment: Optional[CommitmentLevel] = None,
    ) -> BanksTransactionResultWithMeta: ...

class BanksClientError(Exception): ...

class BanksTransactionMeta:
    @property
    def compute_units_consumed(self) -> int: ...
    @property
    def log_messages(self) -> List[str]: ...
    @property
    def return_data(self) -> Optional[TransactionReturnData]: ...
    def __init__(
        self,
        compute_units_consumed: int,
        log_messages: Sequence[str],
        return_data: Optional[TransactionReturnData] = None,
    ) -> None: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "BanksTransactionMeta": ...
    @staticmethod
    def from_json(raw: str) -> "BanksTransactionMeta": ...
    def to_json(self) -> str: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "BanksTransactionMeta", op: int) -> bool: ...

class BanksTransactionResultWithMeta:
    @property
    def meta(self) -> Optional[BanksTransactionMeta]: ...
    @property
    def result(self) -> Optional[TransactionErrorType]: ...
    def __init__(
        self,
        result: Optional[TransactionErrorType] = None,
        meta: Optional[BanksTransactionMeta] = None,
    ) -> None: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "BanksTransactionResultWithMeta": ...
    @staticmethod
    def from_json(raw: str) -> "BanksTransactionResultWithMeta": ...
    def to_json(self) -> str: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "BanksTransactionResultWithMeta", op: int) -> bool: ...

class ProgramTestContext:
    @property
    def banks_client(self) -> BanksClient: ...
    @property
    def last_blockhash(self) -> Hash: ...
    @property
    def payer(self) -> Keypair: ...
    def increment_vote_account_credits(
        self,
        vote_account_address: Pubkey,
        number_of_credits: int,
    ) -> None: ...
    def set_account(self, address: Pubkey, account: Account) -> None: ...
    def set_clock(self, clock: Clock) -> None: ...
    def set_rent(self, rent: Rent) -> None: ...
    def warp_to_slot(self, warp_slot: int) -> None: ...

async def start(
    programs: Optional[Sequence[Tuple[str, Pubkey]]] = None,
    accounts: Optional[Sequence[Tuple[Pubkey, Account]]] = None,
    compute_max_units: Optional[int] = None,
    transaction_account_lock_limit: Optional[int] = None,
    use_bpf_jit: Optional[bool] = None,
) -> ProgramTestContext: ...
