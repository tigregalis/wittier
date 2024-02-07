//! The lifetime `'c` is used to refer to the lifetime of the [`Crate`](rustdoc_types::Crate)

pub trait IdFetchExt<'c> {
    type Item;
    fn fetch(&'c self, krate: &'c Crate) -> Option<Self::Item>;
}

// Id
impl<'c> IdFetchExt<'c> for Id {
    type Item = (&'c Id, &'c rustdoc_types::Item);
    fn fetch(&'c self, krate: &'c Crate) -> Option<Self::Item> {
        krate.index.get(self).map(|item| (self, item))
    }
}

// Option<Id>
impl<'c> IdFetchExt<'c> for Option<Id> {
    type Item = Option<(&'c Id, &'c rustdoc_types::Item)>;
    fn fetch(&'c self, krate: &'c Crate) -> Option<Self::Item> {
        self.as_ref().map(|id| id.fetch(krate))
    }
}

pub trait ManyIdFetchExt<'c, I> {
    fn fetch_many(&'c self, krate: &'c Crate) -> IdIterWrapper<'c, I>;
}

impl<'c, I> ManyIdFetchExt<'c, I> for Vec<I> {
    fn fetch_many(&'c self, krate: &'c Crate) -> IdIterWrapper<'c, I> {
        IdIterWrapper {
            krate,
            iter: self.iter(),
        }
    }
}
pub struct IdIterWrapper<'c, I> {
    krate: &'c Crate,
    iter: core::slice::Iter<'c, I>,
}

// Vec<Id>
impl<'c> Iterator for IdIterWrapper<'c, Id> {
    type Item = (&'c Id, &'c rustdoc_types::Item);

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: this silently drops items not in the index (may be in the paths)
        self.iter.next().and_then(|id| id.fetch(self.krate))
    }
}

// Vec<Option<Id>>
impl<'c> Iterator for IdIterWrapper<'c, Option<Id>> {
    type Item = Option<(&'c Id, &'c rustdoc_types::Item)>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: this silently drops items not in the index (may be in the paths)
        self.iter.next().and_then(|id| id.fetch(self.krate))
    }
}

pub trait ItemEnumExt {
    fn as_module(&self) -> Option<&rustdoc_types::Module>;
    fn as_import(&self) -> Option<&rustdoc_types::Import>;
    fn as_union(&self) -> Option<&rustdoc_types::Union>;
    fn as_struct(&self) -> Option<&rustdoc_types::Struct>;
    fn as_struct_field(&self) -> Option<&rustdoc_types::Type>;
    fn as_enum(&self) -> Option<&rustdoc_types::Enum>;
    fn as_variant(&self) -> Option<&rustdoc_types::Variant>;
    fn as_function(&self) -> Option<&rustdoc_types::Function>;
    fn as_trait(&self) -> Option<&rustdoc_types::Trait>;
    fn as_trait_alias(&self) -> Option<&rustdoc_types::TraitAlias>;
    fn as_impl(&self) -> Option<&rustdoc_types::Impl>;
    fn as_type_alias(&self) -> Option<&rustdoc_types::TypeAlias>;
    fn as_opaque_ty(&self) -> Option<&rustdoc_types::OpaqueTy>;
    fn as_constant(&self) -> Option<&rustdoc_types::Constant>;
    fn as_static(&self) -> Option<&rustdoc_types::Static>;
    fn as_macro(&self) -> Option<Macro>;
    fn as_proc_macro(&self) -> Option<&rustdoc_types::ProcMacro>;
    fn as_primitive(&self) -> Option<&rustdoc_types::Primitive>;
}

impl ItemEnumExt for rustdoc_types::ItemEnum {
    fn as_module(&self) -> Option<&rustdoc_types::Module> {
        if let rustdoc_types::ItemEnum::Module(module) = self {
            Some(module)
        } else {
            None
        }
    }
    fn as_import(&self) -> Option<&rustdoc_types::Import> {
        if let rustdoc_types::ItemEnum::Import(import) = self {
            Some(import)
        } else {
            None
        }
    }

    fn as_union(&self) -> Option<&rustdoc_types::Union> {
        if let rustdoc_types::ItemEnum::Union(union_) = self {
            Some(union_)
        } else {
            None
        }
    }

    fn as_struct(&self) -> Option<&rustdoc_types::Struct> {
        if let rustdoc_types::ItemEnum::Struct(struct_) = self {
            Some(struct_)
        } else {
            None
        }
    }

