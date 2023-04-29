from operator import index
import numpy as np
import matplotlib.pyplot as plt
import scipy.stats
import re


def read_data(fname):
    with open(fname, "r") as f:
        data_raw = f.readlines()

    toc = [
        ("seq", 0, 38),
        ("dpa_t2", 38, 76),
        ("dpa_t4", 76, 114),
        ("dpa_t8", 114, 152),
        ("dpa_t16", 152, 190),
        ("hda_t2", 190, 228),
        ("hda_t4", 228, 266),
        ("hda_t8", 266, 304),
        ("hda_t16", 304, 342),
        ("ahda_t2", 342, 380),
        ("ahda_t4", 380, 418),
        ("ahda_t8", 418, 456),
        ("ahda_t16", 456, 494),
    ]

    data = dict()
    for test, i, j in toc:
        temp = [l.strip().split()[-1] for l in data_raw[i:j] if l.startswith("-")]
        temp1 = []
        for val in temp:
            t = float(re.findall(r"\d+\.\d+", val)[0])
            if val.endswith("ms"):
                t /= 1000
            temp1.append(t)
        data[test] = temp1

    # print(data["seq"])
    # print(data["dpa_t2"])
    # print(data["ahda_t16"])
    return data


def draw_hist(data):
    # draw histogram
    plt.hist(data, 50, facecolor="green", alpha=0.5)

    # draw kernel
    bws = [None, 0.1, 0.01]
    kdes = [scipy.stats.gaussian_kde(data, bw) for bw in bws]
    x_rng = np.linspace(1, 40, 200)
    for i, bw in enumerate(bws):
        plt.plot(x_rng, kdes[i](x_rng), lw=2, label="bw = " + str(bw))
    plt.xlim(-1, 40)
    # plt.show()


def main():
    data = read_data("bench2.txt")
    draw_hist(data["seq"])


if __name__ == "__main__":
    main()
