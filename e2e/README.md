# nfsense e2e tests

Boots the real mkosi image in QEMU/KVM (with swtpm as TPM 2.0) and tests it
through the QEMU guest agent. API calls run *inside* the guest via `curl` on
loopback, so there is no management NIC that a test could break.

## Requirements

`qemu-system-x86_64`, `/dev/kvm` (your user in the `kvm` group), OVMF
(`ovmf` on Debian) and `swtpm`. If any of these or the image is missing,
the tests print `e2e SKIP` and pass, so `cargo test --workspace` stays
green on runners without KVM.

The image needs `qemu-guest-agent` installed (it is in `mkosi.conf`).
Debian's udev rule starts it as soon as the guest-agent port exists.

## Running

```sh
./build-image.sh                                   # or any prod image
cargo test -p nfsense-e2e --no-fail-fast           # all groups
cargo test -p nfsense-e2e --test security          # one group
E2E_IMAGE_DIR=/path/to/image.qcow2 cargo test -p nfsense-e2e
cargo run -p nfsense-e2e                           # scenario driver
cargo run -p nfsense-e2e --bin nfsense-e2e-probe -- /bin/sh -c 'nft list ruleset'
```

By default the newest `.qcow2` in `mkosi/mkosi.output` is used.

The first run for an image cold-boots it (a few minutes), waits until the
boot has settled, and saves an internal snapshot `healthy` in
`e2e/.cache/<image-id>/state.qcow2`. Every test then starts from a fresh
QEMU process with that snapshot loaded (a few seconds). Delete
`e2e/.cache` to reclaim space; it holds about 1.5 GB per image.

## Groups

| File | What |
|---|---|
| `smoke.rs`, `config.rs`, `nftables.rs` | binary, service, API and default ruleset present |
| `apply.rs` | apply a rule; it survives a reboot and a power cut |
| `boot.rs` | image properties: generator, `/var` and `/usr`, hostname, kea6, file modes of what the image ships |
| `api.rs` | delete, failed apply keeps the API alive, discard, graceful stop |
| `security.rs` | service allowlist, WireGuard key and device name validation, IPv6 WireGuard endpoint |

Tests named `f<section>_<n>_…` guard a specific review finding.

## Not covered yet

- Secure Boot: the harness uses the non-Secure-Boot OVMF on the `pc`
  machine. A second mode with the `secboot` firmware, `q35,smm=on` and
  enrolled throwaway keys would cover signing and the signed PCR policy.
- Traffic through the box: the VM has three NICs (`lan1`/`eno16`, `lan2`/`eno17`, `wan`/`eno18`, QEMU user networking with `restrict=on`, unplug with `Ctx::set_link`), but no client machines behind them yet.
- A/B updates.
