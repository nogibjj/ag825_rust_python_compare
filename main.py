import polars as pl
import time
import psutil


def read():
    path = "medallists.csv"
    df = pl.read_csv(path)

    print(f"DataFrame shape: {df.height} rows, {df.width} columns")
    print("Sample 10 rows have been displayed below:")
    print(df.head(10))
    return "Success"


if __name__ == "__main__":
    start_time = time.time()
    process = psutil.Process()
    mem_info = process.memory_info()

    read()

    end_time = time.time()
    elapsed_time = end_time - start_time

    print(f"Total time taken: {elapsed_time} seconds")
    print(f"Total memory used: {mem_info.rss / (1024 * 1024):.2f} MB")
