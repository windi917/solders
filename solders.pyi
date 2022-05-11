from typing import List, Union, Optional, ClassVar, Sequence, Tuple

BytesInput = Union[bytes, Sequence[int]]
Signer = Union["Presigner", "Keypair"]

def is_on_curve(_bytes: bytes) -> bool: ...

class Pubkey:
    LENGTH: ClassVar[int]
    def __init__(self, pubkey_bytes: BytesInput) -> None: ...
    @staticmethod
    def new_unique() -> "Pubkey": ...
    @staticmethod
    def default() -> "Pubkey": ...
    @staticmethod
    def from_string(s: str) -> "Pubkey": ...
    @staticmethod
    def create_with_seed(
        from_public_key: "Pubkey", seed: str, program_id: "Pubkey"
    ) -> "Pubkey": ...
    @staticmethod
    def create_program_address(seeds: Sequence[bytes]) -> "Pubkey": ...
    @staticmethod
    def find_program_address(
        seeds: Sequence[bytes], program_id: "Pubkey"
    ) -> "Pubkey": ...
    def is_on_curve(self) -> bool: ...
    def string(self) -> str: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def to_bytes(self) -> bytes: ...
    def __bytes__(self) -> bytes: ...
    def __richcmp__(self, other: "Pubkey", op: int) -> bool: ...
    def __hash__(self) -> int: ...

class Keypair:
    def __init__(self) -> None: ...
    @staticmethod
    def from_bytes(raw_bytes: BytesInput) -> "Keypair": ...
    @staticmethod
    def from_seed(seed: BytesInput) -> "Keypair": ...
    @staticmethod
    def from_base58_string(s: str) -> "Keypair": ...
    @staticmethod
    def from_seed_phrase_and_passphrase(
        seed_phrase: str, passphrase: str
    ) -> "Keypair": ...
    def secret(self) -> bytes: ...
    def pubkey(self) -> Pubkey: ...
    def sign_message(self, message: bytes) -> Signature: ...
    def to_bytes_array(self) -> List[int]: ...
    def to_base58_string(self) -> str: ...
    def __str__(self) -> str: ...
    def __bytes__(self) -> str: ...
    def __richcmp__(self, other: Signer, op: int) -> bool: ...
    def __hash__(self) -> int: ...
    def is_interactive(self) -> bool: ...

class Signature:
    LENGTH: ClassVar[int]
    def __init__(self, signature_slice: bytes) -> None: ...
    @staticmethod
    def new_unique() -> "Signature": ...
    @staticmethod
    def default() -> "Signature": ...
    @staticmethod
    def from_string(s: str) -> "Signature": ...
    def verify(self, pubkey_bytes: BytesInput, message_bytes: bytes) -> bool: ...
    def to_bytes_array(self) -> List[int]: ...
    def to_bytes(self) -> bytes: ...
    def __bytes__(self) -> bytes: ...
    def to_string(self) -> str: ...
    def __str__(self) -> str: ...
    def __richcmp__(self, other: "Signature", op: int) -> bool: ...
    def __hash__(self) -> int: ...

class AccountMeta:
    def __init__(self, pubkey: Pubkey, is_signer: bool, is_writable: bool) -> None: ...
    @property
    def pubkey(self) -> Pubkey: ...
    @property
    def is_signer(self) -> bool: ...
    @property
    def is_writable(self) -> bool: ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __richcmp__(self, other: "AccountMeta", op: int) -> bool: ...

class Instruction:
    def __init__(
        self, program_id: Pubkey, data: bytes, accounts: Sequence[AccountMeta]
    ) -> None: ...
    @property
    def program_id(self) -> Pubkey: ...
    @property
    def data(self) -> bytes: ...
    @property
    def accounts(self) -> List[AccountMeta]: ...
    @accounts.setter
    def accounts(self, accounts: List[AccountMeta]) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "Instruction", op: int) -> bool: ...
    def serialize(self) -> bytes: ...
    @staticmethod
    def deserialize(data: bytes) -> "Instruction": ...

class CompiledInstruction:
    def __init__(self, program_id_index: int, data: bytes, accounts: bytes) -> None: ...
    def program_id(self, program_ids: Sequence[Pubkey]) -> Pubkey: ...
    @property
    def program_id_index(self) -> int: ...
    @property
    def accounts(self) -> bytes: ...
    @accounts.setter
    def accounts(self, accounts: BytesInput) -> None: ...
    @property
    def data(self) -> bytes: ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def __richcmp__(self, other: "CompiledInstruction", op: int) -> bool: ...
    def serialize(self) -> bytes: ...
    @staticmethod
    def deserialize(data: bytes) -> "CompiledInstruction": ...

