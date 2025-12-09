Mathematics is often presented in other diciplines as this weird dicipline where everything is perfect and knowable and somehow exists on this trancendental plane where its unaffected by uncertantinty or differences of opinion or ideology.

But this isnt really the case, and discussion of the very shaky foundation mathematics is based on is hidden from almost everyone mainly since understanding the prerequesite math behind these problems requires 1-2 years of grad school after 4 years of undergrad math. And talking about it at a pop-sci level doesnt benefit anyone like engineers and computer scientists who need to know small parts of math relavent to their diciplines.

But I think learning about this can be really useful for understanding various problems in philosophy and the social sciences and activism, specifically how to understand connections between competing ideologies.

(Note: This entire talk contains zero actual math, and should be readable even if you took an pre-algebra course in HS 20 years ago.)

# A brief History of Mathematics

Lets go ahead and jump back to 1850. Where after 200 years mathematicians Bolzano, Riemann, and Weierstrass had finally put Calculus on a firm logical foundation.

However, one minor uncomfortable fact remained. The fundamental mathematical object of calculus is a function that takes in real numbers and outputs real numbers.
(For example a function $f$ might take in values 1.0 and 2.3 and output values $f(1.0)=2.0$ and $f(2.3)=6.299$). However, under this current method of mathematics most functions do not behave nicely enough for calculus to work nicely. Most functions have gaps:

graph
f(x) =
1 x > 0
0 x <= 0

graph
f(p/q) = 1/q if x=p/q in Q
f(x)= 0 otherwise

graph
f(x) = 1/x

That make working with them using calculus impossible.

Furthermore, in order to fully utilize all the results you also want your functions to not have gaps, you also want them to be smooth, ie not have any sharp corners or cusps, or other bad behavior.

graph
f(x) = |x|

graph
f(x) = x\*sin(1/x)

graph
f(x) = sum([sin(x*3^i)/2^i for i in 1..100])

But this belies a somewhat curious fact, namely that the vast vast majority of mathematical functions do not behave nicely. In fact if you pick a random mathematical function the chance that it has no gaps is precisely 0%. And even if you are lucky enough to select a function without gaps, the chance that the function is also smooth is 0%.

\footnote{In fact its even worse, functions that are continuous at a single point have measure zero, meaning almost all mathematical functions are discontinuous everywhere. Likewise in the land of continuous functions, functions that have a first order derivative at a single point are measure zero, meaning almost all continuous functions are nowhere differentiable.}

But this is in stark contrast to what we observe in physical reality, where everything in physical reality is governed by smooth functions. The math of the planets move, the cosmos expands, how electronics and circuits behave, and the fields that govern quarks and electrons are all smooth.

This intuitive understanding of the world had lead to the initial assumption by the creators of calculus, Newton and Libnitz

1.
