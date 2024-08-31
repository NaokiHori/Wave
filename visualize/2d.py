import os
import numpy as np
from matplotlib import pyplot
from tqdm import tqdm

root = "output"

x = np.load(f"{root}/x00.npy")
y = np.load(f"{root}/x01.npy")

files = [f"{root}/{file}" for file in os.listdir(root) if file.startswith("pos")]
files = sorted(files)

fig = pyplot.figure()
ax = fig.add_subplot()

vmin = - 0.005
vmax = + 0.005
levels = np.linspace(vmin, vmax, 26, endpoint=True)

for cnt, file in enumerate(tqdm(files)):
    data = np.load(file)
    ax.clear()
    ax.contourf(x, y, data, vmin=vmin, vmax=vmax, levels=levels, extend="both", cmap="seismic")
    ax.set_aspect("equal")
    ax.set_xticks([])
    ax.set_yticks([])
    pyplot.show(block=False)
    pyplot.pause(1.e-1)
    # pyplot.savefig(f"images/image{cnt:03d}.png")

pyplot.close()

