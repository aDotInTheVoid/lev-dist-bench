from collections import defaultdict
from pprint import pp
from typing import Tuple
import re
from matplotlib import pyplot as plt
import numpy as np


def extract_line(l: str) -> Tuple[str, str]:
    l = l.strip()
    parts = re.split("\s+", l)
    return parts[-1], float(parts[2][:-2])


def load_file(f: str):
    m = {}
    with open(f) as f:
        for l in f.readlines():
            if "PASS" in l:
                tname, ttime = extract_line(l)
                m[tname] = ttime

    return m


def merge_dicts(ds):
    m = {}
    for d in ds:
        for k, v in d.items():
            m[k] = m.get(k, []) + [v]

    return m


def partition(d):
    m = defaultdict(dict)
    for k, v in d.items():
        k0, k1 = k.split("::")
        m[k0][k1] = v
    return dict(m)


def make_plot(name, data):
    fig, ax = plt.subplots()

    x_pos = np.arange(len(data))

    vals = []
    errors = []
    labels = []

    for k, v in data.items():
        v = np.array(v)
        vals.append(np.mean(v))
        errors.append(np.std(v))
        labels.append(k)

    ax.bar(x_pos, vals, yerr=errors, align='center', alpha=0.5, ecolor='black', capsize=10)

    ax.set_xticks(x_pos)
    ax.set_xticklabels(labels)
    ax.yaxis.grid(True)

    ax.set_ylabel("Time (s)")

    ax.set_title(f"Time taken to run lenght $\\approx$ {name[1:]} ")

    plt.tight_layout()
    plt.savefig(f"img/{name}.png")

    


if __name__ == "__main__":
    ds = []
    for i in range(1, 11):
        ds.append(load_file(f"data/run_{i}.txt"))

    ds = partition(merge_dicts(ds))

    for k, v in ds.items():
        make_plot(k, v)
