from sklearn.datasets import make_blobs

import ml_demos_rs

def main():
    n_blobs = 5
    n_clusters = 5
    X, _ = make_blobs(n_samples=10000, centers=n_blobs, cluster_std=0.67)#, random_state=0)
    centroids, clusters = ml_demos_rs.kmeans_alamano(X, n_clusters, init="kmeans++")
    return centroids, clusters

if __name__ == "__main__":
    main()