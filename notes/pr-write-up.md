## Quick update

This is an update on the last few weeks' worth of work. Time was spent
on a module resolution algorithm. This is necessary for correct item
paths. An example of this need is libc itself.

The current module tree shape has reexports at the crate root. This
means items should be "referrable" by that path (i.e. their identifiers
alone.) But the current strategy (in this patch) gets their source path.

This does not apply to upstream ctest. ctest currently assumes items are
reexported at the crate root. It only picks up on the item identifiers
during parsing. But module support needs import resolution.

Following a summary of related changes in this patchset is included.
This does not currently include all changes in this PR.

### Changes to FfiItems parsing

The FfiItems type needed to change its use of syn's visitor hooks. These
are not completely useful with the addition of modules. This also
uncovered what is believed to be a bug in upstream ctest.

The main concern here are non-module scope items. The syn visitor hooks
will visit every single matching item. This means items that only exist
within a function are also parsed.

Such an item can not be used outside of the function. Other non-module
scopes also have this limitation (e.g. module-level const blocks.)
Upstream ctest will thus attempt to generate tsets for these as well.

One could argue downstream users can set up skips for these. But that is
only a workaround. This also applies to use-statement scanning. A use
statement can exist within a function. But those should not be resolved.

Bottom line: The syn visitor hooks had to be removed. Though it would be
more accurate to say that they were repurposed. Those trait methods now
live as free functions.

The solution went through the module hooks. These are the only ones we
can safely use. A module may not be declared within a function scope.
And ctest requires only the items that appear within modules.

The current approach wraps the crate root in a module. This is because
the crate root is not parsed as a module. So the hook for modules is not
run in those cases. Then the actual parsing is done in the hook.

Each of the items ctest needs are "manually" parsed. This calls into the
previous hooks. These were repurposed into free functions. This is where
they are now used. This deserves further explanation.

The crate root is wrapped in a virtual module. This is to trigger the
module visitor hook. The virtual module needs to have a special
identifier. This is detected during parsing.

The crate root gets all parsed items added to it. Any other module gets
a new child module with all parsed items added to it. This builds up the
module tree hierarchy as one would expect.

But all of this needs to exist within a wrapper. Filling in an FfiItems
is no more about calling into syn's file hook. It needs to first wrap
the items in the file in a module of their own.

This is what eventually gets parsed. The item container is then
unwrapped from the virtual module. Note the module is a syn module type.
This needs another transformation for the resolution algorithm.

There is one last thing to note about parsing. Use-statements may come
as groups (paths wrapped in curly brackets.) This is not convenient to
handle afterward. These get "normalized" during parsing.

Normalization does one thing alone. It munches through a use-statement.
This may eventually stumble upon a group. That gets flattened into a set
of individual use-statements. A single use-statement then yields a list.

One note about terminology must be made here. Use-statements may have
arbitrary paths as their associated objects. They may also have a single
name. These are all referred to from here on as paths.

### The resolution algorithm

The algorithm for resolution will now be explained. This follows a
fairly straightforward process. There is a single entrypoint routine. It
takes a module and yields another module.

This is meant to be called from the afore mentioned wrapper. It is
called with another virtual module for the root. Recall the crate root
is initially only an item container.

The new virtual module uses ctest's own Module type. This is used to
keep track of the path to the module. This will be needed during
use-statement resolution.

Resolution starts by bottomming out on the list of child modules. This
is to ensure it takes place in a depth-first manner. Then it starts an
adaptive pass system over a single module.

The current approach requires resolution to be depth-first. This is
because of the current limitations the algorithm has. The current
resolution strategy can only resolve "forward" use-statements.

A "forward" use-statement is defined as a use-statement that brings into
scope items from a descendant module. Its dual brings into scope items
from some ascendant module.

These observations are made with respect to the module in which the
use-statement appears.

Note this does not exclude absolute use statements. A use-statement is
determined as one of the above based off of the target item. If the
target item is a descendant then that is a "forward" use-statement.

Current work is focused on support for bidirectional imports. The rest
of the explanation will focus on the initial revision of the algorithm.
This supports only use-statements that refer to descendant items.

