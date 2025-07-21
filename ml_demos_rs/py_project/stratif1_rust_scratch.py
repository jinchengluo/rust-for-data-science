import numpy as np
from sklearn.datasets import make_blobs

import ml_demos_rs

def main():
    n_blobs = 4
    X, y = make_blobs(n_samples=100000, centers=n_blobs, cluster_std=0.7)#, random_state=0)
    y = y.astype(np.int32)
    X_train_py, X_test_py, y_train_py, y_test_py = ml_demos_rs.stratified_train_test_split(X, y, test_size=0.33)
    return X_train_py, X_test_py, y_train_py, y_test_py

if "__name__" == "__main__":
    main()