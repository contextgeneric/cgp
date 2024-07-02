use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Generics, WhereClause, WherePredicate};

pub fn merge_generics(generics_a: &Generics, generics_b: &Generics) -> Generics {
    let mut params = generics_a.params.clone();
    params.extend(generics_b.params.clone());

    let mut predicates: Punctuated<WherePredicate, Comma> = Default::default();

    if let Some(where_clause) = &generics_a.where_clause {
        predicates.extend(where_clause.predicates.clone());
    }

    if let Some(where_clause) = &generics_b.where_clause {
        predicates.extend(where_clause.predicates.clone());
    }

    let where_clause = if predicates.is_empty() {
        None
    } else {
        Some(WhereClause {
            where_token: Default::default(),
            predicates,
        })
    };

    Generics {
        lt_token: generics_a.lt_token,
        params,
        gt_token: generics_a.gt_token,
        where_clause,
    }
}
