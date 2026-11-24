import io
import pathlib
import site
import sys

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.patches import Ellipse, Wedge

site.addsitedir(str(pathlib.Path(__file__).parent))
from script_utils import differing_contents

sincos = 1.0 / np.sqrt(2)


def generate_figure_zone():
    fig, ax = plt.subplots(figsize=(1.2, 0.9), dpi=300)
    ax.set_axis_off()

    h = 0.5
    w = h * 1.6

    rect = plt.Rectangle(
        (0, 0),
        width=w,
        height=h,
        ec="k",
        fc="none",
    )
    ax.add_artist(rect)
    ax.scatter([0, w], [0, h], c="k", marker="o", s=7)

    ax.annotate(
        "$(x_0, y_0)$",
        xy=(0, 0),
        xycoords=rect,
        xytext=(0, -3),
        textcoords="offset points",
        ha="center",
        va="top",
    )
    ax.annotate(
        "$(x_1, y_1)$",
        xy=(1, 1),
        xycoords=rect,
        xytext=(0, 3),
        textcoords="offset points",
        ha="center",
        va="bottom",
    )

    ax.set_xlim(-0.23, w + 0.23)
    ax.set_ylim(-0.18, h + 0.18)
    ax.set_aspect("equal")
    return fig


def generate_figure_box():
    fig, ax = plt.subplots(figsize=(1.2, 1.1), dpi=300)
    ax.set_axis_off()

    h = 1
    w = h * 1.6
    angle = 30

    rect = plt.Rectangle(
        (0, 0),
        width=w,
        height=h,
        angle=angle,
        ec="k",
        fc="none",
    )
    ax.add_artist(rect)

    corners = rect.get_corners()
    center = np.mean(corners, axis=0)
    top = np.mean(corners[2:4], axis=0)
    right = np.mean(corners[1:3], axis=0)

    ax.annotate(
        "$w$",
        xy=top,
        xytext=(0, 0),
        textcoords="offset points",
        ha="right",
        va="bottom",
    )

    ax.annotate(
        "$h$",
        xy=right,
        xytext=(2, 2),
        textcoords="offset points",
        ha="left",
        va="center_baseline",
    )

    ax.scatter(*center, c="k", marker="o", s=7)
    ax.annotate(
        "$(x, y)$",
        xy=center,
        xytext=(0, -3),
        textcoords="offset points",
        ha="center",
        va="top",
    )

    ax.plot(
        [-w * 0.1, w * 0.5],
        [0, 0],
        color="k",
        lw=0.7,
    )
    arc_r = w * 0.2
    ax.add_artist(
        Wedge(
            (0, 0),
            r=arc_r,
            theta1=0,
            theta2=angle,
            ec="k",
            fc="none",
            lw=0.7,
        )
    )
    ax.annotate(
        "$\\alpha$",
        (arc_r * np.cos(np.deg2rad(angle / 2)), arc_r * np.sin(np.deg2rad(angle / 2))),
        xytext=(2, 2),
        textcoords="offset points",
        ha="left",
        va="center_baseline",
    )

    pad = 0.04
    ax.set_xlim(np.min(corners[:, 0]) - pad, np.max(corners[:, 0] + pad))
    ax.set_ylim(np.min(corners[:, 1]) - pad, np.max(corners[:, 1] + pad))
    ax.set_aspect("equal")
    return fig


def generate_figure_cone():
    fig, ax = plt.subplots(figsize=(1.0, 1.0), dpi=300)
    ax.set_axis_off()

    r = 1
    angle = 45

    circle = plt.Circle(
        (0, 0),
        radius=r,
        ec="k",
        fc="none",
    )
    ax.add_artist(circle)

    ax.scatter(0, 0, c="k", marker="o", s=7)
    ax.annotate(
        "$(x, y)$",
        xy=(0, 0),
        xytext=(0, -3),
        textcoords="offset points",
        ha="center",
        va="top",
    )

    line = np.array(
        [[0, 0], [r * np.cos(np.deg2rad(angle)), r * np.sin(np.deg2rad(angle))]]
    )
    ax.plot(*line.T, color="k", lw=0.7)
    ax.annotate(
        "$r$",
        xy=np.mean(line, axis=0),
        xytext=(-1, 3),
        textcoords="offset points",
        ha="right",
        va="center_baseline",
    )

    pad = 0.04
    ax.set_xlim(-r - pad, r + pad)
    ax.set_ylim(-r - pad, r + pad)
    ax.set_aspect("equal")
    return fig


