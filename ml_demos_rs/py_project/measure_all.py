import importlib
import os
from os.path import isfile, join
import sys
import time
from pathlib import Path

if len(sys.argv) < 2:
    print("Usage: python3 measure.py <module_name>")
    sys.exit(1)

module_name = sys.argv[1]

def benchmark(algo_name):
    print(f"Measuring `{algo_name}`")
    algo = importlib.import_module(algo_name)

    t0 = time.perf_counter()
    for i in range(500):
        algo.main()

        if i >= 5 and time.perf_counter() - t0 > 3.0:
            break
    t1 = time.perf_counter()

    num_iter = i + 1

    took = (t1 - t0) / num_iter
    return took, num_iter

def main():
    benches = [f for f in os.listdir(".") if isfile(join(".", f)) and f.startswith(module_name)]
    for alg in benches:
        if alg.__contains__(".ipynb"):
            benches.remove(alg)
    took_baseline, num_iter = benchmark(benches.pop(benches.index(f"{module_name}_py_scratch.py"))[:-3])
    print(
        f"Took an avg of {took_baseline * 1000:6.2f}ms per iteration ({num_iter:>3d} iterations) {1:6.2f}x speedup"
    )
    for alg in benches:
        took_alg, num_iter = benchmark(alg[:-3])
        speedup = took_baseline / took_alg
        print(
        f"Took an avg of {took_alg * 1000:6.2f}ms per iteration ({num_iter:>3d} iterations) {speedup:6.2f}x speedup"
    )

if __name__ == "__main__":
    main()
