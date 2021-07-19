# calculus
- limits
- the beauty of the phrase exact approximation
- partial derivatives
- derivative of vectors and matrices, jacobians, hessians, laplacians etc...

# probability
- Cdf and pdf definitions
- Expected value definitions
- Joint probability definition
- Joint vs Conditional probability
- Functions of random variable
- Random vectors
- baysian networks, HMM, Markov, MDP, Q-learning ...
- How to determine if an experiment is stochastic?
- How to determine where the simple division conditional probability rule is not good?
- P(A, B, C, D) (prob of cartesian product of axes) as chain rule which itself is a DAG.
    - use cond prob to represent, use conditional independence.
    - Space saving with chain form and conditional independence. Assume two values for each axis,
        - O($2^m$) vs O($2^m (1/2 + 1/2^2 + ... + 1/2^m) $) with conditional independence the 1/2^i terms decrease.

# graphs
- DAG terms.
    - children, parents, descendants, ancestors, non-descendants, non-ancestors (defs using reachability using parent and child edges).
    - how do c, p, d, a of a node appear in a topo sort?
    - do all nodes before a node in a topo sort are ancestors? (need not be). similarly after/descendants.

# linear algebra
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

# more linear algebra
- multiplication of quaternions is not commutative
- rot mat $<-`>$ eular angle $<->$ rpy
- what is gimbal lock? not single solution with gimbal lock?
- axis-angle $<->$ rotation matrix - eigen basis, null space finding
- Code/Visualize/Animate the math
- how to tell which system has how many and what type  of solutions? and what is a solution exactly?
- sensing: hw1 p1 how is orientation not determinable?
- $a^T * b = b^T$ * a iff a, b are vectors
- what is determinant of transform matrix with only rotation and translation ? == 1
- composition of linear transformations
- rotation \& quaternions

# optimization and estimation 1
- data is a plural and a pronoun.
- accuracy vs precision
- estimation = estimate + uncertainity
- modelling uncertainity using stochasticity
- given a parametrized "stochastically noisy" model with stimulus and response (vector) variable and some samples, the process of estimating parameters (a.k.a fitting model). Using optimization for this estimation. Other non-optimization ways of doing the same. The essence of ML.

# optimization and estimation 2
- $(x_i - min) / (max - min)$ set of values vs normalizing set of values, show they are not equal in some instances and properties of each
- emphasize diff b/w
    - min(sum(abs(residual))) $\equiv$ min(Sum of Absolute Errors). The average of SAE is MAE
    - min(sum(residual * residual)) $\equiv$ min(Sum of Sqaured Errors). The average of SSE is MSE
    - min(sum(perpendicular distance)) $\equiv$ min(Sum of Perpendicular Errors). The average of SPE is MPE
- Thinking mean and variance visually, mean of a list of numbers plotted on a co-ordinate system is "the rough line passing through the center", take differences from the rough center line to actual values and square them. Each of these towers or inverted towers can be thought to represent "certain variation" from the mean. Average (/n-1) of all this is defined as "variance". Etymology of "variance". observed the sqaured fn to ignore signs and preserve differentiability. Mind blown!!!. And all this from visualizing the definitions.
- R-squared = Explained variation / Total variation. what it means (capturing the variation for just the samples in context and does not say anything about predictablility of model), and especially what it does not mean (good/bad model (patterns in residual plots)). Is it scale invariant?
- Shallow vs deep classifiers $\equiv$ seperate description and classification vs together description and classification.

# optimization and estimation 3
- does uncertainity change during fitting process?
- patterns in residual plots as a measure of good or bad model.
- Compare MLE vs MAP vs Linear Regression on.
    - fitting function
    - measure of accuracy, measure of uncertainity
    - covariance
- Determining uncertainity after fitting in linear regression (or other models). Using info about training datum (a.k.a. sample) error term?

# ironies
- There exist things which we don't know but can approximate (ex. integrals)

# long project lessons
- The best task management system until now is a white board and a marker
- The best information note taking system is taking no notes at all
- The best running notes are the ones that are forgotten shortly later
- The best information gathering system until now is this journal

# fuzzily separated tree searches
- Ones that have no state
- Ones that have a fixed size state (finding nearest vertex to a position)
- Ones that have a per-vertex state (path from start to current vertex)

# misc
- Bitwise operations are much faster than arithmetic operations. So use them if possible. For ex in bit manipulation n, n - 1 bit manipulation trick
- Linked list dummy initial node :P
- Systemetic case expansion
- Working with examples
- Arrays that index main arrays to do things :P
- Sometimes things to be done in second loop can be done directly in first

# languages 1
- Statically typed vs Dynamically typed == Compile time vs Run time type inferrence.
- Compiled vs Interpreted == Transforming code to target machine code before running vs No transformation before running. Compiled might be faster because
    - It is not reading the code and transforming it for every line.
    - It can do global optimizations which might not be possible in interpreted as its field of view is local.
- Statically vs Dynamically typedness is property of language. Compilation vs Interpration is the property of implementation not language; saying python is an interpreted language is wrong. Any language can be compiled. Any language can be interpreted.
- Define, map and visualize the terms: Memory safety, Memory containment and Type safety.

# languages 2
- C/C++/Rust
- Java/Kotlin/Go
- Python
- Shell

# ideas
-
    1. Structures and algorithms
    1. Code/Visualize/Animate the math
-
    1. Combining classical, ML, and DL {sensing, planning, RL}.
    1. Dynamic obstacles.
    1. Multi-robot coordination.
    1. Lidar object detection.
    1. Camera object detection.
    1. Lane detection using camera and lidar.
-
    1. Smart docs, create docs from code directly.
    1. Enhanced Odometry using an android phone.
    1. Online Multiplayer game
    1. Music Buttons
    1. reliable resilient gallery manager
    1. Haptic music
    1. FabX expressive ink cloth
    1. Holotalk
    1. Real roadrash
    1. Live slides

# outdoor list
1. Sleep cycle/Fitness/Summer breezes
1. Climbing
1. Driving
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
1. Surfing, Windsurfing, Kitesurfing
1. SUP
1. Snowboarding

# dp problems
-
    1. Largest Subarray Sum.
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
        1. Iterative Table filling O(mn).
        1. Recursive Table filling O(mn) can prune space.
        1. Recursive without table O(m^n).
-
    1. Count number of ones in bin representation.
    1. Number of unique paths.
    1. Uniform cost search with,
        1. All non-negative weights.
        1. Some negative weights too.

# school bag stuff

## zen_of_coding

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

## disk_cleanup

1. see in /var/cache/apt/archives/ and sudo apt-get clean
1. clear chrome cache
1. intellij product previous version .configs
1. android studio gradles

## things_that_matter_while_buying_a_new_laptop

build quality
	hinges, ports, sturdiness
keyboard experience
trackpad and dedicated mouse keys
enough number of required ports

## ideas

## coding-tips

### android

1. not necessary for ids to be different, but RECOMMENDED - first one in the tree gets called
1. SQLite is a lone wolf- self-contained, serverless, zero conf, out of the box

### js-events-listeners

1. JS event
1. JS listener(s) on DOM nodes
1. Javascript event propagation
    1. Event Capture Phase
    1. Event Target Phase
    1. Event Bubbling Phase
1. Propagation is done statically and stack decided at the point in time when event occurs
1. e.target vs e.currentTarget
1. e.stopPropogation() vs e.stopImmediatePropogation()
1. e.preventDefault()
1. e.eventPhase
1. e.bubbles

### arithmetic-operators

* In general, division operator is much slower than bitwise operators (& | ~), therefore in cases where possible use them

## DB

### ||computing

A function has to be commutative and associative so that it can be performed parallelly

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

## OS

### processes

1. a program in execution, an execution entity to consume CPU, an instance of a program
1. generally one core of a CPU executes one instruction at a time
1. running multiple processes or threads -> CPU switching or time slicing
1. fork -> duplicate (copy on write method)
1. exec -> replace current process
1. hence child before parent is good as memory need not be reproduced
