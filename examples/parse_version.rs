extern crate parse_version;

use parse_version::{Version, VersionPart};

fn main() {
    let version = Version::parse("1.2.3");

    println!("Version: {version}");

    assert_eq!(version.major(), &VersionPart::Num(1));
    assert_eq!(version.minor(), &VersionPart::Num(2));
    assert_eq!(version.patch(), &VersionPart::Num(3));
}
