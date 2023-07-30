+++
template = "reading.html"
+++

# 1

A [diff drive robot](https://en.wikipedia.org/wiki/Differential_wheeled_robot) has a orientation unit vector __c__.

We want to align it with orientation unit vector __t__.

How should robot turn to do this the fastest?

- [ ] Always turn left,
- [ ] Always turn right
- [x] Direction of $+\frac{(c \times t)}{|c| . |t|}$
- [ ] Direction of $-\frac{(c \times t)}{|c| . |t|}$

# 3

A quaternion can be described by 3 numbers.

x-coordinate and z-coordinate of axis of rotation and the angle of rotation.

It can be visualized as a cylinder axis aligned with y-axis.

The claim is that a cylinder with radius 1 and height PI placed on x-z plane spans a cone in terms of physical object rotation. Make an interactive program with this cylinder and corresponding rotation visualized. False claim.
