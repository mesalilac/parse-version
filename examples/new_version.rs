extern crate parse_version;

use parse_version::{Version, VersionPart};

fn main() {
    let mut version = Version::new();

    version.set_major(VersionPart::Num(1));
    version.set_minor(VersionPart::Num(2));
    version.set_patch(VersionPart::Num(3));

    println!("Version: {version}");

    assert_eq!(version.major(), &VersionPart::Num(1));
    assert_eq!(version.minor(), &VersionPart::Num(2));
    assert_eq!(version.patch(), &VersionPart::Num(3));
}
