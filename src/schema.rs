table! {
    links (id) {
        id -> Int4,
        title -> Varchar,
        descripton -> Nullable<Varchar>,
    }
}

table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        stock -> Float8,
        price -> Nullable<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    links,
    products,
);
