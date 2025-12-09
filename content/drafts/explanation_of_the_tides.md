+++
title = "Explanation Of The Tides"
date = 2025-01-01

[extra]
author = "Nicole Venner"
+++

So When I first saw this video by Neptunium, (And later a 3b1b video) I was curious if using only the measurement technhiques that existed back then, with the benefit of hindsight, is there another experiment one could devise to measure the distance to the sun?

Using the math of lunar eclipses, the ancient greeks were able to figure out the following units with pretty high accuracy using the radius of the earth as a measuring stick.

Given that $R_E$ is the radius of the earth

they calculated the distance to the moon by observing that a lunar eclipse takes 3.5 hours, to traverse the shadow of the earth during an eclipse. Given that a lunar month is 28 days,

$$
frac(D_M, R_E) approx frac(28 upright("days"), 3.5 upright("hours"))
$$

Giving you

$$
D_M approx 61 \cdot R_E
$$

And you can use the fact that moonrises and moonsets take 2 minutes to know that

$$
frac(2*R_M, 2*\pi*D_M) =  frac(2 upright("minutes"), 24 upright("hours"))
$$

$$
R_M approx 0.26 *R_E
$$

From here measuring the distance to the sun is much harder considering that the distance is so large:

$$
D_S =23241
$$

I thought I would start off with newton's law of gravitation

$$
F = G frac(m_1m_2, r^2)
$$

From here its actually possible to deduce some basic orbital mechanics without any calculus. By considering discrete timesteps. Lets consider that a small body is orbiting around a large body with radius $cal(R)$. And its orbit is $t$ seconds long, thus the body travels $ alpha = 2 pi t$ radians every second. Using some basic euclidean geometry and the law of sines.

[https://www.geogebra.org/geometry/cjnesrgx]

Lets us conclude that if the object traveled in a straight line instead of a circle for it to travel the same angular distance we observer, would mean that it would have to deviate from the circle by a distance of

$$2cal(R)frac(\sin^2(alpha/2), cos(alpha))$$

furthermore since most celestial bodies have orbits much much longer than one second, you can use the small angle approximation to clean up the math a bit
$cal(R)frac(alpha^2, 2)$$

So in order to force the body back onto the circle you must apply an acceleration every second of

$$cal(R)frac(alpha^2, 2) frac(m, s^2)$$

(dubious math over)

So consider we have a body of radius $R$ orbiting a distance $D$ away from a body of mass $M$. How could we calculate the tidal force on the front and behind the planet.

So the front component of the planet is experiencing an acceleration of $frac(M, (R-D)^2)$ where the other componet is experiencing a force of $frac(M, (R+D)^2)$

So solving for the difference in force $X$ gives us

$$
X &= frac(M, (R-D)^2) - frac(M, (R+D)^2) \
X(R-D)^2(R+D)^2  &= M ( (R+D)^2 - (R-D)^2 ) \
X(R^2-D^2)^2 &= M ( (R^2+2 R D+D^2) - (R^2-2 R D+D^2) ) \
X(R^2-D^2) &= M ( 4 R D ) \
X &= frac(4 M R D, (R^2-D^2)^2)
$$