    fn as_struct_field(&self) -> Option<&rustdoc_types::Type> {
        if let rustdoc_types::ItemEnum::StructField(struct_field) = self {
            Some(struct_field)
        } else {
            None
        }
    }

    fn as_enum(&self) -> Option<&rustdoc_types::Enum> {
        if let rustdoc_types::ItemEnum::Enum(enum_) = self {
            Some(enum_)
        } else {
            None
        }
    }

    fn as_variant(&self) -> Option<&rustdoc_types::Variant> {
        if let rustdoc_types::ItemEnum::Variant(variant) = self {
            Some(variant)
        } else {
            None
        }
    }

    fn as_function(&self) -> Option<&rustdoc_types::Function> {
        if let rustdoc_types::ItemEnum::Function(function) = self {
            Some(function)
        } else {
            None
        }
    }

    fn as_trait(&self) -> Option<&rustdoc_types::Trait> {
        if let rustdoc_types::ItemEnum::Trait(trait_) = self {
            Some(trait_)
        } else {
            None
        }
    }

    fn as_trait_alias(&self) -> Option<&rustdoc_types::TraitAlias> {
        if let rustdoc_types::ItemEnum::TraitAlias(trait_alias) = self {
            Some(trait_alias)
        } else {
            None
        }
    }

    fn as_impl(&self) -> Option<&rustdoc_types::Impl> {
        if let rustdoc_types::ItemEnum::Impl(impl_) = self {
            Some(impl_)
        } else {
            None
        }
    }

    fn as_type_alias(&self) -> Option<&rustdoc_types::TypeAlias> {
        if let rustdoc_types::ItemEnum::TypeAlias(type_alias) = self {
            Some(type_alias)
        } else {
            None
        }
    }

    fn as_opaque_ty(&self) -> Option<&rustdoc_types::OpaqueTy> {
        if let rustdoc_types::ItemEnum::OpaqueTy(opaque_ty) = self {
            Some(opaque_ty)
        } else {
            None
        }
    }

    fn as_constant(&self) -> Option<&rustdoc_types::Constant> {
        if let rustdoc_types::ItemEnum::Constant(constant) = self {
            Some(constant)
        } else {
            None
        }
    }

    fn as_static(&self) -> Option<&rustdoc_types::Static> {
        if let rustdoc_types::ItemEnum::Static(static_) = self {
            Some(static_)
        } else {
            None
        }
    }

    fn as_macro(&self) -> Option<Macro> {
        if let rustdoc_types::ItemEnum::Macro(macro_) = self {
            Some(Macro(macro_.as_str()))
        } else {
            None
        }
    }

    fn as_proc_macro(&self) -> Option<&rustdoc_types::ProcMacro> {
        if let rustdoc_types::ItemEnum::ProcMacro(proc_macro) = self {
            Some(proc_macro)
        } else {
            None
        }
    }

    fn as_primitive(&self) -> Option<&rustdoc_types::Primitive> {
        if let rustdoc_types::ItemEnum::Primitive(primitive) = self {
            Some(primitive)
        } else {
            None
        }
    }
}

// TODO:
//   Type*
//   Path*
//   Impl
//   Function
//   Trait
// pub enum ItemEnum {
//     -Module(Module),
//     ExternCrate {
//         name: String,
//         rename: Option<String>,
//     },
//     Import(Import),

//     Union(Union),
//     -Struct(Struct),
//     -StructField(Type),
//     -Enum(Enum),
//     -Variant(Variant),

//     Function(Function),

//     .Trait(Trait),
//     TraitAlias(TraitAlias),
//     Impl(Impl),

//     TypeAlias(TypeAlias),
//     OpaqueTy(OpaqueTy),
//     Constant(Constant),

//     Static(Static),

//     /// `type`s from an extern block
//     ForeignType,

//     /// Declarative macro_rules! macro
//     Macro(String),
//     ProcMacro(ProcMacro),

//     Primitive(Primitive),

//     AssocConst {
//         #[serde(rename = "type")]
//         type_: Type,
//         /// e.g. `const X: usize = 5;`
//         default: Option<String>,
//     },
//     AssocType {
//         generics: Generics,
//         bounds: Vec<GenericBound>,
//         /// e.g. `type X = usize;`
//         default: Option<Type>,
//     },
// }
//
//= Crate
//  root
//  index
//  paths
//  external_crates
//
// Item
//  links
//  visibility->Visibility
//            ->Visibility::Restricted.parent
// Module
//  items
// Union
//  fields
//  impls
// Struct
//  impls
//  kind->StructKind
//      ->StructKind::Tuple.0
//      ->StructKind::Plain.fields
// Enum
//  variants
//  impls
// Variant
//  kind->VariantKind
//      ->VariantKind::Tuple.0
//      ->VariantKind::Struct.fields
// ...manythings->GenericBound
// ?GenericBound::TraitBound.trait_->Path
// ...manythings->Type
// ?Type::ResolvedPath.0->Path
// ?Type::QualifiedPath.trait_->Path
// ?Type::DynTrait.0->DynTrait
//                  ->DynTrait.traits->PolyTrait
//                                   ->PolyTrait.trait_->Path
// ->Path.id
// Trait
//  items
//  implementations
// Impl
//  items
//  trait_->Path
// Import
//  id
// Primitive
//  impls
//
// take an ID, fetch an item, with a subset of possibilities as to what it is

