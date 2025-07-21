import numpy as np
from sklearn.datasets import make_blobs

def stratif_train_test_split(X, y, test_size=0.2, train_size=None):

    # Parameters
    n_samples, _ = X.shape
    labels = np.unique(y)
    n_labels = len(labels)
    train_input = True
    if train_size == None:
        train_input = False

    X_train_set = []
    X_test_set = []
    y_train_set = []
    y_test_set = []

    # Constructs subsets
    sub_label_sets = dict(zip(labels, [[] for _ in range(n_labels)]))
    for i in range(n_samples):
        sub_label_sets[y[i]].append(X[i])

    # Constructs train and test sets
    sub_test_sets = dict(zip(labels, [[] for _ in range(len(labels))]))
    sub_train_sets = dict(zip(labels, [[] for _ in range(len(labels))]))

    for l in range(n_labels):
        sub_label_set = sub_label_sets[l]
        n_samples_sub_label_set = len(sub_label_set)
        test_prop = 0
        train_prop = 0
        for i in range(n_samples_sub_label_set):
            if test_prop <= test_size:
                sub_test_sets[l].append(sub_label_set[i])
                test_prop += 1 / n_samples_sub_label_set
                y_test_set.append(l)
            else:
                if train_input:
                    if train_prop > train_size:
                        break
                sub_train_sets[l].append(sub_label_set[i])
                train_prop += 1/ n_samples_sub_label_set
                y_train_set.append(l)

    for l in range(n_labels):
        X_train_set += sub_train_sets[l]
        X_test_set += sub_test_sets[l]

    X_train_set = np.array(X_train_set)
    X_test_set = np.array(X_test_set)
    y_train_set = np.array(y_train_set)
    y_test_set = np.array(y_test_set)

    return X_train_set, X_test_set, y_train_set, y_test_set



def main():
    n_blobs = 4
    X, y = make_blobs(n_samples=100000, centers=n_blobs, cluster_std=0.7)#, random_state=0)
    y = y.astype(np.int32)
    X_train_py, X_test_py, y_train_py, y_test_py = stratif_train_test_split(X, y, test_size=0.33)
    return X_train_py, X_test_py, y_train_py, y_test_py

if "__name__" == "__main__":
    main()