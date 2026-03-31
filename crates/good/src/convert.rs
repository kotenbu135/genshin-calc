pub(crate) fn build_imports(good: crate::types::GoodFormat) -> crate::GoodImport {
    crate::GoodImport {
        source: good.source,
        version: good.version,
        builds: vec![],
        warnings: vec![],
    }
}