use std::marker::PhantomData;

use rustdoc_types::{Crate, Id};

pub struct Macro<'c>(&'c str);

pub struct AssocConst<'c> {
    type_: &'c rustdoc_types::Type,
    /// e.g. `const X: usize = 5;`
    default: Option<&'c str>,
}

pub struct AssocType<'c> {
    generics: &'c rustdoc_types::Generics,
    bounds: Vec<&'c rustdoc_types::GenericBound>,
    /// e.g. `type X = usize;`
    default: Option<&'c rustdoc_types::Type>,
}

/// A query on a `Crate`
pub struct CrateQuery<'c> {
    krate: &'c Crate,
}

// TODO: handle paths/ItemSummary
// TODO: handle external_crates
impl<'c> CrateQuery<'c> {
    pub fn new(krate: &'c Crate) -> Self {
        Self { krate }
    }

    pub fn krate(&self) -> &Crate {
        self.krate
    }

    /// The root module (entry point)
    pub fn root(&self) -> Item<'c, &rustdoc_types::Module> {
        let (id, item) = self
            .krate
            .root
            .fetch(self.krate)
            .expect("crate root is not in index");
        item.inner
            .as_module()
            .map(|inner| Item::new(self.krate, id, item, None, inner))
            .expect("root is not a module")
    }

    // flat item accessor
    pub fn all_modules(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Module>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_module()
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    pub fn all_imports(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Import>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_import()
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    pub fn all_unions(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Union>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_union()
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    // flat item accessor
    pub fn all_structs(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Struct>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_struct()
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    // flat item accessor
    pub fn all_plain_structs(&self) -> impl Iterator<Item = Item<'c, Struct<StructPlain>>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_struct()
                .and_then(|struct_| match struct_.kind {
                    rustdoc_types::StructKind::Plain { .. } => Some(struct_),
                    _ => None,
                })
                .map(|inner| Item::new(self.krate, id, item, None, Struct::new(inner)))
        })
    }

    // flat item accessor
    pub fn all_unit_structs(&self) -> impl Iterator<Item = Item<'c, Struct<StructUnit>>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_struct()
                .and_then(|struct_| match struct_.kind {
                    rustdoc_types::StructKind::Unit => Some(struct_),
                    _ => None,
                })
                .map(|inner| Item::new(self.krate, id, item, None, Struct::new(inner)))
        })
    }

    // flat item accessor
    pub fn all_tuple_structs(&self) -> impl Iterator<Item = Item<'c, Struct<StructTuple>>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_struct()
                .and_then(|struct_| match struct_.kind {
                    rustdoc_types::StructKind::Tuple(..) => Some(struct_),
                    _ => None,
                })
                .map(|inner| Item::new(self.krate, id, item, None, Struct::new(inner)))
        })
    }

    // flat item accessor
    pub fn all_struct_fields(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Type>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_struct_field()
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    // flat item accessor
    pub fn all_enums(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Enum>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_enum()
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    // flat item accessor
    pub fn all_variants(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Variant>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_variant()
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    // flat item accessor
    pub fn all_functions(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Function>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_function()
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    // flat item accessor
    pub fn all_traits(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Trait>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_trait()
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    // flat item accessor
    pub fn all_trait_aliases(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::TraitAlias>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_trait_alias()
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    // flat item accessor
    pub fn all_impls(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Impl>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_impl()
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    // flat item accessor
    pub fn all_trait_impls(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Impl>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_impl()
                .filter(|impl_| impl_.trait_.is_some())
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    // flat item accessor
    pub fn all_inherent_impls(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Impl>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_impl()
                .filter(|impl_| impl_.trait_.is_none())
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    // flat item accessor
    pub fn all_macros(&self) -> impl Iterator<Item = Item<'c, Macro>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_macro()
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    // flat item accessor
    pub fn all_proc_macros(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::ProcMacro>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_proc_macro()
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }

    // flat item accessor
    pub fn all_primitives(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Primitive>> {
        self.krate.index.iter().filter_map(|(id, item)| {
            item.inner
                .as_primitive()
                .map(|inner| Item::new(self.krate, id, item, None, inner))
        })
    }
}

pub struct Item<'c, T /*, P*/> {
    krate: &'c Crate,
    pub id: &'c Id,
    pub item: &'c rustdoc_types::Item,
    // TODO: return the parent as an item instead, knowing its type
    // pub parent: Option<&'c Item<'c, P>>,
    pub parent: Option<&'c Id>,
    pub inner: T,
}

impl<'c, T> Item<'c, T> {
    pub fn new(
        krate: &'c Crate,
        id: &'c Id,
        item: &'c rustdoc_types::Item,
        parent: Option<&'c Id>,
        inner: T,
    ) -> Self {
        Self {
            krate,
            id,
            item,
            parent,
            inner,
        }
    }

    pub fn maybe_name(&self) -> Option<&str> {
        self.item.name.as_deref()
        // self.krate
        //     .index
        //     .get(self.id)
        //     .and_then(|item| item.name.as_deref())
    }

    pub fn external_crate(&self) -> &rustdoc_types::ExternalCrate {
        self.krate
            .external_crates
            .get(&self.item.crate_id)
            .expect("external crate is listed")
    }

    pub fn span(&self) -> Option<&rustdoc_types::Span> {
        self.item.span.as_ref()
    }

    pub fn docs(&self) -> Option<&str> {
        self.item.docs.as_deref()
    }

    pub fn attrs(&self) -> &[String] {
        self.item.attrs.as_slice()
    }

    pub fn links(&self) -> impl Iterator<Item = (&'c str, &'c Id, &'c rustdoc_types::Item)> {
        // TODO: this silently drops items not in the index (may be in the paths)
        self.item.links.iter().filter_map(|(name, id)| {
            id.fetch(self.krate)
                .map(|(id, item)| (name.as_str(), id, item))
        })
    }
}

impl<'c> Item<'c, &'c rustdoc_types::Module> {
    pub fn name(&self) -> &str {
        self.maybe_name().expect("module has a name")
    }

    pub fn items(&self) -> impl Iterator<Item = (&'c rustdoc_types::Id, &'c rustdoc_types::Item)> {
        self.inner.items.fetch_many(self.krate)
    }

    pub fn modules(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Module>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_module()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    pub fn imports(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Import>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_import()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    pub fn unions(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Union>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_union()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    pub fn structs(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Struct>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_struct()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    // flat item accessor
    pub fn plain_structs(&self) -> impl Iterator<Item = Item<'c, Struct<StructPlain>>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_struct()
                .and_then(|struct_| match struct_.kind {
                    rustdoc_types::StructKind::Plain { .. } => Some(struct_),
                    _ => None,
                })
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), Struct::new(inner)))
        })
    }

    // flat item accessor
    pub fn unit_structs(&self) -> impl Iterator<Item = Item<'c, Struct<StructUnit>>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_struct()
                .and_then(|struct_| match struct_.kind {
                    rustdoc_types::StructKind::Unit => Some(struct_),
                    _ => None,
                })
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), Struct::new(inner)))
        })
    }

    // flat item accessor
    pub fn tuple_structs(&self) -> impl Iterator<Item = Item<'c, Struct<StructTuple>>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_struct()
                .and_then(|struct_| match struct_.kind {
                    rustdoc_types::StructKind::Tuple(..) => Some(struct_),
                    _ => None,
                })
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), Struct::new(inner)))
        })
    }

    pub fn enums(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Enum>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_enum()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    // TODO: Can variants be exported from modules on their own?
    pub fn variants(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Variant>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_variant()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    pub fn functions(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Function>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_function()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    pub fn traits(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Trait>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_trait()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    pub fn trait_aliases(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::TraitAlias>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_trait_alias()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    // TODO: Can impls be exported from modules on their own
    pub fn impls(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Impl>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_impl()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    pub fn type_aliases(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::TypeAlias>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_type_alias()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    pub fn opaque_tys(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::OpaqueTy>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_opaque_ty()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    pub fn constants(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Constant>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_constant()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    pub fn statics(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Static>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_static()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    pub fn macros(&self) -> impl Iterator<Item = Item<'c, Macro>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_macro()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    pub fn proc_macros(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::ProcMacro>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_proc_macro()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }

    pub fn primitives(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Primitive>> {
        self.items().filter_map(|(id, item)| {
            item.inner
                .as_primitive()
                .map(|inner| Item::new(self.krate, id, item, Some(self.id), inner))
        })
    }
}

impl<'c> Item<'c, &'c rustdoc_types::Function> {
    pub fn name(&self) -> &str {
        self.maybe_name().expect("function has a name")
    }
}

impl<'c> Item<'c, &rustdoc_types::Import> {}

/// A plain struct, e.g. `struct Foo { a: i32 }`
pub struct StructPlain;

/// A unit struct, e.g. `struct Bar;`
pub struct StructUnit;

/// A tuple struct, e.g. `struct Baz(i32)`
pub struct StructTuple;

pub struct Struct<'c, K> {
    inner: &'c rustdoc_types::Struct,
    kind: PhantomData<K>,
}

impl<'c, K> Struct<'c, K> {
    pub fn new(inner: &'c rustdoc_types::Struct) -> Self {
        Self {
            inner,
            kind: PhantomData,
        }
    }
}

/// A trait impl, e.g. `impl Foo for Bar {}`
pub struct ImplTrait;

/// An inherent impl, e.g. `impl Bar {}`
pub struct ImplInherent;

pub struct Impl<'c, K> {
    inner: &'c rustdoc_types::Impl,
    kind: PhantomData<K>,
}

impl<'c, K> Impl<'c, K> {
    pub fn new(inner: &'c rustdoc_types::Impl) -> Self {
        Self {
            inner,
            kind: PhantomData,
        }
    }
}

impl<'c, K> Item<'c, Impl<'c, K>> {
    pub fn items(&self) -> impl Iterator<Item = (&'c rustdoc_types::Id, &'c rustdoc_types::Item)> {
        self.inner.inner.items.fetch_many(self.krate)
    }
}

/// A plain variant, e.g. `Color::Red`
pub struct VariantPlain;

/// A tuple variant, e.g. `Color::Rgb(255, 0, 0)`
pub struct VariantTuple;

/// A struct variant, e.g. `Color::Rgb { red: 255, green: 0, blue: 0 }`
pub struct VariantStruct;

/// The kind of an enum variant
pub enum VariantKind<'c> {
    /// A plain variant, e.g. `Color::Red`
    Plain(Item<'c, Variant<'c, VariantPlain>>),
    /// A tuple variant, e.g. `Color::Rgb(255, 0, 0)`
    Tuple(Item<'c, Variant<'c, VariantTuple>>),
    /// A struct variant, e.g. `Color::Rgb { red: 255, green: 0, blue: 0 }`
    Struct(Item<'c, Variant<'c, VariantStruct>>),
}

/// The variant of an enum
pub struct Variant<'c, K> {
    inner: &'c rustdoc_types::Variant,
    kind: PhantomData<K>,
}

impl<'c, K> Variant<'c, K> {
    pub fn new(inner: &'c rustdoc_types::Variant) -> Self {
        Self {
            inner,
            kind: PhantomData,
        }
    }
}

impl<'c> Item<'c, &rustdoc_types::Trait> {
    pub fn name(&self) -> &str {
        self.maybe_name().expect("trait has a name")
    }
}

impl<'c, K> Item<'c, Variant<'c, K>> {
    pub fn name(&self) -> &str {
        self.maybe_name().expect("variant has a name")
    }
}

impl<'c> Item<'c, Variant<'c, VariantPlain>> {
    // pub fn kind(&self) -> VariantKind {
    //     VariantKind::Plain
    // }
}

impl<'c> Item<'c, Variant<'c, VariantTuple>> {
    // pub fn kind(&self) -> VariantKind {
    //     VariantKind::Tuple
    // }
    pub fn fields(&self) -> impl Iterator<Item = Option<Item<'c, &rustdoc_types::Type>>> {
        match &self.inner.inner.kind {
            rustdoc_types::VariantKind::Tuple(fields) => {
                fields.fetch_many(self.krate).map(|pair| match pair {
                    Some((
                        id,
                        item @ rustdoc_types::Item {
                            inner: rustdoc_types::ItemEnum::StructField(field),
                            ..
                        },
                    )) => Some(Item::new(self.krate, id, item, Some(self.id), field)),
                    _ => None,
                })
            }
            _ => todo!(),
        }
    }
}

