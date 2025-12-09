What I want to prove:

# Proof: Continuity and Connected Graphs in Complete Metric Spaces



     Any continuous function with a disconnected image implies a disconnected preimage.

     Consider a continuous f: S -> X where A u B = X. Then the preimage f^-1(A) u f^-1(B) = f^-1(A u B) = f^-1(X) = S. Thus since S can be represented as a disjoint union then S is disconnected.

3. The Statement is False: A Counterexample

The original statement is false. Here's a counterexample using the topologist's sine curve:

Statement to prove:
For any complete metric space S, a function f: S → S is continuous if and only if the graph 
G = {(x,f(x)) | x ∈ S} is a connected subset of S × S.

## Approach:

2. For (⇒) direction:
   - Key idea: The graph G is the continuous image of S under the mapping x ↦ (x,f(x))
counterexample for other direction. 


Define f: ℝ → ℝ by:
   f(x) = sin(1/x) for x > 0
   f(x) = 0     for x ≤ 0

Provide a formal proofformally prove all the following things 



This function has the following properties:
1. The domain ℝ is indeed a complete metric space
2. The function is clearly discontinuous at x = 0
   - As x approaches 0 from the right, f(x) oscillates between -1 and 1
   - But f(0) = 0
3. However, its graph G is connected!
   - The oscillations become more rapid as x approaches 0
   - The closure of the graph includes all points (0,y) with y ∈ [-1,1]
   - This creates a "vertical line segment" at x = 0 in the closure
   - This property makes the graph connected despite the discontinuity

This counterexample shows that a function with a connected graph in a complete metric space need not be continuous.

