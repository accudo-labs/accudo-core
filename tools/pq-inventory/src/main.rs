use anyhow::{Context, Result};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
};
use toml::Value;

#[derive(Copy, Clone)]
struct DependencyDescriptor {
    name: &'static str,
    category: &'static str,
}

const CRYPTO_DEPENDENCIES: &[DependencyDescriptor] = &[
    DependencyDescriptor {
        name: "accudo-crypto",
        category: "internal",
    },
    DependencyDescriptor {
        name: "aes-gcm",
        category: "symmetric",
    },
    DependencyDescriptor {
        name: "bls12_381",
        category: "signature",
    },
    DependencyDescriptor {
        name: "blst",
        category: "signature",
    },
    DependencyDescriptor {
        name: "blstrs",
        category: "signature",
    },
    DependencyDescriptor {
        name: "curve25519-dalek",
        category: "curve25519",
    },
    DependencyDescriptor {
        name: "ed25519",
        category: "signature",
    },
    DependencyDescriptor {
        name: "ed25519-dalek",
        category: "signature",
    },
    DependencyDescriptor {
        name: "hkdf",
        category: "kdf",
    },
    DependencyDescriptor {
        name: "libsecp256k1",
        category: "signature",
    },
    DependencyDescriptor {
        name: "ring",
        category: "misc",
    },
    DependencyDescriptor {
        name: "sha2",
        category: "hash",
    },
    DependencyDescriptor {
        name: "sha3",
        category: "hash",
    },
    DependencyDescriptor {
        name: "tiny-keccak",
        category: "hash",
    },
    DependencyDescriptor {
        name: "x25519-dalek",
        category: "key-exchange",
    },
];

fn main() -> Result<()> {
    let workspace_root = locate_workspace_root()?;
    let workspace_manifest = workspace_root.join("Cargo.toml");
    let members = load_workspace_members(&workspace_manifest)?;

    let interesting: BTreeMap<&str, DependencyDescriptor> = CRYPTO_DEPENDENCIES
        .iter()
        .map(|descriptor| (descriptor.name, *descriptor))
        .collect();

    let mut report: BTreeMap<String, InventoryEntry> = BTreeMap::new();

    for member in members {
        let manifest_path = workspace_root.join(&member).join("Cargo.toml");
        if !manifest_path.exists() {
            eprintln!(
                "warning: manifest missing for workspace member `{}` ({})",
                member,
                manifest_path.display()
            );
            continue;
        }

        let manifest = fs::read_to_string(&manifest_path)
            .with_context(|| format!("failed to read manifest at {}", manifest_path.display()))?;
        let parsed: Value = manifest
            .parse()
            .with_context(|| format!("failed to parse manifest {}", manifest_path.display()))?;

        let package_name = extract_package_name(&parsed).with_context(|| {
            format!(
                "missing `package.name` in manifest {}",
                manifest_path.display()
            )
        })?;

        let mut hits = BTreeSet::new();
        let mut categories = BTreeSet::new();

        collect_dependency_tables(&parsed, |dep_name, spec| {
            if let Some(descriptor) = match_dependency(dep_name, spec, &interesting) {
                hits.insert(descriptor.name.to_string());
                categories.insert(descriptor.category.to_string());
            }
        });

        if hits.is_empty() {
            continue;
        }

        report.insert(
            package_name,
            InventoryEntry {
                manifest_path,
                crates: hits.into_iter().collect(),
                categories: categories.into_iter().collect(),
            },
        );
    }

    print_report(&report);
    Ok(())
}

struct InventoryEntry {
    manifest_path: PathBuf,
    crates: Vec<String>,
    categories: Vec<String>,
}

fn locate_workspace_root() -> Result<PathBuf> {
    // Prefer CARGO_WORKSPACE_DIR when invoked via cargo, otherwise fallback to current dir.
    let dir = env::var("CARGO_WORKSPACE_DIR")
        .map(PathBuf::from)
        .or_else(|_| env::current_dir())
        .context("failed to determine workspace root")?;
    Ok(dir)
}

fn load_workspace_members(manifest: &Path) -> Result<Vec<String>> {
    let content = fs::read_to_string(manifest)
        .with_context(|| format!("failed to read workspace manifest {}", manifest.display()))?;
    let parsed: Value = content
        .parse()
        .with_context(|| format!("failed to parse workspace manifest {}", manifest.display()))?;

    let workspace = parsed
        .get("workspace")
        .and_then(Value::as_table)
        .context("workspace manifest lacks [workspace] table")?;

    let members = workspace
        .get("members")
        .and_then(Value::as_array)
        .context("[workspace] missing members array")?;

    let mut out = Vec::with_capacity(members.len());
    for entry in members {
        if let Some(member) = entry.as_str() {
            out.push(member.to_string());
        }
    }
    Ok(out)
}

fn extract_package_name(manifest: &Value) -> Option<String> {
    manifest
        .get("package")
        .and_then(Value::as_table)
        .and_then(|package| package.get("name"))
        .and_then(Value::as_str)
        .map(str::to_owned)
}

fn collect_dependency_tables<F>(manifest: &Value, mut visitor: F)
where
    F: FnMut(&str, &Value),
{
    for key in ["dependencies", "dev-dependencies", "build-dependencies"] {
        if let Some(table) = manifest.get(key).and_then(Value::as_table) {
            for (name, entry) in table {
                visitor(name, entry);
            }
        }
    }

    if let Some(target_table) = manifest.get("target").and_then(Value::as_table) {
        for target_entry in target_table.values() {
            if let Some(table) = target_entry.as_table() {
                for key in ["dependencies", "dev-dependencies", "build-dependencies"] {
                    if let Some(dep_table) = table.get(key).and_then(Value::as_table) {
                        for (name, entry) in dep_table {
                            visitor(name, entry);
                        }
                    }
                }
            }
        }
    }
}

fn match_dependency<'a>(
    dep_name: &str,
    spec: &Value,
    interesting: &'a BTreeMap<&str, DependencyDescriptor>,
) -> Option<DependencyDescriptor> {
    if let Some(descriptor) = interesting.get(dep_name) {
        return Some(*descriptor);
    }

    if let Some(table) = spec.as_table() {
        if let Some(package) = table.get("package").and_then(Value::as_str) {
            if let Some(descriptor) = interesting.get(package) {
                return Some(*descriptor);
            }
        }
    }

    None
}

fn print_report(report: &BTreeMap<String, InventoryEntry>) {
    println!("crate,categories,crypto_crates,manifest");
    for (name, entry) in report {
        println!(
            "{name},\"{}\",\"{}\",{}",
            entry.categories.join(" "),
            entry.crates.join(" "),
            entry.manifest_path.display()
        );
    }
}
