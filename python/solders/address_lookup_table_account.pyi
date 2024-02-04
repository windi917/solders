from typing import List, Optional, Sequence, Tuple, Union

from typing_extensions import Final

from solders.hash import Hash
from solders.pubkey import Pubkey

ID: Final[Pubkey]
LOOKUP_TABLE_MAX_ADDRESSES: Final[int]
LOOKUP_TABLE_META_SIZE: Final[int]

class AddressLookupTableAccount:
    def __init__(
        self,
        key: Pubkey,
        addresses: Sequence[Pubkey],
    ) -> None: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "AddressLookupTableAccount", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "AddressLookupTableAccount": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "AddressLookupTableAccount": ...
    @property
    def key(self) -> Pubkey: ...
    @property
    def addresses(self) -> List[Pubkey]: ...

class AddressLookupTable:
    def __init__(self, key: Pubkey, addresses: Sequence[Pubkey]) -> None: ...
    def get_active_addresses_len(
        self, current_slot: int, slot_hashes: SlotHashes
    ) -> int: ...
    def lookup(
        self,
        current_slot: int,
        indexes: Sequence[int],
        slot_hashes: SlotHashes,
    ) -> List[Pubkey]: ...
    @staticmethod
    def deserialize(data: bytes) -> AddressLookupTable: ...
    @property
    def meta(self) -> LookupTableMeta: ...
    @property
    def addresses(self) -> List[Pubkey]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "AddressLookupTable", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "AddressLookupTable": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "AddressLookupTable": ...

class LookupTableMeta:
    def __init__(
        self,
        deactivation_slot: int = 18446744073709551615,  # u64::MAX
        last_extended_slot: int = 0,
        last_extended_slot_start_index: int = 0,
        authority: Optional[Pubkey] = None,
        padding: int = 0,
    ) -> None: ...
    def is_active(self, current_slot: int, slot_hashes: SlotHashes) -> bool: ...
    def status(
        self, current_slot: int, slot_hashes: SlotHashes
    ) -> LookupTableStatusType: ...
    @property
    def deactivation_slot(self) -> int: ...
    @property
    def last_extended_slot(self) -> int: ...
    @property
    def last_extended_slot_start_index(self) -> int: ...
    @property
    def authority(self) -> Optional[Pubkey]: ...
    @property
    def padding(self) -> int: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "LookupTableMeta", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "LookupTableMeta": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "LookupTableMeta": ...

class LookupTableStatusFieldless:
    Activated: "LookupTableStatusFieldless"
    Deactivated: "LookupTableStatusFieldless"
    def __int__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __eq__(self, o: object) -> bool: ...

class LookupTableStatusDeactivating:
    def __init__(self, remaining_slots: int) -> None: ...
    @property
    def remaining_slots(self) -> int: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "LookupTableStatusDeactivating", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "LookupTableStatusDeactivating": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "LookupTableStatusDeactivating": ...

class SlotHashes:
    def __init__(self, slot_hashes: Sequence[Tuple[int, Hash]]) -> None: ...
    @property
    def slot_hashes(self) -> List[Tuple[int, Hash]]: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "SlotHashes", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "SlotHashes": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "SlotHashes": ...

LookupTableStatusType = Union[LookupTableStatusFieldless, LookupTableStatusDeactivating]

def derive_lookup_table_address(
    authority_address: Pubkey, recent_block_slot: int
) -> Tuple[Pubkey, int]: ...
