# ash-laevatain

A trimmed fork of [`ash`](https://github.com/ash-rs/ash) (master, `0.38.0+1.4.352`) that keeps
**only** the modern Vulkan paths. Same low-level bindings, same `unsafe` API surface — just with
the superseded parts of the API physically removed.

```toml
[dependencies]
ash-laevatain = "0.38"
```

```rust
use ash_laevatain as ash;      // optional: keep `ash::` spellings in your code
use ash_laevatain::vk;
```

## What is kept

- **Dynamic rendering** — `VK_KHR_dynamic_rendering`, `..._local_read`
- **Descriptor heap** — `VK_EXT_descriptor_heap`
- **Mesh shaders** — `VK_EXT_mesh_shader`
- **Device address** — `VK_KHR_buffer_device_address`
- **synchronization2** — `VK_KHR_synchronization2`
- **Ray tracing** — `VK_KHR_acceleration_structure`, `ray_tracing_pipeline`, `ray_query`, `..._position_fetch`
- **Shader objects** — `VK_EXT_shader_object`
- **Extended dynamic state 1–3**, **graphics pipeline library**, **pipeline creation cache control**
- **Platform WSI** — surface / swapchain / win32 / xcb / xlib / wayland / display / android / metal
- **Maintenance 1–8**, `copy_commands2`, `draw_indirect_count`, `host_image_copy`, `memory_budget`, `debug_utils`, `descriptor_indexing`, `multi_draw`

48 of the 692 registered extensions survive; the other 644 have their commands stripped.
2,634 Vulkan types drop to 959. `src/` goes from 6.46 MB to 3.26 MB.

## What is gone

`vkCreateRenderPass*`, framebuffers, `vkCreateDescriptorPool` / `vkCreateDescriptorSetLayout` /
`vkUpdateDescriptorSets` / `vkCmdBindDescriptorSets*`, the descriptor-buffer extension,
`vkQueueSubmit`, `vkCmdPipelineBarrier`, `vkCmdWriteTimestamp`, the old event commands,
and **all vendor extensions** (`VK_NV_`, `VK_AMD_`, `VK_ARM_`, …).

This is not "discouraged" — it cannot be reached. Code that renders through render passes or
descriptor sets will not compile.

## Caveats

- **Not API-compatible with `ash`.** Crates that depend on `ash` itself (`ash-window`, `ash-*`)
  will pull the real `ash` and their types will not interoperate with this one.
- `structextends` links that point at trimmed types are filtered out, so for the retained
  structures the set of `Extends<T>` impls is narrower than in the full spec.
- Vendor-named commands that were formally adopted by a kept EXT/KHR extension remain
  (e.g. `vkCmdWriteMarkerToMemoryAMD` via `VK_KHR_device_address_commands`,
  `vkGetTensorOpaqueCaptureDataARM` via `VK_EXT_descriptor_heap`).

## Attribution

Upstream `ash` is dual-licensed MIT OR Apache-2.0; both licenses are kept verbatim in this
repository (`LICENSE-MIT`, `LICENSE-APACHE`). This is an unofficial fork, not affiliated with
the `ash-rs` maintainers.
