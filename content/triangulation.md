+++
template = "reading.html"
description = "developing delaunay triangulation and voronoi diagrams step-by-step."
+++

Into how many regions does a triangle and its circumcircle divide the plane into?
{% mc() %}
- [ ] 4.
- [x] 5.
- [ ] 6.
{% end %}

A point $I$ is inside the triangle $ABC$.
{% mc() %}
- [x] Then $A$ will always be outside circumcirlce made by $BCI$.
- [x] Then $B$ will always be outside circumcirlce made by $CAI$.
- [x] Then $C$ will always be outside circumcirlce made by $ABI$.
- [ ] $I$ is outside circumcircle of $ABC$.
- [x] $I$ is inside circumcircle of $ABC$.
{% end %}

There are a few points on the plane. Can you draw a triangle with them that contains all points always?
{% mc() %}
- [ ] Yes.
- [x] No.
Just imagine 4 points making a square. There are $C_3^4 = 4$ triangles possible. None of them contains all points.
{% end %}


