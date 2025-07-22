import time
import os
import sys
import importlib
os.environ["OPENBLAS_NUM_THREADS"] = "1"

if len(sys.argv) < 2:
    print("Usage: python3 measure.py <module_name>")
    sys.exit(1)

module_name = sys.argv[1]
poly_match = importlib.import_module(module_name)

print(f"Measuring `{poly_match.__name__}`")

t0 = time.perf_counter()
polygons, points = poly_match.generate_example()
t1 = time.perf_counter()

NUM_ITER = 10

t0 = time.perf_counter()
for _ in range(NUM_ITER):
    poly_match.main(polygons, points)
t1 = time.perf_counter()

took = (t1 - t0) / NUM_ITER

print(f"Took an avg of {took * 1000:.2f}ms per iteration")
