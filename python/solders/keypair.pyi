from typing import ClassVar, List, Sequence, Union

from solders.presigner import Presigner
from solders.pubkey import Pubkey
from solders.signature import Signature

class Keypair:
    LENGTH: ClassVar[int]
    def __init__(self) -> None: ...
    @staticmethod
    def from_bytes(raw_bytes: Union[bytes, Sequence[int]]) -> "Keypair": ...
    @staticmethod
    def from_seed(seed: Union[bytes, Sequence[int]]) -> "Keypair": ...
    @staticmethod
    def from_seed_and_derivation_path(seed: Union[bytes, Sequence[int]], dpath: str) -> "Keypair": ...
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
    def __str__(self) -> str: ...
    def __bytes__(self) -> bytes: ...
    def __richcmp__(self, other: Union[Presigner, "Keypair"], op: int) -> bool: ...
    def __hash__(self) -> int: ...
    def is_interactive(self) -> bool: ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "Keypair": ...
