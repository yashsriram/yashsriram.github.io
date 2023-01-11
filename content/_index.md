+++
+++

I am a software engineer at [Braincorp](https://braincorp.com/). I mainly work on on-the-robot software for our autonomous floor cleaners and shelf scanners.

I am interested in the problems at the intersection of Robotics, Computational Bio-mechanics, and Physics-based simulation.

I completed a M.S. program in Computer Science with a focus on Robotics at University of Minnesota, Twin Cities (2021) where I was fortunate to work with [Prof. Stephen Guy](https://www-users.cse.umn.edu/~sjguy/). Before that I completed a B.Tech program in Computer Science and Engineering at Indian Institute of Technology, Bombay (2019), where I closely worked with [Prof. Vinayak Naik](https://www.vinayaknaik.info/) and [Prof. Mythili Vutukuru](https://www.cse.iitb.ac.in/~mythili/).

# Research

## Motion planning for Wall Climbing 🧗

<figure>
<img height="100%" src="research/stick-solo.gif" alt="A stick figure agent sending a route">
<figcaption>
In wall climbing, an agent starts with an initial pose and then uses protrusions on the wall (called holds) to lock onto and climbs to the finish hold at the top. In this project, we achieved reliable climbing with natural looking motion for a two arm agent. We use random sampling in conjunction with gradient descent for inverse kinemetics computation and arm level control. We use a neural network, trained using cross-entropy optimization, as a predictor for optimal neck positioning.

[[Website]](/stick-solo) [[Report]](/stick-solo/report.pdf) [[Code]](https://github.com/yashsriram/stick-solo)

</figcaption>
</figure>


## Douglas–Peucker algorithm for landmark refinement 🚏

<figure>
<img height="100%" src="research/sixth-sense.gif" alt="Landmark refinement using iterative endpoint method">
<figcaption>

For a differential drive robot with noisy lidar and control, we use Extended Kalman Filter to perform simultaneous localization and mapping (SLAM).
Notably we use [Douglas–Peucker algorithm](https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm) to refine landmarks used for SLAM state.

[[Website]](/sixth-sense) [[Report]](/sixth-sense/report.pdf) [[Code]](https://github.com/yashsriram/sixth-sense)

</figcaption>
</figure>

## Real-time vision-based 2D localization on mobile phone 👁️

<figure>
<img height="100%" src="research/hawkeye.gif" alt="Drawing with mobile phone">
<figcaption>

We use a mobile phone's front camera to detect feature points using AKAZE feature detector.
We compare the set of features incoming to that of the first frame.
We then estimate the euclidean transform between these two sets to localize the phone.
We always assume motion in 2D plane.
We peform all computation on the device itself.

[[Website]](/hawkeye) [[Report]](/hawkeye/report.pdf) [[Code]](https://github.com/yashsriram/hawkeye)

</figcaption>
</figure>


## Real-time safety-guaranteed path following


<figure>
<img height="100%" src="research/drive.gif" alt="Real-time safety-guaranteed path following">
<figcaption>

- Real-time algorithms for following a given path while avoiding initially unknown static obstacles.
- Uses RRT\* and Visibility graph along with padding obstacles.

[[Website]](/drive) [[Report]](/drive/report.pdf) [[Code]](https://github.com/yashsriram/drive)

</figcaption>
</figure>

# Physics

## Yet another Ray Tracer

<figure>
<a href="/yart"><img height="100%" src="physics/yart.gif" alt="Ray tracer based on Blinn-Phong model"></a>
<figcaption>Ray tracer based on Blinn-Phong model 

[[Website]](/yart) [[Code]](https://github.com/yashsriram/yart)

</figcaption>
</figure>

## Archer: 2D light reflectons

<figure>
<a href="/archer"><img height="100%" width="50%" src="physics/archer.gif" alt="Light reflections on plane and circular walls"></a>
<figcaption>

Light reflections on plane and circular walls

[[Website]](/archer) [[Code]](https://github.com/yashsriram/archer)

</figcaption>
</figure>
