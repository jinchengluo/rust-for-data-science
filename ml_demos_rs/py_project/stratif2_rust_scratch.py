import numpy as np
from sklearn.datasets import make_blobs

import ml_demos_rs

def main():
    n_blobs = 4
    X, y = make_blobs(n_samples=100000, centers=n_blobs, cluster_std=0.7)#, random_state=0)
    y = y.astype(np.int32)
    X_kfolds, y_kfolds = ml_demos_rs.stratified_kfold_split(X, y, 10)
    return X_kfolds, y_kfolds

if "__name__" == "__main__":
    main()