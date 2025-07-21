import importlib
import sys
import time

if len(sys.argv) < 2:
    print("Usage: python3 measure.py <module_name>")
    sys.exit(1)

module_name = sys.argv[1]
algoritm = importlib.import_module(module_name)

print(f"Measuring `{algoritm.__name__}`")

NUM_ITER = 10

t0 = time.perf_counter()
for i in range(NUM_ITER):
    # p = (i / NUM_ITER) * 100
    # print(f"[ {p:.0f} % ]", end="\r")
    algoritm.main()
t1 = time.perf_counter()

took = (t1 - t0) / NUM_ITER

print(f"Took an avg of {took * 1000:.2f}ms per iteration ({NUM_ITER} Iterations)")