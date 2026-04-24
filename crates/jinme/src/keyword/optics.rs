use crate::keyword::{Keyword, KeywordQualified, KeywordUnqualified};

pub fn preview_unqualified(keyword: Keyword) -> Option<KeywordUnqualified> {
    match keyword {
        Keyword::Unqualified(keyword) => Some(keyword),
        _ => None,
    }
}

pub fn view_unqualified_ref(keyword: &Keyword) -> Option<&KeywordUnqualified> {
    match keyword {
        Keyword::Unqualified(keyword) => Some(keyword),
        _ => None,
    }
}

pub fn preview_qualified(keyword: Keyword) -> Option<KeywordQualified> {
    match keyword {
        Keyword::Qualified(keyword) => Some(keyword),
        _ => None,
    }
}

pub fn view_qualified_ref(keyword: &Keyword) -> Option<&KeywordQualified> {
    match keyword {
        Keyword::Qualified(keyword) => Some(keyword),
        _ => None,
    }
}
