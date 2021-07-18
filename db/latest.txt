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
1. Combining classical, ML, and DL {sensing, planning, RL}.
1. Dynamic obstacles.
1. Multi-robot coordination.

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

1. Smart docs, create docs from code directly.
2. Enhanced Odometry using an android phone.
3. Online Multiplayer game
4. Music Buttons
5. reliable resilient gallery manager

## coding-tips

### android

1. reduce boiler plate code - android annotations, butterknife, lombok
1. modular restapi calls - retrofit
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

## diary

### 5Sep17
did design homework at last! took me 5 hrs
sudharshan sir accepted our project graciously
talked to sherdeep, he explained about the new digital image processing project

### 26Aug17
fixed the websockets problem, dont use async and basic remote end points and remember to restart tomcat server, java is not python
learned about html canvas
did some db outlab
arrested development is nice
didnot do design hw

### 27Aug17
completed little twitter, learned few important things about jsp, servlets
read pipeling implementation of mips
thought there was a SAFE exam the next day, and woke up until 4:00
arrested development

### 28Aug17
implemented second word autocomplete feature in schoolbag
had only 3 hrs of sleep
found that rendering in db mutator controller is not good, as if page is refreshed and db may be unwantedly mutated
helped dinesh with db little twitter db outlab

### 6Sep17
went to design class after many days, learnt about second person view, told us about the how stubborn and immortal we should be at this age, nice class
arch viva and cribs session
booked tickets for dussehra

### 21Aug17
Implemented production level version of calender4j
SingleCycle arch of MIPS

### 22Aug17
Created my first servlet project using tomcat server
Preserved book state in schoolbag
Watched Dhee movie ... bits of course
Missed going to temple
Ahhhh have to do Script Writing assignment

### 23Aug17
bunked the design class
attempted OS quiz with no idea what was going on
enjoyed doing arch lab
improved escape-errands, released v2.0 in https://escape-errands.herokuapp.com and v1.0 in https://eser.herokuapp.com
aghh! have to do design assignment
completed scrubs season 8

### 29Aug17
rejection of my db ideas by sir
floods in hostel
stayed in anirudh room while he slept in PO

### 24Aug17
tweaked sublime text - found out some very useful keymaps, settings, and plugins
found out how to start tomcat server with postgresql in intellij idea
discussed db project ideas, Adometer, multiplayer game, grocery app
asked sherdeep about the next project's status

### 25Aug17
implemented first online multiplayer game, a two player range input changer! using web sockets, tomcat server
today is ganesh chaturthi, went to temple, nice heavy showers greeted me, bathed in rain after so many days
watched arrested development

### 30Aug17
did db inlab
slept in dinesh room
prepared for sony interview
saw that only one person per jaf is being selected, so became a little tensed

### 31Aug17
did the sony interview, it was nice
bought new matress, bucket, slippers and stuff
bought airwick instead of odonil, silly me

### 1Sep17
went to OS class, but it turned out to be a xv6 code exploration tute
os inlab, pages and page table stuff
got the sony internship
slept in 113

### 2Sep17
OS outlab
went to temple, nice evening

### 3Sep17
birthday, cake cutting, chocolate cake by arjun mama was very nice
did DB outlab the whole day
saw a family of monkeys, keeping each other warm in rain

### 4Sep17
sleeping with light and curtains for the first day, worked better than I expected

### 7Sep17
clarified page tables
revised OS labs, prepared for OS quiz and labquiz
went to subhmangal savdhan movie
bunked scriptwriting class

### 8Sep17
first onsite laptop repair, saw the actual problem with hinges, plate of hinge was broken
OS lab, big realization of virtual memory and page tables magic
saw WTFolks webseries

### 9-13Sep17
midsem week
wrote well but did some silly mistakes and learned some really cool things

### 15Sep17
studied for the language exam
decided to make freeflow a multiplayer game

