use core::fmt;
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageLayout.html>"]
pub struct ImageLayout(pub(crate) i32);
impl ImageLayout {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    #[doc = "Implicit layout an image is when its contents are undefined due to various reasons (e.g. right after creation)"]
    pub const UNDEFINED: Self = Self(0);
    #[doc = "General layout when image can be used for any kind of access"]
    pub const GENERAL: Self = Self(1);
    #[doc = "Optimal layout when image is only used for color attachment read/write"]
    pub const COLOR_ATTACHMENT_OPTIMAL: Self = Self(2);
    #[doc = "Optimal layout when image is only used for depth/stencil attachment read/write"]
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(3);
    #[doc = "Optimal layout when image is used for read only depth/stencil attachment and shader access"]
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = Self(4);
    #[doc = "Optimal layout when image is used for read only shader access"]
    pub const SHADER_READ_ONLY_OPTIMAL: Self = Self(5);
    #[doc = "Optimal layout when image is used only as source of transfer operations"]
    pub const TRANSFER_SRC_OPTIMAL: Self = Self(6);
    #[doc = "Optimal layout when image is used only as destination of transfer operations"]
    pub const TRANSFER_DST_OPTIMAL: Self = Self(7);
    #[doc = "Initial layout used when the data is populated by the CPU"]
    pub const PREINITIALIZED: Self = Self(8);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentLoadOp.html>"]
pub struct AttachmentLoadOp(pub(crate) i32);
impl AttachmentLoadOp {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const LOAD: Self = Self(0);
    pub const CLEAR: Self = Self(1);
    pub const DONT_CARE: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentStoreOp.html>"]
pub struct AttachmentStoreOp(pub(crate) i32);
impl AttachmentStoreOp {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const STORE: Self = Self(0);
    pub const DONT_CARE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageType.html>"]
pub struct ImageType(pub(crate) i32);
impl ImageType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const TYPE_1D: Self = Self(0);
    pub const TYPE_2D: Self = Self(1);
    pub const TYPE_3D: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageTiling.html>"]
pub struct ImageTiling(pub(crate) i32);
impl ImageTiling {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OPTIMAL: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewType.html>"]
pub struct ImageViewType(pub(crate) i32);
impl ImageViewType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const TYPE_1D: Self = Self(0);
    pub const TYPE_2D: Self = Self(1);
    pub const TYPE_3D: Self = Self(2);
    pub const CUBE: Self = Self(3);
    pub const TYPE_1D_ARRAY: Self = Self(4);
    pub const TYPE_2D_ARRAY: Self = Self(5);
    pub const CUBE_ARRAY: Self = Self(6);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferLevel.html>"]
pub struct CommandBufferLevel(pub(crate) i32);
impl CommandBufferLevel {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const PRIMARY: Self = Self(0);
    pub const SECONDARY: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkComponentSwizzle.html>"]
pub struct ComponentSwizzle(pub(crate) i32);
impl ComponentSwizzle {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const IDENTITY: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const ONE: Self = Self(2);
    pub const R: Self = Self(3);
    pub const G: Self = Self(4);
    pub const B: Self = Self(5);
    pub const A: Self = Self(6);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorType.html>"]
pub struct DescriptorType(pub(crate) i32);
impl DescriptorType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SAMPLER: Self = Self(0);
    pub const COMBINED_IMAGE_SAMPLER: Self = Self(1);
    pub const SAMPLED_IMAGE: Self = Self(2);
    pub const STORAGE_IMAGE: Self = Self(3);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(4);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(5);
    pub const UNIFORM_BUFFER: Self = Self(6);
    pub const STORAGE_BUFFER: Self = Self(7);
    pub const UNIFORM_BUFFER_DYNAMIC: Self = Self(8);
    pub const STORAGE_BUFFER_DYNAMIC: Self = Self(9);
    pub const INPUT_ATTACHMENT: Self = Self(10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryType.html>"]
pub struct QueryType(pub(crate) i32);
impl QueryType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OCCLUSION: Self = Self(0);
    #[doc = "Optional"]
    pub const PIPELINE_STATISTICS: Self = Self(1);
    pub const TIMESTAMP: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBorderColor.html>"]
pub struct BorderColor(pub(crate) i32);
impl BorderColor {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const FLOAT_TRANSPARENT_BLACK: Self = Self(0);
    pub const INT_TRANSPARENT_BLACK: Self = Self(1);
    pub const FLOAT_OPAQUE_BLACK: Self = Self(2);
    pub const INT_OPAQUE_BLACK: Self = Self(3);
    pub const FLOAT_OPAQUE_WHITE: Self = Self(4);
    pub const INT_OPAQUE_WHITE: Self = Self(5);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBindPoint.html>"]
pub struct PipelineBindPoint(pub(crate) i32);
impl PipelineBindPoint {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const GRAPHICS: Self = Self(0);
    pub const COMPUTE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheHeaderVersion.html>"]
pub struct PipelineCacheHeaderVersion(pub(crate) i32);
impl PipelineCacheHeaderVersion {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ONE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPrimitiveTopology.html>"]
pub struct PrimitiveTopology(pub(crate) i32);
impl PrimitiveTopology {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const POINT_LIST: Self = Self(0);
    pub const LINE_LIST: Self = Self(1);
    pub const LINE_STRIP: Self = Self(2);
    pub const TRIANGLE_LIST: Self = Self(3);
    pub const TRIANGLE_STRIP: Self = Self(4);
    pub const TRIANGLE_FAN: Self = Self(5);
    pub const LINE_LIST_WITH_ADJACENCY: Self = Self(6);
    pub const LINE_STRIP_WITH_ADJACENCY: Self = Self(7);
    pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = Self(8);
    pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = Self(9);
    pub const PATCH_LIST: Self = Self(10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSharingMode.html>"]
pub struct SharingMode(pub(crate) i32);
impl SharingMode {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const EXCLUSIVE: Self = Self(0);
    pub const CONCURRENT: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndexType.html>"]
pub struct IndexType(pub(crate) i32);
impl IndexType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UINT16: Self = Self(0);
    pub const UINT32: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFilter.html>"]
pub struct Filter(pub(crate) i32);
impl Filter {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerMipmapMode.html>"]
pub struct SamplerMipmapMode(pub(crate) i32);
impl SamplerMipmapMode {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    #[doc = "Choose nearest mip level"]
    pub const NEAREST: Self = Self(0);
    #[doc = "Linear filter between mip levels"]
    pub const LINEAR: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerAddressMode.html>"]
pub struct SamplerAddressMode(pub(crate) i32);
impl SamplerAddressMode {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const REPEAT: Self = Self(0);
    pub const MIRRORED_REPEAT: Self = Self(1);
    pub const CLAMP_TO_EDGE: Self = Self(2);
    pub const CLAMP_TO_BORDER: Self = Self(3);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCompareOp.html>"]
pub struct CompareOp(pub(crate) i32);
impl CompareOp {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NEVER: Self = Self(0);
    pub const LESS: Self = Self(1);
    pub const EQUAL: Self = Self(2);
    pub const LESS_OR_EQUAL: Self = Self(3);
    pub const GREATER: Self = Self(4);
    pub const NOT_EQUAL: Self = Self(5);
    pub const GREATER_OR_EQUAL: Self = Self(6);
    pub const ALWAYS: Self = Self(7);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPolygonMode.html>"]
pub struct PolygonMode(pub(crate) i32);
impl PolygonMode {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const FILL: Self = Self(0);
    pub const LINE: Self = Self(1);
    pub const POINT: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFrontFace.html>"]
pub struct FrontFace(pub(crate) i32);
impl FrontFace {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COUNTER_CLOCKWISE: Self = Self(0);
    pub const CLOCKWISE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBlendFactor.html>"]
pub struct BlendFactor(pub(crate) i32);
impl BlendFactor {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const SRC_COLOR: Self = Self(2);
    pub const ONE_MINUS_SRC_COLOR: Self = Self(3);
    pub const DST_COLOR: Self = Self(4);
    pub const ONE_MINUS_DST_COLOR: Self = Self(5);
    pub const SRC_ALPHA: Self = Self(6);
    pub const ONE_MINUS_SRC_ALPHA: Self = Self(7);
    pub const DST_ALPHA: Self = Self(8);
    pub const ONE_MINUS_DST_ALPHA: Self = Self(9);
    pub const CONSTANT_COLOR: Self = Self(10);
    pub const ONE_MINUS_CONSTANT_COLOR: Self = Self(11);
    pub const CONSTANT_ALPHA: Self = Self(12);
    pub const ONE_MINUS_CONSTANT_ALPHA: Self = Self(13);
    pub const SRC_ALPHA_SATURATE: Self = Self(14);
    pub const SRC1_COLOR: Self = Self(15);
    pub const ONE_MINUS_SRC1_COLOR: Self = Self(16);
    pub const SRC1_ALPHA: Self = Self(17);
    pub const ONE_MINUS_SRC1_ALPHA: Self = Self(18);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBlendOp.html>"]
pub struct BlendOp(pub(crate) i32);
impl BlendOp {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ADD: Self = Self(0);
    pub const SUBTRACT: Self = Self(1);
    pub const REVERSE_SUBTRACT: Self = Self(2);
    pub const MIN: Self = Self(3);
    pub const MAX: Self = Self(4);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilOp.html>"]
pub struct StencilOp(pub(crate) i32);
impl StencilOp {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const KEEP: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const REPLACE: Self = Self(2);
    pub const INCREMENT_AND_CLAMP: Self = Self(3);
    pub const DECREMENT_AND_CLAMP: Self = Self(4);
    pub const INVERT: Self = Self(5);
    pub const INCREMENT_AND_WRAP: Self = Self(6);
    pub const DECREMENT_AND_WRAP: Self = Self(7);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkLogicOp.html>"]
pub struct LogicOp(pub(crate) i32);
impl LogicOp {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CLEAR: Self = Self(0);
    pub const AND: Self = Self(1);
    pub const AND_REVERSE: Self = Self(2);
    pub const COPY: Self = Self(3);
    pub const AND_INVERTED: Self = Self(4);
    pub const NO_OP: Self = Self(5);
    pub const XOR: Self = Self(6);
    pub const OR: Self = Self(7);
    pub const NOR: Self = Self(8);
    pub const EQUIVALENT: Self = Self(9);
    pub const INVERT: Self = Self(10);
    pub const OR_REVERSE: Self = Self(11);
    pub const COPY_INVERTED: Self = Self(12);
    pub const OR_INVERTED: Self = Self(13);
    pub const NAND: Self = Self(14);
    pub const SET: Self = Self(15);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkInternalAllocationType.html>"]
pub struct InternalAllocationType(pub(crate) i32);
impl InternalAllocationType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const EXECUTABLE: Self = Self(0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSystemAllocationScope.html>"]
pub struct SystemAllocationScope(pub(crate) i32);
impl SystemAllocationScope {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COMMAND: Self = Self(0);
    pub const OBJECT: Self = Self(1);
    pub const CACHE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const INSTANCE: Self = Self(4);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceType.html>"]
pub struct PhysicalDeviceType(pub(crate) i32);
impl PhysicalDeviceType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const OTHER: Self = Self(0);
    pub const INTEGRATED_GPU: Self = Self(1);
    pub const DISCRETE_GPU: Self = Self(2);
    pub const VIRTUAL_GPU: Self = Self(3);
    pub const CPU: Self = Self(4);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputRate.html>"]
pub struct VertexInputRate(pub(crate) i32);
impl VertexInputRate {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VERTEX: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFormat.html>"]
pub struct Format(pub(crate) i32);
impl Format {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UNDEFINED: Self = Self(0);
    pub const R4G4_UNORM_PACK8: Self = Self(1);
    pub const R4G4B4A4_UNORM_PACK16: Self = Self(2);
    pub const B4G4R4A4_UNORM_PACK16: Self = Self(3);
    pub const R5G6B5_UNORM_PACK16: Self = Self(4);
    pub const B5G6R5_UNORM_PACK16: Self = Self(5);
    pub const R5G5B5A1_UNORM_PACK16: Self = Self(6);
    pub const B5G5R5A1_UNORM_PACK16: Self = Self(7);
    pub const A1R5G5B5_UNORM_PACK16: Self = Self(8);
    pub const R8_UNORM: Self = Self(9);
    pub const R8_SNORM: Self = Self(10);
    pub const R8_USCALED: Self = Self(11);
    pub const R8_SSCALED: Self = Self(12);
    pub const R8_UINT: Self = Self(13);
    pub const R8_SINT: Self = Self(14);
    pub const R8_SRGB: Self = Self(15);
    pub const R8G8_UNORM: Self = Self(16);
    pub const R8G8_SNORM: Self = Self(17);
    pub const R8G8_USCALED: Self = Self(18);
    pub const R8G8_SSCALED: Self = Self(19);
    pub const R8G8_UINT: Self = Self(20);
    pub const R8G8_SINT: Self = Self(21);
    pub const R8G8_SRGB: Self = Self(22);
    pub const R8G8B8_UNORM: Self = Self(23);
    pub const R8G8B8_SNORM: Self = Self(24);
    pub const R8G8B8_USCALED: Self = Self(25);
    pub const R8G8B8_SSCALED: Self = Self(26);
    pub const R8G8B8_UINT: Self = Self(27);
    pub const R8G8B8_SINT: Self = Self(28);
    pub const R8G8B8_SRGB: Self = Self(29);
    pub const B8G8R8_UNORM: Self = Self(30);
    pub const B8G8R8_SNORM: Self = Self(31);
    pub const B8G8R8_USCALED: Self = Self(32);
    pub const B8G8R8_SSCALED: Self = Self(33);
    pub const B8G8R8_UINT: Self = Self(34);
    pub const B8G8R8_SINT: Self = Self(35);
    pub const B8G8R8_SRGB: Self = Self(36);
    pub const R8G8B8A8_UNORM: Self = Self(37);
    pub const R8G8B8A8_SNORM: Self = Self(38);
    pub const R8G8B8A8_USCALED: Self = Self(39);
    pub const R8G8B8A8_SSCALED: Self = Self(40);
    pub const R8G8B8A8_UINT: Self = Self(41);
    pub const R8G8B8A8_SINT: Self = Self(42);
    pub const R8G8B8A8_SRGB: Self = Self(43);
    pub const B8G8R8A8_UNORM: Self = Self(44);
    pub const B8G8R8A8_SNORM: Self = Self(45);
    pub const B8G8R8A8_USCALED: Self = Self(46);
    pub const B8G8R8A8_SSCALED: Self = Self(47);
    pub const B8G8R8A8_UINT: Self = Self(48);
    pub const B8G8R8A8_SINT: Self = Self(49);
    pub const B8G8R8A8_SRGB: Self = Self(50);
    pub const A8B8G8R8_UNORM_PACK32: Self = Self(51);
    pub const A8B8G8R8_SNORM_PACK32: Self = Self(52);
    pub const A8B8G8R8_USCALED_PACK32: Self = Self(53);
    pub const A8B8G8R8_SSCALED_PACK32: Self = Self(54);
    pub const A8B8G8R8_UINT_PACK32: Self = Self(55);
    pub const A8B8G8R8_SINT_PACK32: Self = Self(56);
    pub const A8B8G8R8_SRGB_PACK32: Self = Self(57);
    pub const A2R10G10B10_UNORM_PACK32: Self = Self(58);
    pub const A2R10G10B10_SNORM_PACK32: Self = Self(59);
    pub const A2R10G10B10_USCALED_PACK32: Self = Self(60);
    pub const A2R10G10B10_SSCALED_PACK32: Self = Self(61);
    pub const A2R10G10B10_UINT_PACK32: Self = Self(62);
    pub const A2R10G10B10_SINT_PACK32: Self = Self(63);
    pub const A2B10G10R10_UNORM_PACK32: Self = Self(64);
    pub const A2B10G10R10_SNORM_PACK32: Self = Self(65);
    pub const A2B10G10R10_USCALED_PACK32: Self = Self(66);
    pub const A2B10G10R10_SSCALED_PACK32: Self = Self(67);
    pub const A2B10G10R10_UINT_PACK32: Self = Self(68);
    pub const A2B10G10R10_SINT_PACK32: Self = Self(69);
    pub const R16_UNORM: Self = Self(70);
    pub const R16_SNORM: Self = Self(71);
    pub const R16_USCALED: Self = Self(72);
    pub const R16_SSCALED: Self = Self(73);
    pub const R16_UINT: Self = Self(74);
    pub const R16_SINT: Self = Self(75);
    pub const R16_SFLOAT: Self = Self(76);
    pub const R16G16_UNORM: Self = Self(77);
    pub const R16G16_SNORM: Self = Self(78);
    pub const R16G16_USCALED: Self = Self(79);
    pub const R16G16_SSCALED: Self = Self(80);
    pub const R16G16_UINT: Self = Self(81);
    pub const R16G16_SINT: Self = Self(82);
    pub const R16G16_SFLOAT: Self = Self(83);
    pub const R16G16B16_UNORM: Self = Self(84);
    pub const R16G16B16_SNORM: Self = Self(85);
    pub const R16G16B16_USCALED: Self = Self(86);
    pub const R16G16B16_SSCALED: Self = Self(87);
    pub const R16G16B16_UINT: Self = Self(88);
    pub const R16G16B16_SINT: Self = Self(89);
    pub const R16G16B16_SFLOAT: Self = Self(90);
    pub const R16G16B16A16_UNORM: Self = Self(91);
    pub const R16G16B16A16_SNORM: Self = Self(92);
    pub const R16G16B16A16_USCALED: Self = Self(93);
    pub const R16G16B16A16_SSCALED: Self = Self(94);
    pub const R16G16B16A16_UINT: Self = Self(95);
    pub const R16G16B16A16_SINT: Self = Self(96);
    pub const R16G16B16A16_SFLOAT: Self = Self(97);
    pub const R32_UINT: Self = Self(98);
    pub const R32_SINT: Self = Self(99);
    pub const R32_SFLOAT: Self = Self(100);
    pub const R32G32_UINT: Self = Self(101);
    pub const R32G32_SINT: Self = Self(102);
    pub const R32G32_SFLOAT: Self = Self(103);
    pub const R32G32B32_UINT: Self = Self(104);
    pub const R32G32B32_SINT: Self = Self(105);
    pub const R32G32B32_SFLOAT: Self = Self(106);
    pub const R32G32B32A32_UINT: Self = Self(107);
    pub const R32G32B32A32_SINT: Self = Self(108);
    pub const R32G32B32A32_SFLOAT: Self = Self(109);
    pub const R64_UINT: Self = Self(110);
    pub const R64_SINT: Self = Self(111);
    pub const R64_SFLOAT: Self = Self(112);
    pub const R64G64_UINT: Self = Self(113);
    pub const R64G64_SINT: Self = Self(114);
    pub const R64G64_SFLOAT: Self = Self(115);
    pub const R64G64B64_UINT: Self = Self(116);
    pub const R64G64B64_SINT: Self = Self(117);
    pub const R64G64B64_SFLOAT: Self = Self(118);
    pub const R64G64B64A64_UINT: Self = Self(119);
    pub const R64G64B64A64_SINT: Self = Self(120);
    pub const R64G64B64A64_SFLOAT: Self = Self(121);
    pub const B10G11R11_UFLOAT_PACK32: Self = Self(122);
    pub const E5B9G9R9_UFLOAT_PACK32: Self = Self(123);
    pub const D16_UNORM: Self = Self(124);
    pub const X8_D24_UNORM_PACK32: Self = Self(125);
    pub const D32_SFLOAT: Self = Self(126);
    pub const S8_UINT: Self = Self(127);
    pub const D16_UNORM_S8_UINT: Self = Self(128);
    pub const D24_UNORM_S8_UINT: Self = Self(129);
    pub const D32_SFLOAT_S8_UINT: Self = Self(130);
    pub const BC1_RGB_UNORM_BLOCK: Self = Self(131);
    pub const BC1_RGB_SRGB_BLOCK: Self = Self(132);
    pub const BC1_RGBA_UNORM_BLOCK: Self = Self(133);
    pub const BC1_RGBA_SRGB_BLOCK: Self = Self(134);
    pub const BC2_UNORM_BLOCK: Self = Self(135);
    pub const BC2_SRGB_BLOCK: Self = Self(136);
    pub const BC3_UNORM_BLOCK: Self = Self(137);
    pub const BC3_SRGB_BLOCK: Self = Self(138);
    pub const BC4_UNORM_BLOCK: Self = Self(139);
    pub const BC4_SNORM_BLOCK: Self = Self(140);
    pub const BC5_UNORM_BLOCK: Self = Self(141);
    pub const BC5_SNORM_BLOCK: Self = Self(142);
    pub const BC6H_UFLOAT_BLOCK: Self = Self(143);
    pub const BC6H_SFLOAT_BLOCK: Self = Self(144);
    pub const BC7_UNORM_BLOCK: Self = Self(145);
    pub const BC7_SRGB_BLOCK: Self = Self(146);
    pub const ETC2_R8G8B8_UNORM_BLOCK: Self = Self(147);
    pub const ETC2_R8G8B8_SRGB_BLOCK: Self = Self(148);
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self(149);
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self(150);
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self(151);
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self(152);
    pub const EAC_R11_UNORM_BLOCK: Self = Self(153);
    pub const EAC_R11_SNORM_BLOCK: Self = Self(154);
    pub const EAC_R11G11_UNORM_BLOCK: Self = Self(155);
    pub const EAC_R11G11_SNORM_BLOCK: Self = Self(156);
    pub const ASTC_4X4_UNORM_BLOCK: Self = Self(157);
    pub const ASTC_4X4_SRGB_BLOCK: Self = Self(158);
    pub const ASTC_5X4_UNORM_BLOCK: Self = Self(159);
    pub const ASTC_5X4_SRGB_BLOCK: Self = Self(160);
    pub const ASTC_5X5_UNORM_BLOCK: Self = Self(161);
    pub const ASTC_5X5_SRGB_BLOCK: Self = Self(162);
    pub const ASTC_6X5_UNORM_BLOCK: Self = Self(163);
    pub const ASTC_6X5_SRGB_BLOCK: Self = Self(164);
    pub const ASTC_6X6_UNORM_BLOCK: Self = Self(165);
    pub const ASTC_6X6_SRGB_BLOCK: Self = Self(166);
    pub const ASTC_8X5_UNORM_BLOCK: Self = Self(167);
    pub const ASTC_8X5_SRGB_BLOCK: Self = Self(168);
    pub const ASTC_8X6_UNORM_BLOCK: Self = Self(169);
    pub const ASTC_8X6_SRGB_BLOCK: Self = Self(170);
    pub const ASTC_8X8_UNORM_BLOCK: Self = Self(171);
    pub const ASTC_8X8_SRGB_BLOCK: Self = Self(172);
    pub const ASTC_10X5_UNORM_BLOCK: Self = Self(173);
    pub const ASTC_10X5_SRGB_BLOCK: Self = Self(174);
    pub const ASTC_10X6_UNORM_BLOCK: Self = Self(175);
    pub const ASTC_10X6_SRGB_BLOCK: Self = Self(176);
    pub const ASTC_10X8_UNORM_BLOCK: Self = Self(177);
    pub const ASTC_10X8_SRGB_BLOCK: Self = Self(178);
    pub const ASTC_10X10_UNORM_BLOCK: Self = Self(179);
    pub const ASTC_10X10_SRGB_BLOCK: Self = Self(180);
    pub const ASTC_12X10_UNORM_BLOCK: Self = Self(181);
    pub const ASTC_12X10_SRGB_BLOCK: Self = Self(182);
    pub const ASTC_12X12_UNORM_BLOCK: Self = Self(183);
    pub const ASTC_12X12_SRGB_BLOCK: Self = Self(184);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkStructureType.html>"]
pub struct StructureType(pub(crate) i32);
impl StructureType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const APPLICATION_INFO: Self = Self(0);
    pub const INSTANCE_CREATE_INFO: Self = Self(1);
    pub const DEVICE_QUEUE_CREATE_INFO: Self = Self(2);
    pub const DEVICE_CREATE_INFO: Self = Self(3);
    pub const SUBMIT_INFO: Self = Self(4);
    pub const MEMORY_ALLOCATE_INFO: Self = Self(5);
    pub const MAPPED_MEMORY_RANGE: Self = Self(6);
    pub const BIND_SPARSE_INFO: Self = Self(7);
    pub const FENCE_CREATE_INFO: Self = Self(8);
    pub const SEMAPHORE_CREATE_INFO: Self = Self(9);
    pub const EVENT_CREATE_INFO: Self = Self(10);
    pub const QUERY_POOL_CREATE_INFO: Self = Self(11);
    pub const BUFFER_CREATE_INFO: Self = Self(12);
    pub const BUFFER_VIEW_CREATE_INFO: Self = Self(13);
    pub const IMAGE_CREATE_INFO: Self = Self(14);
    pub const IMAGE_VIEW_CREATE_INFO: Self = Self(15);
    pub const SHADER_MODULE_CREATE_INFO: Self = Self(16);
    pub const PIPELINE_CACHE_CREATE_INFO: Self = Self(17);
    pub const PIPELINE_SHADER_STAGE_CREATE_INFO: Self = Self(18);
    pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: Self = Self(19);
    pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: Self = Self(20);
    pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: Self = Self(21);
    pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: Self = Self(22);
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: Self = Self(23);
    pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: Self = Self(24);
    pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: Self = Self(25);
    pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: Self = Self(26);
    pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: Self = Self(27);
    pub const GRAPHICS_PIPELINE_CREATE_INFO: Self = Self(28);
    pub const COMPUTE_PIPELINE_CREATE_INFO: Self = Self(29);
    pub const PIPELINE_LAYOUT_CREATE_INFO: Self = Self(30);
    pub const SAMPLER_CREATE_INFO: Self = Self(31);
    pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: Self = Self(32);
    pub const DESCRIPTOR_POOL_CREATE_INFO: Self = Self(33);
    pub const DESCRIPTOR_SET_ALLOCATE_INFO: Self = Self(34);
    pub const WRITE_DESCRIPTOR_SET: Self = Self(35);
    pub const COPY_DESCRIPTOR_SET: Self = Self(36);
    pub const FRAMEBUFFER_CREATE_INFO: Self = Self(37);
    pub const RENDER_PASS_CREATE_INFO: Self = Self(38);
    pub const COMMAND_POOL_CREATE_INFO: Self = Self(39);
    pub const COMMAND_BUFFER_ALLOCATE_INFO: Self = Self(40);
    pub const COMMAND_BUFFER_INHERITANCE_INFO: Self = Self(41);
    pub const COMMAND_BUFFER_BEGIN_INFO: Self = Self(42);
    pub const RENDER_PASS_BEGIN_INFO: Self = Self(43);
    pub const BUFFER_MEMORY_BARRIER: Self = Self(44);
    pub const IMAGE_MEMORY_BARRIER: Self = Self(45);
    pub const MEMORY_BARRIER: Self = Self(46);
    #[doc = "Reserved for internal use by the loader, layers, and ICDs"]
    pub const LOADER_INSTANCE_CREATE_INFO: Self = Self(47);
    #[doc = "Reserved for internal use by the loader, layers, and ICDs"]
    pub const LOADER_DEVICE_CREATE_INFO: Self = Self(48);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassContents.html>"]
pub struct SubpassContents(pub(crate) i32);
impl SubpassContents {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const INLINE: Self = Self(0);
    pub const SECONDARY_COMMAND_BUFFERS: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[must_use]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkResult.html>"]
pub struct Result(pub(crate) i32);
impl Result {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    #[doc = "Command completed successfully"]
    pub const SUCCESS: Self = Self(0);
    #[doc = "A fence or query has not yet completed"]
    pub const NOT_READY: Self = Self(1);
    #[doc = "A wait operation has not completed in the specified time"]
    pub const TIMEOUT: Self = Self(2);
    #[doc = "An event is signaled"]
    pub const EVENT_SET: Self = Self(3);
    #[doc = "An event is unsignaled"]
    pub const EVENT_RESET: Self = Self(4);
    #[doc = "A return array was too small for the result"]
    pub const INCOMPLETE: Self = Self(5);
    #[doc = "A host memory allocation has failed"]
    pub const ERROR_OUT_OF_HOST_MEMORY: Self = Self(-1);
    #[doc = "A device memory allocation has failed"]
    pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = Self(-2);
    #[doc = "Initialization of an object has failed"]
    pub const ERROR_INITIALIZATION_FAILED: Self = Self(-3);
    #[doc = "The logical device has been lost. See <https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#devsandqueues-lost-device>"]
    pub const ERROR_DEVICE_LOST: Self = Self(-4);
    #[doc = "Mapping of a memory object has failed"]
    pub const ERROR_MEMORY_MAP_FAILED: Self = Self(-5);
    #[doc = "Layer specified does not exist"]
    pub const ERROR_LAYER_NOT_PRESENT: Self = Self(-6);
    #[doc = "Extension specified does not exist"]
    pub const ERROR_EXTENSION_NOT_PRESENT: Self = Self(-7);
    #[doc = "Requested feature is not available on this device"]
    pub const ERROR_FEATURE_NOT_PRESENT: Self = Self(-8);
    #[doc = "Unable to find a Vulkan driver"]
    pub const ERROR_INCOMPATIBLE_DRIVER: Self = Self(-9);
    #[doc = "Too many objects of the type have already been created"]
    pub const ERROR_TOO_MANY_OBJECTS: Self = Self(-10);
    #[doc = "Requested format is not supported on this device"]
    pub const ERROR_FORMAT_NOT_SUPPORTED: Self = Self(-11);
    #[doc = "A requested pool allocation has failed due to fragmentation of the pool's memory"]
    pub const ERROR_FRAGMENTED_POOL: Self = Self(-12);
    #[doc = "An unknown error has occurred, due to an implementation or application bug"]
    pub const ERROR_UNKNOWN: Self = Self(-13);
}
#[cfg(feature = "std")]
impl std::error::Error for Result {}
impl fmt::Display for Result {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match * self { Self :: SUCCESS => Some ("Command completed successfully") , Self :: NOT_READY => Some ("A fence or query has not yet completed") , Self :: TIMEOUT => Some ("A wait operation has not completed in the specified time") , Self :: EVENT_SET => Some ("An event is signaled") , Self :: EVENT_RESET => Some ("An event is unsignaled") , Self :: INCOMPLETE => Some ("A return array was too small for the result") , Self :: ERROR_OUT_OF_HOST_MEMORY => Some ("A host memory allocation has failed") , Self :: ERROR_OUT_OF_DEVICE_MEMORY => Some ("A device memory allocation has failed") , Self :: ERROR_INITIALIZATION_FAILED => Some ("Initialization of an object has failed") , Self :: ERROR_DEVICE_LOST => Some ("The logical device has been lost. See <https://registry.khronos.org/vulkan/specs/latest/html/vkspec.html#devsandqueues-lost-device>") , Self :: ERROR_MEMORY_MAP_FAILED => Some ("Mapping of a memory object has failed") , Self :: ERROR_LAYER_NOT_PRESENT => Some ("Layer specified does not exist") , Self :: ERROR_EXTENSION_NOT_PRESENT => Some ("Extension specified does not exist") , Self :: ERROR_FEATURE_NOT_PRESENT => Some ("Requested feature is not available on this device") , Self :: ERROR_INCOMPATIBLE_DRIVER => Some ("Unable to find a Vulkan driver") , Self :: ERROR_TOO_MANY_OBJECTS => Some ("Too many objects of the type have already been created") , Self :: ERROR_FORMAT_NOT_SUPPORTED => Some ("Requested format is not supported on this device") , Self :: ERROR_FRAGMENTED_POOL => Some ("A requested pool allocation has failed due to fragmentation of the pool's memory") , Self :: ERROR_UNKNOWN => Some ("An unknown error has occurred, due to an implementation or application bug") , _ => None , } ;
        if let Some(x) = name {
            fmt.write_str(x)
        } else {
            <Self as fmt::Debug>::fmt(self, fmt)
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDynamicState.html>"]
pub struct DynamicState(pub(crate) i32);
impl DynamicState {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VIEWPORT: Self = Self(0);
    pub const SCISSOR: Self = Self(1);
    pub const LINE_WIDTH: Self = Self(2);
    pub const DEPTH_BIAS: Self = Self(3);
    pub const BLEND_CONSTANTS: Self = Self(4);
    pub const DEPTH_BOUNDS: Self = Self(5);
    pub const STENCIL_COMPARE_MASK: Self = Self(6);
    pub const STENCIL_WRITE_MASK: Self = Self(7);
    pub const STENCIL_REFERENCE: Self = Self(8);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateType.html>"]
pub struct DescriptorUpdateTemplateType(pub(crate) i32);
impl DescriptorUpdateTemplateType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    #[doc = "Create descriptor update template for descriptor set updates"]
    pub const DESCRIPTOR_SET: Self = Self(0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkObjectType.html>"]
pub struct ObjectType(pub(crate) i32);
impl ObjectType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UNKNOWN: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
    pub const PHYSICAL_DEVICE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const QUEUE: Self = Self(4);
    pub const SEMAPHORE: Self = Self(5);
    pub const COMMAND_BUFFER: Self = Self(6);
    pub const FENCE: Self = Self(7);
    pub const DEVICE_MEMORY: Self = Self(8);
    pub const BUFFER: Self = Self(9);
    pub const IMAGE: Self = Self(10);
    pub const EVENT: Self = Self(11);
    pub const QUERY_POOL: Self = Self(12);
    pub const BUFFER_VIEW: Self = Self(13);
    pub const IMAGE_VIEW: Self = Self(14);
    pub const SHADER_MODULE: Self = Self(15);
    pub const PIPELINE_CACHE: Self = Self(16);
    pub const PIPELINE_LAYOUT: Self = Self(17);
    pub const RENDER_PASS: Self = Self(18);
    pub const PIPELINE: Self = Self(19);
    pub const DESCRIPTOR_SET_LAYOUT: Self = Self(20);
    pub const SAMPLER: Self = Self(21);
    pub const DESCRIPTOR_POOL: Self = Self(22);
    pub const DESCRIPTOR_SET: Self = Self(23);
    pub const FRAMEBUFFER: Self = Self(24);
    pub const COMMAND_POOL: Self = Self(25);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreType.html>"]
pub struct SemaphoreType(pub(crate) i32);
impl SemaphoreType {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BINARY: Self = Self(0);
    pub const TIMELINE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentModeKHR.html>"]
pub struct PresentModeKHR(pub(crate) i32);
impl PresentModeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const IMMEDIATE: Self = Self(0);
    pub const MAILBOX: Self = Self(1);
    pub const FIFO: Self = Self(2);
    pub const FIFO_RELAXED: Self = Self(3);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkColorSpaceKHR.html>"]
pub struct ColorSpaceKHR(pub(crate) i32);
impl ColorSpaceKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const SRGB_NONLINEAR: Self = Self(0);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkViewportCoordinateSwizzleNV.html>"]
pub struct ViewportCoordinateSwizzleNV(pub(crate) i32);
impl ViewportCoordinateSwizzleNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const POSITIVE_X: Self = Self(0);
    pub const NEGATIVE_X: Self = Self(1);
    pub const POSITIVE_Y: Self = Self(2);
    pub const NEGATIVE_Y: Self = Self(3);
    pub const POSITIVE_Z: Self = Self(4);
    pub const NEGATIVE_Z: Self = Self(5);
    pub const POSITIVE_W: Self = Self(6);
    pub const NEGATIVE_W: Self = Self(7);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPointClippingBehavior.html>"]
pub struct PointClippingBehavior(pub(crate) i32);
impl PointClippingBehavior {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const ALL_CLIP_PLANES: Self = Self(0);
    pub const USER_CLIP_PLANES_ONLY: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerReductionMode.html>"]
pub struct SamplerReductionMode(pub(crate) i32);
impl SamplerReductionMode {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const WEIGHTED_AVERAGE: Self = Self(0);
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTessellationDomainOrigin.html>"]
pub struct TessellationDomainOrigin(pub(crate) i32);
impl TessellationDomainOrigin {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UPPER_LEFT: Self = Self(0);
    pub const LOWER_LEFT: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrModelConversion.html>"]
pub struct SamplerYcbcrModelConversion(pub(crate) i32);
impl SamplerYcbcrModelConversion {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const RGB_IDENTITY: Self = Self(0);
    #[doc = "just range expansion"]
    pub const YCBCR_IDENTITY: Self = Self(1);
    #[doc = "aka HD YUV"]
    pub const YCBCR_709: Self = Self(2);
    #[doc = "aka SD YUV"]
    pub const YCBCR_601: Self = Self(3);
    #[doc = "aka UHD YUV"]
    pub const YCBCR_2020: Self = Self(4);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrRange.html>"]
pub struct SamplerYcbcrRange(pub(crate) i32);
impl SamplerYcbcrRange {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    #[doc = "Luma 0..1 maps to 0..255, chroma -0.5..0.5 to 1..255 (clamped)"]
    pub const ITU_FULL: Self = Self(0);
    #[doc = "Luma 0..1 maps to 16..235, chroma -0.5..0.5 to 16..240"]
    pub const ITU_NARROW: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkChromaLocation.html>"]
pub struct ChromaLocation(pub(crate) i32);
impl ChromaLocation {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COSITED_EVEN: Self = Self(0);
    pub const MIDPOINT: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBlendOverlapEXT.html>"]
pub struct BlendOverlapEXT(pub(crate) i32);
impl BlendOverlapEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const UNCORRELATED: Self = Self(0);
    pub const DISJOINT: Self = Self(1);
    pub const CONJOINT: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCoverageModulationModeNV.html>"]
pub struct CoverageModulationModeNV(pub(crate) i32);
impl CoverageModulationModeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const NONE: Self = Self(0);
    pub const RGB: Self = Self(1);
    pub const ALPHA: Self = Self(2);
    pub const RGBA: Self = Self(3);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCoverageReductionModeNV.html>"]
pub struct CoverageReductionModeNV(pub(crate) i32);
impl CoverageReductionModeNV {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const MERGE: Self = Self(0);
    pub const TRUNCATE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueGlobalPriority.html>"]
pub struct QueueGlobalPriority(pub(crate) i32);
impl QueueGlobalPriority {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const LOW: Self = Self(128);
    pub const MEDIUM: Self = Self(256);
    pub const HIGH: Self = Self(512);
    pub const REALTIME: Self = Self(1_024);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkConservativeRasterizationModeEXT.html>"]
pub struct ConservativeRasterizationModeEXT(pub(crate) i32);
impl ConservativeRasterizationModeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DISABLED: Self = Self(0);
    pub const OVERESTIMATE: Self = Self(1);
    pub const UNDERESTIMATE: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVendorId.html>"]
pub struct VendorId(pub(crate) i32);
impl VendorId {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    #[doc = "Khronos vendor ID"]
    pub const KHRONOS: Self = Self(0x10000);
    #[doc = "Vivante vendor ID"]
    pub const VIV: Self = Self(0x10001);
    #[doc = "VeriSilicon vendor ID"]
    pub const VSI: Self = Self(0x10002);
    #[doc = "Kazan Software Renderer"]
    pub const KAZAN: Self = Self(0x10003);
    #[doc = "Codeplay Software Ltd. vendor ID"]
    pub const CODEPLAY: Self = Self(0x10004);
    #[doc = "Mesa vendor ID"]
    pub const MESA: Self = Self(0x10005);
    #[doc = "PoCL vendor ID"]
    pub const POCL: Self = Self(0x10006);
    #[doc = "Mobileye vendor ID"]
    pub const MOBILEYE: Self = Self(0x10007);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDriverId.html>"]
pub struct DriverId(pub(crate) i32);
impl DriverId {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    #[doc = "Advanced Micro Devices, Inc."]
    pub const AMD_PROPRIETARY: Self = Self(1);
    #[doc = "Advanced Micro Devices, Inc."]
    pub const AMD_OPEN_SOURCE: Self = Self(2);
    #[doc = "Mesa open source project"]
    pub const MESA_RADV: Self = Self(3);
    #[doc = "NVIDIA Corporation"]
    pub const NVIDIA_PROPRIETARY: Self = Self(4);
    #[doc = "Intel Corporation"]
    pub const INTEL_PROPRIETARY_WINDOWS: Self = Self(5);
    #[doc = "Intel Corporation"]
    pub const INTEL_OPEN_SOURCE_MESA: Self = Self(6);
    #[doc = "Imagination Technologies"]
    pub const IMAGINATION_PROPRIETARY: Self = Self(7);
    #[doc = "Qualcomm Technologies, Inc."]
    pub const QUALCOMM_PROPRIETARY: Self = Self(8);
    #[doc = "Arm Limited"]
    pub const ARM_PROPRIETARY: Self = Self(9);
    #[doc = "Google LLC"]
    pub const GOOGLE_SWIFTSHADER: Self = Self(10);
    #[doc = "Google LLC"]
    pub const GGP_PROPRIETARY: Self = Self(11);
    #[doc = "Broadcom Inc."]
    pub const BROADCOM_PROPRIETARY: Self = Self(12);
    #[doc = "Mesa"]
    pub const MESA_LLVMPIPE: Self = Self(13);
    #[doc = "MoltenVK"]
    pub const MOLTENVK: Self = Self(14);
    #[doc = "Core Avionics & Industrial Inc."]
    pub const COREAVI_PROPRIETARY: Self = Self(15);
    #[doc = "Juice Technologies, Inc."]
    pub const JUICE_PROPRIETARY: Self = Self(16);
    #[doc = "Verisilicon, Inc."]
    pub const VERISILICON_PROPRIETARY: Self = Self(17);
    #[doc = "Mesa open source project"]
    pub const MESA_TURNIP: Self = Self(18);
    #[doc = "Mesa open source project"]
    pub const MESA_V3DV: Self = Self(19);
    #[doc = "Mesa open source project"]
    pub const MESA_PANVK: Self = Self(20);
    #[doc = "Samsung Electronics Co., Ltd."]
    pub const SAMSUNG_PROPRIETARY: Self = Self(21);
    #[doc = "Mesa open source project"]
    pub const MESA_VENUS: Self = Self(22);
    #[doc = "Mesa open source project"]
    pub const MESA_DOZEN: Self = Self(23);
    #[doc = "Mesa open source project"]
    pub const MESA_NVK: Self = Self(24);
    #[doc = "Imagination Technologies"]
    pub const IMAGINATION_OPEN_SOURCE_MESA: Self = Self(25);
    #[doc = "Mesa open source project"]
    pub const MESA_HONEYKRISP: Self = Self(26);
    #[doc = "Vulkan SC Emulation on Vulkan"]
    pub const VULKAN_SC_EMULATION_ON_VULKAN: Self = Self(27);
    #[doc = "Mesa open source project"]
    pub const MESA_KOSMICKRISP: Self = Self(28);
    #[doc = "Mesa open source project"]
    pub const MESA_GFXSTREAM: Self = Self(29);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyAccelerationStructureModeKHR.html>"]
pub struct CopyAccelerationStructureModeKHR(pub(crate) i32);
impl CopyAccelerationStructureModeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const CLONE: Self = Self(0);
    pub const COMPACT: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureModeKHR.html>"]
pub struct BuildAccelerationStructureModeKHR(pub(crate) i32);
impl BuildAccelerationStructureModeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BUILD: Self = Self(0);
    pub const UPDATE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureTypeKHR.html>"]
pub struct AccelerationStructureTypeKHR(pub(crate) i32);
impl AccelerationStructureTypeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const TOP_LEVEL: Self = Self(0);
    pub const BOTTOM_LEVEL: Self = Self(1);
    pub const GENERIC: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryTypeKHR.html>"]
pub struct GeometryTypeKHR(pub(crate) i32);
impl GeometryTypeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const TRIANGLES: Self = Self(0);
    pub const AABBS: Self = Self(1);
    pub const INSTANCES: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureBuildTypeKHR.html>"]
pub struct AccelerationStructureBuildTypeKHR(pub(crate) i32);
impl AccelerationStructureBuildTypeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const HOST: Self = Self(0);
    pub const DEVICE: Self = Self(1);
    pub const HOST_OR_DEVICE: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingShaderGroupTypeKHR.html>"]
pub struct RayTracingShaderGroupTypeKHR(pub(crate) i32);
impl RayTracingShaderGroupTypeKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const GENERAL: Self = Self(0);
    pub const TRIANGLES_HIT_GROUP: Self = Self(1);
    pub const PROCEDURAL_HIT_GROUP: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCompatibilityKHR.html>"]
pub struct AccelerationStructureCompatibilityKHR(pub(crate) i32);
impl AccelerationStructureCompatibilityKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const COMPATIBLE: Self = Self(0);
    pub const INCOMPATIBLE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderGroupShaderKHR.html>"]
pub struct ShaderGroupShaderKHR(pub(crate) i32);
impl ShaderGroupShaderKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const GENERAL: Self = Self(0);
    pub const CLOSEST_HIT: Self = Self(1);
    pub const ANY_HIT: Self = Self(2);
    pub const INTERSECTION: Self = Self(3);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderFloatControlsIndependence.html>"]
pub struct ShaderFloatControlsIndependence(pub(crate) i32);
impl ShaderFloatControlsIndependence {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const TYPE_32_ONLY: Self = Self(0);
    pub const ALL: Self = Self(1);
    pub const NONE: Self = Self(2);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkLineRasterizationMode.html>"]
pub struct LineRasterizationMode(pub(crate) i32);
impl LineRasterizationMode {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEFAULT: Self = Self(0);
    pub const RECTANGULAR: Self = Self(1);
    pub const BRESENHAM: Self = Self(2);
    pub const RECTANGULAR_SMOOTH: Self = Self(3);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkProvokingVertexModeEXT.html>"]
pub struct ProvokingVertexModeEXT(pub(crate) i32);
impl ProvokingVertexModeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const FIRST_VERTEX: Self = Self(0);
    pub const LAST_VERTEX: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessBufferBehavior.html>"]
pub struct PipelineRobustnessBufferBehavior(pub(crate) i32);
impl PipelineRobustnessBufferBehavior {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEVICE_DEFAULT: Self = Self(0);
    pub const DISABLED: Self = Self(1);
    pub const ROBUST_BUFFER_ACCESS: Self = Self(2);
    pub const ROBUST_BUFFER_ACCESS_2: Self = Self(3);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessImageBehavior.html>"]
pub struct PipelineRobustnessImageBehavior(pub(crate) i32);
impl PipelineRobustnessImageBehavior {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const DEVICE_DEFAULT: Self = Self(0);
    pub const DISABLED: Self = Self(1);
    pub const ROBUST_IMAGE_ACCESS: Self = Self(2);
    pub const ROBUST_IMAGE_ACCESS_2: Self = Self(3);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCodeTypeEXT.html>"]
pub struct ShaderCodeTypeEXT(pub(crate) i32);
impl ShaderCodeTypeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const BINARY: Self = Self(0);
    pub const SPIRV: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLayeredApiKHR.html>"]
pub struct PhysicalDeviceLayeredApiKHR(pub(crate) i32);
impl PhysicalDeviceLayeredApiKHR {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VULKAN: Self = Self(0);
    pub const D3D12: Self = Self(1);
    pub const METAL: Self = Self(2);
    pub const OPENGL: Self = Self(3);
    pub const OPENGLES: Self = Self(4);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDepthClampModeEXT.html>"]
pub struct DepthClampModeEXT(pub(crate) i32);
impl DepthClampModeEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const VIEWPORT_RANGE: Self = Self(0);
    pub const USER_DEFINED_RANGE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorMappingSourceEXT.html>"]
pub struct DescriptorMappingSourceEXT(pub(crate) i32);
impl DescriptorMappingSourceEXT {
    #[inline]
    pub const fn from_raw(x: i32) -> Self {
        Self(x)
    }
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
    pub const HEAP_WITH_CONSTANT_OFFSET: Self = Self(0);
    pub const HEAP_WITH_PUSH_INDEX: Self = Self(1);
    pub const HEAP_WITH_INDIRECT_INDEX: Self = Self(2);
    pub const HEAP_WITH_INDIRECT_INDEX_ARRAY: Self = Self(3);
    pub const RESOURCE_HEAP_DATA: Self = Self(4);
    pub const PUSH_DATA: Self = Self(5);
    pub const PUSH_ADDRESS: Self = Self(6);
    pub const INDIRECT_ADDRESS: Self = Self(7);
}
impl fmt::Debug for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::UNKNOWN => Some("UNKNOWN"),
            Self::INSTANCE => Some("INSTANCE"),
            Self::PHYSICAL_DEVICE => Some("PHYSICAL_DEVICE"),
            Self::DEVICE => Some("DEVICE"),
            Self::QUEUE => Some("QUEUE"),
            Self::SEMAPHORE => Some("SEMAPHORE"),
            Self::COMMAND_BUFFER => Some("COMMAND_BUFFER"),
            Self::FENCE => Some("FENCE"),
            Self::DEVICE_MEMORY => Some("DEVICE_MEMORY"),
            Self::BUFFER => Some("BUFFER"),
            Self::IMAGE => Some("IMAGE"),
            Self::EVENT => Some("EVENT"),
            Self::QUERY_POOL => Some("QUERY_POOL"),
            Self::BUFFER_VIEW => Some("BUFFER_VIEW"),
            Self::IMAGE_VIEW => Some("IMAGE_VIEW"),
            Self::SHADER_MODULE => Some("SHADER_MODULE"),
            Self::PIPELINE_CACHE => Some("PIPELINE_CACHE"),
            Self::PIPELINE_LAYOUT => Some("PIPELINE_LAYOUT"),
            Self::RENDER_PASS => Some("RENDER_PASS"),
            Self::PIPELINE => Some("PIPELINE"),
            Self::DESCRIPTOR_SET_LAYOUT => Some("DESCRIPTOR_SET_LAYOUT"),
            Self::SAMPLER => Some("SAMPLER"),
            Self::DESCRIPTOR_POOL => Some("DESCRIPTOR_POOL"),
            Self::DESCRIPTOR_SET => Some("DESCRIPTOR_SET"),
            Self::FRAMEBUFFER => Some("FRAMEBUFFER"),
            Self::COMMAND_POOL => Some("COMMAND_POOL"),
            Self::SURFACE_KHR => Some("SURFACE_KHR"),
            Self::SWAPCHAIN_KHR => Some("SWAPCHAIN_KHR"),
            Self::DISPLAY_KHR => Some("DISPLAY_KHR"),
            Self::DISPLAY_MODE_KHR => Some("DISPLAY_MODE_KHR"),
            Self::DEBUG_REPORT_CALLBACK_EXT => Some("DEBUG_REPORT_CALLBACK_EXT"),
            Self::VIDEO_SESSION_KHR => Some("VIDEO_SESSION_KHR"),
            Self::VIDEO_SESSION_PARAMETERS_KHR => Some("VIDEO_SESSION_PARAMETERS_KHR"),
            Self::CU_MODULE_NVX => Some("CU_MODULE_NVX"),
            Self::CU_FUNCTION_NVX => Some("CU_FUNCTION_NVX"),
            Self::DEBUG_UTILS_MESSENGER_EXT => Some("DEBUG_UTILS_MESSENGER_EXT"),
            Self::GPA_SESSION_AMD => Some("GPA_SESSION_AMD"),
            Self::ACCELERATION_STRUCTURE_KHR => Some("ACCELERATION_STRUCTURE_KHR"),
            Self::VALIDATION_CACHE_EXT => Some("VALIDATION_CACHE_EXT"),
            Self::ACCELERATION_STRUCTURE_NV => Some("ACCELERATION_STRUCTURE_NV"),
            Self::PERFORMANCE_CONFIGURATION_INTEL => Some("PERFORMANCE_CONFIGURATION_INTEL"),
            Self::DEFERRED_OPERATION_KHR => Some("DEFERRED_OPERATION_KHR"),
            Self::INDIRECT_COMMANDS_LAYOUT_NV => Some("INDIRECT_COMMANDS_LAYOUT_NV"),
            #[cfg(feature = "provisional")]
            Self::CUDA_MODULE_NV => Some("CUDA_MODULE_NV"),
            #[cfg(feature = "provisional")]
            Self::CUDA_FUNCTION_NV => Some("CUDA_FUNCTION_NV"),
            Self::BUFFER_COLLECTION_FUCHSIA => Some("BUFFER_COLLECTION_FUCHSIA"),
            Self::MICROMAP_EXT => Some("MICROMAP_EXT"),
            Self::TENSOR_ARM => Some("TENSOR_ARM"),
            Self::TENSOR_VIEW_ARM => Some("TENSOR_VIEW_ARM"),
            Self::OPTICAL_FLOW_SESSION_NV => Some("OPTICAL_FLOW_SESSION_NV"),
            Self::SHADER_EXT => Some("SHADER_EXT"),
            Self::PIPELINE_BINARY_KHR => Some("PIPELINE_BINARY_KHR"),
            Self::DATA_GRAPH_PIPELINE_SESSION_ARM => Some("DATA_GRAPH_PIPELINE_SESSION_ARM"),
            Self::EXTERNAL_COMPUTE_QUEUE_NV => Some("EXTERNAL_COMPUTE_QUEUE_NV"),
            Self::INDIRECT_COMMANDS_LAYOUT_EXT => Some("INDIRECT_COMMANDS_LAYOUT_EXT"),
            Self::INDIRECT_EXECUTION_SET_EXT => Some("INDIRECT_EXECUTION_SET_EXT"),
            Self::SHADER_INSTRUMENTATION_ARM => Some("SHADER_INSTRUMENTATION_ARM"),
            Self::DESCRIPTOR_UPDATE_TEMPLATE => Some("DESCRIPTOR_UPDATE_TEMPLATE"),
            Self::SAMPLER_YCBCR_CONVERSION => Some("SAMPLER_YCBCR_CONVERSION"),
            Self::PRIVATE_DATA_SLOT => Some("PRIVATE_DATA_SLOT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for Result {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::SUCCESS => Some("SUCCESS"),
            Self::NOT_READY => Some("NOT_READY"),
            Self::TIMEOUT => Some("TIMEOUT"),
            Self::EVENT_SET => Some("EVENT_SET"),
            Self::EVENT_RESET => Some("EVENT_RESET"),
            Self::INCOMPLETE => Some("INCOMPLETE"),
            Self::ERROR_OUT_OF_HOST_MEMORY => Some("ERROR_OUT_OF_HOST_MEMORY"),
            Self::ERROR_OUT_OF_DEVICE_MEMORY => Some("ERROR_OUT_OF_DEVICE_MEMORY"),
            Self::ERROR_INITIALIZATION_FAILED => Some("ERROR_INITIALIZATION_FAILED"),
            Self::ERROR_DEVICE_LOST => Some("ERROR_DEVICE_LOST"),
            Self::ERROR_MEMORY_MAP_FAILED => Some("ERROR_MEMORY_MAP_FAILED"),
            Self::ERROR_LAYER_NOT_PRESENT => Some("ERROR_LAYER_NOT_PRESENT"),
            Self::ERROR_EXTENSION_NOT_PRESENT => Some("ERROR_EXTENSION_NOT_PRESENT"),
            Self::ERROR_FEATURE_NOT_PRESENT => Some("ERROR_FEATURE_NOT_PRESENT"),
            Self::ERROR_INCOMPATIBLE_DRIVER => Some("ERROR_INCOMPATIBLE_DRIVER"),
            Self::ERROR_TOO_MANY_OBJECTS => Some("ERROR_TOO_MANY_OBJECTS"),
            Self::ERROR_FORMAT_NOT_SUPPORTED => Some("ERROR_FORMAT_NOT_SUPPORTED"),
            Self::ERROR_FRAGMENTED_POOL => Some("ERROR_FRAGMENTED_POOL"),
            Self::ERROR_UNKNOWN => Some("ERROR_UNKNOWN"),
            Self::ERROR_SURFACE_LOST_KHR => Some("ERROR_SURFACE_LOST_KHR"),
            Self::ERROR_NATIVE_WINDOW_IN_USE_KHR => Some("ERROR_NATIVE_WINDOW_IN_USE_KHR"),
            Self::SUBOPTIMAL_KHR => Some("SUBOPTIMAL_KHR"),
            Self::ERROR_OUT_OF_DATE_KHR => Some("ERROR_OUT_OF_DATE_KHR"),
            Self::ERROR_INCOMPATIBLE_DISPLAY_KHR => Some("ERROR_INCOMPATIBLE_DISPLAY_KHR"),
            Self::ERROR_INVALID_SHADER_NV => Some("ERROR_INVALID_SHADER_NV"),
            Self::ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR => {
                Some("ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR")
            }
            Self::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR => {
                Some("ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR")
            }
            Self::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR => {
                Some("ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR")
            }
            Self::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR => {
                Some("ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR")
            }
            Self::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR => {
                Some("ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR")
            }
            Self::ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR => {
                Some("ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR")
            }
            Self::ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT => {
                Some("ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT")
            }
            Self::ERROR_PRESENT_TIMING_QUEUE_FULL_EXT => {
                Some("ERROR_PRESENT_TIMING_QUEUE_FULL_EXT")
            }
            Self::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => {
                Some("ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT")
            }
            Self::THREAD_IDLE_KHR => Some("THREAD_IDLE_KHR"),
            Self::THREAD_DONE_KHR => Some("THREAD_DONE_KHR"),
            Self::OPERATION_DEFERRED_KHR => Some("OPERATION_DEFERRED_KHR"),
            Self::OPERATION_NOT_DEFERRED_KHR => Some("OPERATION_NOT_DEFERRED_KHR"),
            Self::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR => {
                Some("ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR")
            }
            Self::ERROR_COMPRESSION_EXHAUSTED_EXT => Some("ERROR_COMPRESSION_EXHAUSTED_EXT"),
            Self::INCOMPATIBLE_SHADER_BINARY_EXT => Some("INCOMPATIBLE_SHADER_BINARY_EXT"),
            Self::PIPELINE_BINARY_MISSING_KHR => Some("PIPELINE_BINARY_MISSING_KHR"),
            Self::ERROR_NOT_ENOUGH_SPACE_KHR => Some("ERROR_NOT_ENOUGH_SPACE_KHR"),
            Self::ERROR_VALIDATION_FAILED => Some("ERROR_VALIDATION_FAILED"),
            Self::ERROR_OUT_OF_POOL_MEMORY => Some("ERROR_OUT_OF_POOL_MEMORY"),
            Self::ERROR_INVALID_EXTERNAL_HANDLE => Some("ERROR_INVALID_EXTERNAL_HANDLE"),
            Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS => {
                Some("ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS")
            }
            Self::ERROR_FRAGMENTATION => Some("ERROR_FRAGMENTATION"),
            Self::PIPELINE_COMPILE_REQUIRED => Some("PIPELINE_COMPILE_REQUIRED"),
            Self::ERROR_NOT_PERMITTED => Some("ERROR_NOT_PERMITTED"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