impl<'c> Item<'c, Variant<'c, VariantStruct>> {
    // pub fn kind(&self) -> VariantKind {
    //     VariantKind::Struct
    // }

    pub fn fields(&self) -> impl Iterator<Item = (&str, Item<'c, &rustdoc_types::Type>)> {
        match &self.inner.inner.kind {
            rustdoc_types::VariantKind::Struct { fields, .. } => fields
                .fetch_many(self.krate)
                .filter_map(|(id, item)| match &item.inner {
                    rustdoc_types::ItemEnum::StructField(field) => {
                        let name = item.name.as_deref().unwrap();
                        Some((
                            name,
                            Item::<&rustdoc_types::Type>::new(
                                self.krate,
                                id,
                                item,
                                Some(self.id),
                                field,
                            ),
                        ))
                    }
                    _ => None,
                }),
            _ => todo!(),
        }
    }

    pub fn fields_stripped(&self) -> bool {
        matches!(&self.inner.inner.kind, rustdoc_types::VariantKind::Struct { fields_stripped, .. } if *fields_stripped)
    }
}

impl<'c> Item<'c, &'c rustdoc_types::Variant> {
    pub fn name(&self) -> &str {
        self.maybe_name().expect("variant has a name")
    }

    pub fn kind(&self) -> VariantKind {
        match &self.inner.kind {
            rustdoc_types::VariantKind::Plain => VariantKind::Plain(self.as_plain_kind().unwrap()),
            rustdoc_types::VariantKind::Tuple(_) => {
                VariantKind::Tuple(self.as_tuple_kind().unwrap())
            }
            rustdoc_types::VariantKind::Struct { .. } => {
                VariantKind::Struct(self.as_struct_kind().unwrap())
            }
        }
    }

    pub fn as_plain_kind(&self) -> Option<Item<'c, Variant<'c, VariantPlain>>> {
        match &self.inner.kind {
            rustdoc_types::VariantKind::Plain => Some(Item::new(
                self.krate,
                self.id,
                self.item,
                self.parent,
                Variant::<VariantPlain>::new(self.inner),
            )),
            _ => None,
        }
    }

    pub fn as_tuple_kind(&self) -> Option<Item<'c, Variant<'c, VariantTuple>>> {
        match &self.inner.kind {
            rustdoc_types::VariantKind::Tuple(_) => Some(Item::new(
                self.krate,
                self.id,
                self.item,
                self.parent,
                Variant::<VariantTuple>::new(self.inner),
            )),
            _ => None,
        }
    }

    pub fn as_struct_kind(&self) -> Option<Item<'c, Variant<'c, VariantStruct>>> {
        match &self.inner.kind {
            rustdoc_types::VariantKind::Struct { .. } => Some(Item::new(
                self.krate,
                self.id,
                self.item,
                self.parent,
                Variant::<VariantStruct>::new(self.inner),
            )),
            _ => None,
        }
    }
}

