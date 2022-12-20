+++
+++

# Research Projects

## Motion planning for Wall Climbing 🧗


<img width="50%" src="stick-figure-agent-sending-a-route.gif" alt="Stick figure agent sending a route">

In wall climbing, an agent starts with an initial pose and then uses protrusions on the wall (called holds) to lock onto and climbs to the finish hold at the top. In this project, we achieved reliable climbing with natural looking motion for a two arm agent. We use random sampling in conjunction with gradient descent for inverse kinemetics computation and arm level control. We use a neural network, trained using cross-entropy optimization, as a predictor for optimal neck positioning.

[[Website]](https://yashsriram.github.io/stick-solo) [[Report]](https://yashsriram.github.io/stick-solo/report.pdf) [[Code]](https://github.com/yashsriram/stick-solo)

---

## Noisy Lidar-equipped differential drive Robot Navigation🚏

<img width="50%" src="iep.png" alt="Stick figure agent sending a route">

For a differential drive robot with noisy lidar and control, we use Extended Kalman Filter to perform simultaneous localization and mapping (SLAM).
Notably we use [Douglas–Peucker algorithm](https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm) to refine landmarks used for SLAM state.

[[Website]](https://yashsriram.github.io/sixth-sense) [[Report]](https://yashsriram.github.io/sixth-sense/report.pdf) [[Code]](https://github.com/yashsriram/sixth-sense)


---

## Real-time vision-based 2D localization on mobile phone 👁️

<img width="20%" src="drawing-star-with-mobile.gif" alt="Drawing a star using a mobile">

We use a mobile phone's front camera to detect feature points using AKAZE feature detector.
We compare the set of features incoming to that of the first frame.
We then estimate the euclidean transform between these two sets to localize the phone.
We always assume motion in 2D plane.
We peform all computation on the device itself.

[[Website]](https://yashsriram.github.io/hawkeye) [[Report]](https://yashsriram.github.io/hawkeye/report.pdf) [[Code]](https://github.com/yashsriram/hawkeye)



---
