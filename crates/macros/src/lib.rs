extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{self, Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Error, Ident, Token, UseTree};

use winmd::{dependencies, NamespaceTypes, TypeLimit, TypeLimits, TypeReader, TypeStage};

use std::{collections::BTreeSet, path::PathBuf};

/// A macro for generating WinRT modules into the current module.
///
/// This macro can be used to import WinRT APIs from OS dependencies as well
/// as NuGet packages. Use the `import` macro to directly include the generated code
/// into any module.
///
/// # Usage
/// To use, first specify which dependencies you are relying on. This can be both
/// `os` for depending on WinRT metadata shipped with Windows or `nuget: My.Package`
/// for NuGet packages.
///
/// ## NuGet
/// NuGet dependencies are expected in a well defined place. The `winmd` metadata files
/// should be in the cargo workspace's `target` directory in a subdirectory `nuget\My.Package`
/// where `My.Package` is the name of the NuGet package.
///
/// Any DLLs needed for the NuGet package to work should be next to work must be next to the final
/// executable.
///
/// Instead of handling this yourself, you can use the [`cargo winrt`](https://github.com/microsoft/winrt-rs/tree/master/crates/cargo-winrt)
/// helper subcommand.
///
/// ## Types
/// After specifying the dependencies, you must then specify which types you want to use. These
/// follow the same convention as Rust `use` paths. Types know which other types they depend on so
/// `import` will generate any other WinRT types needed for the specified type to work.
///
/// # Example
/// The following `import!` depends on both `os` metadata (i.e., metadata shipped on Windows 10), as well
/// as a 3rd-party NuGet dependency. It then generates all types inside of the `microsoft::ai::machine_learning`
/// namespace.
///
/// ```rust,ignore
/// import!(
///     dependencies
///         os
///         nuget: Microsoft.AI.MachineLearning
///     types
///         microsoft::ai::machine_learning::*
/// );
/// ```
#[proc_macro]
pub fn import(stream: TokenStream) -> TokenStream {
    to_tokens(stream)
}

/// A macro for generating WinRT modules to a .rs file at build time.
///
/// This macro can be used to import WinRT APIs from OS dependencies as well
/// as NuGet packages. It is only intended for use from a crate's build.rs script.
///
/// The macro generates a single `build` function which can be used in build scripts
/// to generate the WinRT bindings. After using the `build` macro, call the
/// generated `build` function somewhere in the build.rs script's main function.
///
/// # Usage
/// To use, first specify which dependencies you are relying on. This can be both
/// `os` for depending on WinRT metadata shipped with Windows or `nuget: My.Package`
/// for NuGet packages.
///
/// ## NuGet
/// NuGet dependencies are expected in a well defined place. The `winmd` metadata files
/// should be in the cargo workspace's `target` directory in a subdirectory `nuget\My.Package`
/// where `My.Package` is the name of the NuGet package.
///
/// Any DLLs needed for the NuGet package to work should be next to work must be next to the final
/// executable.
///
/// Instead of handling this yourself, you can use the [`cargo winrt`](https://github.com/microsoft/winrt-rs/tree/master/crates/cargo-winrt)
/// helper subcommand.
///
/// ## Types
/// After specifying the dependencies, you must then specify which types you want to use. These
/// follow the same convention as Rust `use` paths. Types know which other types they depend on so
/// `import` will generate any other WinRT types needed for the specified type to work.
///
/// # Example
/// The following `build!` depends on both `os` metadata (i.e., metadata shipped on Windows 10), as well
/// as a 3rd-party NuGet dependency. It then generates all types inside of the `microsoft::ai::machine_learning`
/// namespace.
///
/// ```rust,ignore
/// build!(
///     dependencies
///         os
///         nuget: Microsoft.AI.MachineLearning
///     types
///         microsoft::ai::machine_learning::*
/// );
/// ```
#[proc_macro]
pub fn build(stream: TokenStream) -> TokenStream {
    let tokens = to_tokens(stream);
    let tokens = tokens.to_string();

    let tokens = quote! {
        fn build() {
            use ::std::io::Write;
            let mut path = ::std::path::PathBuf::from(
                ::std::env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"),
            );

            path.push("winrt.rs");
            let mut file = ::std::fs::File::create(&path).expect("Failed to create winrt.rs");

            let mut cmd = ::std::process::Command::new("rustfmt");
            cmd.arg("--emit").arg("stdout");
            cmd.stdin(::std::process::Stdio::piped());
            cmd.stdout(::std::process::Stdio::piped());
            {
                let child = cmd.spawn().unwrap();
                let mut stdin = child.stdin.unwrap();
                let stdout = child.stdout.unwrap();

                let t = ::std::thread::spawn(move || {
                    let mut s = stdout;
                    ::std::io::copy(&mut s, &mut file).unwrap();
                });

                // Only rerun if the output file has changed
                println!("cargo:rerun-if-env-changed={}", path.display());

                writeln!(&mut stdin, "{}", #tokens).unwrap();
                // drop stdin to close that end of the pipe
                ::std::mem::drop(stdin);

                t.join().unwrap();
            }

            let status = cmd.status().unwrap();
            assert!(status.success(), "Could not successfully build");
        }
    };
    tokens.into()
}

fn to_tokens(stream: TokenStream) -> TokenStream {
    let import = parse_macro_input!(stream as ImportMacro);

    let dependencies = import
        .dependencies
        .0
        .into_iter()
        .map(|p| winmd::WinmdFile::new(p))
        .collect();
    let reader = &TypeReader::new(dependencies);

    let mut limits = TypeLimits::new(reader);

    for limit in import.types.0 {
        limits.insert(limit);
    }

    let stage = TypeStage::from_limits(reader, &limits);
    let tree = stage.into_tree();
    tree.to_tokens().into()
}

/// A parsed `import!` macro
#[derive(Debug)]
struct ImportMacro {
    dependencies: Dependencies,
    types: Types,
}

impl Parse for ImportMacro {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let _ = input.parse::<keywords::dependencies>()?;
        let dependencies: Dependencies = input.parse()?;
        let _ = input.parse::<keywords::types>()?;
        let types: Types = input.parse()?;

        Ok(ImportMacro {
            dependencies,
            types,
        })
    }
}

