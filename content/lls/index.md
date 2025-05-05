+++
+++

Using linear least squares to fit a line given some samples
<!-- more -->

Samples are $(x_i, y_i)$ pairs. To find a line $y = mx + c$ that fits them best we form matrices A and b where

$A = \begin{bmatrix} x_1 & 1 \\\\ x_2 & 1 \\\\ ... \\\\ x_n & 1 \end{bmatrix} $
$fit = \begin{bmatrix} m \\\\ c \end{bmatrix}$
$b = \begin{bmatrix} y_1 \\\\ y_2 \\\\ ... \\\\ y_n \end{bmatrix}$

$A * fit = b$

$fit = (A^TA)^{-1}(A^Tb)$

{{ video(name="rec.mp4", caption="white circles are samples, yellow line is the lls fit, white lines are perpendiculars which are getting minimized") }}
