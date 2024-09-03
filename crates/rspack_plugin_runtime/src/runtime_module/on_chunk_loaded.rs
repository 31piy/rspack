use rspack_collections::Identifier;
use rspack_core::{
  impl_runtime_module,
  rspack_sources::{BoxSource, RawSource, SourceExt},
  Compilation, RuntimeModule,
};

#[impl_runtime_module]
#[derive(Debug)]
pub struct OnChunkLoadedRuntimeModule {
  id: Identifier,
}

impl Default for OnChunkLoadedRuntimeModule {
  fn default() -> Self {
    Self::with_default(Identifier::from("webpack/runtime/on_chunk_loaded"))
  }
}

impl RuntimeModule for OnChunkLoadedRuntimeModule {
  fn name(&self) -> Identifier {
    self.id
  }

  fn template(&self) -> Vec<(String, String)> {
    vec![(
      self.id.to_string(),
      include_str!("runtime/on_chunk_loaded.ejs").to_string(),
    )]
  }

  fn generate(&self, compilation: &Compilation) -> rspack_error::Result<BoxSource> {
    let content = compilation
      .runtime_template
      .render(self.id.as_str(), None)?;
    Ok(RawSource::from(content).boxed())
  }
}
