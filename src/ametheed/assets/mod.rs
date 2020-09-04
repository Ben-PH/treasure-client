use crate::ametheed::assets::reload::Reload;
use std::sync::Arc;
use crate::ametheed::error::Error;
use std::fmt::Debug;
use crate::ametheed::assets::dyn_format::FormatRegisteredData;
use specs::storage::UnprotectedStorage;

pub mod progress;
pub mod prefab;
pub mod storage;
pub mod loader;
pub mod reload;
pub mod system;
pub mod system_desc;
pub mod timing;
mod dyn_format;
use storage::Handle;

/// One of the three core traits of this crate.
///
/// You want to implement this for every type of asset like
///
/// * `Mesh`
/// * `Texture`
/// * `Terrain`
///
/// and so on. Now, an asset may be available in different formats.
/// That's why we have the `Data` associated type here. You can specify
/// an intermediate format here, like the vertex data for a mesh or the samples
/// for audio data.
///
/// This data is then generated by the `Format` trait.
pub trait Asset: Send + Sync + 'static {
    /// An identifier for this asset used for debugging.
    const NAME: &'static str;

    /// The `Data` type the asset can be created from.
    type Data: Send + Sync + 'static;

    /// The ECS storage type to be used. You'll want to use `DenseVecStorage` in most cases.
    type HandleStorage: UnprotectedStorage<Handle<Self>> + Send + Sync;
}

/// A format, providing a conversion from bytes to asset data, which is then
/// in turn accepted by `Asset::from_data`. Examples for formats are
/// `Png`, `Obj` and `Wave`.
///
/// The format type itself represents loading options, which are passed to `import`.
/// E.g. for textures this would be stuff like mipmap levels and
/// sampler info.
pub trait Format<D: 'static>: dyn_clone::DynClone + Debug + Send + Sync + 'static {
    /// A unique identifier for this format.
    fn name(&self) -> &'static str;

    /// Produces asset data from given bytes.
    /// This method is a simplified version of `format`.
    /// This format assumes that the asset name is the full path and the asset is only
    /// contained in one file.
    ///
    /// If you are implementing `format` yourself, this method will never be used
    /// and can be left unimplemented.
    ///
    fn import_simple(&self, _bytes: Vec<u8>) -> Result<D, Error> {
        unimplemented!("You must implement either `import_simple` or `import`.")
    }

    // /// Reads the given bytes and produces asset data.
    // ///
    // /// You can implement `import_simple` instead of simpler formats.
    // ///
    // /// ## Reload
    // ///
    // /// The reload structure has metadata which allows the asset management
    // /// to reload assets if necessary (for hot reloading).
    // /// You should only create `Reload` when `create_reload` is `Some`.
    // /// Also, the parameter is just a request, which means it's optional either way.
    // fn import(
    //     &self,
    //     name: String,
    //     source: Arc<dyn Source>,
    //     create_reload: Option<Box<dyn Format<D>>>,
    // ) -> Result<FormatValue<D>, Error> {
    //     #[cfg(feature = "profiler")]
    //     profile_scope!("import_asset");
    //     if let Some(boxed_format) = create_reload {
    //         let (b, m) = source
    //             .load_with_metadata(&name)
    //             .with_context(|_| crate::error::Error::Source)?;
    //         Ok(FormatValue {
    //             data: self.import_simple(b)?,
    //             reload: Some(Box::new(SingleFile::new(boxed_format, m, name, source))),
    //         })
    //     } else {
    //         let b = source
    //             .load(&name)
    //             .with_context(|_| crate::error::Error::Source)?;
    //         Ok(FormatValue::data(self.import_simple(b)?))
    //     }
    // }
}

impl<D: FormatRegisteredData + 'static> SerializableFormat<D> for Box<dyn SerializableFormat<D>> {}

/// The `Ok` return value of `Format::import` for a given asset type `A`.
pub struct FormatValue<D> {
    /// The format data.
    pub data: D,
    /// An optional reload structure
    pub reload: Option<Box<dyn Reload<D>>>,
}

impl<D> FormatValue<D> {
    /// Creates a `FormatValue` from only the data (setting `reload` to `None`).
    pub fn data(data: D) -> Self {
        FormatValue { data, reload: None }
    }
}

/// SerializableFormat is a marker trait which is required for Format types that are supposed
/// to be serialized. This trait implies both `Serialize` and `Deserialize` implementation.
///
/// **Note:** This trait should never be implemented manually.
/// Use the `register_format` macro to register it correctly.
/// See [FormatRegisteredData](trait.FormatRegisteredData.html) for the full example.
pub trait SerializableFormat<D: FormatRegisteredData + 'static>:
    Format<D> + erased_serde::Serialize + 'static
{
    // Empty.
}
