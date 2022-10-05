#!/usr/bin/env python3
import sys
import pandas as pd
import matplotlib.pyplot as plt


def parse_csv(csv_path):
    df = pd.read_csv(csv_path)
    header = 'github.com/AlDanial/cloc'
    df = df.drop(df.columns[df.columns.str.contains(header)], axis=1)
    df = df.drop(df.tail(1).index)
    return df


def get_column(df, column_name):
    return df[column_name]


def box_plot(data):
    fig, axis = plt.subplots()
    axis.set_title('Distribution of lines of code per file')
    axis.set_ylabel('Lines of code per file')
    axis.set_xlabel('Project')
    plt.boxplot(data)
    plt.show()


def main():
    if len(sys.argv) != 2:
        print(f"Usage:  {sys.argv[0]} loc.csv")
        print("     where the .csv file is created with")
        print("       cloc --by-file --csv --out loc.csv src")
    table = parse_csv(sys.argv[1])
    loc = get_column(table, 'code')
    box_plot(loc)


if __name__ == "__main__":
    main()
