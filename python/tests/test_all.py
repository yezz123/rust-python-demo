import rust_calculator


def test_sum_as_string():
    assert rust_calculator.sum_as_string(1, 1) == "2"
