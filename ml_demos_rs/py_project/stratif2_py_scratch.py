import numpy as np
from sklearn.datasets import make_blobs

def stratif_kfold_split(X, y, K, sizes=None):

    # Parameters
    n_samples, _ = X.shape
    labels = np.unique(y)
    n_labels = len(labels)

    X_kfolds = [[] for _ in range(K)]
    y_kfolds = [[] for _ in range(K)]

    # Constructs subsets
    sub_label_sets = dict(zip(labels, [[] for _ in range(n_labels)]))
    for i in range(n_samples):
        sub_label_sets[y[i]].append(X[i])

    # Constructs folds
    current_fold = 0
    for l in range(n_labels):
        sub_label_set = sub_label_sets[l]
        n_samples_sub_label_set = len(sub_label_set)
        for i in range(n_samples_sub_label_set):
            X_kfolds[current_fold].append(sub_label_set[i])
            y_kfolds[current_fold].append(l)
            current_fold += 1
            current_fold %= K

    return X_kfolds, y_kfolds


def main():
    n_blobs = 4
    X, y = make_blobs(n_samples=100000, centers=n_blobs, cluster_std=0.7)#, random_state=0)
    y = y.astype(np.int32)
    X_kfolds, y_kfolds = stratif_kfold_split(X, y, 10)
    return X_kfolds, y_kfolds

if "__name__" == "__main__":
    main()