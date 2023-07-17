+++
+++

Which of the following is true for pose graph and its optimization for a planar robot with wheel encoders and lidar scans?

[ref matlab autonomous navigation series](https://youtu.be/saVZtgPyyJQ)

- [x] Kalman filter and Particle filter estimate optimal current state given new measurements.
- [x] Pose graph optimization estimates the optimal trajectory using all the measurements until now.
---
- [ ] Graph nodes represent positions.
- [ ] Graph nodes represent positions + orientation.
- [x] Graph nodes represent position + orientation + scans.
---
- [ ] Graph edges represent change in position.
- [x] Graph edges represent change in position + orientation.
- [ ] Graph edges represent change in position + orientation + scans.
---
- [x] Graph edges stiffness is proportional to the certainity of the transform.
- [ ] Graph edges stiffness is inversely proportional to the certainity of the transform.
---
- [x] Loop closure adds tension in a free graph.
- [ ] Loop closure is the only thing that adds tension in a graph.
- [ ] Loop closure can only be done once.