impl<'c> Item<'c, &'c rustdoc_types::Enum> {
    pub fn name(&self) -> &str {
        self.maybe_name().expect("enum has a name")
    }

    pub fn inherent_impls(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Impl>> {
        self.inner
            .impls
            .fetch_many(self.krate)
            .filter_map(|(id, item)| match &item.inner {
                rustdoc_types::ItemEnum::Impl(impl_ @ rustdoc_types::Impl { trait_: None, .. }) => {
                    Some(Item::<&rustdoc_types::Impl>::new(
                        self.krate,
                        id,
                        item,
                        Some(self.id),
                        impl_,
                    ))
                }
                _ => None,
            })
    }

    pub fn trait_impls(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Impl>> {
        self.inner
            .impls
            .fetch_many(self.krate)
            .filter_map(|(id, item)| match &item.inner {
                rustdoc_types::ItemEnum::Impl(
                    impl_ @ rustdoc_types::Impl {
                        trait_: Some(_), ..
                    },
                ) => Some(Item::<&rustdoc_types::Impl>::new(
                    self.krate,
                    id,
                    item,
                    Some(self.id),
                    impl_,
                )),
                _ => None,
            })
    }

    pub fn variants(&self) -> impl Iterator<Item = Item<'c, &rustdoc_types::Variant>> {
        self.inner
            .variants
            .fetch_many(self.krate)
            .filter_map(|(id, item)| match &item.inner {
                rustdoc_types::ItemEnum::Variant(variant) => {
                    Some(Item::<&rustdoc_types::Variant>::new(
                        self.krate,
                        id,
                        item,
                        Some(self.id),
                        variant,
                    ))
                }
                _ => None,
            })
    }

    pub fn generics(&self) -> &rustdoc_types::Generics {
        &self.inner.generics
    }
}

