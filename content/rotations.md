+++
template = "reading.html"
description = "developing a keen sense for rotations in 3d."
+++

A robot has orientation $\bar{r}$.
We want to align it $\bar{t}$.
How should robot turn to do this the fastest?
{% mc() %}
- [ ] Always turn left.
- [ ] Always turn right.
- [x] Direction of $\bar{r} \times \bar{t}$.
The direction of cross product gives the fastest alignment direction.
- [ ] Direction of $\bar{t} \times \bar{r}$.
{% end %}

A quaternion can be described by 3 numbers.
x-coordinate and z-coordinate of axis of rotation and the angle of rotation.
It can be visualized as a cylinder axis aligned with y-axis.
The claim is that a cylinder with radius 1 and height PI placed on x-z plane spans a cone in terms of physical object rotation. Make an interactive program with this cylinder and corresponding rotation visualized. False claim.
