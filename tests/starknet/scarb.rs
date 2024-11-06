use anyhow::Error;
use scarb::{
    core::{Config, PackageId, PackageName, SourceId, TargetKind},
    ops::{self, CompileOpts, FeaturesOpts, FeaturesSelector},
};
use semver::Version;
use std::{fs::canonicalize, path::PathBuf};

#[allow(dead_code)]
pub async fn compile_blocking(toml: String) -> Result<(), Error> {
    tokio::task::spawn_blocking(move || -> Result<(), Error> { compile(toml) })
        .await?
}

#[allow(dead_code)]
pub fn compile(toml: String) -> Result<(), Error> {
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
    let config = Config::builder(toml_absolute.to_str().unwrap()).build()?;
    let ws = ops::read_workspace(config.manifest_path(), &config)?;

    scarb::ops::compile(packages, opts, &ws)
}
