use std::collections::HashMap;

struct FormatId(u64);

struct ConverterId(u64);

struct ConverterHash(u64); // should be an IPFS Multihash

struct Galaxy {
    formats: HashMap<FormatId, FileFormat>,
}

struct FileFormat {
    name: String,
    desc: String, // should be valid markdown
    converter: HashMap<ConverterId, Converter>,
}

struct Converter {
    name: String,
    desc: String, // should be valid markdown
    versions: Vec<(String, ConverterHash)>,  // version string (e.g. "1.0.0-alpha") and hash of the wasm module
}
