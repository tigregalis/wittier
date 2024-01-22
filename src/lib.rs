use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    // io::Write,
    fs,
};

use clap::Parser;
use colored::{ColoredString, Colorize};
use convert_case::{Case, Casing};
use io_adapters::WriteExtension;
use rustdoc_types::{
    Crate, Function, GenericArg, GenericArgs, Id, Item, ItemEnum, ItemKind, ItemSummary, Type,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args;

pub fn main(_args: Args) {
    let file = fs::read_to_string("macroquad.json").unwrap();
    let krate: Crate = serde_json::from_str(&file)
        .expect("Not a valid rustdoc JSON export, or not the right version");

    // let functions = krate
    //     .index
    //     .values()
    //     .filter_map(|item| match &item.inner {
    //         ItemEnum::Function(function) => Some(function),
    //         _ => None,
    //     })
    //     .collect::<Vec<_>>();

    // let mut wit_buffer = String::new();
    // let mut rust_buffer = String::new();
    let mut stdout = std::io::stdout();
    let mut stdout = stdout.write_adapter();

    let ids_from_paths = krate.paths.keys().collect::<HashSet<_>>();
    let ids_from_index = krate.index.keys().collect::<HashSet<_>>();
    let difference = ids_from_paths
        .difference(&ids_from_index)
        .collect::<HashSet<_>>();
    let union_ = ids_from_paths.union(&ids_from_index).collect::<Vec<_>>();
    let intersection = ids_from_paths
        .intersection(&ids_from_index)
        .cloned()
        .collect::<HashSet<_>>();
    let only_in_paths = ids_from_paths
        .difference(&intersection)
        .collect::<HashSet<_>>();
    let only_in_index = ids_from_index
        .difference(&intersection)
        .collect::<HashSet<_>>();
    dbg!(ids_from_paths.len());
    dbg!(ids_from_index.len());
    dbg!(difference.len());
    dbg!(union_.len());
    dbg!(intersection.len());
    dbg!(only_in_paths.len());
    dbg!(only_in_index.len());

    let mut items = krate
        .index
        .iter()
        .filter(|(id, _)| intersection.contains(id))
        .map(|(id, item)| (id, &krate.paths[id], item))
        .collect::<Vec<_>>();
    items.sort_by_key(|item| item.1.path.join("::"));
    for (_id, item_summary, item) in items {
        match &item.inner {
            ItemEnum::Function(func) => {
                println!();
                handle_func_print(
                    // &mut rust_buffer,
                    &mut stdout,
                    "rust",
                    path_join_rust,
                    print_type_rust,
                    print_func_rust,
                    &krate,
                    item_summary,
                    func,
                );
                handle_func_print(
                    // &mut wit_buffer,
                    &mut stdout,
                    "wit",
                    path_join_wit,
                    print_type_wit,
                    print_func_wit,
                    &krate,
                    item_summary,
                    func,
                );

                // let (key, pj, pt, print) = ("rust", path_join_rust, print_type_rust, print_rust);
                // let (key, pj, pt, print) = ("wit", path_join_wit, print_type_wit, print_wit);

                // let path = pj(&item_summary.path);

                // let output = match func.decl.output {
                //     Some(ref typ) => format!(" -> {}", pt(&krate, typ)),
                //     None => "".to_string(),
                // };

                // let inputs = func
                //     .decl
                //     .inputs
                //     .iter()
                //     .map(|(name, typ)| format!("{name}: {typ}", typ = pt(&krate, typ)))
                //     .collect::<Vec<_>>()
                //     .join(", ");

                // print(key, &path, &inputs, &output);
            }
            ItemEnum::Primitive(_prim) => {
                continue;
            }
            ItemEnum::Module(_modl) => {
                continue;
            }
            ItemEnum::Struct(struct_) => {
                dbg!(struct_, item_summary);
                todo!("ItemEnum::Struct")
            }
            // ItemEnum::ExternCrate { name, rename } => todo!("ItemEnum::ExternCrate"),
            // ItemEnum::Import(_) => todo!("ItemEnum::Import"),
            // ItemEnum::Union(_) => todo!("ItemEnum::Union"),
            // ItemEnum::StructField(_) => todo!("ItemEnum::StructField"),
            // ItemEnum::Enum(_) => todo!("ItemEnum::Enum"),
            // ItemEnum::Variant(_) => todo!("ItemEnum::Variant"),
            // ItemEnum::Trait(_) => todo!("ItemEnum::Trait"),
            // ItemEnum::TraitAlias(_) => todo!("ItemEnum::TraitAlias"),
            // ItemEnum::Impl(_) => todo!("ItemEnum::Impl"),
            // ItemEnum::TypeAlias(_) => todo!("ItemEnum::TypeAlias"),
            // ItemEnum::OpaqueTy(_) => todo!("ItemEnum::OpaqueTy"),
            // ItemEnum::Constant(_) => todo!("ItemEnum::Constant"),
            // ItemEnum::Static(_) => todo!("ItemEnum::Static"),
            // ItemEnum::ForeignType => todo!("ItemEnum::ForeignType"),
            // ItemEnum::Macro(_) => todo!("ItemEnum::Macro"),
            // ItemEnum::ProcMacro(_) => todo!("ItemEnum::ProcMacro"),
            // ItemEnum::AssocConst { type_, default } => todo!("ItemEnum::AssocConst"),
            // ItemEnum::AssocType {
            //     generics,
            //     bounds,
            //     default,
            // } => todo!("ItemEnum::AssocType"),
            _ => {}
        };
    }

    // print!("{}", wit_buffer);
    // print!("{}", rust_buffer);
}

#[allow(clippy::too_many_arguments)]
fn handle_func_print<W: Write>(
    buffer: &mut W,
    key: &str,
    path_join: impl Fn(&[String]) -> String,
    print_type: impl Fn(&Crate, &Type) -> ColoredString,
    print_func: impl Fn(&mut W, &str, &str, &str, &str),
    krate: &Crate,
    item_summary: &ItemSummary,
    func: &Function,
) {
    let path = path_join(&item_summary.path);

    let output = match func.decl.output {
        Some(ref typ) => format!(" -> {}", print_type(krate, typ)),
        None => "".to_string(),
    };

    let inputs = func
        .decl
        .inputs
        .iter()
        .map(|(name, typ)| format!("{name}: {typ}", typ = print_type(krate, typ)))
        .collect::<Vec<_>>()
        .join(", ");

    print_func(buffer, key, &path, &inputs, &output);
}

fn print_func_rust<W: Write>(buffer: &mut W, key: &str, path: &str, inputs: &str, outputs: &str) {
    writeln!(buffer, "{key}: fn {path}({inputs}){outputs}").unwrap();
}

fn print_func_wit<W: Write>(buffer: &mut W, key: &str, path: &str, inputs: &str, outputs: &str) {
    writeln!(buffer, "{key}: {path}: func({inputs}){outputs}").unwrap();
}

fn path_join_rust(path: &[String]) -> String {
    path.join("::")
}

fn path_join_wit(path: &[String]) -> String {
    path.iter()
        .map(|x| x.to_case(Case::Kebab))
        .collect::<Vec<_>>()
        .join(":")
}

fn print_type_rust(krate: &Crate, typ: &Type) -> ColoredString {
    match typ {
        Type::ResolvedPath(path) => {
            let args = path
                .args
                .as_ref()
                .map(|args| match args.as_ref() {
                    GenericArgs::AngleBracketed { args, bindings } => {
                        if args.is_empty() {
                            return "".to_string().into();
                        }
                        let args = args
                            .iter()
                            .map(|arg| match arg {
                                GenericArg::Lifetime(lifetime) => lifetime.clone().into(),
                                GenericArg::Type(ty) => print_type_rust(krate, ty),
                                GenericArg::Const(constant) => {
                                    format!("const {}: TODO", constant.expr).on_black()
                                }
                                GenericArg::Infer => "_".to_string().into(),
                            })
                            .collect::<Vec<_>>();
                        let args = join_colored_str(&args, ", ");
                        format!("<{args}>").on_cyan()
                    }
                    GenericArgs::Parenthesized { inputs, output } => {
                        "<TODO Fn(A, B) -> C>".on_black()
                    }
                })
                .unwrap_or("".to_string().into());
            let name = match krate.paths.get(&path.id) {
                Some(item_summary) => item_summary.path.join("::").on_green(),
                None => {
                    println!(
                        "{}",
                        format!("--- not in crate.paths: {} ---", path.name).on_red()
                    );
                    path.name.on_red()
                }
            };
            format!("{name}{args}").into()
        }
        Type::Primitive(primitive) => primitive.clone().on_purple(),
        Type::BorrowedRef {
            lifetime,
            mutable,
            type_,
        } => format!(
            "&{}{}{}",
            lifetime.as_deref().unwrap_or_default(),
            if *mutable { "mut " } else { "" },
            print_type_rust(krate, type_),
        )
        .to_string()
        .into(),
        Type::Slice(type_) => format!("[{}]", print_type_rust(krate, type_)).on_yellow(),
        Type::Tuple(types) => {
            let types = types
                .iter()
                .map(|ty| print_type_rust(krate, ty))
                .collect::<Vec<_>>();
            // let mut s = String::new();
            // for (i, ty) in types.iter().enumerate() {
            //     if i == 0 {
            //         s = format!("{ty}");
            //     } else {
            //         s = format!("{s}, {ty}");
            //     }
            // }
            // format!("({s})").into()
            format!("({})", join_colored_str(&types, ", ")).into()
        }
        _ => format!("TODO<{typ:?}>").on_black(),
    }
}

fn push_colored_str(s: ColoredString, add: &str) -> ColoredString {
    format!("{s}{add}").into()
}

fn join_colored_str(items: &[ColoredString], delimiter: &str) -> ColoredString {
    let mut s = String::new();
    for (i, item) in items.iter().enumerate() {
        if i == 0 {
            s = format!("{item}");
        } else {
            s = format!("{s}{delimiter}{item}");
        }
    }
    s.into()
}

fn print_type_wit(krate: &Crate, typ: &Type) -> ColoredString {
    match typ {
        Type::ResolvedPath(path) => {
            // TODO: core::result::Result<T, E> maps to result<T, E>
            // TODO: core::option::Option<T, E> maps to option<T, E>
            // TODO: alloc::vec::Vec<T> maps to list<T>
            let args = path
                .args
                .as_ref()
                .map(|args| match args.as_ref() {
                    GenericArgs::AngleBracketed { args, bindings } => {
                        if args.is_empty() {
                            return "".to_string().into();
                        }
                        let args = args
                            .iter()
                            .map(|arg| match arg {
                                GenericArg::Lifetime(lifetime) => lifetime.clone().into(),
                                GenericArg::Type(ty) => print_type_wit(krate, ty),
                                GenericArg::Const(constant) => {
                                    format!("const {}: TODO", constant.expr).on_black()
                                }
                                GenericArg::Infer => "_".to_string().into(),
                            })
                            .collect::<Vec<_>>();
                        let args = join_colored_str(&args, ", ");
                        format!("<{args}>").on_cyan()
                    }
                    GenericArgs::Parenthesized { inputs, output } => {
                        "<UNSUPPORTED Fn(A, B) -> C>".on_black()
                    }
                })
                .unwrap_or("".to_string().into());
            let name = match krate.paths.get(&path.id) {
                Some(item_summary) => {
                    match item_summary
                        .path
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .as_slice()
                    {
                        ["core", "result", "Result"] => "result".on_yellow(),
                        ["core", "option", "Option"] => "option".on_yellow(),
                        ["alloc", "vec", "Vec"] => "list".on_yellow(),
                        ["alloc", "string", "String"] => "string".on_yellow(),
                        _ => path_join_wit(&item_summary.path).on_green(),
                    }
                    // if item_summary.path == ["alloc", "vec", "Vec"] {
                    //     "list".on_yellow()
                    // } else if item_summary.path == ["core", "result", "result"] {
                    //     "list".on_yellow()
                    // } else {
                    //     item_summary.path.join(":").on_green()
                    // }
                }
                None => {
                    println!(
                        "{}",
                        format!("--- not in crate.paths: {} ---", path.name).on_red()
                    );
                    path.name.on_red()
                }
            };
            format!("{name}{args}").into()
        }
        Type::Primitive(primitive) => map_rust_primitive_to_wit(primitive).on_purple(),
        Type::BorrowedRef {
            lifetime: _,
            mutable: _,
            type_,
        } => {
            if type_.as_ref() == &Type::Primitive("str".into()) {
                "string".on_purple()
            } else {
                format!("borrow<{}>", print_type_wit(krate, type_),)
                    .to_string()
                    .into()
            }
        }
        Type::Slice(type_) => format!("list<{}>", print_type_wit(krate, type_))
            .to_string()
            .on_yellow(),
        Type::Tuple(types) => {
            let types = types
                .iter()
                .map(|ty| print_type_wit(krate, ty))
                .collect::<Vec<_>>();
            // let mut s = String::new();
            // for (i, ty) in types.iter().enumerate() {
            //     if i == 0 {
            //         s = format!("{ty}");
            //     } else {
            //         s = format!("{s}, {ty}");
            //     }
            // }
            format!("tuple<{}>", join_colored_str(&types, ", ")).into()
        }
        _ => format!("TODO<{typ:?}>").on_black(),
    }
}

// "u8" => "u8",
// "u16" => "u16",
// "u32" => "u32",
// "u64"=> "u64",
// "i8" => "s8",
// "i16" => "s16",
// "i32" => "s32",
// "i64"=> "s64",
// "f32" => "float32",
// "f64" => "float64",
// "char" => "char",
// "bool" => "bool",
// "String" => "string",
// "str" => "string",
// tuple
// list
// option
// result
// handle
// id

fn map_rust_primitive_to_wit(ty: &str) -> &str {
    match ty {
        "u8" => "u8",
        "u16" => "u16",
        "u32" => "u32",
        "u64" => "u64",
        "i8" => "s8",
        "i16" => "s16",
        "i32" => "s32",
        "i64" => "s64",
        "f32" => "float32",
        "f64" => "float64",
        "char" => "char",
        "bool" => "bool",
        "String" => "string",
        "str" => "string",
        _ => ty,
    }
}
