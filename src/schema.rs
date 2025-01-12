diesel::table! {
    use diesel::sql_types::*;

    source_documents (version_id) {
        version_id -> Text,
    }
}