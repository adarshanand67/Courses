from pyspark import SparkConf, SparkContext

conf = SparkConf().setMaster("local").setAppName("CustomerOrders")
sc = SparkContext(conf=conf)
sc.setLogLevel("ERROR")


def parseOrders(line):
    fields = line.split(",")
    return (int(fields[0]), float(fields[2]))


orders = sc.textFile("./ml-32m/customer-orders.csv")
mappedInput = orders.map(parseOrders)
result = mappedInput.reduceByKey(lambda x, y: x + y).collect()
sorted_result = sorted(result, key=lambda x: x[1], reverse=True)
for key, value in sorted_result:
    print(f"{key} {value:.2f}")
print("Highest spending customer: ", sorted_result[0][0], sorted_result[0][1])