### 14Sep17
went to lenovo service center
	struggled and proved that my system is in warranty
	learned warranties are a joke
	learned about warranties of lenovo and never to trust any funny talking and stupid
 		customer care people

my adp is until august 30 2018 and any part can be replaced once a year, so no replacement for my hinges from now on in warranty, but other parts have one chance

### 16Sep17
studied like a roadrunner for language
went to the temple

### 17-20Sep17
discusses with sherdeep about new project
reactified escape errands
continuous rains full wet climate
OS cribs
DB lab Apache spark
no design class Yahoooooo!
went to OS class 15 minutes late, nice
no SAFE quiz in arch but surprise one in DB, Arghh!
cool Arch lab

### 09Nov17
created 2nd logo of muffin using inkscape nice feel

### 20-26Sep17
dblab on Spark cool lab
reactified the goal glance of escape errands completely
played a little prince of persia
cool OS lab about synchronization locks and stuff
did not go to friday design class, must go from now onwards
arch pipeline control cool
language quiz cancelled due to problems again, although I was not prepared so okay
started image analysis partsavatar project

### 14Nov17
yeyy childrens day
got the idea of seeks in muffin

### 13Nov17
db endsem, studied on a very high level, fortunately paper was not too deep and wrote better than expected
implemented togglefollows profile and improved review design of muffin, have to do something better with the posts

### 27Nov17
cleaned my old pc, air blowed it
	new perception of mother board, ram stick (surprisingly only 2GB), hard disk and cd drive
	opened fan and took out processor (goosebumps)
	cleaned the processor thermal slug and the exhaust vents, one screw of the fan broken
	cleaned the power adapter
	switched it on (tense moments), the fan of processor and power adapter were working
	connected to tv as monitor but no signal, have to look into it

removed arch linux between windows C drive and hyperdrive Y and shifted hyperdrive backwards(leftwards)
	no change in disk usage at startup but should be definitely good for seek times between two drives when in windows
		also after some time it showed 0-1% disk usage
	about 216 free space for new OS
	somehow it marked swap space also as unallocated which made ubuntu startup slow
		it was due to /etc/fstab entry
		once the prev swap partition was marked as swap, boot up time was normal again (nice feel)
	also around 1GB space at start of hard disk was freed (probably be due to removal of arch linux)
		(probably) /dev/sda1 -> windows bootup
			   /dev/sda2 -> ubuntu bootup
	backed up /School/5 into hardisk and fee reciepts to Google drive

### 27Sep-04Oct17
OS lab on syncronization, got references online chill lab
	pthreads, condition variables, semaphores
went home on dussehra enjoyed very nicely

### 04-06Oct17
wednesday no lab, hurray
discussed with sherdeep about project next steps
wed- thurs
	Arggh! not in mood to do image analysis
did db android lab
	learnt a lot of things
	recycler view
	naming conventions
	new javaish things
	layout templates
	designed better network task template
	handled cookies using okhttp cookie jar - really cool
saw a mail from bhaskar sir, regarding question paper making for safe and volunteered for it

### 7Oct17
understood how to use escape errands better
improved escape errands, goal search, so very cool
understood and played with image segmentation in matlab - ahh eye candy when works

### 06-11Oct17
thought of doing many things but mainly did db android lab
	learnt a lot of things about android, saw android from a new perspective
found no lab on next tuesday also, so can go to home on saturday itself

### 26Oct17
had 3 exams, arch safe, lang moodle, db quiz
arch - safe app didn't work, lang - caught giving exam from room, db - wrote what i could
learnt few nice things about jsps
worked on memories of future, emma's opening

### 25Oct17
read muffin opening in class - really nice
implemented server response and ensuredsessionservlet classes for muffin - really nice
arch lab

### 17-24Oct17
diwali holidays - enjoyed
green diwali - no crackers
drew a big swan on diwali - nice
installed RR rom - really cool OS
worked on db schema
