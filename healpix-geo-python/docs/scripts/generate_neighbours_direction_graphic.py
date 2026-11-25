import io
import pathlib
import site
import sys

site.addsitedir(str(pathlib.Path(__file__).parent))

import matplotlib.pyplot as plt
import numpy as np
from script_utils import differing_contents

sincos = 1.0 / np.sqrt(2)


def get_cell_centers():
    xc, yc = np.meshgrid(np.arange(3) - 2.5, np.arange(3) + 0.5)
    rot_mat = np.array([[sincos, -sincos], [sincos, sincos]])
    xy = np.stack([xc, yc], axis=-1).reshape(-1, 2)
    rot_xy = (xy @ rot_mat).reshape([*xc.shape, 2])
    xc, yc = rot_xy[:, :, 0], rot_xy[:, :, 1]

    xc = xc.ravel()
    yc = yc.ravel()

    direction = ["W", "SW", "S", "NW", "", "SE", "N", "NE", "E"]

    return xc, yc, direction


def generate_figure():
    xc, yc, direction = get_cell_centers()

    fig, ax = plt.subplots(figsize=(3, 3), dpi=200, layout="constrained")
    ax.set_axis_off()

    for i, (x, y) in enumerate(zip(xc.flat, yc.flat, strict=False)):
        d = direction[i]
        ax.annotate(
            d,
            xy=(x, y),
            ha="center",
            va="center_baseline",
            fontsize=11,
        )
        if not d:
            continue
        color = "orchid" if len(d) == 1 else "forestgreen"
        ax.add_artist(
            plt.Rectangle(
                (x - sincos, y),
                width=1,
                height=1,
                angle=-45.0,
                ec="none",
                fc=color,
                alpha=0.3,
            )
        )

    ax.add_artist(
        plt.Rectangle(
            (0, 0),
            width=3,
            height=3,
            angle=45.0,
            ec="k",
            fc="none",
            lw=1.5,
        )
    )
    for i in range(2):
        ax.plot(
            [sincos * (i + 1), -sincos * (2 - i)],
            [sincos * (i + 1), sincos * (i + 4)],
            color="k",
            lw=1,
        )
        ax.plot(
            [-sincos * (i + 1), sincos * (2 - i)],
            [sincos * (i + 1), sincos * (i + 4)],
            color="k",
            lw=1,
        )

    ax.set_xlim(-3.05 * sincos, 3.05 * sincos)
    ax.set_ylim(-0.05, 6.05 * sincos)
    ax.set_aspect("equal")
    return fig


def main():
    fig = generate_figure()

    if len(sys.argv) != 2:
        raise ValueError("invalid number of arguments")

    buffer = io.BytesIO()
    fig.savefig(buffer, dpi=600, bbox_inches="tight", pad_inches=0)
    contents = buffer.getvalue()

    path = pathlib.Path(sys.argv[1])
    if not path.exists() or differing_contents(contents, path.read_bytes()):
        path.write_bytes(contents)


if __name__ == "__main__":
    main()
