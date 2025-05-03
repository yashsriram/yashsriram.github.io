+++
title = "Bizzarely the variance of MAP estimate can only increase"
+++

In max a-posteriori estimate __map__

__input__ $\mu_0$ $\sigma_0$ (prior) $x_i$ (samples) and __output__ $\mu$ $\sigma$

The variance of a MAP estimate can only decrease viz. MAP can only get more confident which seems bizzare.

This is because

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
\frac{1}{\sigma_{N + 1}^2}
$

will only decrease $\sigma$
