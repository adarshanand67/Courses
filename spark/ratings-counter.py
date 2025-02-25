from pyspark import SparkConf, SparkContext
import collections
import matplotlib.pyplot as plt

# Configure Spark
conf = SparkConf().setMaster("local").setAppName("RatingsHistogram")
sc = SparkContext(conf=conf)
sc.setLogLevel("ERROR")

# Read ratings data
ratings = sc.textFile("./ml-32m/ratings.csv").map(lambda x: x.split(",")[2])

# Count ratings by value
result = collections.OrderedDict(sorted(ratings.countByValue().items()))

# Print results
for key, value in result.items():
    print(f"{key} {value}")

# Plot histogram
keys, values = zip(*result.items())
plt.bar(keys, values)
plt.xlabel("Rating")
plt.ylabel("Frequency")
plt.title("Ratings Histogram")
plt.savefig("ratings-histogram.png")
plt.show()
