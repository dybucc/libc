The new algorithm needs to support bidirectional use statements. Current
support is based around one invariant. This holds only for forward use
statements. The depth-first approach exploits the invariant. The problem
comes from the possibility for leaf modules to contain use statements.
These were previously also considered. But previously these were
guaranteed to be unresolvable import statements. These refer to use
statements that target items from third-party crates. But bidirectional
imports means there are also backward use statements. These may refer to
some item in an ancestor module. And these are resolvable use
statements. The one thing relying on this the most is the single-module
resolution proposition. It bottoms out on the list of child modules. It
resoves leaf modules first. Then it bubbles back up. Ancestor module
then get resolved with the guarantee of a fully resolved subtree. This
needs to change. But prior notes revealed order matters not. To start
resolution from the crate root does not improve things. The ideal
resolution process is fairly clear. One import gets resolved. If the
target gets some changes within it then the source of the import
observes those changes. This may seem like it calls for nifty pointer
tricks. But a pure solution is preferred. One must specify what it means
to "observe" the changes in the target. Currently target merging implies
use statement removal. That means the whole thing is settled once a
target is found. But it does not have to be this way. A target may be
merged without need for use statement removal. The pass system observes
the number of use statements before and after a pass. This should in
theory prevent potential infinite loops. But that only holds under one
condition. Single-module resolution currently never goes through the
same module again. To "observe" the changes in the use statement target
requires another pass over the use statement source module. And yet
preservation of the use-statement implies no changes to the list of use
statements. So the pass system concludes the remaining use statements
are unresolvable. It then moves on to another module. This implies the
overarching flow of resolution through the module tree has to be thought
anew. It is insufficient for a given number of passes over a module to
all happen consecutively. But keeping track of the tree nodes that
potentially require another pass is a hassle. It is best if the
top-level call to resolve the module tree from the crate root is
changed. It should loop over all modules in the tree instead of over a
single module. It should be made into a potentially non-terminating
loop. A check would be performed after one full module pass were
performed. This would be very similar to the one currently used in the
pass system. But it would instead recursively check the lists of use
statements of all modules in the tree. If they all compared the same
then it would be safe to say the algorithm may now terminate. The point
here would be to exploit deduplication. An abstract example may be in
order. Consider the algorithm is at the crate root. It finds a
dependency-free reexport. This means the reexport can be immediately
resolved. And it chooses to resolve the reexport. This means it will be
produced as a resolution item. This item will then be merged into the
module in which the use statement appeared at. But here's the difference
with the prior revision of the algorithm. The use statement is not
removed. It is instead kept as-is. The termination condition in the
single module resolution proposition is then removed. A single pass is
the only pass ever performed on a given module. It is within one global
module tree pass. The termination check is then moved to the top-level
resolution proposition. This is the one that originally called into the
former with the crate root. But now it has been made into an "infinite"
loop. The termination condition stops being the number of use statements
pre- and post-pass. It becomes a full comparison of the module tree
before and after the global resolution pass. This should reveal whether
some module has had its list of items modified. The trick here would be
to perform deduplication everytime a single-module pass is done. This
would ensure one can merge one use statement over and over again without
tricking this check into believing there have been changes. The details
of this are still fuzzy. Further specification is required. Consider the
prior case of the crate root again. The use statement is resolved.
Assume the crate root has only one child module. Assume as well this
child module is a leaf module. Suppose this child module contains a use
statement pointing to the crate root. The module resolves this use
statement by adding all items from the root module. That will merge anew
its own items. But deduplication will remove them during merging. The
module-wide pass system will then check for changes in the tree. It will
see there were changes. So it will trigger another pass. This will
repeat the same steps. But the use statement in the crate root will
already cause deduplication to remove both the child module and its own
items from the child module. The same thing will happen in the child
module. This seems trivial. But it may very well be the solution. More
thought has to be put into test cases. There are likely edge cases that
this does not consider. The current data structures are a potentially
subpar fit for the task. That should be the task of benchmarks. A recap
of the plan is in order. The idea is to perform module-wide passes.
These happen at the top-level resolution proposition. The termination
check should also be moved there. The check should not base itself off
of a diff of use statements. The check is over the entire tree. This
implies need for a copy of the module tree. That would have happened
anyway. That is because of the pure nature of the computations.
Discussion about other areas is in order. The single-module resolution
proposition should change. It should now be simpler. The pass system
does not exist anymore there. A single resolution should be made. Though
this should continue to be followed up by use of the merge proposition.
The other areas of the current revision can be left as-is. Path
manipulation will come in very handy. The current merging strategy is
key to the new algorithm. That should likely be left unchanged. A more
complex test case must be thought up. The test cases should change.
They've all had the same shape thus far. Consider the case of an
absolute use statement. Suppose it is found in some arbitrarily deep
module. Then suppose every other module has a reexport to it. The first
pass would import the module over to all other modules. This pass would
also resolve the absolute use statement. Some modules would already
observe this. These would be the descendants to the former. A second
pass would then be performed. There have been changes in the first pass.
That is why a second pass would be in order. The second pass would go
through the same use statements anew. This is where deduplication comes
in. The same use statements would be resolved anew. Deduplication would
remove duplicate items. The import to the denoted module would be kept.
These are key. An observation can be made about this algorithm. The
amount of work per module-wide pass is fixed in some ways. The number of
use statements considered is always the same. Dependent use statements
will not be resolved. They will once their dependency exists no more.
That is the one way in which work varies across passes. This can be
modeled in terms of a set of sentences. These sentences await
satisfiability. Satisfiability of some sentence depends on
satisfiability of some other sentence. Dependent sentences are defined
as such. At least one sentence must be dependency-free. This triggers
resolution of other sentences. There is an exception here. Some
sentences are unresolvable. And yet they appear as dependent. These
correspond with use statements from third-party crates. The algorithm
relies not on these. Unresolvable sentences are eventually skipped. The
algorithm has no notion of unresolvable sentences. It only ever
determines dependency of sentences. It does not determine which are the
dependencies. A dependent use statement can not be immediately resolved.
Some name in its path does not currently exist. At one point it may
exist. It is unknown whether that point is reachable. The diff
comparison determines if it may be reachable. An observable change in
the tree indicates further resolution is in order. No observable change
indicates no further resolution is possible. The diff thus determines if
some dependent sentence is satisfiable. It does not determine which
sentence is satisfiable. The algorithm always goes through all
sentences. It attempts to satisfy each sentence in turn. Some may yield
observable change. Some may not. Termination follows when there is no
observable change at all. Discussion is needed of the new machinery this
algorithm needs. Resolution of bidirectional use statements needs access
to the entire module tree. The stateful module in the
reexport-resolution proposition is not enough. There is need for the
entire module tree. This should reflect the most up-to-date snapshot.
That can be found after each single-module resolution. This was already
explored in the latest revision of the Idris proof. The depth-first
approach in the single-module resolution proposition must be kept. There
is now need for this to carry state. The state should consist of the
module tree snapshot. The seed should be the same as the module to
resolve. This means the initial module is the crate root. So should the
stateful module tree be. Then the single-module resolution proposition
can use this alongside the single-reexport resolution proposition. The
latter will need to add another piece of state. The state is now
composed of the parent module and the entire module tree. Note the
module tree state need not change across each reexport resolution. It
changes once the merging proposition is triggered. That will need
another auxiliary proposition. It should observe the changes in a given
subtree. Then it should effect those in a new crate module tree. The
details of the single-reexport resolution proof need discussion. This
may now support path keywords. These include: crate, super and self. A
review of the Rust reference may be in order. These are the only
keywords that must be considered. The syn use tree does not include
special support for any of these. That likely means they are to be used
as identifiers. The syn documentation on identifiers mentions that.
Keywords will be parsed as identifiers. A manually-constructed
identifier may not be a keyword. The logic is then simplified. The
basecase for names needs modification. It should check whether the
identifier refers to a keyword. This also affects the rename case. The
check will need to consult the syn documentation. Maybe there is already
a utility proposition for keywords. There is none. The string conversion
utility will be used instead. This should yield reasonable results. The
case is of a single identifier. There should be no need for
sanitization. This is a good opportunity for implementation. A change to
the above cases will not affect the prior revision of the algorithm. The
self keyword requires no module tree state. It may be readily
implemented.