class Hash:
    def __init__(self, hash_bytes: bytes) -> None: ...
    def to_string(self) -> str: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    @staticmethod
    def from_string(s: str) -> "Hash": ...
    @staticmethod
    def new_unique() -> "Hash": ...
    @staticmethod
    def default() -> "Hash": ...
    def to_bytes(self) -> bytes: ...
    def __bytes__(self) -> bytes: ...
    def __richcmp__(self, other: "Hash", op: int) -> bool: ...
    @staticmethod
    def hash(val: bytes) -> "Hash": ...
    def __hash__(self) -> int: ...

def decode_length(raw_bytes: bytes) -> tuple[int, int]: ...
def encode_length(value: int) -> List[int]: ...

class MessageHeader:
    LENGTH: ClassVar[int]
    def __init__(
        self,
        num_required_signatures: int,
        num_readonly_signed_accounts: int,
        num_readonly_unsigned_accounts: int,
    ) -> None: ...
    @staticmethod
    def default() -> "MessageHeader": ...
    @property
    def num_required_signatures(self) -> int: ...
    @property
    def num_readonly_signed_accounts(self) -> int: ...
    @property
    def num_readonly_unsigned_accounts(self) -> int: ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...

class Message:
    def __init__(
        self,
        instructions: Sequence[Instruction],
        payer: Optional[Pubkey] = None,
    ) -> None: ...
    @property
    def header(self) -> MessageHeader: ...
    @property
    def account_keys(self) -> List[Pubkey]: ...
    @property
    def recent_blockhash(self) -> Hash: ...
    @property
    def instructions(self) -> List[CompiledInstruction]: ...
    @staticmethod
    def new_with_blockhash(
        instructions: Sequence[Instruction], payer: Optional[Pubkey], blockhash: Hash
    ) -> "Message": ...
    @staticmethod
    def new_with_compiled_instructions(
        num_required_signatures: int,
        num_readonly_signed_accounts: int,
        num_readonly_unsigned_accounts: int,
        account_keys: Sequence[Pubkey],
        recent_blockhash: Hash,
        instructions: Sequence[CompiledInstruction],
    ) -> "Message": ...
    def hash(self) -> Hash: ...
    @staticmethod
    def hash_raw_message(message_bytes: bytes) -> Hash: ...
    def compile_instruction(self, ix: Instruction) -> CompiledInstruction: ...
    def serialize(self) -> bytes: ...
    def program_id(self, instruction_index: int) -> Optional[Pubkey]: ...
    def program_index(self, instruction_index: int) -> Optional[int]: ...
    def program_ids(self) -> List[Pubkey]: ...
    def is_key_passed_to_program(self, key_index: int) -> bool: ...
    def is_key_called_as_program(self, key_index: int) -> bool: ...
    def is_non_loader_key(self, key_index: int) -> bool: ...
    def program_position(self, index: int) -> Optional[int]: ...
    def maybe_executable(self, i: int) -> bool: ...
    def is_writable(self, i: int) -> bool: ...
    def is_signer(self, i: int) -> bool: ...
    def signer_keys(self) -> List[Pubkey]: ...
    def has_duplicates(self) -> bool: ...
    @staticmethod
    def default() -> "Message": ...
    @staticmethod
    def deserialize(data: bytes) -> "Message": ...
    def __richcmp__(self, other: "Message", op: int) -> bool: ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...

class Transaction:
    def __init__(
        self,
        from_keypairs: Sequence[Signer],
        message: Message,
        recent_blockhash: Hash,
    ) -> None: ...
    @property
    def signatures(self) -> List[Signature]: ...
    @property
    def message(self) -> Message: ...
    @staticmethod
    def new_unsigned(message: Message) -> "Transaction": ...
    @staticmethod
    def new_with_payer(
        instructions: Sequence[Instruction],
        payer: Optional[Pubkey] = None,
    ) -> "Transaction": ...
    @staticmethod
    def new_signed_with_payer(
        instructions: Sequence[Instruction],
        payer: Optional[Pubkey],
        signing_keypairs: Sequence[Signer],
        recent_blockhash: Hash,
    ) -> "Transaction": ...
    @staticmethod
    def new_with_compiled_instructions(
        from_keypairs: Sequence[Signer],
        keys: Sequence[Pubkey],
        recent_blockhash: Hash,
        program_ids: Sequence[Pubkey],
        instructions: Sequence[CompiledInstruction],
    ) -> "Transaction": ...
    @staticmethod
    def populate(
        message: Message, signatures: Sequence[Signature]
    ) -> "Transaction": ...
    def data(self, instruction_index: int) -> bytes: ...
    def key(self, instruction_index: int, accounts_index: int) -> Optional[Pubkey]: ...
    def signer_key(
        self, instruction_index: int, accounts_index: int
    ) -> Optional[Pubkey]: ...
    def message_data(self) -> bytes: ...
    def sign(self, keypairs: Sequence[Signer], recent_blockhash: Hash) -> None: ...
    def partial_sign(
        self,
        keypairs: Sequence[Signer],
        recent_blockhash: Hash,
    ) -> None: ...
    def verify(self) -> None: ...
    def verify_and_hash_message(self) -> Hash: ...
    def verify_with_results(self) -> List[bool]: ...
    def get_signing_keypair_positions(
        self,
        pubkeys: Sequence[Pubkey],
    ) -> List[Optional[int]]: ...
    def replace_signatures(
        self, signers: Sequence[Tuple[Pubkey, Signature]]
    ) -> None: ...
    def is_signed(self) -> bool: ...
    def uses_durable_nonce(self) -> Optional[CompiledInstruction]: ...
    def sanitize(self) -> None: ...
    def serialize(self) -> bytes: ...
    @staticmethod
    def default() -> "Transaction": ...
    @staticmethod
    def deserialize(data: bytes) -> "Transaction": ...
    def __richcmp__(self, other: "Transaction", op: int) -> bool: ...
    def __repr__(self) -> str: ...
    def __str__(self) -> str: ...
    def get_nonce_pubkey_from_instruction(
        self, ix: CompiledInstruction
    ) -> Optional[Pubkey]: ...

