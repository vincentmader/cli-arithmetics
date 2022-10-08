#!/usr/bin/env python3
from datetime import datetime as dt
import os

import matplotlib.pyplot as plt
import numpy as np

PATH_TO_FIGURES = './figures'
PATH_TO_STATFILES = './saves'
N_MOVING_AVG = 50


def load_savefile_to_string(filename):
    # Load savefile to string, pretty self-explanatory.
    print(f"Loading stat-file \"{filename}\"")
    path_to_file = os.path.join(PATH_TO_STATFILES, filename)
    with open(path_to_file, "r") as fp:
        return fp.readlines()


def load_from_savefile_into_lists():
    # Instantiate lists for:
    #  - time that question asked at       [dt.datetime]
    #  - question                          [string]
    #  - correct solution                  [float]
    #  - user's answer                     [float]
    #  - duration of time taken to answer  [float (s)]
    t, q, s, a, d = [], [], [], [], []
    for line in content:
        # Get cells from CSV row.
        split = line[:-1].split(";")
        # Convert UNIX ms-timestamp to python-datetime object.
        time = float(split[0]) / 1000 if split[0] != "None" else None
        time = dt.fromtimestamp(time) if time != None else None
        # Append cell values to lists.
        t.append(time)
        q.append(split[1])
        s.append(float(split[2]))
        a.append(float(split[3]))
        d.append(float(split[4]) / 1000)
    return t, q, s, a, d


def check_correctness(t, q, s, a, d):
    c = []
    # Loop over all question-events, check if correct.
    for t, q, s, a, d in zip(t, q, s, a, d):
        correct = a == s
        c.append(correct)
    # Return list containing boolean values (correctness) for each question.
    return c


def get_moving_avg_from_correctness_list(c, N):
    accuracy_moving_avg = []
    # Loop over list containing correctness-boolean values.
    for i, _ in enumerate(c):
        summa = 0
        # Loop over last N questions (if N<i), add 1 to sum if correct.
        for j in range(min(i, N)):
            correct = c[i-j]
            if correct:
                summa += 1
        # Calculate accurary for time-window.
        accuracy = summa / min(max(i, 1), N)
        accuracy_moving_avg.append(accuracy)
    return accuracy_moving_avg


def create_plot(filename, moving_avg):
    plt.plot(moving_avg)
    plt.title(f"{filename}")
    plt.xlabel("Question Number (\"Time\")")
    plt.ylabel(f"Accuracy Moving Average (N={N_MOVING_AVG})")
    plt.show()
    filename = filename.split('.')[0] + '.png'
    path_to_fig = os.path.join(PATH_TO_FIGURES, filename)
    plt.savefig(path_to_fig)


if __name__ == "__main__":
    for filename in sorted(os.listdir(PATH_TO_STATFILES)):
        if not filename.endswith(".txt"):
            continue
        content = load_savefile_to_string(filename)

        t, q, s, a, d = load_from_savefile_into_lists()
        c = check_correctness(t, q, s, a, d)

        moving_avg = get_moving_avg_from_correctness_list(c, N_MOVING_AVG)
        create_plot(filename, moving_avg)
