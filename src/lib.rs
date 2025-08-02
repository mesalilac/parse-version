use std::fmt;

#[derive(Debug, PartialEq)]
pub enum VersionPart {
    Num(usize),
    Wildcard,
}

impl fmt::Display for VersionPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersionPart::Num(n) => write!(f, "{n}"),
            VersionPart::Wildcard => write!(f, "*"),
        }
    }
}

#[derive(Debug)]
pub struct Version {
    major: VersionPart,
    minor: VersionPart,
    patch: VersionPart,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl Version {
    pub fn new() -> Self {
        Self {
            major: VersionPart::Wildcard,
            minor: VersionPart::Wildcard,
            patch: VersionPart::Wildcard,
        }
    }

    pub fn parse(s: &str) -> Self {
        let mut major = VersionPart::Wildcard;
        let mut minor = VersionPart::Wildcard;
        let mut patch = VersionPart::Wildcard;

        let parts = s.split('.').collect::<Vec<&str>>();

        for (index, part) in parts.iter().enumerate() {
            let Ok(num) = part.parse::<usize>() else {
                continue;
            };

            match index {
                0 => major = VersionPart::Num(num),
                1 => minor = VersionPart::Num(num),
                2 => patch = VersionPart::Num(num),
                _ => (),
            }
        }

        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn major(&self) -> &VersionPart {
        &self.major
    }

    pub fn set_major(&mut self, value: VersionPart) {
        self.major = value
    }

    pub fn minor(&self) -> &VersionPart {
        &self.minor
    }

    pub fn set_minor(&mut self, value: VersionPart) {
        self.minor = value
    }

    pub fn patch(&self) -> &VersionPart {
        &self.patch
    }

    pub fn set_patch(&mut self, value: VersionPart) {
        self.patch = value
    }

    pub fn bump_major(&mut self) {
        if let VersionPart::Num(major) = self.major {
            self.major = VersionPart::Num(major + 1);

            if let VersionPart::Num(_) = self.minor {
                self.minor = VersionPart::Num(0);
            }

            if let VersionPart::Num(_) = self.patch {
                self.patch = VersionPart::Num(0);
            }
        }
    }

    pub fn bump_minor(&mut self) {
        if let VersionPart::Num(minor) = self.minor {
            self.minor = VersionPart::Num(minor + 1);
        }

        if let VersionPart::Num(_) = self.patch {
            self.patch = VersionPart::Num(0);
        }
    }

    pub fn bump_patch(&mut self) {
        if let VersionPart::Num(patch) = self.patch {
            self.patch = VersionPart::Num(patch + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_version() {
        let version = Version::parse("");
        assert_eq!(version.major(), &VersionPart::Wildcard);
        assert_eq!(version.minor(), &VersionPart::Wildcard);
        assert_eq!(version.patch(), &VersionPart::Wildcard);
    }

    #[test]
    fn full_wildcard() {
        let version = Version::parse("*");
        assert_eq!(version.major(), &VersionPart::Wildcard);
        assert_eq!(version.minor(), &VersionPart::Wildcard);
        assert_eq!(version.patch(), &VersionPart::Wildcard);
    }

    #[test]
    fn full_version() {
        let version = Version::parse("1.2.3");
        assert_eq!(version.major(), &VersionPart::Num(1));
        assert_eq!(version.minor(), &VersionPart::Num(2));
        assert_eq!(version.patch(), &VersionPart::Num(3));
    }

    #[test]
    fn wildcard_major() {
        let version = Version::parse("*.2.3");
        assert_eq!(version.major(), &VersionPart::Wildcard);
        assert_eq!(version.minor(), &VersionPart::Num(2));
        assert_eq!(version.patch(), &VersionPart::Num(3));
    }

    #[test]
    fn wildcard_minor() {
        let version = Version::parse("1.*.3");
        assert_eq!(version.major(), &VersionPart::Num(1));
        assert_eq!(version.minor(), &VersionPart::Wildcard);
        assert_eq!(version.patch(), &VersionPart::Num(3));
    }

    #[test]
    fn wildcard_patch() {
        let version = Version::parse("1.2.*");
        assert_eq!(version.major(), &VersionPart::Num(1));
        assert_eq!(version.minor(), &VersionPart::Num(2));
        assert_eq!(version.patch(), &VersionPart::Wildcard);
    }

    #[test]
    fn wildcard_parts() {
        let version = Version::parse("*.*.*");
        assert_eq!(version.major(), &VersionPart::Wildcard);
        assert_eq!(version.minor(), &VersionPart::Wildcard);
        assert_eq!(version.patch(), &VersionPart::Wildcard);
    }

    #[test]
    fn bump_major() {
        let mut version = Version::parse("1.2.3");

        version.bump_major();

        assert_eq!(version.major(), &VersionPart::Num(2));
        assert_eq!(version.minor(), &VersionPart::Num(0));
        assert_eq!(version.patch(), &VersionPart::Num(0));
    }

    #[test]
    fn bump_minor() {
        let mut version = Version::parse("1.2.3");

        version.bump_minor();

        assert_eq!(version.major(), &VersionPart::Num(1));
        assert_eq!(version.minor(), &VersionPart::Num(3));
        assert_eq!(version.patch(), &VersionPart::Num(0));
    }

    #[test]
    fn bump_patch() {
        let mut version = Version::parse("1.2.3");

        version.bump_patch();

        assert_eq!(version.major(), &VersionPart::Num(1));
        assert_eq!(version.minor(), &VersionPart::Num(2));
        assert_eq!(version.patch(), &VersionPart::Num(4));
    }

    #[test]
    fn new_version() {
        let mut version = Version::new();

        version.set_major(VersionPart::Num(1));
        version.set_minor(VersionPart::Num(2));
        version.set_patch(VersionPart::Num(3));

        assert_eq!(version.major(), &VersionPart::Num(1));
        assert_eq!(version.minor(), &VersionPart::Num(2));
        assert_eq!(version.patch(), &VersionPart::Num(3));
    }

    #[test]
    fn set_version() {
        let mut version = Version::new();

        version.set_major(VersionPart::Num(2));
        version.set_minor(VersionPart::Num(2));
        version.set_patch(VersionPart::Num(3));

        assert_eq!(version.major(), &VersionPart::Num(2));
        assert_eq!(version.minor(), &VersionPart::Num(2));
        assert_eq!(version.patch(), &VersionPart::Num(3));
    }

    #[test]
    fn display_version() {
        let version = Version::parse("1.2.3");

        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn display_version_missing_minor() {
        let version = Version::parse("1");

        assert_eq!(version.to_string(), "1.*.*");
    }

    #[test]
    fn display_version_missing_patch() {
        let version = Version::parse("1.2");

        assert_eq!(version.to_string(), "1.2.*");
    }
}
