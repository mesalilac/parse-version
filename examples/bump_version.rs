extern crate parse_version;

use parse_version::{Version, VersionPart};

fn main() {
    let mut version = Version::parse("1.2.3");

    println!("Version old: {version}");

    version.bump_major();

    println!("Version new: {version}");

    assert_eq!(version.major(), &VersionPart::Num(2));
    assert_eq!(version.minor(), &VersionPart::Num(0));
    assert_eq!(version.patch(), &VersionPart::Num(0));
}