impl<'c, K> Item<'c, Struct<'c, K>> {
    pub fn name(&self) -> &str {
        self.maybe_name().expect("struct has a name")
    }

    pub fn impls(&self) -> impl Iterator<Item = Item<&rustdoc_types::Impl>> {
        self.inner
            .inner
            .impls
            .fetch_many(self.krate)
            .filter_map(|(id, item)| match &item.inner {
                rustdoc_types::ItemEnum::Impl(impl_ @ rustdoc_types::Impl { trait_: None, .. }) => {
                    Some(Item::<&rustdoc_types::Impl>::new(
                        self.krate,
                        id,
                        item,
                        Some(self.id),
                        impl_,
                    ))
                }
                _ => None,
            })
    }

    pub fn generics(&self) -> &rustdoc_types::Generics {
        &self.inner.inner.generics
    }
}

impl<'c> Item<'c, Struct<'c, StructPlain>> {
    pub fn fields(&self) -> impl Iterator<Item = (&str, Item<'c, &rustdoc_types::Type>)> {
        match &self.inner.inner.kind {
            rustdoc_types::StructKind::Plain { fields, .. } => fields
                .fetch_many(self.krate)
                .filter_map(|(id, item)| match &item.inner {
                    rustdoc_types::ItemEnum::StructField(field) => {
                        let name = item.name.as_deref().unwrap();
                        Some((
                            name,
                            Item::<&rustdoc_types::Type>::new(
                                self.krate,
                                id,
                                item,
                                Some(self.id),
                                field,
                            ),
                        ))
                    }
                    _ => None,
                }),
            _ => todo!(),
        }
    }

    pub fn fields_stripped(&self) -> bool {
        matches!(&self.inner.inner.kind, rustdoc_types::StructKind::Plain { fields_stripped, .. } if *fields_stripped)
    }
}

