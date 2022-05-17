from solders.pubkey import Pubkey
from solders.instruction import AccountMeta

PUBKEY = Pubkey.default()


def test_eq() -> None:
    am1 = AccountMeta(PUBKEY, True, True)
    am2 = AccountMeta(PUBKEY, True, True)
    assert am1 == am2


def test_attributes() -> None:
    am = AccountMeta(PUBKEY, True, True)
    assert am.pubkey == PUBKEY
    assert am.is_signer
    assert am.is_writable
