# Apache Spark

### Overview

Apache Spark is an analytics engine for large-scale data processing. It provides high-level APIs in Java, Scala, Python, and R, and an optimized engine that supports general execution graphs. It also supports a rich set of higher-level tools including:

- **Spark SQL**: For SQL and structured data processing.
- **MLlib**: For machine learning.
- **GraphX**: For graph processing.
- **Spark Streaming**: For stream processing.

### Key Features

- **Speed**: Processes data in memory, making it much faster than traditional disk-based processing.
- **Ease of Use**: High-level APIs in Java, Scala, Python, and R, and an interactive shell.
- **General Execution Model**: Supports a wide range of applications, from batch processing to streaming, machine learning, and graph processing.
- **Advanced Analytics**: Includes libraries for SQL, machine learning, graph computation, and stream processing.

For more details, refer to the [Taming Big Data with Apache Spark](https://intel.udemy.com/course/taming-big-data-with-apache-spark-hands-on/learn/lecture/3708614#content) course on Udemy.

### Course Contents

1. Intro to Spark architecture
2. RDD (Resilient Distributed Dataset)
3. Transformations on RDD:
  - map, flatmap, filter, distinct, sample, union, intersection, subtract, cartesian, groupByKey, reduceByKey, aggregateByKey, sortByKey
4. Actions on RDD:
  - collect, count, take, reduce, saveAsTextFile, countByKey, foreach
5. SparkSQL:
  - Dataframe, Dataset, SQL queries
6. Transformations on Dataframe:
  - select, filter, groupBy, sort, join, union, dropDuplicates, withColumn, withColumnRenamed, drop, describe, printSchema
7. Joins on Dataframes
8. Broadcast variables
9. Degrees of separation
10. Item-based collaborative filtering:
   - cache(), persist()
11. Recommender systems:
   - cosine similarity, Pearson correlation
12. Running Spark on cluster:
   - Amazon Elastic MapReduce cloud [turn off clusters when not in use]
13. Troubleshooting:
   - Spark history server
14. Dataframes spark.ml (machine learning library)
15. Spark ML ALS (alternating least squares) for recommendation system:
   - linear regression, logistic regression, decision tree, random forest, gradient boosting, k-means clustering
16. Spark Streaming:
   - processing log data real-time, HDFS, Kafka, checkpointing (fault tolerance), DStream, microbatching, sliding window, stateful transformations
17. GraphX (graph processing)

### Future Learnings Pathway

- **Advanced Spark Programming**: Learn advanced concepts in Spark programming, such as Spark internals, tuning, and optimization.
- **Spark Streaming**: Explore Spark Streaming in more depth, including real-time data processing and stream analytics.
- **Machine Learning with Spark**: Dive deeper into machine learning with Spark, including advanced algorithms and techniques.
- **Big Data Ecosystem**: Learn about other tools in the big data ecosystem, such as Hadoop, Kafka, and Flink, and how they integrate with Spark.
- **Cloud Computing**: Understand how to deploy Spark on cloud platforms like AWS, Azure, and Google Cloud, and leverage cloud services for big data processing.
- **Real-world Projects**: Work on real-world projects using Spark to gain practical experience and apply your knowledge to solve business problems.

### Code Formatting

We use `black` for Python code formatting to ensure consistency across the codebase. To format the code, run the following command:

```sh
black .
```
