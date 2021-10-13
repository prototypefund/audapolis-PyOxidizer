// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Defines known Python distributions.

use {
    crate::py_packaging::distribution::{
        DistributionFlavor, PythonDistributionLocation, PythonDistributionRecord,
    },
    itertools::Itertools,
    once_cell::sync::Lazy,
};

pub struct PythonDistributionCollection {
    dists: Vec<PythonDistributionRecord>,
}

impl PythonDistributionCollection {
    /// Find a Python distribution given requirements.
    ///
    /// `target_triple` is the Rust machine triple the distribution is built for.
    /// `flavor` is the type of Python distribution.
    /// `python_major_minor_version` is an optional `X.Y` version string being
    /// requested. If `None`, `3.9` is assumed.
    pub fn find_distribution(
        &self,
        target_triple: &str,
        flavor: &DistributionFlavor,
        python_major_minor_version: Option<&str>,
    ) -> Option<PythonDistributionRecord> {
        let python_major_minor_version = python_major_minor_version.unwrap_or("3.9");

        self.dists
            .iter()
            .filter(|dist| dist.python_major_minor_version == python_major_minor_version)
            .filter(|dist| dist.target_triple == target_triple)
            .filter(|dist| match flavor {
                DistributionFlavor::Standalone => true,
                DistributionFlavor::StandaloneStatic => !dist.supports_prebuilt_extension_modules,
                DistributionFlavor::StandaloneDynamic => dist.supports_prebuilt_extension_modules,
            })
            .cloned()
            .next()
    }

    /// Obtain records for all registered distributions.
    #[allow(unused)]
    pub fn iter(&self) -> impl Iterator<Item = &PythonDistributionRecord> {
        self.dists.iter()
    }

    /// All target triples of distributions in this collection.
    #[allow(unused)]
    pub fn all_target_triples(&self) -> impl Iterator<Item = &str> {
        self.dists
            .iter()
            .map(|dist| dist.target_triple.as_str())
            .sorted()
            .dedup()
    }
}

pub static PYTHON_DISTRIBUTIONS: Lazy<PythonDistributionCollection> = Lazy::new(|| {
    let dists = vec![
        // Linux glibc linked.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.8.12-x86_64-unknown-linux-gnu-pgo-20211011T1926.tar.zst".to_string(),
                sha256: "2eacfec519467efd5b553758ab33160362865cacc709f3ec9e5ae5a89f40aa8d".to_string(),
            },
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.9.7-x86_64-unknown-linux-gnu-pgo-20211011T1926.tar.zst".to_string(),
                sha256: "3c59653724686e634e36a4441b04e9652349ec8ed3316275619bf28e426aec2c".to_string(),
            },
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },

        // Linux musl.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.8.12-x86_64-unknown-linux-musl-noopt-20211011T1926.tar.zst".to_string(),
                sha256: "37d4f2250965d584517f9b534aa37cab8a99864c70f8399353c9c02f958ff43b".to_string(),
            },
            target_triple: "x86_64-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.9.7-x86_64-unknown-linux-musl-noopt-20211011T1926.tar.zst".to_string(),
                sha256: "f188e8cc0fa68fcda22e4f5423ce51a53cd8791419023651553ec947dfb0185e".to_string(),
            },
            target_triple: "x86_64-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: false,
        },

        // The order here is important because we will choose the
        // first one. We prefer shared distributions on Windows because
        // they are more versatile: statically linked Windows distributions
        // don't declspec(dllexport) Python symbols and can't load shared
        // shared library Python extensions, making them a pain to work
        // with.

        // Windows shared.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.8.12-i686-pc-windows-msvc-shared-pgo-20211011T1926.tar.zst".to_string(),
                sha256: "45540517e36df0033057f3c0a4cef1947448ae42783bfa85fc2b4f0071c3b24f".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.9.7-i686-pc-windows-msvc-shared-pgo-20211011T1926.tar.zst".to_string(),
                sha256: "5d512a83cdfab847b45cc02b603b7d267d720c35416b57b574bb85f2edf43d77".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.8.12-x86_64-pc-windows-msvc-shared-pgo-20211011T1926.tar.zst".to_string(),
                sha256: "f33ce6a53c389e53d37fd21f0e923e255cf1d7e957cbd65229c09d14bdd2e443".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.9.7-x86_64-pc-windows-msvc-shared-pgo-20211011T1926.tar.zst".to_string(),
                sha256: "19ca3321853ee1882dd40e13a2cfc183414c6b0b8f8c453ef41c454688c9d682".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },

        // Windows static.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.8.12-i686-pc-windows-msvc-static-noopt-20211011T1926.tar.zst".to_string(),
                sha256: "ead7eec3d3a5d3a58a76d6382d8f397fbef971665858f289af52756bb190dc59".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.9.7-i686-pc-windows-msvc-static-noopt-20211011T1926.tar.zst".to_string(),
                sha256: "e7d2e97aa5e52817266a2bace944fe95ca03f396bd7317f205dc9f27ac6713b4".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.8.12-x86_64-pc-windows-msvc-static-noopt-20211011T1926.tar.zst".to_string(),
                sha256: "822420c7ae4ed9aec268f77588a69378d0fb8f8227a3ce6c8139f1398e5d064a".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.9.7-x86_64-pc-windows-msvc-static-noopt-20211011T1926.tar.zst".to_string(),
                sha256: "196b210061d4ea6ca138fc1d66ed76c0879156e29112eeb3d2cc106c27805a89".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },

        // macOS.
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.9.7-aarch64-apple-darwin-pgo-20211011T1926.tar.zst".to_string(),
                sha256: "dbf69f4fc42501a4d2a23c0c87a5f136b2a5beb37e0b544e214d4af8c0ec90fa".to_string(),
            },
            target_triple: "aarch64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.8.12-x86_64-apple-darwin-pgo-20211011T1926.tar.zst".to_string(),
                sha256: "1157308dbd6227d6b9463f556261e413c7bd3827471829fa4365e254b25520a9".to_string(),
            },
            target_triple: "x86_64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20211012/cpython-3.9.7-x86_64-apple-darwin-pgo-20211011T1926.tar.zst".to_string(),
                sha256: "1cabf3c7adf492bc194bc6fa3040943bf0ae2aa274ee7fa95b2908615c01b830".to_string(),
            },
            target_triple: "x86_64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
    ];

    PythonDistributionCollection { dists }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_target_triples() {
        assert_eq!(
            PYTHON_DISTRIBUTIONS
                .all_target_triples()
                .collect::<Vec<_>>(),
            vec![
                "aarch64-apple-darwin",
                "i686-pc-windows-msvc",
                "x86_64-apple-darwin",
                "x86_64-pc-windows-msvc",
                "x86_64-unknown-linux-gnu",
                "x86_64-unknown-linux-musl",
            ]
        );
    }
}
