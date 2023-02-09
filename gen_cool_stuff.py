# This is just a poc

import numpy as np
import matplotlib.pyplot as plt
import os
import sys

scale = 50
r = 20

l = [(0,-20)]
for i in range(int(scale*-0.5*np.pi), int(scale*1.5*np.pi)):
    i /= scale
    x = int(np.cos(i) * r)
    z = int(np.sin(i) * r) - 20
    if len(l) == 0 or l[-1] != (x,z):
        l.append((x,z))
        os.system(f"cargo run --bin main -- --width=1200 --height=800 -a2 --look-at --camera-pos {x},10,{z} --look-at-pos 0,-4,-20 --output out/gif2 --versionize")
