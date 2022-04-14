from vector import Vector


def test_basic_op():
    va = Vector(1, 2)
    vb = Vector(1, 3)
    vc = va + vb
    assert vc == Vector(2, 5)