from functools import reduce


def fact(n):
    return reduce(lambda a, b: a*b, range(1, n+1))

def test_fact():
    assert fact(1) == 1
    assert fact(2) == 2
    assert fact(3) == 6