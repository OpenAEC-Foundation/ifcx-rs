//! Binary cache for fast re-opening of IFCX documents.
//!
//! First open: parse JSON → build index → serialize to rkyv binary.
//! Second open: zero-copy mmap of binary cache → instant access.

// TODO: Fase 1 — implement rkyv binary cache
//
// Design:
//
// ```rust,ignore
// use rkyv::{Archive, Serialize, Deserialize};
// use memmap2::Mmap;
//
// /// Cached version of an IFCX document
// #[derive(Archive, Serialize, Deserialize)]
// pub struct CachedIfcx {
//     entries: Vec<CachedEntry>,
//     path_index: HashMap<String, usize>,
//     class_index: HashMap<String, Vec<usize>>,
//     spatial_index: Vec<BoundingBox>,  // for BVH queries
// }
//
// pub fn cache_path(ifcx_path: &Path) -> PathBuf {
//     ifcx_path.with_extension("ifcx.cache")
// }
//
// pub fn is_cache_valid(ifcx_path: &Path) -> bool {
//     // Compare file hash / modification time
// }
//
// pub fn open_cached(path: &Path) -> Result<ArchivedCachedIfcx> {
//     let file = File::open(cache_path(path))?;
//     let mmap = unsafe { Mmap::map(&file)? };
//     Ok(rkyv::check_archived_root::<CachedIfcx>(&mmap)?)
// }
// ```
