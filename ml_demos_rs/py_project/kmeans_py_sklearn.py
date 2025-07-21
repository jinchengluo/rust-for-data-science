from sklearn.datasets import make_blobs
from sklearn.cluster import KMeans

def main():
    n_blobs = 5
    n_clusters = 5
    X, _ = make_blobs(n_samples=10000, centers=n_blobs, cluster_std=0.67)#, random_state=0)
    kmeans = KMeans(n_clusters=n_clusters, init="k-means++")
    model = kmeans.fit(X)
    clusters = model.predict(X)
    centroids = model.cluster_centers_
    return centroids, clusters

if __name__ == "__main__":
    main()