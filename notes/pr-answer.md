>This PR broke <https://github.com/rust-wii/ogc-rs> by removing PowerPC
>support, which has been in `libc` since 2020 when it was added and
>documented as being used for the Wii in
><https://github.com/rust-lang/libc/pull/1851>
>
>Why was that done in this PR and how was it concluded that PowerPC
>wasn't in use?

The reason is in the patch message. But "Rust target" may have been too
broad a term. Sorry for that. The change meant no in-tree Rust target in
upstream `rust-lang/rust`.

The PR merged after the new usage guidelines merged. These indicate in
[^1] the current support policy. That mentions `rust-lang/libc` support
depends on Rust support in `rust-lang/rust`.

libc takes the liberty to remove other kinds of targets. This change was
accepted in the PR. Whether libc also reserves the right to maintain
support is unknown. Though @tgross35 or @JohnTitor may think otherwise.

There's also another argument to be made for out-of-tree targets. The
new usage guidelines point out in [^2] the API breakage for in-tree
tiered targets. Almost anything goes for tier 3.

There's tier 3 targets where `core` fails to build. It would seem like
truly _anything_ goes for out-of-tree targets. Full breakage seems like
a feasible expectation.

[^1]: <https://github.com/rust-lang/libc/blob/b739c733e06a5f184bae7e296919c15d6ccd8ec8/CONTRIBUTING.md#supported-target-policy>
[^2]: <https://github.com/rust-lang/libc/blob/b739c733e06a5f184bae7e296919c15d6ccd8ec8/src/lib.rs#L184-L185>
