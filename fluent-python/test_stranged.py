
def test_iadd():
    t = (1, 2, [3, 4])
    try:
        t[2] += [50, 60]
    finally:
        print(t)
        # ok
        t[2].extend([50, 60])
    