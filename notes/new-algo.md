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
subpar fit for the task. But benchmarking should reveal that once the
thing is implemented.
