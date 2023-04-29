# import numpy as np
import re

def read_data(fname):
    with open(fname, "r") as f:
        data_raw = f.readlines()

    data = dict()
    for l in data_raw:
        if l.startswith('dpa'):
            k = re.findall(r"temp\/\S+", l)[0].split('/')[-1]
            v = [float(v) for v in re.findall(r'(\d+\.\d+)\s+', l)]
            # print(k, v)
            data[k] = v
    print(data)


def draw_heat(data):
    pass

def main():
    data = read_data('temp1.txt')
    pass


if __name__ == "__main__":
    main()