def generate_figure_elliptical_cone():
    fig, ax = plt.subplots(figsize=(1.4, 1.1), dpi=300)
    ax.set_axis_off()

    a = 1
    b = 0.7
    angle = 30
    a_rad = np.deg2rad(angle)

    ellps = Ellipse(
        (0, 0),
        width=2 * a,
        height=2 * b,
        angle=30,
        ec="k",
        fc="none",
    )
    ax.add_artist(ellps)

    ax.scatter(0, 0, c="k", marker="o", s=7)
    ax.annotate(
        "$(x, y)$",
        xy=(0, 0),
        xytext=(0, -3),
        textcoords="offset points",
        ha="center",
        va="top",
    )

    line_a = np.array([[0, 0], [a * np.cos(a_rad), a * np.sin(a_rad)]])
    line_b = np.array(
        [[0, 0], [b * np.cos(a_rad + np.pi / 2), b * np.sin(a_rad + np.pi / 2)]]
    )

    ax.plot(*line_a.T, color="k", lw=0.7)
    ax.plot(*line_b.T, color="k", lw=0.7)

    ax.annotate(
        "$a$",
        xy=(0.7 * a * np.cos(a_rad), 0.7 * a * np.sin(a_rad)),
        xytext=(0, 4),
        textcoords="offset points",
        ha="right",
        va="center_baseline",
    )
    ax.annotate(
        "$b$",
        xy=np.mean(line_b, axis=0),
        xytext=(-2, 0),
        textcoords="offset points",
        ha="right",
        va="center_baseline",
    )

    ax.plot([-0.1 * a, 0.5 * a], [0, 0], color="k", lw=0.7)
    arc_r = a * 0.25
    ax.add_artist(
        Wedge(
            (0, 0),
            r=arc_r,
            theta1=0,
            theta2=angle,
            ec="k",
            fc="none",
            lw=0.7,
        )
    )
    ax.annotate(
        "$\\alpha$",
        (arc_r * np.cos(a_rad / 2), arc_r * np.sin(a_rad / 2)),
        xytext=(2, 2),
        textcoords="offset points",
        ha="left",
        va="center_baseline",
    )

    ax.set_xlim(-a, a)
    ax.set_ylim(-b * 1.15, b * 1.15)
    ax.set_aspect("equal")
    return fig


def generate_figure_polygon():
    fig, ax = plt.subplots(figsize=(1.1, 1.1), dpi=300)
    ax.set_axis_off()

    points = np.array(
        [
            [0, 0],
            [-0.7, 0.8],
            [0.05, 1.5],
            [0.2, 1.0],
            [1, 0.6],
        ]
    )

    ax.scatter(*points.T, c="k", marker="o", s=7)
    ax.annotate(
        "$(x_i, y_i)$",
        xy=(0, 0),
        xytext=(0, -3),
        textcoords="offset points",
        ha="center",
        va="top",
    )

    ax.add_artist(plt.Polygon(points, ec="k", fc="none"))

    pad = 0.06
    ax.set_xlim(np.min(points[:, 0]) - pad, np.max(points[:, 0] + pad))
    ax.set_ylim(np.min(points[:, 1]) - pad, np.max(points[:, 1] + pad))
    ax.set_aspect("equal")
    return fig


def main():
    figures = {
        "zone": generate_figure_zone(),
        "box": generate_figure_box(),
        "cone": generate_figure_cone(),
        "elliptical_cone": generate_figure_elliptical_cone(),
        "polygon": generate_figure_polygon(),
    }

    if len(sys.argv) != 2:
        raise ValueError("invalid number of arguments")

    for kind, fig in figures.items():
        buffer = io.BytesIO()
        fig.savefig(buffer, dpi=400, bbox_inches="tight", pad_inches=0)
        contents = buffer.getvalue()

        path = pathlib.Path(sys.argv[1], f"coverage_{kind}.png")
        if not path.exists() or differing_contents(contents, path.read_bytes()):
            path.write_bytes(contents)


if __name__ == "__main__":
    main()
