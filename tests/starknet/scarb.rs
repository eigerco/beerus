use anyhow::{anyhow, Error};
use scarb::{
    core::{Config, PackageId, PackageName, SourceId, TargetKind},
    ops::{self, CompileOpts, FeaturesOpts, FeaturesSelector},
};
use semver::Version;
use std::{fs::canonicalize, path::PathBuf};

#[allow(dead_code)]
pub struct Compiler {
    toml: PathBuf,
    opts: CompileOpts,
    packages: Vec<PackageId>,
}

#[allow(dead_code)]
impl Compiler {
    pub fn new(toml: &str) -> Result<Self, Error> {
        let toml_absolute = canonicalize(PathBuf::from(toml))?;
        let opts = CompileOpts {
            include_target_kinds: vec![],
            exclude_target_kinds: vec![TargetKind::new("test")],
            include_target_names: vec![],
            features: FeaturesOpts {
                features: FeaturesSelector::Features(vec![]),
                no_default_features: false,
            },
        };
        let packages = vec![PackageId::new(
            PackageName::new("account"),
            Version::new(0, 1, 0),
            SourceId::for_path(toml_absolute.to_str().unwrap().into())?,
        )];

        Ok(Compiler { toml: toml_absolute, opts, packages })
    }

    pub async fn compile(self) -> Result<(), Error> {
        let compilation =
            tokio::task::spawn_blocking(move || -> Result<(), Error> {
                self.run_compilation()
            });
        match compilation.await {
            Ok(val) => Ok(val?),
            Err(e) => Err(anyhow!(
                "Error during thread execution. Original error message: {:#?}",
                e,
            )),
        }
    }

    fn run_compilation(self) -> Result<(), Error> {
        let config = Config::builder(self.toml.to_str().unwrap()).build()?;
        let ws = ops::read_workspace(config.manifest_path(), &config)?;
        scarb::ops::compile(self.packages, self.opts, &ws)
    }
}
