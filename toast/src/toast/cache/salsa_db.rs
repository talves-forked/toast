use crate::toast::breadbox::ImportMap;
use crate::toast::swc_ops;
use crate::toast::swc_ops::{compile_js_for_browser, compile_js_for_server};
use std::collections::HashMap;
use std::sync::Arc;

#[salsa::query_group(FilesStorage)]
pub trait Files: salsa::Database {
    #[salsa::input]
    fn source(&self, key: String) -> Arc<Vec<u8>>;

    fn js_for_browser(&self, key: String, npm_bin_dir: String, import_map: ImportMap) -> String;
    fn js_for_server(&self, key: String, npm_bin_dir: String) -> String;
}
fn js_for_browser(
    db: &dyn Files,
    key: String,
    npm_bin_dir: String,
    import_map: ImportMap,
) -> String {
    let source = &*db.source(key.to_string());
    let string = String::from_utf8(source.to_vec()).unwrap();
    return compile_js_for_browser(string, key, npm_bin_dir.clone(), import_map);
}
fn js_for_server(db: &dyn Files, key: String, npm_bin_dir: String) -> String {
    let source = &*db.source(key.to_string());
    let string = String::from_utf8(source.to_vec()).unwrap();
    return compile_js_for_server(string, key, npm_bin_dir.clone());
}

#[salsa::database(FilesStorage)]
#[derive(Default)]
pub struct SalsaToastDatabaseStruct {
    pub storage: salsa::Storage<Self>,
}

impl salsa::Database for SalsaToastDatabaseStruct {}
