//! Config group: the pristine image ships a sane default config.

mod common;

use common::with_ctx;
use nfsense_e2e::checks;

#[test]
fn default_config() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        checks::default_config(qga)
    });
}
