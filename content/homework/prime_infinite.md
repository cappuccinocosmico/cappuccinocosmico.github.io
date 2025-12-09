Lemma 1 
if \prod x_n converges to a nonzero value and all x_n are positive if and only if \sum ln(x_n) converges.


Assume that \prod x_n converges to x. Namely for any epsilon, there exists an N such that for all m> N x_m -x < epsilon.

Now we want to show that ln(x_n )




FALSE consider 
\prod_n=1 ^\infty 0 converged but ln doesnt


so \sum 1/x doesnt converge, and therefore \prod (\sum 1/p^n) = \prod (1/1-(1/p)) = \prod (p/(p-1))

so \sum ln(1/(1-(1/p))) diverges or \sum -ln(1-1/p) diverges.


but heres the kicker, for ln(1+x) = x-x^2/2+x^3/3 + x^4/4-x^5/5 + \cdots


so we have finally name our initial sequence Y= \sum_p - (\sum_k 1/kp^k ) for k going from 1 to infinity.

Now a small change we are going to make is a sequence Z = \sum_p -(\sum_k 1/kp^k) for k going from 2 to infinity. 

Now if  Y diverges and Z converges, then Z-Y is a sequence that diverges. And Z-Y happens to be \sum_p 1/p


So we can factor out a copy of p^2 from all terms like so giving us 
Z = \sum_p 1/p^2 \sum_k 1/(k+2)p^k for k=0 to infinity.
Z <  \sum_p 1/p^2 \sum_k 1/p^k for k=0 to infinity.
Z <  \sum_p 1/p^2 1/(1-1/p)
Since all prime numbers are greater then 2, then 1/(1-1/p) is less than 2 
Z <  2\sum_p 1/p^2
Which converges since it must be less then the basal sum \pi^2/6


1 + r + r^2 = x 
x-xr= 1
x(1-r)
