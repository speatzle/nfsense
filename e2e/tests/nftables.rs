mod common;

use common::with_ctx;
use nfsense_e2e::checks;

#[test]
fn nftables_table() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        checks::nftables_table(qga)
    });
}

#[test]
fn nftables_rules() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        checks::nftables_rules(qga)
    });
}
