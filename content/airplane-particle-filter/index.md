+++
+++

Estimate plane position using particle filter.
<!-- more -->

A airplane (green circle) is moving right on line (green line). The altitude of airplane is known. The position of airplane is NOT known to us.

A mountain range (white curve) is known already to us.

We get distance from airplane to mountains as a measurement. This has no noise in it.

We need to estimate where the airplane is based on the terrain and measurements using particle filter method.

Particles are represented by (red circle)s. Convergence of red particles indicate the guess of the method.

{{ video(name="rec.mp4", caption="white curve is terrain (known exactly), green circle and horizontal line are plane and its path, blue vertical line is plane's distance from terrain (measurement, known exactly), blue horizontal line indicates possible positions for the plane's measurement, red circles and lines are particles and their distances from terrain") }}
