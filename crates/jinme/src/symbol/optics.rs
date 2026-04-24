use crate::symbol::{Symbol, SymbolQualified, SymbolUnqualified};

pub fn preview_unqualified(symbol: Symbol) -> Option<SymbolUnqualified> {
    match symbol {
        Symbol::Unqualified(symbol) => Some(symbol),
        _ => None,
    }
}

pub fn preview_unqualified_ref(symbol: &Symbol) -> Option<&SymbolUnqualified> {
    match symbol {
        Symbol::Unqualified(symbol) => Some(symbol),
        _ => None,
    }
}

pub fn preview_qualified(symbol: Symbol) -> Option<SymbolQualified> {
    match symbol {
        Symbol::Qualified(symbol) => Some(symbol),
        _ => None,
    }
}

pub fn preview_qualified_ref(symbol: &Symbol) -> Option<&SymbolQualified> {
    match symbol {
        Symbol::Qualified(symbol) => Some(symbol),
        _ => None,
    }
}
