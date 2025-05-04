+++
+++

The variance of a MAP estimate can only decrease viz. MAP can only get more confident which seems bizzare.
<!-- more -->

In max a-posteriori estimate __map__, we find a gaussian that fits the data in a certain way.

Since

$
\frac{1}{\sigma^2} =
\frac{1}{\sigma_1^2} +
\frac{1}{\sigma_2^2} +
\frac{1}{\sigma_3^2} +
...
\frac{1}{\sigma_N^2}
$

Adding a new sample

$
\frac{1}{\sigma^2} =
\frac{1}{\sigma_1^2} +
\frac{1}{\sigma_2^2} +
\frac{1}{\sigma_3^2} +
...
\frac{1}{\sigma_N^2} +
\frac{1}{\sigma_N+1^2}
$

$\sigma$ will only ever decrease.

I was under the impression that MAP and MLE are same. Probably because they are taught as limiting cases. But they have entirely different formulation. For ex. MLE variance can increase or decrease whereas MAP variance can only ever decrease.

{{ video(name="rec.mp4", caption="yellow circles are samples $x_i$, red circle, green line is the map $(\mu, \sigma)$") }}
