// Integration test for SigmaOS-emacs-typescript
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_emacs_typescript as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
