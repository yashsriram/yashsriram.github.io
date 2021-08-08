# get out
1. Sleep cycle/Fitness/Food
1. Climbing
1. Driving
1. Swimming, Surfing, Windsurfing, Kitesurfing
1. Archery
1. Film making
1. Longboard tricks
1. Camping
1. Hammock/Frisbee
1. Rafting
1. Gaming
1. Canoeing
1. Hiking
1. Kayaking
1. Skydiving license
1. SUP
1. Snowboarding
1. Guitar

# hard earned wisdom
- The best task management system until now is a white board and a marker.
- The best information note taking system is taking no notes at all.
- The best running notes are the ones that are forgotten shortly later.
- The best information gathering system until now is this.
- __Smart docs, create docs from code directly__ (code is documentation of itself).

# what i know? (bad question)

# sanity check
- [ ] Implement it in Rust (really avoid C++).
- [ ] Few small independent problems rather than a big problem.
- [ ] Increment problem as we go.
- [ ] Starting should consist mostly of what I already know.
- [ ] Crisp problem statement.
    - [ ] Fixed.
    - [ ] Not too ambitious.
    - [ ] Towards target topics.
    - [ ] Be carefully ambitious.
- [ ] Rigourously understand solution.  (unlike some deep neural networks).
- [ ] Would like if it has some physical manifestation.
- [ ] Would like to provide some theoretical guarantees (correctness, time, space usage).
- [ ] Would like to visualize stuff.
- [ ] Try to take dynamics/planning/sensing from nature. Less is better.
- [ ] Try to avoid "natural looking" motion generation problems.

# ideas
- Math.
    - Structures and algorithms.
        -
            1. Largest Subarray Absolute Sum.
            1. Largest Absolute Subarray Sum.
        -
            1. Range Sum Query.
            1. Best time to buy and sell stock.
            1. Min Cost Climbing stairs.
            1. Number of ways of climbing stairs.
            1. Min falling path sum (adjacent).
            1. Min falling path sum (non-zero shift).
        -
            1. Is subsequence.
            1. Longest common subsequence.
            1. Longest palindrome subsequence.
            1. Longest increasing subsequence.
                1. sort + dedup + lcs
                1. lis-to
                1. binary search?
        -
            1. Min elements to remove to make a mountain array. == largest mountain array.
        -
            1. DP computation strategies.
                1. Iterative Table filling O(mn) (sometimes called tabulation).
                1. Recursive Table filling O(mn) can prune space (sometimes called memoization).
                1. Recursive without table O(m^n).
        -
            1. Count number of ones in bin representation.
            1. Uniform cost search with,
                1. All non-negative weights.
                1. Some negative weights too.
    - Code/Visualize/Animate the math (name: math in action).
        1. Small interesting problems.
            1. Polynomial project improvement using (lin + non-lin) opts.
            1. Tesselation based vis of 3d functions.
            1. Use 2d vis to illustrate diff randomness.
        1. Freeflow
        1. FK, IK (backward kineamtics) animation.
        1. Fourier bots
        1. SPH
        1. Parallelization.
        1. Simulations
            1. Visualizing love or love languages
            1. Visualizing/Simulating life
            1. Pandemic simulation
- Sensing Obstacle (static, dynamic), localization, mapping.
    1. Matrix theory as a tool.
    1. Sensing, Probability and estimation as seperate legos.
    1. CV course and geometric CV and graphics course problems.
        - Camera Intrinsic (sensor and lens) and Extrinsic parameters, Single camera calibration, keypoint detectors, descriptors
        - Rolling shutter measurement models, plane-projective geometry, fundamental/essential matrix, epipolar constraints , multi camera calibration, tracking
        - Depth parameterizations and their linearity properties; Levenberg Marquadt, Visual SLAM, SFM
    1. Lidar object detection.
    1. Camera object detection.
    1. Hawkeye\*, Enhanced inertial and visual Odometry using an android phone.
    1. Lane detection using camera and lidar.
    1. Sensor fusion
        - Loose-coupling between sensors (e.g. GPS and INS)
        - Tight-coupling between sensors, sensor measurement noise models, Odometry guided by LiDAR, RADAR, GPS and/or vision
        - Observability gramians, Delayed measurement compensation, colored and correlated sensor noise, Sliding window based online fusion approaches (e.g., MSCKF)
    1. Sensing dynamic obstacles.
- Planning (simple hand tuned agents, optimization based agents, bio-inspired agents).
    1. Prob/Bayes/MDP/RL
        - Basic continuous and discrete distributions. Usage of off-the-shelf Kalman filters (or other estimation algs.) software packages
        - Baye's theorem, MLE, MAP estimates, Kalman filters and variants, Mahalanobis metric
        - Estimation of nonlinear kinematics, Parameterizations based on linearization properties. Probability distributions w.r.t. nonlinear functions; Joseph forms of covariance updates; Design decisions around numeric stability. .
    1. Combining classical, ML, and DL {sensing, planning, RL}.
    1. Planning in Dynamic obstacles.
    1. Multi-robot coordination.
    1. Bio-agents. Improving climbing agents/ Co-operating climbing agents like naruto.
    1. Space X landing agent. Interplanetary travelling agent.
    1. Agents for games. Exploratory Horror Ghosts. Road crossing game.
    1. Non-holonomic systems. Parallel parking. Autonomous driving agent. Improved Wall-e, sensing wise, planning wise, dynamics wise. Use simple and effective methods from research papers.
    1. Safety critical planning.
    1. Nd space search
      1. Locomotion tasks (dynamics-wise)
      1. Kinematic chain (serial manipulator)
      1. Rocket (gravity wells + orbits + transfers)
      1. Ship (non-holonomic + water sim + replanning)
      1. Submarine (non-holonomic 3d)
      1. Car (non-holonomic 3d + road)
      1. Airplane/Drone (fluid dynamics)
