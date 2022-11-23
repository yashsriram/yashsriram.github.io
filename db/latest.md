## calculus
- limits
- the beauty of the phrase exact approximation
- (irony) There exist things which we don't know but can approximate (ex. integrals)
- partial derivatives
- derivative of vectors and matrices, jacobians, hessians, laplacians etc...

## probability
- Relating probability to real world. Is there any law like newton's laws?
- Is probability a belief (just a function) or limit of experiment?
    - If it is a belief how do you perceive it?
    - If it is a limit of experiement how to you make simple arithemetic out of it?
- For now assuming probability is just a function with no semantics (kind of makes it hard to develop it since there is no meaning or use).
- Cdf and pdf definitions
- Expected value definitions
- Joint probability definition
- Joint vs Conditional probability
- Functions of random variable
- Random vectors
- How to determine if an experiment is stochastic?
- How to determine where the simple division conditional probability rule is not good?
- P(A, B, C, D) (prob of cartesian product of axes) as chain rule which itself is a DAG.
    - use cond prob to represent, use conditional independence.
    - Space saving with chain form and conditional independence. Assume two values for each axis,
        - O($2^m$) vs O($2^m (1/2 + 1/2^2 + ... + 1/2^m) $) with conditional independence the 1/2^i terms decrease.

## graphs
- DAG terms.
    - children, parents, descendants, ancestors, non-descendants, non-ancestors (defs using reachability using parent and child edges).
    - how do c, p, d, a of a node appear in a topo sort?
    - do all nodes before a node in a topo sort are ancestors? (need not be). similarly after/descendants.
### fuzzily separated tree searches
- Ones that have no state
- Ones that have a fixed size state (finding nearest vertex to a position)
- Ones that have a per-vertex state (path from start to current vertex)


## linear algebra
- composition of linear transforms, composition as product of matrices, building matrix as tool
- proof of composition of linear transform is a linear transform, algebraically and geometrically
- misconceptions: \xcancel{animating vectors} vs \xcancel{animating coordinate frames} don't try to animate (lerp) anything
- latest frame vs initial frame
- representation of rotation: rot mat, quaternions, axis-angle, eular, roll pitch yaw, gimbal lock
- determinant, descriminant whatever
- cross product
- inverses
- column spaces, rank
- null spaces
- eigen stuff, eigen basis
- orthogonal, orthonormal
- diagnoal
- quaternions
- SVD, other easy useful decompositions
- quadratic systems using matrices like $X^TAX$.
- derivatives of matrices, matrix products like $X^T X$.
1. Linear Algebra
    - Line fitting, L1 and L2 norms, least squares costs, Matrix invertibility/rank properties
    - Null space of a matrix, Matrix norms, Gradients, Jacobians, Hessians, Eigen Decompositions, SVD
    - Matrix decompositions (e.g. UDUt, LU, QR), Schur Complements, Projective geometry w.r.t. matrix rank-1 updates
1. 3D euclidean geometry
    - Points, Lines, Vectors, Cross products, Dot Products. Right and left handed coordinate systems, coordinate transforms
    - 3d line formulations; SE3 group and se3 algebra, SLERP interpolation
    - Tradeoffs of different forms of rotations (e.g., Euler angles, Rodrigues rotations. SO(3), Quaternions); Vector representations in frames of reference that are in motion relative to one another.

