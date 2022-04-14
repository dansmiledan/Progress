import bisect

def demo(bisect_fn):
    HAYSTACK = [1, 4, 5, 6, 8, 12, 15, 20, 21, 23, 23, 26, 29, 30]
    NEEDLES = [0, 1, 2, 5, 8, 10, 22, 23, 29, 30, 31]  
    ROW_FMT = '{0:2d} @ {1:2d}    {2}{0:<2d}'
    print('\nhaystack ->', ' '.join('%2d' % n for n in HAYSTACK))
    for needle in reversed(NEEDLES):         
        position = bisect_fn(HAYSTACK, needle)
        offset = position * '  |'
        print(ROW_FMT.format(needle, position, offset))


def custom_bisect(items, item):
    l, r = 0, len(items)
    while l < r:
        m = (l + r) // 2
        if items[m] > item:
            r = m
        else:
            l = m + 1
    return l


def custom_bisect_left(items, item):
    l, r = 0, len(items)
    while l < r:
        m = (l + r) // 2
        if items[m] < item:
            l = m + 1
        else:
            r = m
    return l


def test_multi_bisect_func():
    demo(bisect.bisect)
    demo(custom_bisect)
    demo(bisect.bisect_left)
    demo(custom_bisect_left)