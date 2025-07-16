# /// script
# requires-python = ">=3.11"
# dependencies = [
#     "matplotlib",
#     "numpy",
# ]
# ///
import itertools
from collections import Counter, deque
from pprint import pprint

import matplotlib.pyplot as plt
import numpy as np
from mpl_toolkits.mplot3d import Axes3D
from mpl_toolkits.mplot3d.art3d import Poly3DCollection


def read_coordinates(filename):
    """Read coordinates from file in X,Y,Z format"""
    coordinates = []

    with open(filename, "r") as file:
        for line in file:
            line = line.strip()
            if line:  # Skip empty lines
                x, y, z = map(float, line.split(","))
                coordinates.append((2 * x, 2 * y, 2 * z))

    return coordinates


def get_voxel_faces(x, y, z, size=2):
    """Get the 6 faces of a voxel as vertex arrays"""
    # Half size for centering
    s = size / 2

    # Define 8 vertices of the cube
    vertices = np.array(
        [
            [x - s, y - s, z - s],  # 0: bottom-left-back
            [x + s, y - s, z - s],  # 1: bottom-right-back
            [x + s, y + s, z - s],  # 2: top-right-back
            [x - s, y + s, z - s],  # 3: top-left-back
            [x - s, y - s, z + s],  # 4: bottom-left-front
            [x + s, y - s, z + s],  # 5: bottom-right-front
            [x + s, y + s, z + s],  # 6: top-right-front
            [x - s, y + s, z + s],  # 7: top-left-front
        ]
    )

    # Define the 6 faces using vertex indices
    faces = [
        [vertices[0], vertices[1], vertices[2], vertices[3]],  # back face (z-)
        [vertices[4], vertices[7], vertices[6], vertices[5]],  # front face (z+)
        [vertices[0], vertices[4], vertices[5], vertices[1]],  # bottom face (y-)
        [vertices[2], vertices[6], vertices[7], vertices[3]],  # top face (y+)
        [vertices[0], vertices[3], vertices[7], vertices[4]],  # left face (x-)
        [vertices[1], vertices[5], vertices[6], vertices[2]],  # right face (x+)
    ]

    return faces


def get_face_color(x, y, z, face, colored):
    face_center_x = np.mean([v[0] for v in face])
    face_center_y = np.mean([v[1] for v in face])
    face_center_z = np.mean([v[2] for v in face])
    face_center = (face_center_x, face_center_y, face_center_z)

    return colored.get(face_center, "green")


def create_voxel_model(coordinates):
    """Create and display a 3D voxel model with individual faces"""

    coordinates_set = set(coordinates)

    c = {}
    for x, y, z in coordinates:
        for axis in [0, 1, 2]:
            for delta in [-1, 1]:
                face_center = [x, y, z]
                face_center[axis] += delta
                c.setdefault(tuple(face_center), []).append((x, y, z))
    colored = {k: "red" for k, v in c.items() if len(v) == 1}
    print(len(colored))

    x_min = int(min(x for x, _, _ in coordinates)) - 4
    y_min = int(min(y for _, y, _ in coordinates)) - 4
    z_min = int(min(z for _, _, z in coordinates)) - 4
    x_max = int(max(x for x, _, _ in coordinates)) + 4
    y_max = int(max(y for _, y, _ in coordinates)) + 4
    z_max = int(max(z for _, _, z in coordinates)) + 4
    print(f"Bounds: x({x_min}, {x_max}), y({y_min}, {y_max}), z({z_min}, {z_max})")

    def in_bounds(coord):
        return (
            x_min <= coord[0] <= x_max and y_min <= coord[1] <= y_max and z_min <= coord[2] <= z_max
        )

    q = [(x_max - 10, y_max - 10, z_max - 10)]
    visited = set()

    while q:
        x, y, z = q.pop()
        visited.add((x, y, z))

        # Check neighbors
        for dx, dy, dz in itertools.product([-2, 0, 2], repeat=3):
            if sum(dd == 0 for dd in (dx, dy, dz)) != 2:  # Only allow one axis to change
                continue
            neighbor = (x + dx, y + dy, z + dz)
            if neighbor not in visited and in_bounds(neighbor) and neighbor not in coordinates_set:
                q.append(neighbor)

    part2 = set()
    for x, y, z in visited:
        for axis in [0, 1, 2]:
            for delta in [-1, 1]:
                face_center = [x, y, z]
                face_center[axis] += delta
                if colored.get(tuple(face_center), None) == "red":
                    part2.add(tuple(face_center))
                else:
                    colored[tuple(face_center)] = "purple"
    print(len(part2))
    for k in part2:
        colored[k] = "blue"

    # Create figure and 3D axes
    fig = plt.figure(figsize=(12, 10))
    ax = fig.add_subplot(111, projection="3d")

    # Collect all faces and their colors
    all_faces = []
    all_colors = []

    # Generate faces for each voxel
    for x, y, z in coordinates:
        voxel_faces = get_voxel_faces(x, y, z, size=2)

        for face_index, face in enumerate(voxel_faces):
            color = get_face_color(x, y, z, face, colored)
            all_faces.append(face)
            all_colors.append(color)

    # Create 3D polygon collection
    poly3d = Poly3DCollection(
        all_faces, facecolors=all_colors, alpha=0.9, edgecolors="black", linewidths=0.5
    )

    # Add to axes
    ax.add_collection3d(poly3d)

    x_centers = [coord[0] for coord in coordinates]
    y_centers = [coord[1] for coord in coordinates]
    z_centers = [coord[2] for coord in coordinates]

    ax.scatter(
        x_centers,
        y_centers,
        z_centers,
        c="black",  # Color of the points
        s=50,  # Size of the points
        alpha=1.0,  # Opacity
        marker="o",
    )  # Point shape

    x_centers = [coord[0] for coord in visited]
    y_centers = [coord[1] for coord in visited]
    z_centers = [coord[2] for coord in visited]

    ax.scatter(
        x_centers,
        y_centers,
        z_centers,
        c="purple",  # Color of the points
        s=50,  # Size of the points
        alpha=0.6,  # Opacity
        marker="o",
    )  # Point shape

    # Set axis limits based on voxel positions
    x_coords = [coord[0] for coord in coordinates]
    y_coords = [coord[1] for coord in coordinates]
    z_coords = [coord[2] for coord in coordinates]

    margin = 2  # Add margin around the model
    ax.set_xlim(min(x_coords) - margin, max(x_coords) + margin)
    ax.set_ylim(min(y_coords) - margin, max(y_coords) + margin)
    ax.set_zlim(min(z_coords) - margin, max(z_coords) + margin)

    # Set labels
    ax.set_xlabel("X")
    ax.set_ylabel("Y")
    ax.set_zlabel("Z")
    ax.set_title("3D Voxel Model (2x2x2 voxels)")

    # Set equal aspect ratio for all axes
    ax.set_box_aspect([1, 1, 1])

    plt.show()


def main():
    # Read coordinates from file
    coordinates = read_coordinates("src/input.txt")

    # Create and display the 3D model
    create_voxel_model(coordinates)


if __name__ == "__main__":
    main()
