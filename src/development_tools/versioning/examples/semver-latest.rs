use std::error::Error;
use semver::{VersionReq, Version};

fn find_max_matching_version<'a, I>(version_req_str: &str, iterable: I) -> Result<Option<Version>, Box<dyn Error>>
where
    I: IntoIterator<Item = &'a str>,
{
    let vreq = VersionReq::parse(version_req_str)?;

    Ok(
        iterable
            .into_iter()
            .filter_map(|s| Version::parse(s).ok())
            .filter(|s| vreq.matches(s))
            .max(),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        find_max_matching_version("<= 1.0.0", vec!["0.9.0", "1.0.0", "1.0.1"])?,
        Some(Version::parse("1.0.0")?)
    );

    assert_eq!(
        find_max_matching_version(
            ">1.2.3-alpha.3",
            vec![
                "1.2.3-alpha.3",
                "1.2.3-alpha.4",
                "1.2.3-alpha.10",
                "1.2.3-beta.4",
                "3.4.5-alpha.9",
            ]
        )?,
        Some(Version::parse("1.2.3-beta.4")?)
    );

    Ok(())
}