/// keywords used in the `import!` macro
mod keywords {
    syn::custom_keyword!(os);
    syn::custom_keyword!(nuget);
    syn::custom_keyword!(dependencies);
    syn::custom_keyword!(types);
}

/// A parsed `dependencies` section of the `import!` macro
#[derive(Debug)]
struct Dependencies(BTreeSet<PathBuf>);

impl Parse for Dependencies {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        enum Keyword {
            Os,
            Nuget,
        }
        let mut dependencies = BTreeSet::new();
        while let Some(keyword) = {
            if input.peek(keywords::os) {
                let _ = input.parse::<keywords::os>();
                Some(Keyword::Os)
            } else if input.peek(keywords::nuget) {
                let _ = input.parse::<keywords::nuget>();
                Some(Keyword::Nuget)
            } else {
                None
            }
        } {
            match keyword {
                Keyword::Os => {
                    let path = winmd::dependencies::system_metadata_root();

                    dependencies::expand_paths(path, &mut dependencies, false);
                }
                Keyword::Nuget => {
                    input.parse::<Token![:]>()?;

                    let package = Punctuated::<Ident, Token![.]>::parse_separated_nonempty(input)?;

                    let name = package
                        .iter()
                        .map(|ident| ident.to_string())
                        .collect::<Vec<String>>()
                        .join(".");
                    let mut path = winmd::dependencies::nuget_root();
                    path.push(name);

                    dependencies::expand_paths(path, &mut dependencies, true);
                }
            }
        }
        Ok(Dependencies(dependencies))
    }
}

/// A parsed `types` section of the `import!` macro
#[derive(Debug)]
struct Types(BTreeSet<NamespaceTypes>);

impl Parse for Types {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let mut limits = BTreeSet::<NamespaceTypes>::new();
        loop {
            if input.is_empty() {
                break;
            }

            let use_tree: syn::UseTree = input.parse()?;

            limits.insert(use_tree_to_namespace_types(use_tree)?);
        }
        Ok(Self(limits))
    }
}

// Snake <-> camel casing is lossy so we go for character but not case conversion
// and deal with casing once we have an index of namespaces to compare against.
fn namespace_literal_to_rough_namespace(namespace: &str) -> String {
    let mut result = String::with_capacity(namespace.len());
    for c in namespace.chars() {
        if c != '"' && c != '_' {
            result.extend(c.to_lowercase());
        }
    }
    result
}

fn use_tree_to_namespace_types(use_tree: syn::UseTree) -> parse::Result<NamespaceTypes> {
    fn recurse(tree: UseTree, current: &mut String) -> parse::Result<NamespaceTypes> {
        fn check_for_module_instead_of_type(name: &str, span: Span) -> parse::Result<()> {
            let error = Err(Error::new(
                span,
                "Expected `*` or type name, but found what appears to be a module",
            ));
            if name.to_lowercase() == name {
                return error;
            }
            Ok(())
        }

        match tree {
            UseTree::Path(p) => {
                if !current.is_empty() {
                    current.push('.');
                }

                current.push_str(&p.ident.to_string());

                recurse(*p.tree, current)
            }
            UseTree::Glob(_) => {
                let namespace = namespace_literal_to_rough_namespace(&current.clone());
                Ok(NamespaceTypes {
                    namespace,
                    limit: TypeLimit::All,
                })
            }
            UseTree::Group(g) => {
                let namespace = namespace_literal_to_rough_namespace(&current.clone());

                let mut types = Vec::with_capacity(g.items.len());
                for tree in g.items {
                    match &tree {
                        UseTree::Name(n) => {
                            let name = n.ident.to_string();
                            check_for_module_instead_of_type(&name, n.span())?;
                            types.push(name);
                        }
                        UseTree::Rename(_) => {
                            return Err(Error::new(tree.span(), "Renaming syntax is not supported"))
                        }
                        _ => return Err(Error::new(tree.span(), "Nested paths not allowed")),
                    }
                }
                Ok(NamespaceTypes {
                    namespace,
                    limit: TypeLimit::Some(types),
                })
            }
            UseTree::Name(n) => {
                let namespace = namespace_literal_to_rough_namespace(&current.clone());
                let name = n.ident.to_string();
                check_for_module_instead_of_type(&name, n.span())?;
                Ok(NamespaceTypes {
                    namespace,
                    limit: TypeLimit::Some(vec![name]),
                })
            }
            UseTree::Rename(r) => {
                return Err(Error::new(r.span(), "Renaming syntax is not supported"))
            }
        }
    }

    recurse(use_tree, &mut String::new())
}
