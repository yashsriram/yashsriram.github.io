+++
+++

I am a software engineer at [Braincorp](https://braincorp.com/). I mainly work on on-the-robot software for our autonomous floor cleaners and shelf scanners.

I am interested in the problems at the intersection of Robotics, Computational Bio-mechanics, and Physics-based simulation.

I completed a M.S. program in Computer Science with a focus on Robotics at University of Minnesota, Twin Cities (2021) where I was fortunate to work with [Prof. Stephen Guy](https://www-users.cse.umn.edu/~sjguy/). Before that I completed a B.Tech program in Computer Science and Engineering at Indian Institute of Technology, Bombay (2019), where I closely worked with [Prof. Vinayak Naik](https://www.vinayaknaik.info/) and [Prof. Mythili Vutukuru](https://www.cse.iitb.ac.in/~mythili/).

# Research

## Motion planning for Wall Climbing 🧗

<div style="display: flex;">
<div style="flex-grow: 1; flex-shrink: 1">
<figure>
<img width="500px" height="500px" src="research/stick-figure-agent-sending-a-route.gif" alt="A stick figure agent sending a route">
<figcaption>A stick figure agent sending a route</figcaption>
</figure>
</div>
<div style="flex-grow: 1; flex-shrink: 1">

In wall climbing, an agent starts with an initial pose and then uses protrusions on the wall (called holds) to lock onto and climbs to the finish hold at the top. In this project, we achieved reliable climbing with natural looking motion for a two arm agent. We use random sampling in conjunction with gradient descent for inverse kinemetics computation and arm level control. We use a neural network, trained using cross-entropy optimization, as a predictor for optimal neck positioning.

[[Website]](/stick-solo) [[Report]](/stick-solo/report.pdf) [[Code]](https://github.com/yashsriram/stick-solo)
</div>
</div>

## Douglas–Peucker algorithm for landmark refinement 🚏

<div style="display: flex;">
<div style="flex-grow: 1; flex-shrink: 1">
<figure>
<img width="500px" height="500px" src="research/landmark-refinement-using-iep.gif" alt="Landmark refinement using iterative endpoint method">
<figcaption>Landmark refinement using iterative endpoint method</figcaption>
</figure>
</div>
<div style="flex-grow: 1; flex-shrink: 1">

For a differential drive robot with noisy lidar and control, we use Extended Kalman Filter to perform simultaneous localization and mapping (SLAM).
Notably we use [Douglas–Peucker algorithm](https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm) to refine landmarks used for SLAM state.

[[Website]](/sixth-sense) [[Report]](/sixth-sense/report.pdf) [[Code]](https://github.com/yashsriram/sixth-sense)
</div>
</div>

## Real-time vision-based 2D localization on mobile phone 👁️

<div style="display: flex;">
<div style="flex-grow: 1; flex-shrink: 1">
<figure>
<img width="500px" height="500px" src="research/drawing-star-with-mobile.gif" alt="Drawing with mobile phone">
<figcaption>Drawing with mobile phone</figcaption>
</figure>
</div>
<div style="flex-grow: 1; flex-shrink: 1">

We use a mobile phone's front camera to detect feature points using AKAZE feature detector.
We compare the set of features incoming to that of the first frame.
We then estimate the euclidean transform between these two sets to localize the phone.
We always assume motion in 2D plane.
We peform all computation on the device itself.

[[Website]](/hawkeye) [[Report]](/hawkeye/report.pdf) [[Code]](https://github.com/yashsriram/hawkeye)
</div>
</div>

## Real-time safety-guaranteed path following

<div style="display: flex;">
<div style="flex-grow: 1; flex-shrink: 1">
<figure>
<img width="500px" height="500px" src="research/orrt-vis-sideview.gif" alt="Real-time safety-guaranteed path following">
<figcaption>Real-time safety-guaranteed path following</figcaption>
</figure>
</div>
<div style="flex-grow: 1; flex-shrink: 1">

- Real-time algorithms for following a given path while avoiding initially unknown static obstacles.
- Uses RRT\* and Visibility graph along with padding obstacles.

[[Website]](/drive) [[Report]](/drive/report.pdf) [[Code]](https://github.com/yashsriram/drive)
</div>
</div>
