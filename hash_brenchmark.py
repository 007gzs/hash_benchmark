import time
from random import Random
import mmh3
import xxhash


def speed(size_per_sec):
    units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "YB", "BB"]
    base = 1024
    speed = size_per_sec
    unit_index = 0
    while unit_index + 1 < len(units) and speed > base:
        unit_index += 1
        speed /= base
    return "%.2s%s/s" % (speed, units[unit_index])


def hash_brenchmark(name, data, seed, func):
    start = time.perf_counter_ns()
    res = func(data, seed)
    end = time.perf_counter_ns()
    duration_ns = end - start
    size_per_sec = 1_000_000_000/duration_ns * len(data)
    return {
        "name": name,
        "size": len(data),
        "seed": seed,
        "duration_ns": duration_ns,
        "size_per_sec": size_per_sec,
        "speed": speed(size_per_sec),
        "res": res
    }


def test_brenchmark(size, seed):
    if size == 0:
        data = b"1234567890abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    else:
        random = Random(seed)
        data = random.randbytes(size)
    for (name, func) in (
        ("mmh3_32_uintdigest", mmh3.mmh3_32_uintdigest),
        ("mmh3_x64_128_uintdigest", mmh3.mmh3_x64_128_uintdigest),
        ("mmh3_x86_128_uintdigest", mmh3.mmh3_x86_128_uintdigest),
        ("xxh32_intdigest", xxhash.xxh32_intdigest),
        ("xxh3_64_intdigest", xxhash.xxh3_64_intdigest),
        ("xxh3_128_intdigest", xxhash.xxh3_128_intdigest),
    ):
        yield hash_brenchmark(name, data, seed, func)


def test_value():
    for item in test_brenchmark(0, 0):
        print(item)


def main():
    seed = 0
    with open("stat_py.csv", "w", encoding="utf8") as f:
        f.write("name,size,duration,duration(s),speed,size_per_sec\n")
        for bit in range(25):
            size = 1 << bit
            for item in test_brenchmark(size, seed):
                f.write(f"{item['name']},{item['size']},{item['duration_ns']},{item['duration_ns']},{item['speed']},{item['size_per_sec']}\n")


if __name__ == '__main__':
    main()
