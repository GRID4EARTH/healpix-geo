import io
import pathlib
import site
import sys

import matplotlib.pyplot as plt
import numpy as np

site.addsitedir(str(pathlib.Path(__file__).parent))
from script_utils import differing_contents

sincos = 1.0 / np.sqrt(2)


def get_cell_centers():
    xc, yc = np.meshgrid(np.arange(4) - 3.5, np.arange(4) + 0.5)
    rot_mat = np.array([[sincos, -sincos], [sincos, sincos]])
    xy = np.stack([xc, yc], axis=-1).reshape(-1, 2)
    rot_xy = (xy @ rot_mat).reshape([*xc.shape, 2])
    xc, yc = rot_xy[:, :, 0], rot_xy[:, :, 1]

    # put in Z-order
    indices = [3, 7, 2, 6, 11, 15, 10, 14, 1, 5, 0, 4, 9, 13, 8, 12]  # indices = []
    xc = xc.ravel()[indices]
    yc = yc.ravel()[indices]

    return xc, yc


def generate_figure():
    xc, yc = get_cell_centers()

    fig, ax = plt.subplots(figsize=(3, 3))
    ax.set_axis_off()

    # grid
    ax.add_artist(
        plt.Rectangle(
            (0, 0),
            width=4,
            height=4,
            angle=45.0,
            ec="k",
            fc="none",
            lw=1.5,
        )
    )
    for i in range(3):
        lw = 1.5 if i % 2 == 1 else 1
        ax.plot(
            [sincos * (i + 1), -sincos * (3 - i)],
            [sincos * (i + 1), sincos * (i + 5)],
            color="k",
            lw=lw,
        )
        ax.plot(
            [-sincos * (i + 1), sincos * (3 - i)],
            [sincos * (i + 1), sincos * (i + 5)],
            color="k",
            lw=lw,
        )

    # Cell numbers
    for i, (x, y) in enumerate(zip(xc.flat, yc.flat, strict=False)):
        ax.annotate(
            i,
            xy=(x, y),
            xytext=(0, 3),
            textcoords="offset points",
            ha="center",
            va="bottom",
        )
        s = f"{i:04b}"
        s = f"{s[:2]} {s[2:]}"
        ax.annotate(
            s,
            xy=(x, y),
            xytext=(0, -5),
            textcoords="offset points",
            ha="center",
            va="top",
            fontsize=6,
        )

    # Z-order path
    ax.plot(xc, yc, ls="-", alpha=0.5, color="orchid")

    ax.set_xlim(-4.05 * sincos, 4.05 * sincos)
    ax.set_ylim(-0.05, 8.05 * sincos)
    ax.set_aspect("equal")

    return fig


def main():
    fig = generate_figure()

    if len(sys.argv) != 2:
        raise ValueError("invalid number of arguments")

    buffer = io.BytesIO()
    fig.savefig(buffer, dpi=400, bbox_inches="tight", pad_inches=0)
    contents = buffer.getvalue()

    path = pathlib.Path(sys.argv[1])
    if not path.exists() or differing_contents(contents, path.read_bytes()):
        path.write_bytes(contents)


if __name__ == "__main__":
    main()