class SystemProgram:
    ID: ClassVar[Pubkey]
    @staticmethod
    def create_account(
        from_pubkey: Pubkey,
        to_pubkey: Pubkey,
        lamports: int,
        space: int,
        owner: Pubkey,
    ) -> Instruction: ...
    @staticmethod
    def create_account_with_seed(
        from_pubkey: Pubkey,
        to_pubkey: Pubkey,
        base: Pubkey,
        seed: str,
        lamports: int,
        space: int,
        owner: Pubkey,
    ) -> Instruction: ...
    @staticmethod
    def assign(pubkey: Pubkey, owner: Pubkey) -> Instruction: ...
    @staticmethod
    def assign_with_seed(
        address: Pubkey,
        base: Pubkey,
        seed: str,
        owner: Pubkey,
    ) -> Instruction: ...
    @staticmethod
    def transfer(
        from_pubkey: Pubkey, to_pubkey: Pubkey, lamports: int
    ) -> Instruction: ...
    @staticmethod
    def transfer_with_seed(
        from_pubkey: Pubkey,
        from_base: Pubkey,
        from_seed: str,
        from_owner: Pubkey,
        to_pubkey: Pubkey,
        lamports: int,
    ) -> Instruction: ...
    @staticmethod
    def allocate(pubkey: Pubkey, space: int) -> Instruction: ...
    @staticmethod
    def allocate_with_seed(
        address: Pubkey,
        base: Pubkey,
        seed: str,
        space: int,
        owner: Pubkey,
    ) -> Instruction: ...
    @staticmethod
    def transfer_many(
        from_pubkey: Pubkey,
        to_lamports: Sequence[Tuple[Pubkey, int]],
    ) -> List[Instruction]: ...
    @staticmethod
    def create_nonce_account_with_seed(
        from_pubkey: Pubkey,
        nonce_pubkey: Pubkey,
        base: Pubkey,
        seed: str,
        authority: Pubkey,
        lamports: int,
    ) -> Tuple[Instruction, Instruction]: ...
    @staticmethod
    def create_nonce_account(
        from_pubkey: Pubkey,
        nonce_pubkey: Pubkey,
        authority: Pubkey,
        lamports: int,
    ) -> Tuple[Instruction, Instruction]: ...
    @staticmethod
    def advance_nonce_account(
        nonce_pubkey: Pubkey, authorized_pubkey: Pubkey
    ) -> Instruction: ...
    @staticmethod
    def withdraw_nonce_account(
        nonce_pubkey: Pubkey,
        authorized_pubkey: Pubkey,
        to_pubkey: Pubkey,
        lamports: int,
    ) -> Instruction: ...

class Sysvar:
    CLOCK: ClassVar[Pubkey]
    RECENT_BLOCKHASHES: ClassVar[Pubkey]
    RENT: ClassVar[Pubkey]
    REWARDS: ClassVar[Pubkey]
    STAKE_HISTORY: ClassVar[Pubkey]
    EPOCH_SCHEDULE: ClassVar[Pubkey]
    INSTRUCTIONS: ClassVar[Pubkey]
    SLOT_HASHES: ClassVar[Pubkey]

class Presigner:
    def __init__(self, pubkey: Pubkey, signature: Signature) -> None: ...
    def pubkey(self) -> Pubkey: ...
    def sign_message(self, message: BytesInput) -> Signature: ...
    def __richcmp__(self, other: Signer, op: int) -> bool: ...
    @staticmethod
    def default() -> "Presigner": ...
    def __repr__(self) -> str: ...

class ParseHashError(Exception): ...
class BincodeError(Exception): ...
