use anyhow::{anyhow, Result};
use std::process::Command;
use semver::{Version, VersionReq};

fn main() -> Result<()> {
    let version_constraint = "> 1.12.0";
    let version_test = VersionReq::parse(version_constraint)?;

    let output = Command::new("git").arg("--version").output()?;
    if !output.status.success() {
        return Err(anyhow!("Command executed with failing error code"));
    }
    let stdout = String::from_utf8(output.stdout)?;
    let version = stdout.split(" ").last().ok_or_else(|| {
        anyhow!("Invalid command output")
    })?.trim_end();

    let parsed_version = Version::parse(version)?;
    if !version_test.matches(&parsed_version) {
        return Err(anyhow!("Command version lower than minimum supported version (found {}, need {})",
            parsed_version, version_constraint));
    } else {
        println!("git version constraint '{version_constraint}' satisfied for: {version}");
    }

    Ok(())
}
