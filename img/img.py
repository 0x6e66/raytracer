import matplotlib.pyplot as plt

anti_aliasing = 2

anti_aliasing_offsets = []
for i in range(1, anti_aliasing + 1):
    for j in range(1, anti_aliasing + 1):
        anti_aliasing_offsets.append(
            (j / (anti_aliasing + 1), -i / (anti_aliasing + 1))
        )

for point in anti_aliasing_offsets:
    plt.plot(point[0], point[1], "ro")

plt.xlim([0,1])
plt.ylim([-1,0])
ax = plt.gca()
ax.set_aspect('equal', adjustable='box')
plt.show()