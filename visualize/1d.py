import os
import numpy as np
# import matplotlib
# matplotlib.use("Agg")
from matplotlib import pyplot
from tqdm import tqdm

root = "output"

x = np.load(f"{root}/x00.npy")

files = [f"{root}/{file}" for file in os.listdir(root) if file.startswith("pos")]
files = sorted(files)

fig = pyplot.figure()
ax = fig.add_subplot()

for cnt, file in enumerate(tqdm(files)):
    data = np.load(file)
    ax.clear()
    ax.plot(x, data)
    ax.set_ylim([-0.25, +0.25])
    pyplot.show(block=False)
    pyplot.pause(1.e-1)
    # pyplot.savefig(f"images/image{cnt:03d}.png")

pyplot.close()

