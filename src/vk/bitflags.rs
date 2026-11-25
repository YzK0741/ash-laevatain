use super::definitions::*;
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheCreateFlagBits.html>"]
pub struct PipelineCacheCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineCacheCreateFlags, Flags);
impl PipelineCacheCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFlagBits.html>"]
pub struct QueueFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueueFlags, Flags);
impl QueueFlags {
    #[doc = "Queue supports graphics operations"]
    pub const GRAPHICS: Self = Self(0b1);
    #[doc = "Queue supports compute operations"]
    pub const COMPUTE: Self = Self(0b10);
    #[doc = "Queue supports transfer operations"]
    pub const TRANSFER: Self = Self(0b100);
    #[doc = "Queue supports sparse resource memory management operations"]
    pub const SPARSE_BINDING: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCullModeFlagBits.html>"]
pub struct CullModeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CullModeFlags, Flags);
impl CullModeFlags {
    pub const NONE: Self = Self(0);
    pub const FRONT: Self = Self(0b1);
    pub const BACK: Self = Self(0b10);
    pub const FRONT_AND_BACK: Self = Self(0x00000003);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreateFlagBits.html>"]
pub struct RenderPassCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(RenderPassCreateFlags, Flags);
impl RenderPassCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueCreateFlagBits.html>"]
pub struct DeviceQueueCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DeviceQueueCreateFlags, Flags);
impl DeviceQueueCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryPropertyFlagBits.html>"]
pub struct MemoryPropertyFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryPropertyFlags, Flags);
impl MemoryPropertyFlags {
    #[doc = "If otherwise stated, then allocate memory on device"]
    pub const DEVICE_LOCAL: Self = Self(0b1);
    #[doc = "Memory is mappable by host"]
    pub const HOST_VISIBLE: Self = Self(0b10);
    #[doc = "Memory will have i/o coherency. If not set, application may need to use vkFlushMappedMemoryRanges and vkInvalidateMappedMemoryRanges to flush/invalidate host cache"]
    pub const HOST_COHERENT: Self = Self(0b100);
    #[doc = "Memory will be cached by the host"]
    pub const HOST_CACHED: Self = Self(0b1000);
    #[doc = "Memory may be allocated by the driver when it is required"]
    pub const LAZILY_ALLOCATED: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryHeapFlagBits.html>"]
pub struct MemoryHeapFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryHeapFlags, Flags);
impl MemoryHeapFlags {
    #[doc = "If set, heap represents device memory"]
    pub const DEVICE_LOCAL: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits.html>"]
pub struct AccessFlags(pub(crate) Flags);
vk_bitflags_wrapped!(AccessFlags, Flags);
impl AccessFlags {
    #[doc = "Controls coherency of indirect command reads"]
    pub const INDIRECT_COMMAND_READ: Self = Self(0b1);
    #[doc = "Controls coherency of index reads"]
    pub const INDEX_READ: Self = Self(0b10);
    #[doc = "Controls coherency of vertex attribute reads"]
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(0b100);
    #[doc = "Controls coherency of uniform buffer reads"]
    pub const UNIFORM_READ: Self = Self(0b1000);
    #[doc = "Controls coherency of input attachment reads"]
    pub const INPUT_ATTACHMENT_READ: Self = Self(0b1_0000);
    #[doc = "Controls coherency of shader reads"]
    pub const SHADER_READ: Self = Self(0b10_0000);
    #[doc = "Controls coherency of shader writes"]
    pub const SHADER_WRITE: Self = Self(0b100_0000);
    #[doc = "Controls coherency of color attachment reads"]
    pub const COLOR_ATTACHMENT_READ: Self = Self(0b1000_0000);
    #[doc = "Controls coherency of color attachment writes"]
    pub const COLOR_ATTACHMENT_WRITE: Self = Self(0b1_0000_0000);
    #[doc = "Controls coherency of depth/stencil attachment reads"]
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(0b10_0000_0000);
    #[doc = "Controls coherency of depth/stencil attachment writes"]
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(0b100_0000_0000);
    #[doc = "Controls coherency of transfer reads"]
    pub const TRANSFER_READ: Self = Self(0b1000_0000_0000);
    #[doc = "Controls coherency of transfer writes"]
    pub const TRANSFER_WRITE: Self = Self(0b1_0000_0000_0000);
    #[doc = "Controls coherency of host reads"]
    pub const HOST_READ: Self = Self(0b10_0000_0000_0000);
    #[doc = "Controls coherency of host writes"]
    pub const HOST_WRITE: Self = Self(0b100_0000_0000_0000);
    #[doc = "Controls coherency of memory reads"]
    pub const MEMORY_READ: Self = Self(0b1000_0000_0000_0000);
    #[doc = "Controls coherency of memory writes"]
    pub const MEMORY_WRITE: Self = Self(0b1_0000_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits.html>"]
pub struct BufferUsageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(BufferUsageFlags, Flags);
impl BufferUsageFlags {
    #[doc = "Can be used as a source of transfer operations"]
    pub const TRANSFER_SRC: Self = Self(0b1);
    #[doc = "Can be used as a destination of transfer operations"]
    pub const TRANSFER_DST: Self = Self(0b10);
    #[doc = "Can be used as TBO"]
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(0b100);
    #[doc = "Can be used as IBO"]
    pub const STORAGE_TEXEL_BUFFER: Self = Self(0b1000);
    #[doc = "Can be used as UBO"]
    pub const UNIFORM_BUFFER: Self = Self(0b1_0000);
    #[doc = "Can be used as SSBO"]
    pub const STORAGE_BUFFER: Self = Self(0b10_0000);
    #[doc = "Can be used as source of fixed-function index fetch (index buffer)"]
    pub const INDEX_BUFFER: Self = Self(0b100_0000);
    #[doc = "Can be used as source of fixed-function vertex fetch (VBO)"]
    pub const VERTEX_BUFFER: Self = Self(0b1000_0000);
    #[doc = "Can be the source of indirect parameters (e.g. indirect buffer, parameter buffer)"]
    pub const INDIRECT_BUFFER: Self = Self(0b1_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits2.html>"]
pub struct BufferUsageFlags2(pub(crate) Flags64);
vk_bitflags_wrapped!(BufferUsageFlags2, Flags64);
impl BufferUsageFlags2 {
    pub const TRANSFER_SRC: Self = Self(0b1);
    pub const TRANSFER_DST: Self = Self(0b10);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(0b100);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(0b1000);
    pub const UNIFORM_BUFFER: Self = Self(0b1_0000);
    pub const STORAGE_BUFFER: Self = Self(0b10_0000);
    pub const INDEX_BUFFER: Self = Self(0b100_0000);
    pub const VERTEX_BUFFER: Self = Self(0b1000_0000);
    pub const INDIRECT_BUFFER: Self = Self(0b1_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCreateFlagBits.html>"]
pub struct BufferCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(BufferCreateFlags, Flags);
impl BufferCreateFlags {
    #[doc = "Buffer should support sparse backing"]
    pub const SPARSE_BINDING: Self = Self(0b1);
    #[doc = "Buffer should support sparse backing with partial residency"]
    pub const SPARSE_RESIDENCY: Self = Self(0b10);
    #[doc = "Buffer should support constant data access to physical memory ranges mapped into multiple locations of sparse buffers"]
    pub const SPARSE_ALIASED: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderStageFlagBits.html>"]
pub struct ShaderStageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ShaderStageFlags, Flags);
impl ShaderStageFlags {
    pub const VERTEX: Self = Self(0b1);
    pub const TESSELLATION_CONTROL: Self = Self(0b10);
    pub const TESSELLATION_EVALUATION: Self = Self(0b100);
    pub const GEOMETRY: Self = Self(0b1000);
    pub const FRAGMENT: Self = Self(0b1_0000);
    pub const COMPUTE: Self = Self(0b10_0000);
    pub const ALL_GRAPHICS: Self = Self(0x0000001f);
    pub const ALL: Self = Self(0x7fffffff);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlagBits.html>"]
pub struct ImageUsageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageUsageFlags, Flags);
impl ImageUsageFlags {
    #[doc = "Can be used as a source of transfer operations"]
    pub const TRANSFER_SRC: Self = Self(0b1);
    #[doc = "Can be used as a destination of transfer operations"]
    pub const TRANSFER_DST: Self = Self(0b10);
    #[doc = "Can be sampled from (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)"]
    pub const SAMPLED: Self = Self(0b100);
    #[doc = "Can be used as storage image (STORAGE_IMAGE descriptor type)"]
    pub const STORAGE: Self = Self(0b1000);
    #[doc = "Can be used as framebuffer color attachment"]
    pub const COLOR_ATTACHMENT: Self = Self(0b1_0000);
    #[doc = "Can be used as framebuffer depth/stencil attachment"]
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(0b10_0000);
    #[doc = "Image data not needed outside of rendering"]
    pub const TRANSIENT_ATTACHMENT: Self = Self(0b100_0000);
    #[doc = "Can be used as framebuffer input attachment"]
    pub const INPUT_ATTACHMENT: Self = Self(0b1000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlagBits.html>"]
pub struct ImageCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageCreateFlags, Flags);
impl ImageCreateFlags {
    #[doc = "Image should support sparse backing"]
    pub const SPARSE_BINDING: Self = Self(0b1);
    #[doc = "Image should support sparse backing with partial residency"]
    pub const SPARSE_RESIDENCY: Self = Self(0b10);
    #[doc = "Image should support constant data access to physical memory ranges mapped into multiple locations of sparse images"]
    pub const SPARSE_ALIASED: Self = Self(0b100);
    #[doc = "Allows image views to have different format than the base image"]
    pub const MUTABLE_FORMAT: Self = Self(0b1000);
    #[doc = "Allows creating image views with cube type from the created image"]
    pub const CUBE_COMPATIBLE: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewCreateFlagBits.html>"]
pub struct ImageViewCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageViewCreateFlags, Flags);
impl ImageViewCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCreateFlagBits.html>"]
pub struct SamplerCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SamplerCreateFlags, Flags);
impl SamplerCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlagBits.html>"]
pub struct PipelineCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineCreateFlags, Flags);
impl PipelineCreateFlags {
    pub const DISABLE_OPTIMIZATION: Self = Self(0b1);
    pub const ALLOW_DERIVATIVES: Self = Self(0b10);
    pub const DERIVATIVE: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlagBits2.html>"]
pub struct PipelineCreateFlags2(pub(crate) Flags64);
vk_bitflags_wrapped!(PipelineCreateFlags2, Flags64);
impl PipelineCreateFlags2 {
    pub const DISABLE_OPTIMIZATION: Self = Self(0b1);
    pub const ALLOW_DERIVATIVES: Self = Self(0b10);
    pub const DERIVATIVE: Self = Self(0b100);
    pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(0b1000);
    pub const DISPATCH_BASE: Self = Self(0b1_0000);
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(0b1_0000_0000);
    pub const EARLY_RETURN_ON_FAILURE: Self = Self(0b10_0000_0000);
    pub const NO_PROTECTED_ACCESS: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    pub const PROTECTED_ACCESS_ONLY: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageCreateFlagBits.html>"]
pub struct PipelineShaderStageCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineShaderStageCreateFlags, Flags);
impl PipelineShaderStageCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkColorComponentFlagBits.html>"]
pub struct ColorComponentFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ColorComponentFlags, Flags);
impl ColorComponentFlags {
    pub const R: Self = Self(0b1);
    pub const G: Self = Self(0b10);
    pub const B: Self = Self(0b100);
    pub const A: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceCreateFlagBits.html>"]
pub struct FenceCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FenceCreateFlags, Flags);
impl FenceCreateFlags {
    pub const SIGNALED: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreCreateFlagBits.html>"]
pub struct SemaphoreCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SemaphoreCreateFlags, Flags);
impl SemaphoreCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits.html>"]
pub struct FormatFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FormatFeatureFlags, Flags);
impl FormatFeatureFlags {
    #[doc = "Format can be used for sampled images (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)"]
    pub const SAMPLED_IMAGE: Self = Self(0b1);
    #[doc = "Format can be used for storage images (STORAGE_IMAGE descriptor type)"]
    pub const STORAGE_IMAGE: Self = Self(0b10);
    #[doc = "Format supports atomic operations in case it is used for storage images"]
    pub const STORAGE_IMAGE_ATOMIC: Self = Self(0b100);
    #[doc = "Format can be used for uniform texel buffers (TBOs)"]
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(0b1000);
    #[doc = "Format can be used for storage texel buffers (IBOs)"]
    pub const STORAGE_TEXEL_BUFFER: Self = Self(0b1_0000);
    #[doc = "Format supports atomic operations in case it is used for storage texel buffers"]
    pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(0b10_0000);
    #[doc = "Format can be used for vertex buffers (VBOs)"]
    pub const VERTEX_BUFFER: Self = Self(0b100_0000);
    #[doc = "Format can be used for color attachment images"]
    pub const COLOR_ATTACHMENT: Self = Self(0b1000_0000);
    #[doc = "Format supports blending in case it is used for color attachment images"]
    pub const COLOR_ATTACHMENT_BLEND: Self = Self(0b1_0000_0000);
    #[doc = "Format can be used for depth/stencil attachment images"]
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(0b10_0000_0000);
    #[doc = "Format can be used as the source image of blits with vkCmdBlitImage"]
    pub const BLIT_SRC: Self = Self(0b100_0000_0000);
    #[doc = "Format can be used as the destination image of blits with vkCmdBlitImage"]
    pub const BLIT_DST: Self = Self(0b1000_0000_0000);
    #[doc = "Format can be filtered with VK_FILTER_LINEAR when being sampled"]
    pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(0b1_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryControlFlagBits.html>"]
pub struct QueryControlFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueryControlFlags, Flags);
impl QueryControlFlags {
    #[doc = "Require precise results to be collected by the query"]
    pub const PRECISE: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryResultFlagBits.html>"]
pub struct QueryResultFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueryResultFlags, Flags);
impl QueryResultFlags {
    #[doc = "Results of the queries are written to the destination buffer as 64-bit values"]
    pub const TYPE_64: Self = Self(0b1);
    #[doc = "Results of the queries are waited on before proceeding with the result copy"]
    pub const WAIT: Self = Self(0b10);
    #[doc = "Besides the results of the query, the availability of the results is also written"]
    pub const WITH_AVAILABILITY: Self = Self(0b100);
    #[doc = "Copy the partial results of the query even if the final results are not available"]
    pub const PARTIAL: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferUsageFlagBits.html>"]
pub struct CommandBufferUsageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandBufferUsageFlags, Flags);
impl CommandBufferUsageFlags {
    pub const ONE_TIME_SUBMIT: Self = Self(0b1);
    pub const RENDER_PASS_CONTINUE: Self = Self(0b10);
    #[doc = "Command buffer may be submitted/executed more than once simultaneously"]
    pub const SIMULTANEOUS_USE: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPipelineStatisticFlagBits.html>"]
pub struct QueryPipelineStatisticFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueryPipelineStatisticFlags, Flags);
impl QueryPipelineStatisticFlags {
    #[doc = "Optional"]
    pub const INPUT_ASSEMBLY_VERTICES: Self = Self(0b1);
    #[doc = "Optional"]
    pub const INPUT_ASSEMBLY_PRIMITIVES: Self = Self(0b10);
    #[doc = "Optional"]
    pub const VERTEX_SHADER_INVOCATIONS: Self = Self(0b100);
    #[doc = "Optional"]
    pub const GEOMETRY_SHADER_INVOCATIONS: Self = Self(0b1000);
    #[doc = "Optional"]
    pub const GEOMETRY_SHADER_PRIMITIVES: Self = Self(0b1_0000);
    #[doc = "Optional"]
    pub const CLIPPING_INVOCATIONS: Self = Self(0b10_0000);
    #[doc = "Optional"]
    pub const CLIPPING_PRIMITIVES: Self = Self(0b100_0000);
    #[doc = "Optional"]
    pub const FRAGMENT_SHADER_INVOCATIONS: Self = Self(0b1000_0000);
    #[doc = "Optional"]
    pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self = Self(0b1_0000_0000);
    #[doc = "Optional"]
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self = Self(0b10_0000_0000);
    #[doc = "Optional"]
    pub const COMPUTE_SHADER_INVOCATIONS: Self = Self(0b100_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMapFlagBits.html>"]
pub struct MemoryMapFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryMapFlags, Flags);
impl MemoryMapFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageAspectFlagBits.html>"]
pub struct ImageAspectFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageAspectFlags, Flags);
impl ImageAspectFlags {
    pub const COLOR: Self = Self(0b1);
    pub const DEPTH: Self = Self(0b10);
    pub const STENCIL: Self = Self(0b100);
    pub const METADATA: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageFormatFlagBits.html>"]
pub struct SparseImageFormatFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SparseImageFormatFlags, Flags);
impl SparseImageFormatFlags {
    #[doc = "Image uses a single mip tail region for all array layers"]
    pub const SINGLE_MIPTAIL: Self = Self(0b1);
    #[doc = "Image requires mip level dimensions to be an integer multiple of the sparse image block dimensions for non-tail mip levels."]
    pub const ALIGNED_MIP_SIZE: Self = Self(0b10);
    #[doc = "Image uses a non-standard sparse image block dimensions"]
    pub const NONSTANDARD_BLOCK_SIZE: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseMemoryBindFlagBits.html>"]
pub struct SparseMemoryBindFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SparseMemoryBindFlags, Flags);
impl SparseMemoryBindFlags {
    #[doc = "Operation binds resource metadata to memory"]
    pub const METADATA: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlagBits.html>"]
pub struct PipelineStageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineStageFlags, Flags);
impl PipelineStageFlags {
    #[doc = "Before subsequent commands are processed"]
    pub const TOP_OF_PIPE: Self = Self(0b1);
    #[doc = "Draw/DispatchIndirect command fetch"]
    pub const DRAW_INDIRECT: Self = Self(0b10);
    #[doc = "Vertex/index fetch"]
    pub const VERTEX_INPUT: Self = Self(0b100);
    #[doc = "Vertex shading"]
    pub const VERTEX_SHADER: Self = Self(0b1000);
    #[doc = "Tessellation control shading"]
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(0b1_0000);
    #[doc = "Tessellation evaluation shading"]
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(0b10_0000);
    #[doc = "Geometry shading"]
    pub const GEOMETRY_SHADER: Self = Self(0b100_0000);
    #[doc = "Fragment shading"]
    pub const FRAGMENT_SHADER: Self = Self(0b1000_0000);
    #[doc = "Early fragment (depth and stencil) tests"]
    pub const EARLY_FRAGMENT_TESTS: Self = Self(0b1_0000_0000);
    #[doc = "Late fragment (depth and stencil) tests"]
    pub const LATE_FRAGMENT_TESTS: Self = Self(0b10_0000_0000);
    #[doc = "Color attachment writes"]
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(0b100_0000_0000);
    #[doc = "Compute shading"]
    pub const COMPUTE_SHADER: Self = Self(0b1000_0000_0000);
    #[doc = "Transfer/copy operations"]
    pub const TRANSFER: Self = Self(0b1_0000_0000_0000);
    #[doc = "After previous commands have completed"]
    pub const BOTTOM_OF_PIPE: Self = Self(0b10_0000_0000_0000);
    #[doc = "Indicates host (CPU) is a source/sink of the dependency"]
    pub const HOST: Self = Self(0b100_0000_0000_0000);
    #[doc = "All stages of the graphics pipeline"]
    pub const ALL_GRAPHICS: Self = Self(0b1000_0000_0000_0000);
    #[doc = "All stages supported on the queue"]
    pub const ALL_COMMANDS: Self = Self(0b1_0000_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolCreateFlagBits.html>"]
pub struct CommandPoolCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandPoolCreateFlags, Flags);
impl CommandPoolCreateFlags {
    #[doc = "Command buffers have a short lifetime"]
    pub const TRANSIENT: Self = Self(0b1);
    #[doc = "Command buffers may release their memory individually"]
    pub const RESET_COMMAND_BUFFER: Self = Self(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolResetFlagBits.html>"]
pub struct CommandPoolResetFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandPoolResetFlags, Flags);
impl CommandPoolResetFlags {
    #[doc = "Release resources owned by the pool"]
    pub const RELEASE_RESOURCES: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferResetFlagBits.html>"]
pub struct CommandBufferResetFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandBufferResetFlags, Flags);
impl CommandBufferResetFlags {
    #[doc = "Release resources owned by the buffer"]
    pub const RELEASE_RESOURCES: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleCountFlagBits.html>"]
pub struct SampleCountFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SampleCountFlags, Flags);
impl SampleCountFlags {
    #[doc = "Sample count 1 supported"]
    pub const TYPE_1: Self = Self(0b1);
    #[doc = "Sample count 2 supported"]
    pub const TYPE_2: Self = Self(0b10);
    #[doc = "Sample count 4 supported"]
    pub const TYPE_4: Self = Self(0b100);
    #[doc = "Sample count 8 supported"]
    pub const TYPE_8: Self = Self(0b1000);
    #[doc = "Sample count 16 supported"]
    pub const TYPE_16: Self = Self(0b1_0000);
    #[doc = "Sample count 32 supported"]
    pub const TYPE_32: Self = Self(0b10_0000);
    #[doc = "Sample count 64 supported"]
    pub const TYPE_64: Self = Self(0b100_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescriptionFlagBits.html>"]
pub struct AttachmentDescriptionFlags(pub(crate) Flags);
vk_bitflags_wrapped!(AttachmentDescriptionFlags, Flags);
impl AttachmentDescriptionFlags {
    #[doc = "The attachment may alias physical memory of another attachment in the same render pass"]
    pub const MAY_ALIAS: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilFaceFlagBits.html>"]
pub struct StencilFaceFlags(pub(crate) Flags);
vk_bitflags_wrapped!(StencilFaceFlags, Flags);
impl StencilFaceFlags {
    #[doc = "Front face"]
    pub const FRONT: Self = Self(0b1);
    #[doc = "Back face"]
    pub const BACK: Self = Self(0b10);
    #[doc = "Front and back faces"]
    pub const FRONT_AND_BACK: Self = Self(0x00000003);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolCreateFlagBits.html>"]
pub struct DescriptorPoolCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DescriptorPoolCreateFlags, Flags);
impl DescriptorPoolCreateFlags {
    #[doc = "Descriptor sets may be freed individually"]
    pub const FREE_DESCRIPTOR_SET: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDependencyFlagBits.html>"]
pub struct DependencyFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DependencyFlags, Flags);
impl DependencyFlags {
    #[doc = "Dependency is per pixel region "]
    pub const BY_REGION: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitFlagBits.html>"]
pub struct SemaphoreWaitFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SemaphoreWaitFlags, Flags);
impl SemaphoreWaitFlags {
    pub const ANY: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPlaneAlphaFlagBitsKHR.html>"]
pub struct DisplayPlaneAlphaFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(DisplayPlaneAlphaFlagsKHR, Flags);
impl DisplayPlaneAlphaFlagsKHR {
    pub const OPAQUE: Self = Self(0b1);
    pub const GLOBAL: Self = Self(0b10);
    pub const PER_PIXEL: Self = Self(0b100);
    pub const PER_PIXEL_PREMULTIPLIED: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCompositeAlphaFlagBitsKHR.html>"]
pub struct CompositeAlphaFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(CompositeAlphaFlagsKHR, Flags);
impl CompositeAlphaFlagsKHR {
    pub const OPAQUE: Self = Self(0b1);
    pub const PRE_MULTIPLIED: Self = Self(0b10);
    pub const POST_MULTIPLIED: Self = Self(0b100);
    pub const INHERIT: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceTransformFlagBitsKHR.html>"]
pub struct SurfaceTransformFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(SurfaceTransformFlagsKHR, Flags);
impl SurfaceTransformFlagsKHR {
    pub const IDENTITY: Self = Self(0b1);
    pub const ROTATE_90: Self = Self(0b10);
    pub const ROTATE_180: Self = Self(0b100);
    pub const ROTATE_270: Self = Self(0b1000);
    pub const HORIZONTAL_MIRROR: Self = Self(0b1_0000);
    pub const HORIZONTAL_MIRROR_ROTATE_90: Self = Self(0b10_0000);
    pub const HORIZONTAL_MIRROR_ROTATE_180: Self = Self(0b100_0000);
    pub const HORIZONTAL_MIRROR_ROTATE_270: Self = Self(0b1000_0000);
    pub const INHERIT: Self = Self(0b1_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubgroupFeatureFlagBits.html>"]
pub struct SubgroupFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SubgroupFeatureFlags, Flags);
impl SubgroupFeatureFlags {
    #[doc = "Basic subgroup operations"]
    pub const BASIC: Self = Self(0b1);
    #[doc = "Vote subgroup operations"]
    pub const VOTE: Self = Self(0b10);
    #[doc = "Arithmetic subgroup operations"]
    pub const ARITHMETIC: Self = Self(0b100);
    #[doc = "Ballot subgroup operations"]
    pub const BALLOT: Self = Self(0b1000);
    #[doc = "Shuffle subgroup operations"]
    pub const SHUFFLE: Self = Self(0b1_0000);
    #[doc = "Shuffle relative subgroup operations"]
    pub const SHUFFLE_RELATIVE: Self = Self(0b10_0000);
    #[doc = "Clustered subgroup operations"]
    pub const CLUSTERED: Self = Self(0b100_0000);
    #[doc = "Quad subgroup operations"]
    pub const QUAD: Self = Self(0b1000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateFlagBits.html>"]
pub struct PrivateDataSlotCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PrivateDataSlotCreateFlags, Flags);
impl PrivateDataSlotCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutCreateFlagBits.html>"]
pub struct DescriptorSetLayoutCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DescriptorSetLayoutCreateFlags, Flags);
impl DescriptorSetLayoutCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagBits.html>"]
pub struct ExternalMemoryHandleTypeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalMemoryHandleTypeFlags, Flags);
impl ExternalMemoryHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(0b1);
    pub const OPAQUE_WIN32: Self = Self(0b10);
    pub const OPAQUE_WIN32_KMT: Self = Self(0b100);
    pub const D3D11_TEXTURE: Self = Self(0b1000);
    pub const D3D11_TEXTURE_KMT: Self = Self(0b1_0000);
    pub const D3D12_HEAP: Self = Self(0b10_0000);
    pub const D3D12_RESOURCE: Self = Self(0b100_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagBits.html>"]
pub struct ExternalMemoryFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalMemoryFeatureFlags, Flags);
impl ExternalMemoryFeatureFlags {
    pub const DEDICATED_ONLY: Self = Self(0b1);
    pub const EXPORTABLE: Self = Self(0b10);
    pub const IMPORTABLE: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreHandleTypeFlagBits.html>"]
pub struct ExternalSemaphoreHandleTypeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalSemaphoreHandleTypeFlags, Flags);
impl ExternalSemaphoreHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(0b1);
    pub const OPAQUE_WIN32: Self = Self(0b10);
    pub const OPAQUE_WIN32_KMT: Self = Self(0b100);
    pub const D3D12_FENCE: Self = Self(0b1000);
    pub const D3D11_FENCE: Self = Self::D3D12_FENCE;
    pub const SYNC_FD: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreFeatureFlagBits.html>"]
pub struct ExternalSemaphoreFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalSemaphoreFeatureFlags, Flags);
impl ExternalSemaphoreFeatureFlags {
    pub const EXPORTABLE: Self = Self(0b1);
    pub const IMPORTABLE: Self = Self(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreImportFlagBits.html>"]
pub struct SemaphoreImportFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SemaphoreImportFlags, Flags);
impl SemaphoreImportFlags {
    pub const TEMPORARY: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceHandleTypeFlagBits.html>"]
pub struct ExternalFenceHandleTypeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalFenceHandleTypeFlags, Flags);
impl ExternalFenceHandleTypeFlags {
    pub const OPAQUE_FD: Self = Self(0b1);
    pub const OPAQUE_WIN32: Self = Self(0b10);
    pub const OPAQUE_WIN32_KMT: Self = Self(0b100);
    pub const SYNC_FD: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceFeatureFlagBits.html>"]
pub struct ExternalFenceFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalFenceFeatureFlags, Flags);
impl ExternalFenceFeatureFlags {
    pub const EXPORTABLE: Self = Self(0b1);
    pub const IMPORTABLE: Self = Self(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceImportFlagBits.html>"]
pub struct FenceImportFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FenceImportFlags, Flags);
impl FenceImportFlags {
    pub const TEMPORARY: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPeerMemoryFeatureFlagBits.html>"]
pub struct PeerMemoryFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PeerMemoryFeatureFlags, Flags);
impl PeerMemoryFeatureFlags {
    #[doc = "Can read with vkCmdCopy commands"]
    pub const COPY_SRC: Self = Self(0b1);
    #[doc = "Can write with vkCmdCopy commands"]
    pub const COPY_DST: Self = Self(0b10);
    #[doc = "Can read with any access type/command"]
    pub const GENERIC_SRC: Self = Self(0b100);
    #[doc = "Can write with and access type/command"]
    pub const GENERIC_DST: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlagBits.html>"]
pub struct MemoryAllocateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryAllocateFlags, Flags);
impl MemoryAllocateFlags {
    #[doc = "Force allocation on specific devices"]
    pub const DEVICE_MASK: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupPresentModeFlagBitsKHR.html>"]
pub struct DeviceGroupPresentModeFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(DeviceGroupPresentModeFlagsKHR, Flags);
impl DeviceGroupPresentModeFlagsKHR {
    #[doc = "Present from local memory"]
    pub const LOCAL: Self = Self(0b1);
    #[doc = "Present from remote memory"]
    pub const REMOTE: Self = Self(0b10);
    #[doc = "Present sum of local and/or remote memory"]
    pub const SUM: Self = Self(0b100);
    #[doc = "Each physical device presents from local memory"]
    pub const LOCAL_MULTI_DEVICE: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainCreateFlagBitsKHR.html>"]
pub struct SwapchainCreateFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(SwapchainCreateFlagsKHR, Flags);
impl SwapchainCreateFlagsKHR {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescriptionFlagBits.html>"]
pub struct SubpassDescriptionFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SubpassDescriptionFlags, Flags);
impl SubpassDescriptionFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageSeverityFlagBitsEXT.html>"]
pub struct DebugUtilsMessageSeverityFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(DebugUtilsMessageSeverityFlagsEXT, Flags);
impl DebugUtilsMessageSeverityFlagsEXT {
    pub const VERBOSE: Self = Self(0b1);
    pub const INFO: Self = Self(0b1_0000);
    pub const WARNING: Self = Self(0b1_0000_0000);
    pub const ERROR: Self = Self(0b1_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageTypeFlagBitsEXT.html>"]
pub struct DebugUtilsMessageTypeFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(DebugUtilsMessageTypeFlagsEXT, Flags);
impl DebugUtilsMessageTypeFlagsEXT {
    pub const GENERAL: Self = Self(0b1);
    pub const VALIDATION: Self = Self(0b10);
    pub const PERFORMANCE: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBindingFlagBits.html>"]
pub struct DescriptorBindingFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DescriptorBindingFlags, Flags);
impl DescriptorBindingFlags {
    pub const UPDATE_AFTER_BIND: Self = Self(0b1);
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(0b10);
    pub const PARTIALLY_BOUND: Self = Self(0b100);
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveModeFlagBits.html>"]
pub struct ResolveModeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ResolveModeFlags, Flags);
impl ResolveModeFlags {
    pub const NONE: Self = Self(0);
    pub const SAMPLE_ZERO: Self = Self(0b1);
    pub const AVERAGE: Self = Self(0b10);
    pub const MIN: Self = Self(0b100);
    pub const MAX: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryInstanceFlagBitsKHR.html>"]
pub struct GeometryInstanceFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(GeometryInstanceFlagsKHR, Flags);
impl GeometryInstanceFlagsKHR {
    pub const TRIANGLE_FACING_CULL_DISABLE: Self = Self(0b1);
    pub const TRIANGLE_FLIP_FACING: Self = Self(0b10);
    pub const FORCE_OPAQUE: Self = Self(0b100);
    pub const FORCE_NO_OPAQUE: Self = Self(0b1000);
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE: Self = Self::TRIANGLE_FLIP_FACING;
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryFlagBitsKHR.html>"]
pub struct GeometryFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(GeometryFlagsKHR, Flags);
impl GeometryFlagsKHR {
    pub const OPAQUE: Self = Self(0b1);
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION: Self = Self(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureFlagBitsKHR.html>"]
pub struct BuildAccelerationStructureFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(BuildAccelerationStructureFlagsKHR, Flags);
impl BuildAccelerationStructureFlagsKHR {
    pub const ALLOW_UPDATE: Self = Self(0b1);
    pub const ALLOW_COMPACTION: Self = Self(0b10);
    pub const PREFER_FAST_TRACE: Self = Self(0b100);
    pub const PREFER_FAST_BUILD: Self = Self(0b1000);
    pub const LOW_MEMORY: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCreateFlagBitsKHR.html>"]
pub struct AccelerationStructureCreateFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(AccelerationStructureCreateFlagsKHR, Flags);
impl AccelerationStructureCreateFlagsKHR {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferCreateFlagBits.html>"]
pub struct FramebufferCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FramebufferCreateFlags, Flags);
impl FramebufferCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolCreateFlagBits.html>"]
pub struct QueryPoolCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueryPoolCreateFlags, Flags);
impl QueryPoolCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackFlagBits.html>"]
pub struct PipelineCreationFeedbackFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineCreationFeedbackFlags, Flags);
impl PipelineCreationFeedbackFlags {
    pub const VALID: Self = Self(0b1);
    pub const APPLICATION_PIPELINE_CACHE_HIT: Self = Self(0b10);
    pub const BASE_PIPELINE_ACCELERATION: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModuleCreateFlagBits.html>"]
pub struct ShaderModuleCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ShaderModuleCreateFlags, Flags);
impl ShaderModuleCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkToolPurposeFlagBits.html>"]
pub struct ToolPurposeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ToolPurposeFlags, Flags);
impl ToolPurposeFlags {
    pub const VALIDATION: Self = Self(0b1);
    pub const PROFILING: Self = Self(0b10);
    pub const TRACING: Self = Self(0b100);
    pub const ADDITIONAL_FEATURES: Self = Self(0b1000);
    pub const MODIFYING_FEATURES: Self = Self(0b1_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits2.html>"]
pub struct AccessFlags2(pub(crate) Flags64);
vk_bitflags_wrapped!(AccessFlags2, Flags64);
impl AccessFlags2 {
    pub const NONE: Self = Self(0);
    pub const INDIRECT_COMMAND_READ: Self = Self(0b1);
    pub const INDEX_READ: Self = Self(0b10);
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(0b100);
    pub const UNIFORM_READ: Self = Self(0b1000);
    pub const INPUT_ATTACHMENT_READ: Self = Self(0b1_0000);
    pub const SHADER_READ: Self = Self(0b10_0000);
    pub const SHADER_WRITE: Self = Self(0b100_0000);
    pub const COLOR_ATTACHMENT_READ: Self = Self(0b1000_0000);
    pub const COLOR_ATTACHMENT_WRITE: Self = Self(0b1_0000_0000);
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(0b10_0000_0000);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(0b100_0000_0000);
    pub const TRANSFER_READ: Self = Self(0b1000_0000_0000);
    pub const TRANSFER_WRITE: Self = Self(0b1_0000_0000_0000);
    pub const HOST_READ: Self = Self(0b10_0000_0000_0000);
    pub const HOST_WRITE: Self = Self(0b100_0000_0000_0000);
    pub const MEMORY_READ: Self = Self(0b1000_0000_0000_0000);
    pub const MEMORY_WRITE: Self = Self(0b1_0000_0000_0000_0000);
    pub const SHADER_SAMPLED_READ: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const SHADER_STORAGE_READ: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const SHADER_STORAGE_WRITE: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlagBits2.html>"]
pub struct PipelineStageFlags2(pub(crate) Flags64);
vk_bitflags_wrapped!(PipelineStageFlags2, Flags64);
impl PipelineStageFlags2 {
    pub const NONE: Self = Self(0);
    pub const TOP_OF_PIPE: Self = Self(0b1);
    pub const DRAW_INDIRECT: Self = Self(0b10);
    pub const VERTEX_INPUT: Self = Self(0b100);
    pub const VERTEX_SHADER: Self = Self(0b1000);
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(0b1_0000);
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(0b10_0000);
    pub const GEOMETRY_SHADER: Self = Self(0b100_0000);
    pub const FRAGMENT_SHADER: Self = Self(0b1000_0000);
    pub const EARLY_FRAGMENT_TESTS: Self = Self(0b1_0000_0000);
    pub const LATE_FRAGMENT_TESTS: Self = Self(0b10_0000_0000);
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(0b100_0000_0000);
    pub const COMPUTE_SHADER: Self = Self(0b1000_0000_0000);
    pub const ALL_TRANSFER: Self = Self(0b1_0000_0000_0000);
    pub const TRANSFER: Self = Self::ALL_TRANSFER;
    pub const BOTTOM_OF_PIPE: Self = Self(0b10_0000_0000_0000);
    pub const HOST: Self = Self(0b100_0000_0000_0000);
    pub const ALL_GRAPHICS: Self = Self(0b1000_0000_0000_0000);
    pub const ALL_COMMANDS: Self = Self(0b1_0000_0000_0000_0000);
    pub const COPY: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const RESOLVE: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const BLIT: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const CLEAR: Self = Self(0b1000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const INDEX_INPUT: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const VERTEX_ATTRIBUTE_INPUT: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const PRE_RASTERIZATION_SHADERS: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitFlagBits.html>"]
pub struct SubmitFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SubmitFlags, Flags);
impl SubmitFlags {
    pub const PROTECTED: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkEventCreateFlagBits.html>"]
pub struct EventCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(EventCreateFlags, Flags);
impl EventCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayoutCreateFlagBits.html>"]
pub struct PipelineLayoutCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineLayoutCreateFlags, Flags);
impl PipelineLayoutCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineColorBlendStateCreateFlagBits.html>"]
pub struct PipelineColorBlendStateCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineColorBlendStateCreateFlags, Flags);
impl PipelineColorBlendStateCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineDepthStencilStateCreateFlagBits.html>"]
pub struct PipelineDepthStencilStateCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineDepthStencilStateCreateFlags, Flags);
impl PipelineDepthStencilStateCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsPipelineLibraryFlagBitsEXT.html>"]
pub struct GraphicsPipelineLibraryFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(GraphicsPipelineLibraryFlagsEXT, Flags);
impl GraphicsPipelineLibraryFlagsEXT {
    pub const VERTEX_INPUT_INTERFACE: Self = Self(0b1);
    pub const PRE_RASTERIZATION_SHADERS: Self = Self(0b10);
    pub const FRAGMENT_SHADER: Self = Self(0b100);
    pub const FRAGMENT_OUTPUT_INTERFACE: Self = Self(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyFlagBits.html>"]
pub struct HostImageCopyFlags(pub(crate) Flags);
vk_bitflags_wrapped!(HostImageCopyFlags, Flags);
impl HostImageCopyFlags {
    pub const MEMCPY: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits2.html>"]
pub struct FormatFeatureFlags2(pub(crate) Flags64);
vk_bitflags_wrapped!(FormatFeatureFlags2, Flags64);
impl FormatFeatureFlags2 {
    pub const SAMPLED_IMAGE: Self = Self(0b1);
    pub const STORAGE_IMAGE: Self = Self(0b10);
    pub const STORAGE_IMAGE_ATOMIC: Self = Self(0b100);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(0b1000);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(0b1_0000);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(0b10_0000);
    pub const VERTEX_BUFFER: Self = Self(0b100_0000);
    pub const COLOR_ATTACHMENT: Self = Self(0b1000_0000);
    pub const COLOR_ATTACHMENT_BLEND: Self = Self(0b1_0000_0000);
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(0b10_0000_0000);
    pub const BLIT_SRC: Self = Self(0b100_0000_0000);
    pub const BLIT_DST: Self = Self(0b1000_0000_0000);
    pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(0b1_0000_0000_0000);
    pub const TRANSFER_SRC: Self = Self(0b100_0000_0000_0000);
    pub const TRANSFER_DST: Self = Self(0b1000_0000_0000_0000);
    pub const SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(0b1_0000_0000_0000_0000);
    pub const MIDPOINT_CHROMA_SAMPLES: Self = Self(0b10_0000_0000_0000_0000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(0b100_0000_0000_0000_0000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self =
        Self(0b1000_0000_0000_0000_0000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self =
        Self(0b1_0000_0000_0000_0000_0000);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self =
        Self(0b10_0000_0000_0000_0000_0000);
    pub const DISJOINT: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const COSITED_CHROMA_SAMPLES: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const STORAGE_READ_WITHOUT_FORMAT: Self = Self(0b1000_0000_0000_0000_0000_0000_0000_0000);
    pub const STORAGE_WRITE_WITHOUT_FORMAT: Self =
        Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFlagBits.html>"]
pub struct RenderingFlags(pub(crate) Flags);
vk_bitflags_wrapped!(RenderingFlags, Flags);
impl RenderingFlags {
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS: Self = Self(0b1);
    pub const SUSPENDING: Self = Self(0b10);
    pub const RESUMING: Self = Self(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkInstanceCreateFlagBits.html>"]
pub struct InstanceCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(InstanceCreateFlags, Flags);
impl InstanceCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCreateFlagBitsEXT.html>"]
pub struct ShaderCreateFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(ShaderCreateFlagsEXT, Flags);
impl ShaderCreateFlagsEXT {
    pub const LINK_STAGE: Self = Self(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapFlagBits.html>"]
pub struct MemoryUnmapFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryUnmapFlags, Flags);
impl MemoryUnmapFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkWaylandSurfaceCreateFlagBitsKHR.html>"]
pub struct WaylandSurfaceCreateFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(WaylandSurfaceCreateFlagsKHR, Flags);
impl WaylandSurfaceCreateFlagsKHR {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits3KHR.html>"]
pub struct AccessFlags3KHR(pub(crate) Flags64);
vk_bitflags_wrapped!(AccessFlags3KHR, Flags64);
impl AccessFlags3KHR {
    pub const NONE: Self = Self(0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewCreateFlagBitsARM.html>"]
pub struct TensorViewCreateFlagsARM(pub(crate) Flags64);
vk_bitflags_wrapped!(TensorViewCreateFlagsARM, Flags64);
impl TensorViewCreateFlagsARM {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSpirvResourceTypeFlagBitsEXT.html>"]
pub struct SpirvResourceTypeFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(SpirvResourceTypeFlagsEXT, Flags);
impl SpirvResourceTypeFlagsEXT {
    pub const ALL: Self = Self(0x7fffffff);
    pub const SAMPLER: Self = Self(0b1);
    pub const SAMPLED_IMAGE: Self = Self(0b10);
    pub const READ_ONLY_IMAGE: Self = Self(0b100);
    pub const READ_WRITE_IMAGE: Self = Self(0b1000);
    pub const COMBINED_SAMPLED_IMAGE: Self = Self(0b1_0000);
    pub const UNIFORM_BUFFER: Self = Self(0b10_0000);
    pub const READ_ONLY_STORAGE_BUFFER: Self = Self(0b100_0000);
    pub const READ_WRITE_STORAGE_BUFFER: Self = Self(0b1000_0000);
}