### more linear algebra
- multiplication of quaternions is not commutative
- rot mat $<-\`>$ eular angle $<->$ rpy
- what is gimbal lock? not single solution with gimbal lock?
- axis-angle $<->$ rotation matrix - eigen basis, null space finding
- Code/Visualize/Animate the math
- how to tell which system has how many and what type  of solutions? and what is a solution exactly?
- sensing: hw1 p1 how is orientation not determinable?
- $a^T * b = b^T$ * a iff a, b are vectors
- what is determinant of transform matrix with only rotation and translation ? == 1
- composition of linear transformations
- rotation \& quaternions

### matrix theory course
- Last lectures and compile doubts.

## estimation and optimization
- Rigourously understood pattern recognition (classical, deep).
- Reinforcement learning.

### 1
- data is a plural and a pronoun.
- accuracy vs precision
- estimation = estimate + uncertainity
- modelling uncertainity using stochasticity
- given a parametrized "stochastically noisy" model with stimulus and response (vector) variable and some samples, the process of estimating parameters (a.k.a fitting model). Using optimization for this estimation. Other non-optimization ways of doing the same. The essence of ML.

### 2
- $(x_i - min) / (max - min)$ set of values vs normalizing set of values, show they are not equal in some instances and properties of each
- emphasize diff b/w
    - min(sum(abs(residual))) $\equiv$ min(Sum of Absolute Errors). The average of SAE is MAE
    - min(sum(residual * residual)) $\equiv$ min(Sum of Sqaured Errors). The average of SSE is MSE
    - min(sum(perpendicular distance)) $\equiv$ min(Sum of Perpendicular Errors). The average of SPE is MPE
- Thinking mean and variance visually, mean of a list of numbers plotted on a co-ordinate system is "the rough line passing through the center", take differences from the rough center line to actual values and square them. Each of these towers or inverted towers can be thought to represent "certain variation" from the mean. Average (/n-1) of all this is defined as "variance". Etymology of "variance". observed the sqaured fn to ignore signs and preserve differentiability. Mind blown!!!. And all this from visualizing the definitions.
- R-squared = Explained variation / Total variation. what it means (capturing the variation for just the samples in context and does not say anything about predictablility of model), and especially what it does not mean (good/bad model (patterns in residual plots)). Is it scale invariant?
- Shallow vs deep classifiers $\equiv$ seperate description and classification vs together description and classification.

### 3
- does uncertainity change during fitting process?
- patterns in residual plots as a measure of good or bad model.
- Compare MLE vs MAP vs Linear Regression on.
    - fitting function
    - measure of accuracy, measure of uncertainity
    - covariance
- Determining uncertainity after fitting in linear regression (or other models). Using info about training datum (a.k.a. sample) error term?

### opt problems
- math opt = non-lin opt (convex + concave) + lin opt.

### substructure based opt problems
- dp = dynamic programming (misnomer). dynamic = time based (uses solutions of prev time steps i.e. sub structures), programming = optimization.
- From Richard Bellman autobiography,
```
The 1950s were not good years for mathematical research. We had a very interesting gentleman in Washington named Wilson. He was secretary of Defense, and he actually had a pathological fear and hatred of the word ‘research’. I'm not using the term lightly; I'm using it precisely. His face would suffuse, he would turn red, and he would get violent if people used the term ‘research’ in his presence. You can imagine how he felt, then, about the term ‘mathematical’. The RAND Corporation was employed by the Air Force, and the Air Force had Wilson as its boss, essentially. Hence, I felt I had to do something to shield Wilson and the Air Force from the fact that I was really doing mathematics inside the RAND Corporation.
    What title, what name, could I choose? In the first place I was interested in planning, in decision making, in thinking. But planning, is not a good word for various reasons. I decided therefore to use the word ‘programming’. I wanted to get across the idea that this was dynamic, this was multistage, this was time-varying—I thought, let’s kill two birds with one stone. Let’s take a word that has an absolutely precise meaning, namely ‘dynamic’, in the classical physical sense. It also has a very interesting property as an adjective, and that is it’s impossible to use the word ‘dynamic’ in a pejorative sense. Try thinking of some combination that will possibly give it a pejorative meaning. It’s impossible. Thus, I thought “dynamic programming’ was a good name. It was something not even a Congressman could object to. So I used it as an umbrella for my activities.
```
- The term "Substructure" is identical to "optimal substructure". The term itself means it can be decomposed into substructures and solved, the term optimal does not have much more meaning there.
- Substructure computation strategies (whether to reuse answers of already computed substructures or not, the "overlapping subproblems" term. Again not the best name).
    - Bottom up Table filling O(mn) (tabulation, generally iterative).
    - Top down Table filling O(mn) can prune entries (memoization, generally recursive).
    - Not reusing O(exp).
- both substructure based and greedy want to optimize globally only.
    - substructure based general charecteristics = non-lin optimization + optimal substructure + overlapping subproblems.
    - greedy general charecteristics = lin optimization; so local opt becomes global opt.
    - divide and conquer = lacks overlapping subproblems.
- This partition of problems is vague, non-exhaustive and sometimes really annoying.

## fourier analysis.

## language, grammer, automata, logic, parser, compiler
- Statically typed vs Dynamically typed == Compile time vs Run time type inferrence.
- Compiled vs Interpreted == Transforming code to target machine code before running vs No transformation before running. Compiled might be faster because
    - It is not reading the code and transforming it for every line.
    - It can do global optimizations which might not be possible in interpreted as its field of view is local.
- Statically vs Dynamically typedness is property of language. Compilation vs Interpration is the property of implementation not language; saying python is an interpreted language is wrong. Any language can be compiled. Any language can be interpreted.
- Define, map and visualize the terms: Memory safety, Memory containment and Type safety.
- Concurrency vs parallel.
