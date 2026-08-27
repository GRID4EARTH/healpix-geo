import io
import pathlib
import site
import sys

site.addsitedir(str(pathlib.Path(__file__).parent))

import matplotlib.pyplot as plt
import numpy as np
from conversions import authalic_radius, convert
from matplotlib.patches import Arc, Ellipse
from script_utils import differing_contents

a = 1
f_ = 3
f = 1 / f_
e = np.sqrt(2 * f - f**2)

b = a * (1 - f)


def angle_to_xy(angle, a, b):
    x = a * np.cos(angle)
    y = b * np.sin(angle)

    return x, y


def normal_at(angle_p, a, b):
    x_p, y_p = angle_to_xy(angle_p, a, b)

    m_x = 2 * x_p / a**2
    m_y = 2 * y_p / b**2

    return m_x, m_y


def generate_figure():
    fig, ax = plt.subplots(figsize=(12, 12))
    # ellipse
    ellipse = Ellipse(
        (0, 0),
        width=2 * a,
        height=2 * b,
        edgecolor="black",
        linewidth=1,
        facecolor="none",
    )
    ax.add_patch(ellipse)

    ax.hlines(y=0, xmin=-a, xmax=a, color="black", linewidth=1)
    ax.vlines(x=0, ymin=-b, ymax=b, color="black", linewidth=1)
    ax.annotate("a", xy=(0.5 * a, 0), xytext=(0.5 * a, -0.05 * b))
    ax.annotate("b", xy=(0, 0.5 * b), xytext=(-0.04 * a, 0.5 * b))

    parametric_latitude = np.deg2rad(50)
    m_x, m_y = normal_at(parametric_latitude, a, b)

    # points
    h_p = 0.05 * a
    x_p0, y_p0 = angle_to_xy(parametric_latitude, a, b)
    x_p = x_p0 + h_p * m_x
    y_p = y_p0 + h_p * m_y

    ax.scatter(x_p0, y_p0, zorder=4)
    ax.scatter(x_p, y_p, zorder=4)

    ax.annotate("$P$", xy=(x_p, y_p), xytext=(x_p + 0.015 * a, y_p - 0.05 * b))
    ax.annotate("$P_0$", xy=(x_p0, y_p0), xytext=(x_p0 + 0.015 * a, y_p0))
    ax.annotate("$h$", xy=(x_p0, y_p0), xytext=(x_p0 + 0.05 * a, y_p0 + 0.05 * a))

    # tangent at P_0
    t = np.linspace(-0.05, 0.05, 100)
    x_t = x_p0 - m_y * t
    y_t = y_p0 + m_x * t
    ax.plot(x_t, y_t, linewidth=1, color="black")

    # geographic latitude
    t_y = -x_p / m_x
    n = np.linspace(t_y, 0, 200)
    x_geographic = x_p + m_x * n
    y_geographic = y_p + m_y * n
    ax.plot(x_geographic, y_geographic, linewidth=1, color="black")

    t_x = -y_p / m_y
    x_g0 = x_p + m_x * t_x
    y_g0 = y_p + m_y * t_x
    geographic_latitude = np.arctan(a / b * np.tan(parametric_latitude))
    geographic_arc = Arc(
        (x_g0, y_g0),
        width=0.25 * a,
        height=0.25 * a,
        theta1=0,
        theta2=np.rad2deg(geographic_latitude),
    )
    ax.add_patch(geographic_arc)
    ax.annotate(r"$\phi$", xy=(0, 0), xytext=(0 + 0.4 * a, 0 + 0.02 * a))

    rectangular_on_tangent1 = Arc(
        (x_p0, y_p0),
        width=0.08 * a,
        height=0.08 * a,
        theta1=np.rad2deg(geographic_latitude),
        theta2=np.rad2deg(geographic_latitude) + 90,
        linewidth=0.5,
    )
    rectangular_on_tangent2 = Arc(
        (x_p0, y_p0),
        width=0.095 * a,
        height=0.095 * a,
        theta1=np.rad2deg(geographic_latitude),
        theta2=np.rad2deg(geographic_latitude) + 90,
        linewidth=0.5,
    )
    ax.add_patch(rectangular_on_tangent1)
    ax.add_patch(rectangular_on_tangent2)

    # geocentric latitude
    v = np.linspace(0, 1, 200)
    x_geocentric = 0 + x_p0 * v
    y_geocentric = 0 + y_p0 * v
    ax.plot(x_geocentric, y_geocentric, linewidth=1, color="black")

    geocentric_latitude = np.arctan(b**2 / a**2 * np.tan(geographic_latitude))
    geocentric_arc = Arc(
        (0.0, 0.0),
        width=0.5 * a,
        height=0.5 * a,
        theta1=0,
        theta2=np.rad2deg(geocentric_latitude),
    )
    ax.add_patch(geocentric_arc)
    ax.annotate(r"$\theta$", xy=(0, 0), xytext=(0 + 0.19 * a, 0 + 0.02 * a))

    # reduced latitude
    semi_major_circle = Arc(
        (0, 0),
        width=2 * a,
        height=2 * a,
        angle=0,
        theta1=0,
        theta2=90,
        linestyle="--",
    )
    ax.add_patch(semi_major_circle)
    ax.vlines(x=0, ymin=b, ymax=a, color="black", linestyle="--", linewidth=1)
    x_r0 = x_p0
    y_r0 = np.sqrt(a**2 - x_p0**2)
    ax.scatter(x_r0, y_r0, zorder=4)
    ax.annotate(r"$P_\beta$", xy=(x_r0, y_r0), xytext=(x_r0 + 0.015 * a, y_r0))

    ax.vlines(x=x_r0, ymin=0, ymax=y_r0, color="black", linestyle="--", linewidth=1)
    ax.plot([0, x_r0], [0, y_r0], color="black", linewidth=1)

    reduced_arc = Arc(
        (0, 0),
        width=0.3 * a,
        height=0.3 * a,
        theta1=0,
        theta2=np.rad2deg(parametric_latitude),
    )
    ax.add_patch(reduced_arc)
    ax.annotate(r"$\beta$", xy=(0, 0), xytext=(0 + 0.07 * a, 0 + 0.02 * a))

    # authalic latitude
    r_a = authalic_radius(a, e)
    authalic_latitude = convert(geographic_latitude, e)
    x_a = 0 + r_a * np.cos(authalic_latitude)
    y_a = 0 + r_a * np.sin(authalic_latitude)

    authalic_arc = Arc(
        (0, 0),
        width=0.7 * a,
        height=0.7 * a,
        theta1=0,
        theta2=np.rad2deg(authalic_latitude),
    )
    ax.add_patch(authalic_arc)

    ax.scatter(x_a, y_a, zorder=4)
    ax.annotate(r"$P_\xi$", xy=(x_a, y_a), xytext=(x_a - 0.015 * a, y_a - 0.045 * a))

    ax.plot([0, x_a], [0, y_a], color="black", linewidth=1)
    ax.annotate(
        r"$r_\xi$",
        xy=(
            0.5 * r_a * np.cos(authalic_latitude),
            0.5 * r_a * np.sin(authalic_latitude),
        ),
        xytext=(
            0.7 * r_a * np.cos(authalic_latitude) + 0.015 * a,
            0.7 * r_a * np.sin(authalic_latitude) - 0.015 * a,
        ),
    )

    authalic_circle = Arc(
        (0, 0),
        width=2 * r_a,
        height=2 * r_a,
        angle=0,
        theta1=0,
        theta2=90,
        linestyle="--",
    )
    ax.add_patch(authalic_circle)
    ax.annotate(r"$\xi$", xy=(0, 0), xytext=(0 + 0.29 * a, 0 + 0.02 * a))

    ax.axis("equal")
    ax.axis("off")

    return fig


def main():
    fig = generate_figure()

    if len(sys.argv) != 2:
        raise ValueError("invalid number of arguments")

    buffer = io.BytesIO()
    fig.savefig(buffer, dpi=600, bbox_inches="tight")
    contents = buffer.getvalue()

    path = pathlib.Path(sys.argv[1])
    if not path.exists() or differing_contents(contents, path.read_bytes()):
        path.write_bytes(contents)


if __name__ == "__main__":
    main()
