from typing import ClassVar, Sequence, Union, Tuple

class Pubkey:
    LENGTH: ClassVar[int]
    def __init__(self, pubkey_bytes: Union[bytes, Sequence[int]]) -> None: ...
    @staticmethod
    def new_unique() -> "Pubkey": ...
    @staticmethod
    def default() -> "Pubkey": ...
    @staticmethod
    def from_string(s: str) -> "Pubkey": ...
    @staticmethod
    def create_with_seed(
        base: "Pubkey", seed: str, program_id: "Pubkey"
    ) -> "Pubkey": ...
    @staticmethod
    def create_program_address(
        seeds: Sequence[bytes], program_id: "Pubkey"
    ) -> "Pubkey": ...
    @staticmethod
    def find_program_address(
        seeds: Sequence[bytes], program_id: "Pubkey"
    ) -> Tuple["Pubkey", int]: ...
    def is_on_curve(self) -> bool: ...
    def string(self) -> str: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __bytes__(self) -> bytes: ...
    def __richcmp__(self, other: "Pubkey", op: int) -> bool: ...
    def __hash__(self) -> int: ...