- Acting (dynamic models).
- Useful Bio-inspired agents.
- Misc
    1. reliable resilient gallery manager
    1. Music Buttons, Haptic music
    1. FabX expressive ink cloth
    1. Holotalk, Live slides
    1. Real roadrash

# DAG stuff

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
### AI2
- HW3, HW5 understand

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
    `The 1950s were not good years for mathematical research. We had a very interesting gentleman in Washington named Wilson. He was secretary of Defense, and he actually had a pathological fear and hatred of the word ‘research’. I'm not using the term lightly; I'm using it precisely. His face would suffuse, he would turn red, and he would get violent if people used the term ‘research’ in his presence. You can imagine how he felt, then, about the term ‘mathematical’. The RAND Corporation was employed by the Air Force, and the Air Force had Wilson as its boss, essentially. Hence, I felt I had to do something to shield Wilson and the Air Force from the fact that I was really doing mathematics inside the RAND Corporation.

    What title, what name, could I choose? In the first place I was interested in planning, in decision making, in thinking. But planning, is not a good word for various reasons. I decided therefore to use the word ‘programming’. I wanted to get across the idea that this was dynamic, this was multistage, this was time-varying—I thought, let’s kill two birds with one stone. Let’s take a word that has an absolutely precise meaning, namely ‘dynamic’, in the classical physical sense. It also has a very interesting property as an adjective, and that is it’s impossible to use the word ‘dynamic’ in a pejorative sense. Try thinking of some combination that will possibly give it a pejorative meaning. It’s impossible. Thus, I thought “dynamic programming’ was a good name. It was something not even a Congressman could object to. So I used it as an umbrella for my activities.`
- both substructure based and greedy want to optimize globally only.
    -  substructure based general charecteristics = non-lin optimization + optimal substructure + overlapping subproblems.
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

- C/C++/Rust
- Java/Kotlin/Go
- Python
- Shell

# UMN papers proxy
http://login.ezproxy.lib.umn.edu/login?url=

# school bag stuff

## zen_of_coding
- Bitwise operations are much faster than arithmetic operations. So use them if possible. For ex in bit manipulation n, n - 1 bit manipulation trick
- Linked list dummy initial node :P
- Systemetic case expansion
- Working with examples
- Arrays that index main arrays to do things :P
- Sometimes things to be done in second loop can be done directly in first

if the code has too many different things => it's bad
	if you can manage to make too many different things into too many similar things => you are on right path, but still it's bad
	if you can manage to make too many similar things into one thing and use it repeatedly => you're code is probably good

any code preferably should not exit() per se, it is better to return to the caller with ROLLED BACK state
if you need to write to a block with bad offsets => read, write in memory and write back the whole block

## todo-in-escape-errands

Client:
	daily analysis and summary
	notifications
	* scroll to refresh

Server:
	improve goal search app
		* group by families
		set bg of overdue goals to red
	Job model
	goal color CU features
	keyboard shortcuts
	* title
	password

## things_that_matter_while_buying_a_new_laptop

build quality
	hinges, ports, sturdiness
keyboard experience
trackpad and dedicated mouse keys
enough number of required ports

## coding-tips

### android

1. not necessary for ids to be different, but RECOMMENDED - first one in the tree gets called
1. SQLite is a lone wolf- self-contained, serverless, zero conf, out of the box

### arithmetic-operators

* In general, division operator is much slower than bitwise operators (& | ~), therefore in cases where possible use them

## DB

### ||computing

A function has to be commutative and associative so that it can be performed parallelly

## OS

### processes

1. a program in execution, an execution entity to consume CPU, an instance of a program
1. generally one core of a CPU executes one instruction at a time
1. running multiple processes or threads -> CPU switching or time slicing
1. fork -> duplicate (copy on write method)
1. exec -> replace current process
1. hence child before parent is good as memory need not be reproduced

## apping

### listofunivs

#### US
MIT
Stanford University
University of California—Berkeley
Harvard University
California Institute of Technology
Carnegie Mellon University
Cornell University
University of Illinois—Urbana-Champaign
University of Washington
Princeton University

Univs(PT Cell): IST Austria, TU Braunschweig, EPFL, Virginia Tech, Johns Hopkins, Purdue University,
Univs(Apping): Inria, TU Kaiserlautern, NUS, Princeton, ENS Cachan, Chennai Mathematical Institute, HBCSE, University of Liege
Companies(PT Cell): Dreamworks, Edelweiss, Silverleaf, Altisource, Reliance Jio, EFI, Stratbeans Consulting, Siemens, Indus OS, Wellthy, Tech Mahindra, Infurnia, Evok Analytics, Carnot Tech, Get Focus, Novanet, Adoro, Forbes Marshall, Saavn, Philips Research, Truckola Technologies, MRCC, Diamond Technology Solutions, Cogo Freight, RBL Bank, Wrig Nanosystems.
Companies(Apping): Fractal Analytics, KPMG
Winter:
Companies(PT Cell): Edelweiss, Cityflo, Zupp, Creative Concepts, Genext, Zoomot Services, Novanet, NextGen PMS, Tavaga, Darwin Inc, Mia Mia, Zouk, HolaMed, Toppr, PS Takecare<>, Greendzine, Plus Capital, Gupshup Technology
I'm not sure how useful this info will be, but it should give you an idea that there are some options available to you people.
