mod common;

use common::with_ctx;
use nfsense_e2e::checks;

#[test]
fn nfsense_installed() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        checks::nfsense_installed(qga)
    });
}

#[test]
fn nfsense_service() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        checks::nfsense_service(qga)
    });
}

#[test]
fn api_health() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        checks::api_health(qga)
    });
}
