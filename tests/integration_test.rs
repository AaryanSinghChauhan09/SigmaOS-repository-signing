// Integration test for SigmaOS-repository-signing
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use repository_signing as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
