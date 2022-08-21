from typing import Tuple

class EpochSchedule:
    def __init__(self, slots_per_epoch: int) -> None: ...
    @property
    def slots_per_epoch(self) -> int: ...
    @property
    def leader_schedule_slot_offset(self) -> int: ...
    @property
    def warmup(self) -> bool: ...
    @property
    def first_normal_epoch(self) -> int: ...
    @property
    def first_normal_slot(self) -> int: ...
    @staticmethod
    def default() -> "EpochSchedule": ...
    @staticmethod
    def without_warmup() -> "EpochSchedule": ...
    @staticmethod
    def custom(
        slots_per_epoch: int, leader_schedule_slot_offset: int, warmup: bool
    ) -> "EpochSchedule": ...
    def get_slots_in_epoch(self, epoch: int) -> int: ...
    def get_leader_schedule_epoch(self, slot: int) -> int: ...
    def get_epoch(self, slot: int) -> int: ...
    def get_epoch_and_slot_index(self, slot: int) -> Tuple[int, int]: ...
    def get_first_slot_in_epoch(self, epoch: int) -> int: ...
    def get_last_slot_in_epoch(self, epoch: int) -> int: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __richcmp__(self, other: "EpochSchedule", op: int) -> bool: ...
    @staticmethod
    def from_bytes(raw_bytes: bytes) -> "EpochSchedule": ...
    def to_json(self) -> str: ...
    @staticmethod
    def from_json(raw: str) -> "EpochSchedule": ...