This justifies the need for a depth-first approach. A leaf module in the
crate's module tree will have no resolvable use-statements. This is also
a good time to define "resolvable" and "unresolvable" use-statements.

A "resolvable" use-statement is said to be resolvable in some finite
number of resolution passes. Its dual is impossible to resolve in any
number of resolution passes.

One example of the latter are use-statements from third-party crates.
These will never resolve. The algorithm could attempt to inject the
crates into the module tree. But a simpler solution was found.

That shall be explained soon.

The pass system explanation will now be resumed. A single resolution
pass is given by two steps. The first one is the production of a list of
resolution items. The second one merges these into the module at hand.

The former resolves all use-statements within a given module. This
yields a list of resolution items. Each resolution item corresponds with
one use-statement. It can be "resolved" or "unresolved."

A use-statement is said to be "resolved" if the path it denotes has
yield a concrete item. Its dual refers to an item path that was not
found with respect to the module where the use-statement appears at.

Consider the following example.

use foo::*;

use bar::*;

use std::any::*;

mod bar {
    mod foo {
        pub Foo;
    }
}

Resolution of the first use-statement depends on the second. The first
use-statement will yiled an unresolved item in the first pass. Only upon
the second use-statement resolving will the former also resolve.

Though note both statements in the above example are said to be
resolvable. A definition for this is given further above. The third use
statement is said to be unresolvable.

A single pass maps the list of use-statements. It uses the use-statement
resolution routine as map transform. This routine munches through a
given use-statement's path.

Each path of the segment consists of an identifier. This routine carries
with it a module. This serves as state. This state is initially seeded
by the module where the use-statement appears at.

Each identifier in the path gets looked up in this stateful module. A
match means the routine may continue munching. A mismatch means the
routine must yield an unresolved item.

A munch that reaches the basecase denotes a resolved item. This is
produced much like an unresolved item. This bubbles up to the single
module resolution routine. That one mapped the list of use-statements.

This eventually yields a potentially non-empty list of resolution items.
These now need to be merged into the top-level module. This is the
module where the use-statements that sourced them lived at.

This now enters the second stage of a given resolution pass. Merging
iterates through the list of resolution items. Resolved items pack the
item the use-statement referred to. This deserves further explanation.

A use-statement can refer to one of a non-glob or a glob. Non-globs may
be further specified as non-modules or modules. A glob packs in the
resolved item an item container with all items from the parent module.

A non-glob packs in the resolved item an item container with itself only
as the contents. Notice how a non-module is small. A module carries with
it the subtree rooted at it in the crate's module tree.

A resolved glob has a very similar topology to a module. It also packs
the subtree rooted at it in the crate's module tree. But it contains no
information on the "subroot" where that subtree was sourced from.

Now back to the merging routine. Each resolved item also carries with it
the source use-statement. This is looked up in the module into which to
merge. Then this is removed from the module into which to merge.

The last step is to extend the module into which to merge. This is the
actual "merging." The packed item container is deconstructed into its
component items. These then extend the lists of items in the target.

That finishes a single resolution pass. The next thing it does is go
back to the top-level resolution routine. That now determines whether
another pass is in order.

This is determined by diffing the number of use-statements. The operands
are the pre-pass module and the post-pass module. Passes over a single
module terminate if this comparison yields no differences.

This is also what allows unresolvable use-statements not to break the
algorithm. These will not change once resolvable use-statements are
fully resolved. That is the sign that the pass system catches on.

This is then repeated depth-first across all modules. The root module is
processed last. This one has at that point the entire subtree resolved.
It may need more than one pass. But it will surely find the items.

### Extra notes

A few details were glossed over the above explanation. These are not
essential to understanding the algorithm. Chief among them is the path
manipulation routine and the rename case of use-statement resolution.

The latter will be explained first. A use-statement in Rust can have a
trailing renamed identifier. This identifier needs to also be reflected
in ctest's view of the crate's module tree.

An item's path is often straightforward to rename. Modification of the
last segment of a path is all it takes to rename a non-module. But a
module is also a subtree for other items and modules.

<!-- [TODO] Finish this up. -->

A module thus needs to recursively rename all items within the tree.
