+++
template = "reading.html"
description = "will these two intersect?"
+++

These are interactive examples (not mobile friendly yet). They are compiled to wasm format. Each example needs ~20Mb binary download.
- [2 line segement intersection](/wasm.html?name=xn_2_lnsegs)
- [2 rays intersection](/wasm.html?name=xn_2_rays)
- [convex hull](/wasm.html?name=convex_hull)
- [convex spiral](/wasm.html?name=convex_spiral)

Consider
- $L_1 \equiv (1, 2, 3) + t (1, -1, 2)$
- $L_2 \equiv (3, 4, -12) + t (4, 1, 8)$

Select true statements.
{% mc() %}
- [ ] $L_1$ and $L_2$ are parallel.
- [x] $L_1$ and $L_2$ are not parallel.
- [ ] $L_1$ and $L_2$ are perpendicular.
- [ ] $(-2, 0, 1)$ is the unit normal vector to the plane by $L_1$ and $L_2$.
$(-2/\sqrt{5}, 0, 1/\sqrt{5})$ is the unit normal vector to the plane by $L_1$ and $L_2$.
{% end %}


Two line segments $PQ$ and $RS$ intersect $\leftrightarrow$
{% mc() %}
- [x] $P, Q$ should be on opposite sides of line $RS$.

- [x] $R, S$ should be on opposite sides of line $PQ$.

line RS divides points P, Q and line PQ divides points R, S $\rightarrow$ segments PR and RS intersect
<img src="./lineseg_lineseg2.jpg" height="100px"/>


segments PR and RS intersect $\rightarrow$ line RS divides points P, Q and line PQ divides points R, S
<img src="./lineseg_lineseg1.jpg" height="100px"/>

{% end %}

Two rays intersect $\leftrightarrow$
{% mc() %}
- [x] They should move towards each other cumulatively.
- [x] They should be on the same side of their base.
{% end %}

