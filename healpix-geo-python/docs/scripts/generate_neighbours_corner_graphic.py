import io
import pathlib
import site
import sys

site.addsitedir(str(pathlib.Path(__file__).parent))

import cartopy.crs as ccrs
import matplotlib.pyplot as plt
import numpy as np
from cartopy.mpl.ticker import LatitudeFormatter, LongitudeFormatter
from matplotlib.collections import PolyCollection
from matplotlib.colors import to_rgba
from script_utils import differing_contents

import healpix_geo


def generate_figure():
    fig, ax = plt.subplots(figsize=(7, 7))

    level = 5
    cell_ids = np.array(
        [
            [5116, 5117, 5118, 5119],
            [680, 681, 682, 683],
            [3412, 3413, 3414, 3415],
        ]
    )
    center = 682
    neighbours = [5117, 680, 681, 683, 3415, 3413, 5119]

    fig, ax = plt.subplots(subplot_kw={"projection": ccrs.PlateCarree()})

    all_cell_ids, _, _ = healpix_geo.nested.zone_coverage(
        (-6, 34.5, 6, 46), level, ellipsoid="WGS84"
    )
    lon, lat = healpix_geo.nested.vertices(all_cell_ids, level, ellipsoid="WGS84")
    lon = (lon + 180) % 360 - 180
    poly = PolyCollection(np.stack([lon, lat], axis=-1), ec="k", fc="none", lw=0.2)
    ax.add_collection(poly)

    for cell_id in cell_ids.flat:
        xc, yc = healpix_geo.nested.healpix_to_lonlat(cell_id, level, ellipsoid="WGS84")
        xc = xc.squeeze()
        yc = yc.squeeze()
        lon, lat = healpix_geo.nested.vertices(cell_id, level, ellipsoid="WGS84")
        lon = (lon + 180) % 360 - 180

        kw = {"ec": "k", "fc": "none"}
        if cell_id == center:
            c = list(to_rgba("firebrick"))
            c[3] = 0.5
            kw["fc"] = c
        if cell_id in neighbours:
            c = list(to_rgba("firebrick"))
            c[3] = 0.2
            kw["fc"] = c

            i = neighbours.index(cell_id)
            ax.annotate(i + 1, (xc, yc), ha="center", va="center_baseline", fontsize=12)

        poly = PolyCollection(np.stack([lon, lat], axis=-1), **kw)
        ax.add_collection(poly)

    ax.coastlines(lw=0.5, alpha=0.5)
    ax.set_aspect("equal")
    ax.set_extent((-6, 6, 34.5, 46), crs=ccrs.PlateCarree())

    ax.set_xticks([-5, 0, 5], crs=ccrs.PlateCarree())
    ax.xaxis.set_major_formatter(LongitudeFormatter())

    ax.set_yticks([35, 40, 45], crs=ccrs.PlateCarree())
    ax.yaxis.set_major_formatter(LatitudeFormatter())

    ax.grid(False)
    ax.tick_params(labelsize=9)

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
