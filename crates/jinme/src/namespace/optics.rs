use crate::{
    namespace::{Aliases, Imports, Refers, Vars},
    optics::lens::{Lens, LensImpl, LensMut, LensMutImpl},
    prelude::*,
};

pub fn get_name(ns: &Namespace) -> SymbolUnqualified {
    ns.name.clone()
}
pub fn get_vars(ns: &Namespace) -> Vars {
    ns.vars.lock().unwrap().clone()
}
pub fn get_aliases(ns: &Namespace) -> Aliases {
    ns.aliases.lock().unwrap().clone()
}
pub fn get_imports(ns: &Namespace) -> Imports {
    ns.imports.lock().unwrap().clone()
}
pub fn get_refers(ns: &Namespace) -> Refers {
    ns.refers.lock().unwrap().clone()
}

pub fn set_name(ns: &Namespace, name: SymbolUnqualified) -> Namespace {
    Namespace::new(
        name,
        get_vars(ns),
        get_aliases(ns),
        get_imports(ns),
        get_refers(ns),
    )
}
pub fn set_vars(ns: &Namespace, vars: Vars) -> Namespace {
    Namespace::new(
        get_name(ns),
        vars,
        get_aliases(ns),
        get_imports(ns),
        get_refers(ns),
    )
}
pub fn set_aliases(ns: &Namespace, aliases: Aliases) -> Namespace {
    Namespace::new(
        get_name(ns),
        get_vars(ns),
        aliases,
        get_imports(ns),
        get_refers(ns),
    )
}
pub fn set_imports(ns: &Namespace, imports: Imports) -> Namespace {
    Namespace::new(
        get_name(ns),
        get_vars(ns),
        get_aliases(ns),
        imports,
        get_refers(ns),
    )
}
pub fn set_refers(ns: &Namespace, refers: Refers) -> Namespace {
    Namespace::new(
        get_name(ns),
        get_vars(ns),
        get_aliases(ns),
        get_imports(ns),
        refers,
    )
}

pub fn set_name_mut(ns: &mut Namespace, name: SymbolUnqualified) {
    ns.name = name;
}
pub fn set_vars_mut(ns: &mut Namespace, vars: Vars) {
    *ns.vars.lock().unwrap() = vars;
}
pub fn set_aliases_mut(ns: &mut Namespace, aliases: Aliases) {
    *ns.aliases.lock().unwrap() = aliases;
}
pub fn set_imports_mut(ns: &mut Namespace, imports: Imports) {
    *ns.imports.lock().unwrap() = imports;
}
pub fn set_refers_mut(ns: &mut Namespace, refers: Refers) {
    *ns.refers.lock().unwrap() = refers;
}

pub fn lens_name() -> impl Lens<Namespace, SymbolUnqualified> {
    LensImpl::new(get_name, set_name)
}
pub fn lens_vars() -> impl Lens<Namespace, Vars> {
    LensImpl::new(get_vars, set_vars)
}
pub fn lens_aliases() -> impl Lens<Namespace, Aliases> {
    LensImpl::new(get_aliases, set_aliases)
}
pub fn lens_imports() -> impl Lens<Namespace, Imports> {
    LensImpl::new(get_imports, set_imports)
}
pub fn lens_refers() -> impl Lens<Namespace, Refers> {
    LensImpl::new(get_refers, set_refers)
}

pub fn lens_name_mut() -> impl LensMut<Namespace, SymbolUnqualified> {
    LensMutImpl::new(get_name, set_name_mut)
}
pub fn lens_vars_mut() -> impl LensMut<Namespace, Vars> {
    LensMutImpl::new(get_vars, set_vars_mut)
}
pub fn lens_aliases_mut() -> impl LensMut<Namespace, Aliases> {
    LensMutImpl::new(get_aliases, set_aliases_mut)
}
pub fn lens_imports_mut() -> impl LensMut<Namespace, Imports> {
    LensMutImpl::new(get_imports, set_imports_mut)
}
pub fn lens_refers_mut() -> impl LensMut<Namespace, Refers> {
    LensMutImpl::new(get_refers, set_refers_mut)
}