impl<'c> Item<'c, Struct<'c, StructTuple>> {
    pub fn fields(&self) -> impl Iterator<Item = Option<Item<'c, &rustdoc_types::Type>>> {
        match &self.inner.inner.kind {
            rustdoc_types::StructKind::Tuple(fields) => {
                fields.fetch_many(self.krate).map(|pair| match pair {
                    Some((
                        id,
                        item @ rustdoc_types::Item {
                            inner: rustdoc_types::ItemEnum::StructField(field),
                            ..
                        },
                    )) => Some(Item::new(self.krate, id, item, Some(self.id), field)),
                    _ => None,
                })
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustdoc_types::Crate;

    fn format_type(krate: &Crate, typ: &rustdoc_types::Type) -> String {
        match typ {
            rustdoc_types::Type::ResolvedPath(path) => {
                let args = path
                    .args
                    .as_ref()
                    .map(|args| match args.as_ref() {
                        rustdoc_types::GenericArgs::AngleBracketed { args, bindings } => {
                            if args.is_empty() {
                                return "".to_string();
                            }
                            let args = args
                                .iter()
                                .map(|arg| match arg {
                                    rustdoc_types::GenericArg::Lifetime(lifetime) => {
                                        lifetime.clone()
                                    }
                                    rustdoc_types::GenericArg::Type(ty) => format_type(krate, ty),
                                    rustdoc_types::GenericArg::Const(constant) => {
                                        format!("const {}: TODO", constant.expr)
                                    }
                                    rustdoc_types::GenericArg::Infer => "_".to_string(),
                                })
                                .collect::<Vec<_>>();
                            let args = args.join(", ");
                            format!("<{args}>")
                        }
                        rustdoc_types::GenericArgs::Parenthesized { inputs, output } => {
                            "<TODO Fn(A, B) -> C>".to_string()
                        }
                    })
                    .unwrap_or("".to_string())
                    .to_string();
                let name = match krate.paths.get(&path.id) {
                    Some(item_summary) => item_summary.path.join("::"),
                    None => match krate.index.get(&path.id) {
                        Some(rustdoc_types::Item {
                            name: Some(name),
                            visibility: rustdoc_types::Visibility::Public,
                            ..
                        }) => {
                            let mut parts = path.name.split("::").collect::<Vec<_>>();
                            assert_eq!(parts.last().unwrap(), name);
                            if parts[0] == "crate" {
                                parts[0] = krate.index[&krate.root].name.as_ref().unwrap().as_str();
                            }
                            let parts = parts.iter().map(|s| s.to_string()).collect::<Vec<_>>();
                            join_path(&parts)
                        }
                        _ => {
                            println!("--- unhandled: {} ---", path.name);
                            path.name.clone()
                        }
                    },
                };
                format!("{name}{args}")
            }
            rustdoc_types::Type::Primitive(primitive) => primitive.clone(),
            rustdoc_types::Type::BorrowedRef {
                lifetime,
                mutable,
                type_,
            } => format!(
                "&{}{}{}",
                lifetime.as_deref().unwrap_or_default(),
                if *mutable { "mut " } else { "" },
                format_type(krate, type_),
            ),
            rustdoc_types::Type::Slice(type_) => {
                format!("[{}]", format_type(krate, type_))
            }
            rustdoc_types::Type::Tuple(types) => {
                let types = types
                    .iter()
                    .map(|ty| format_type(krate, ty))
                    .collect::<Vec<_>>();
                format!("({})", &types.join(", "))
            }
            rustdoc_types::Type::Generic(typ) if typ == "Self" => "Self".into(),
            _ => format!("TODO<{typ:?}>"),
        }
    }

    fn join_path(path: &[String]) -> String {
        path.join("::")
    }

    fn print_count(indent: &str, count: usize, singular: &str, plural: &str) {
        if count == 0 {
        } else if count == 1 {
            println!("{indent}    /* {count} {singular} */");
        } else {
            println!("{indent}    /* {count} {plural} */");
        }
    }

    fn print_module(module: &Item<'_, &rustdoc_types::Module>, depth: usize) {
        let indent = " ".repeat(depth * 4);

        println!(
            "{indent}mod {module} {{ // {items} item(s)",
            module = module.name(),
            items = module.items().count()
        );

        print_count(&indent, module.imports().count(), "import", "imports");
        for import in module.imports() {
            println!("{indent}    use {:?};", import.inner.id.fetch(import.krate));
        }

        print_count(
            &indent,
            module.unit_structs().count(),
            "unit struct",
            "unit structs",
        );
        for unit_struct in module.unit_structs() {
            println!("{indent}    struct {};", unit_struct.name());
        }

        print_count(
            &indent,
            module.plain_structs().count(),
            "plain struct",
            "plain structs",
        );
        for plain_struct in module.plain_structs() {
            println!("{indent}    struct {} {{", plain_struct.name());

            for (field_name, field_type) in plain_struct.fields() {
                println!(
                    "{indent}        {field_name}: {},",
                    format_type(module.krate, field_type.inner)
                );
            }
            if plain_struct.fields_stripped() {
                println!("{indent}        /* private fields */");
            }
            println!("{indent}    }}")
        }

        print_count(
            &indent,
            module.tuple_structs().count(),
            "tuple struct",
            "tuple structs",
        );
        for tuple_struct in module.tuple_structs() {
            print!("{indent}    struct {}(", tuple_struct.name());
            for (i, field) in tuple_struct.fields().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                match field {
                    Some(field) => {
                        print!("{}", format_type(module.krate, field.inner));
                    }
                    None => {
                        print!("/* private field */",);
                    }
                }
            }
            println!(");");
        }

        print_count(&indent, module.enums().count(), "enum", "enums");
        for enum_ in module.enums() {
            println!("{indent}    enum {} {{", enum_.name());
            for variant in enum_.variants() {
                match variant.kind() {
                    VariantKind::Plain(plain_variant) => {
                        println!("{indent}        {},", plain_variant.name());
                    }
                    VariantKind::Tuple(tuple_variant) => {
                        print!("{indent}        {}(", tuple_variant.name());
                        for (i, field) in tuple_variant.fields().enumerate() {
                            if i > 0 {
                                print!(", ");
                            }
                            match field {
                                Some(field) => {
                                    print!("{}", format_type(module.krate, field.inner));
                                }
                                None => {
                                    print!("/* private field */",);
                                }
                            }
                        }
                        println!("),");
                    }
                    VariantKind::Struct(struct_variant) => {
                        println!("{indent}        {} {{", struct_variant.name());
                        for (field_name, field_type) in struct_variant.fields() {
                            println!(
                                "{indent}            {field_name}: {},",
                                format_type(module.krate, field_type.inner)
                            )
                        }
                        if struct_variant.fields_stripped() {
                            println!("{indent}            /* private fields */",);
                        }
                        println!("{indent}        }},");
                    }
                }
            }
            println!("{indent}    }}")
        }

        print_count(&indent, module.modules().count(), "module", "modules");
        for module in module.modules() {
            print_module(&module, depth + 1);
        }
        println!("{indent}}}");
    }

    #[test]
    fn test() {
        let krate: Crate =
            serde_json::from_reader(std::fs::File::open("macroquad.json").unwrap()).unwrap();
        let query = CrateQuery::new(&krate);
        let root = query.root();

        println!("# crate {root}:", root = root.name());

        print_module(&root, 0);
    }
}
