from pytest import raises
from sol_rust_py import is_on_curve


def test_is_on_curve(benchmark):
    data = (
        b"\xc1M\xce\x1e\xa4\x86<\xf1\xbc\xfc\x12\xf4\xf2\xe2Y"
        b"\xf4\x8d\xe4V\xb7\xf9\xd4\\!{\x04\x89j\x1f\xfeA\xdc"
    )
    result = benchmark(is_on_curve, data)
    assert result


def test_is_on_curve_wrong_length():
    data = b"\xc1M"
    with raises(BaseException):
        is_on_curve(data)
