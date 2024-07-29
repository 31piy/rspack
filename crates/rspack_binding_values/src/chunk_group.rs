use napi_derive::napi;
use rspack_core::{ChunkGroup, ChunkGroupUkey, Compilation};

use crate::{JsChunk, JsCompilation};

#[napi(object)]
pub struct JsChunkGroup {
  #[napi(js_name = "__inner_ukey")]
  pub inner_ukey: u32,
  pub chunks: Vec<JsChunk>,
  pub index: Option<u32>,
  pub name: Option<String>,
  pub is_initial: bool,
}

impl JsChunkGroup {
  pub fn from_chunk_group(
    cg: &rspack_core::ChunkGroup,
    compilation: &rspack_core::Compilation,
  ) -> Self {
    Self {
      chunks: cg
        .chunks
        .iter()
        .map(|k| JsChunk::from(compilation.chunk_by_ukey.expect_get(k)))
        .collect(),
      index: cg.index,
      inner_ukey: cg.ukey.as_u32(),
      name: cg.name().map(|name| name.to_string()),
      is_initial: cg.is_initial(),
    }
  }
}

fn chunk_group(ukey: u32, compilation: &Compilation) -> &ChunkGroup {
  let ukey = ChunkGroupUkey::from(ukey);
  compilation.chunk_group_by_ukey.expect_get(&ukey)
}

#[napi(js_name = "__chunk_group_inner_get_chunk_group")]
pub fn get_chunk_group(ukey: u32, compilation: &JsCompilation) -> JsChunkGroup {
  let compilation = &compilation.0;
  let cg = chunk_group(ukey, compilation);
  JsChunkGroup::from_chunk_group(cg, compilation)
}

#[napi(js_name = "__chunk_group_inner_parents_iterable")]
pub fn parents_iterable(ukey: u32, compilation: &JsCompilation) -> Vec<JsChunkGroup> {
  let compilation = &compilation.0;
  let cg = chunk_group(ukey, compilation);
  cg.parents_iterable()
    .map(|k| {
      JsChunkGroup::from_chunk_group(compilation.chunk_group_by_ukey.expect_get(&k), compilation)
    })
    .collect()
}

#[napi(js_name = "__chunk_group_inner_children_iterable")]
pub fn children_iterable(ukey: u32, compilation: &JsCompilation) -> Vec<JsChunkGroup> {
  let compilation = &compilation.0;
  let cg = chunk_group(ukey, compilation);
  cg.children_iterable()
    .map(|k| {
      JsChunkGroup::from_chunk_group(compilation.chunk_group_by_ukey.expect_get(&k), compilation)
    })
    .collect()
}

#[napi(js_name = "__entrypoint_inner_get_runtime_chunk")]
pub fn get_runtime_chunk(ukey: u32, compilation: &JsCompilation) -> JsChunk {
  let compilation = &compilation.0;
  let entrypoint = chunk_group(ukey, compilation);
  let chunk_ukey = entrypoint.get_runtime_chunk(&compilation.chunk_group_by_ukey);
  let chunk = compilation.chunk_by_ukey.expect_get(&chunk_ukey);
  JsChunk::from(chunk)
}
