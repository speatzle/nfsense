mod common;

use common::{ensure, sh, with_ctx};

#[test]
fn generator_succeeded() {
    with_ctx(|ctx| {
        let (_, out) = sh(
            ctx.agent()?,
            "systemctl show -p Result -p ExecMainStatus nfsense-generator.service",
        )?;
        ensure(
            out.contains("Result=success") && out.contains("ExecMainStatus=0"),
            "nfsense-generator.service did not succeed at boot",
            &out,
        )
    });
}

#[test]
fn var_is_persistent_ext4_and_usr_is_erofs() {
    with_ctx(|ctx| {
        let (_, out) = sh(
            ctx.agent()?,
            "findmnt -no FSTYPE /var; findmnt -no FSTYPE /usr",
        )?;
        let lines: Vec<&str> = out.lines().map(str::trim).collect();
        ensure(
            lines == ["ext4", "erofs"],
            "expected /var on ext4 and /usr on erofs",
            &out,
        )
    });
}

#[test]
fn f4_9_hostname_and_localtime_resolve() {
    with_ctx(|ctx| {
        let (rc, out) = sh(
            ctx.agent()?,
            "cat /etc/hostname && readlink -e /etc/localtime",
        )?;
        ensure(
            rc == 0 && !out.trim().is_empty(),
            "/etc/hostname or /etc/localtime dangles",
            &out,
        )
    });
}

#[test]
fn f4_8_kea_dhcp6_not_enabled() {
    with_ctx(|ctx| {
        let (_, out) = sh(
            ctx.agent()?,
            "systemctl is-enabled kea-dhcp6-server.service; systemctl is-failed kea-dhcp6-server.service",
        )?;
        ensure(
            !out.lines().next().unwrap_or("").starts_with("enabled") && !out.contains("\nfailed"),
            "kea-dhcp6-server is enabled or failed",
            &out,
        )
    });
}

// The modes git means; the checkout's group/other bits depend on the umask.
fn expected_extra_modes() -> Vec<(String, u32)> {
    use std::os::unix::fs::PermissionsExt;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../mkosi/mkosi.extra");
    let mut out = Vec::new();
    let mut stack = vec![root.join("usr")];
    while let Some(p) = stack.pop() {
        let Ok(meta) = std::fs::symlink_metadata(&p) else {
            continue;
        };
        if meta.file_type().is_symlink() {
            continue;
        }
        let rel = p
            .strip_prefix(&root)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if meta.is_dir() {
            out.push((format!("/{rel}"), 0o755));
            for e in std::fs::read_dir(&p).into_iter().flatten().flatten() {
                stack.push(e.path());
            }
        } else {
            let exec = meta.permissions().mode() & 0o100 != 0;
            out.push((format!("/{rel}"), if exec { 0o755 } else { 0o644 }));
        }
    }
    out.sort();
    out
}

// mkosi copies with the checkout's modes, so a builder with umask 077 would
// ship /usr/share 0700 and configs unreadable for unbound and kea.
#[test]
fn shipped_files_have_the_intended_modes() {
    with_ctx(|ctx| {
        let qga = ctx.agent()?;
        let expected = expected_extra_modes();
        ensure(
            expected.len() > 20,
            "found too few files in mkosi.extra (wrong path?)",
            &format!("{expected:?}"),
        )?;
        let paths: Vec<String> = expected.iter().map(|(p, _)| format!("'{p}'")).collect();
        // Paths missing from the image are not this test's concern (the
        // checkout may differ from the image under test).
        let (_, out) = sh(
            qga,
            &format!(
                "for p in {}; do [ -e \"$p\" ] && stat -c '%a %n' -- \"$p\"; done; true",
                paths.join(" ")
            ),
        )?;
        let actual: std::collections::BTreeMap<&str, &str> = out
            .lines()
            .filter_map(|l| l.split_once(' ').map(|(m, p)| (p, m)))
            .collect();
        let wrong: Vec<String> = expected
            .iter()
            .filter(|(p, _)| actual.contains_key(p.as_str()))
            .filter(|(p, mode)| actual.get(p.as_str()) != Some(&format!("{mode:o}").as_str()))
            .map(|(p, mode)| format!("{p}: want {mode:o}, got {:?}", actual.get(p.as_str())))
            .collect();
        ensure(
            wrong.is_empty(),
            "shipped files do not have the intended modes",
            &wrong.join("\n"),
        )?;
        let (_, out) = sh(
            qga,
            "find /usr/share/nfsense /usr/sbin/nfsense /usr/sbin/nfctl -xdev \\( -type d ! -perm -o+rx \\) -o \\( -type f ! -perm -o+r \\) 2>&1 | head -20",
        )?;
        ensure(
            out.trim().is_empty(),
            "build output is not world-readable",
            &out,
        )
    });
}
