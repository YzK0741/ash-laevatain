use super::bitflags::*;
use super::debug_flags;
use super::definitions::*;
use super::enums::*;
use core::fmt;
impl fmt::Debug for AccelerationStructureBuildTypeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::HOST => Some("HOST"),
            Self::DEVICE => Some("DEVICE"),
            Self::HOST_OR_DEVICE => Some("HOST_OR_DEVICE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for AccelerationStructureCompatibilityKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::COMPATIBLE => Some("COMPATIBLE"),
            Self::INCOMPATIBLE => Some("INCOMPATIBLE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for AccelerationStructureCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                AccelerationStructureCreateFlagsKHR::DEVICE_ADDRESS_CAPTURE_REPLAY.0,
                "DEVICE_ADDRESS_CAPTURE_REPLAY",
            ),
            (
                AccelerationStructureCreateFlagsKHR::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0,
                "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT",
            ),
            (
                AccelerationStructureCreateFlagsKHR::MOTION_NV.0,
                "MOTION_NV",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for AccelerationStructureTypeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::TOP_LEVEL => Some("TOP_LEVEL"),
            Self::BOTTOM_LEVEL => Some("BOTTOM_LEVEL"),
            Self::GENERIC => Some("GENERIC"),
            Self::OPACITY_MICROMAP => Some("OPACITY_MICROMAP"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for AccessFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                AccessFlags::INDIRECT_COMMAND_READ.0,
                "INDIRECT_COMMAND_READ",
            ),
            (AccessFlags::INDEX_READ.0, "INDEX_READ"),
            (
                AccessFlags::VERTEX_ATTRIBUTE_READ.0,
                "VERTEX_ATTRIBUTE_READ",
            ),
            (AccessFlags::UNIFORM_READ.0, "UNIFORM_READ"),
            (
                AccessFlags::INPUT_ATTACHMENT_READ.0,
                "INPUT_ATTACHMENT_READ",
            ),
            (AccessFlags::SHADER_READ.0, "SHADER_READ"),
            (AccessFlags::SHADER_WRITE.0, "SHADER_WRITE"),
            (
                AccessFlags::COLOR_ATTACHMENT_READ.0,
                "COLOR_ATTACHMENT_READ",
            ),
            (
                AccessFlags::COLOR_ATTACHMENT_WRITE.0,
                "COLOR_ATTACHMENT_WRITE",
            ),
            (
                AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ.0,
                "DEPTH_STENCIL_ATTACHMENT_READ",
            ),
            (
                AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE.0,
                "DEPTH_STENCIL_ATTACHMENT_WRITE",
            ),
            (AccessFlags::TRANSFER_READ.0, "TRANSFER_READ"),
            (AccessFlags::TRANSFER_WRITE.0, "TRANSFER_WRITE"),
            (AccessFlags::HOST_READ.0, "HOST_READ"),
            (AccessFlags::HOST_WRITE.0, "HOST_WRITE"),
            (AccessFlags::MEMORY_READ.0, "MEMORY_READ"),
            (AccessFlags::MEMORY_WRITE.0, "MEMORY_WRITE"),
            (
                AccessFlags::TRANSFORM_FEEDBACK_WRITE_EXT.0,
                "TRANSFORM_FEEDBACK_WRITE_EXT",
            ),
            (
                AccessFlags::TRANSFORM_FEEDBACK_COUNTER_READ_EXT.0,
                "TRANSFORM_FEEDBACK_COUNTER_READ_EXT",
            ),
            (
                AccessFlags::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT.0,
                "TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT",
            ),
            (
                AccessFlags::CONDITIONAL_RENDERING_READ_EXT.0,
                "CONDITIONAL_RENDERING_READ_EXT",
            ),
            (
                AccessFlags::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT.0,
                "COLOR_ATTACHMENT_READ_NONCOHERENT_EXT",
            ),
            (
                AccessFlags::ACCELERATION_STRUCTURE_READ_KHR.0,
                "ACCELERATION_STRUCTURE_READ_KHR",
            ),
            (
                AccessFlags::ACCELERATION_STRUCTURE_WRITE_KHR.0,
                "ACCELERATION_STRUCTURE_WRITE_KHR",
            ),
            (
                AccessFlags::FRAGMENT_DENSITY_MAP_READ_EXT.0,
                "FRAGMENT_DENSITY_MAP_READ_EXT",
            ),
            (
                AccessFlags::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR.0,
                "FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR",
            ),
            (
                AccessFlags::COMMAND_PREPROCESS_READ_EXT.0,
                "COMMAND_PREPROCESS_READ_EXT",
            ),
            (
                AccessFlags::COMMAND_PREPROCESS_WRITE_EXT.0,
                "COMMAND_PREPROCESS_WRITE_EXT",
            ),
            (AccessFlags::NONE.0, "NONE"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for AccessFlags2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags64, &str)] = &[
            (AccessFlags2::NONE.0, "NONE"),
            (
                AccessFlags2::INDIRECT_COMMAND_READ.0,
                "INDIRECT_COMMAND_READ",
            ),
            (AccessFlags2::INDEX_READ.0, "INDEX_READ"),
            (
                AccessFlags2::VERTEX_ATTRIBUTE_READ.0,
                "VERTEX_ATTRIBUTE_READ",
            ),
            (AccessFlags2::UNIFORM_READ.0, "UNIFORM_READ"),
            (
                AccessFlags2::INPUT_ATTACHMENT_READ.0,
                "INPUT_ATTACHMENT_READ",
            ),
            (AccessFlags2::SHADER_READ.0, "SHADER_READ"),
            (AccessFlags2::SHADER_WRITE.0, "SHADER_WRITE"),
            (
                AccessFlags2::COLOR_ATTACHMENT_READ.0,
                "COLOR_ATTACHMENT_READ",
            ),
            (
                AccessFlags2::COLOR_ATTACHMENT_WRITE.0,
                "COLOR_ATTACHMENT_WRITE",
            ),
            (
                AccessFlags2::DEPTH_STENCIL_ATTACHMENT_READ.0,
                "DEPTH_STENCIL_ATTACHMENT_READ",
            ),
            (
                AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE.0,
                "DEPTH_STENCIL_ATTACHMENT_WRITE",
            ),
            (AccessFlags2::TRANSFER_READ.0, "TRANSFER_READ"),
            (AccessFlags2::TRANSFER_WRITE.0, "TRANSFER_WRITE"),
            (AccessFlags2::HOST_READ.0, "HOST_READ"),
            (AccessFlags2::HOST_WRITE.0, "HOST_WRITE"),
            (AccessFlags2::MEMORY_READ.0, "MEMORY_READ"),
            (AccessFlags2::MEMORY_WRITE.0, "MEMORY_WRITE"),
            (AccessFlags2::SHADER_SAMPLED_READ.0, "SHADER_SAMPLED_READ"),
            (AccessFlags2::SHADER_STORAGE_READ.0, "SHADER_STORAGE_READ"),
            (AccessFlags2::SHADER_STORAGE_WRITE.0, "SHADER_STORAGE_WRITE"),
            (
                AccessFlags2::VIDEO_DECODE_READ_KHR.0,
                "VIDEO_DECODE_READ_KHR",
            ),
            (
                AccessFlags2::VIDEO_DECODE_WRITE_KHR.0,
                "VIDEO_DECODE_WRITE_KHR",
            ),
            (
                AccessFlags2::SAMPLER_HEAP_READ_EXT.0,
                "SAMPLER_HEAP_READ_EXT",
            ),
            (
                AccessFlags2::RESOURCE_HEAP_READ_EXT.0,
                "RESOURCE_HEAP_READ_EXT",
            ),
            (
                AccessFlags2::VIDEO_ENCODE_READ_KHR.0,
                "VIDEO_ENCODE_READ_KHR",
            ),
            (
                AccessFlags2::VIDEO_ENCODE_WRITE_KHR.0,
                "VIDEO_ENCODE_WRITE_KHR",
            ),
            (
                AccessFlags2::SHADER_TILE_ATTACHMENT_READ_QCOM.0,
                "SHADER_TILE_ATTACHMENT_READ_QCOM",
            ),
            (
                AccessFlags2::SHADER_TILE_ATTACHMENT_WRITE_QCOM.0,
                "SHADER_TILE_ATTACHMENT_WRITE_QCOM",
            ),
            (
                AccessFlags2::TRANSFORM_FEEDBACK_WRITE_EXT.0,
                "TRANSFORM_FEEDBACK_WRITE_EXT",
            ),
            (
                AccessFlags2::TRANSFORM_FEEDBACK_COUNTER_READ_EXT.0,
                "TRANSFORM_FEEDBACK_COUNTER_READ_EXT",
            ),
            (
                AccessFlags2::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT.0,
                "TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT",
            ),
            (
                AccessFlags2::CONDITIONAL_RENDERING_READ_EXT.0,
                "CONDITIONAL_RENDERING_READ_EXT",
            ),
            (
                AccessFlags2::COMMAND_PREPROCESS_READ_EXT.0,
                "COMMAND_PREPROCESS_READ_EXT",
            ),
            (
                AccessFlags2::COMMAND_PREPROCESS_WRITE_EXT.0,
                "COMMAND_PREPROCESS_WRITE_EXT",
            ),
            (
                AccessFlags2::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR.0,
                "FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR",
            ),
            (
                AccessFlags2::ACCELERATION_STRUCTURE_READ_KHR.0,
                "ACCELERATION_STRUCTURE_READ_KHR",
            ),
            (
                AccessFlags2::ACCELERATION_STRUCTURE_WRITE_KHR.0,
                "ACCELERATION_STRUCTURE_WRITE_KHR",
            ),
            (
                AccessFlags2::FRAGMENT_DENSITY_MAP_READ_EXT.0,
                "FRAGMENT_DENSITY_MAP_READ_EXT",
            ),
            (
                AccessFlags2::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT.0,
                "COLOR_ATTACHMENT_READ_NONCOHERENT_EXT",
            ),
            (
                AccessFlags2::DESCRIPTOR_BUFFER_READ_EXT.0,
                "DESCRIPTOR_BUFFER_READ_EXT",
            ),
            (
                AccessFlags2::INVOCATION_MASK_READ_HUAWEI.0,
                "INVOCATION_MASK_READ_HUAWEI",
            ),
            (
                AccessFlags2::SHADER_BINDING_TABLE_READ_KHR.0,
                "SHADER_BINDING_TABLE_READ_KHR",
            ),
            (AccessFlags2::MICROMAP_READ_EXT.0, "MICROMAP_READ_EXT"),
            (AccessFlags2::MICROMAP_WRITE_EXT.0, "MICROMAP_WRITE_EXT"),
            (AccessFlags2::OPTICAL_FLOW_READ_NV.0, "OPTICAL_FLOW_READ_NV"),
            (
                AccessFlags2::OPTICAL_FLOW_WRITE_NV.0,
                "OPTICAL_FLOW_WRITE_NV",
            ),
            (AccessFlags2::DATA_GRAPH_READ_ARM.0, "DATA_GRAPH_READ_ARM"),
            (AccessFlags2::DATA_GRAPH_WRITE_ARM.0, "DATA_GRAPH_WRITE_ARM"),
            (
                AccessFlags2::MEMORY_DECOMPRESSION_READ_EXT.0,
                "MEMORY_DECOMPRESSION_READ_EXT",
            ),
            (
                AccessFlags2::MEMORY_DECOMPRESSION_WRITE_EXT.0,
                "MEMORY_DECOMPRESSION_WRITE_EXT",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for AccessFlags3KHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags64, &str)] = &[(AccessFlags3KHR::NONE.0, "NONE")];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for AndroidSurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for AttachmentDescriptionFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (AttachmentDescriptionFlags::MAY_ALIAS.0, "MAY_ALIAS"),
            (
                AttachmentDescriptionFlags::RESOLVE_SKIP_TRANSFER_FUNCTION_KHR.0,
                "RESOLVE_SKIP_TRANSFER_FUNCTION_KHR",
            ),
            (
                AttachmentDescriptionFlags::RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR.0,
                "RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for AttachmentLoadOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::LOAD => Some("LOAD"),
            Self::CLEAR => Some("CLEAR"),
            Self::DONT_CARE => Some("DONT_CARE"),
            Self::NONE => Some("NONE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for AttachmentStoreOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::STORE => Some("STORE"),
            Self::DONT_CARE => Some("DONT_CARE"),
            Self::NONE => Some("NONE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for BlendFactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::ZERO => Some("ZERO"),
            Self::ONE => Some("ONE"),
            Self::SRC_COLOR => Some("SRC_COLOR"),
            Self::ONE_MINUS_SRC_COLOR => Some("ONE_MINUS_SRC_COLOR"),
            Self::DST_COLOR => Some("DST_COLOR"),
            Self::ONE_MINUS_DST_COLOR => Some("ONE_MINUS_DST_COLOR"),
            Self::SRC_ALPHA => Some("SRC_ALPHA"),
            Self::ONE_MINUS_SRC_ALPHA => Some("ONE_MINUS_SRC_ALPHA"),
            Self::DST_ALPHA => Some("DST_ALPHA"),
            Self::ONE_MINUS_DST_ALPHA => Some("ONE_MINUS_DST_ALPHA"),
            Self::CONSTANT_COLOR => Some("CONSTANT_COLOR"),
            Self::ONE_MINUS_CONSTANT_COLOR => Some("ONE_MINUS_CONSTANT_COLOR"),
            Self::CONSTANT_ALPHA => Some("CONSTANT_ALPHA"),
            Self::ONE_MINUS_CONSTANT_ALPHA => Some("ONE_MINUS_CONSTANT_ALPHA"),
            Self::SRC_ALPHA_SATURATE => Some("SRC_ALPHA_SATURATE"),
            Self::SRC1_COLOR => Some("SRC1_COLOR"),
            Self::ONE_MINUS_SRC1_COLOR => Some("ONE_MINUS_SRC1_COLOR"),
            Self::SRC1_ALPHA => Some("SRC1_ALPHA"),
            Self::ONE_MINUS_SRC1_ALPHA => Some("ONE_MINUS_SRC1_ALPHA"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for BlendOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::ADD => Some("ADD"),
            Self::SUBTRACT => Some("SUBTRACT"),
            Self::REVERSE_SUBTRACT => Some("REVERSE_SUBTRACT"),
            Self::MIN => Some("MIN"),
            Self::MAX => Some("MAX"),
            Self::ZERO_EXT => Some("ZERO_EXT"),
            Self::SRC_EXT => Some("SRC_EXT"),
            Self::DST_EXT => Some("DST_EXT"),
            Self::SRC_OVER_EXT => Some("SRC_OVER_EXT"),
            Self::DST_OVER_EXT => Some("DST_OVER_EXT"),
            Self::SRC_IN_EXT => Some("SRC_IN_EXT"),
            Self::DST_IN_EXT => Some("DST_IN_EXT"),
            Self::SRC_OUT_EXT => Some("SRC_OUT_EXT"),
            Self::DST_OUT_EXT => Some("DST_OUT_EXT"),
            Self::SRC_ATOP_EXT => Some("SRC_ATOP_EXT"),
            Self::DST_ATOP_EXT => Some("DST_ATOP_EXT"),
            Self::XOR_EXT => Some("XOR_EXT"),
            Self::MULTIPLY_EXT => Some("MULTIPLY_EXT"),
            Self::SCREEN_EXT => Some("SCREEN_EXT"),
            Self::OVERLAY_EXT => Some("OVERLAY_EXT"),
            Self::DARKEN_EXT => Some("DARKEN_EXT"),
            Self::LIGHTEN_EXT => Some("LIGHTEN_EXT"),
            Self::COLORDODGE_EXT => Some("COLORDODGE_EXT"),
            Self::COLORBURN_EXT => Some("COLORBURN_EXT"),
            Self::HARDLIGHT_EXT => Some("HARDLIGHT_EXT"),
            Self::SOFTLIGHT_EXT => Some("SOFTLIGHT_EXT"),
            Self::DIFFERENCE_EXT => Some("DIFFERENCE_EXT"),
            Self::EXCLUSION_EXT => Some("EXCLUSION_EXT"),
            Self::INVERT_EXT => Some("INVERT_EXT"),
            Self::INVERT_RGB_EXT => Some("INVERT_RGB_EXT"),
            Self::LINEARDODGE_EXT => Some("LINEARDODGE_EXT"),
            Self::LINEARBURN_EXT => Some("LINEARBURN_EXT"),
            Self::VIVIDLIGHT_EXT => Some("VIVIDLIGHT_EXT"),
            Self::LINEARLIGHT_EXT => Some("LINEARLIGHT_EXT"),
            Self::PINLIGHT_EXT => Some("PINLIGHT_EXT"),
            Self::HARDMIX_EXT => Some("HARDMIX_EXT"),
            Self::HSL_HUE_EXT => Some("HSL_HUE_EXT"),
            Self::HSL_SATURATION_EXT => Some("HSL_SATURATION_EXT"),
            Self::HSL_COLOR_EXT => Some("HSL_COLOR_EXT"),
            Self::HSL_LUMINOSITY_EXT => Some("HSL_LUMINOSITY_EXT"),
            Self::PLUS_EXT => Some("PLUS_EXT"),
            Self::PLUS_CLAMPED_EXT => Some("PLUS_CLAMPED_EXT"),
            Self::PLUS_CLAMPED_ALPHA_EXT => Some("PLUS_CLAMPED_ALPHA_EXT"),
            Self::PLUS_DARKER_EXT => Some("PLUS_DARKER_EXT"),
            Self::MINUS_EXT => Some("MINUS_EXT"),
            Self::MINUS_CLAMPED_EXT => Some("MINUS_CLAMPED_EXT"),
            Self::CONTRAST_EXT => Some("CONTRAST_EXT"),
            Self::INVERT_OVG_EXT => Some("INVERT_OVG_EXT"),
            Self::RED_EXT => Some("RED_EXT"),
            Self::GREEN_EXT => Some("GREEN_EXT"),
            Self::BLUE_EXT => Some("BLUE_EXT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for BlendOverlapEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::UNCORRELATED => Some("UNCORRELATED"),
            Self::DISJOINT => Some("DISJOINT"),
            Self::CONJOINT => Some("CONJOINT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for BorderColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::FLOAT_TRANSPARENT_BLACK => Some("FLOAT_TRANSPARENT_BLACK"),
            Self::INT_TRANSPARENT_BLACK => Some("INT_TRANSPARENT_BLACK"),
            Self::FLOAT_OPAQUE_BLACK => Some("FLOAT_OPAQUE_BLACK"),
            Self::INT_OPAQUE_BLACK => Some("INT_OPAQUE_BLACK"),
            Self::FLOAT_OPAQUE_WHITE => Some("FLOAT_OPAQUE_WHITE"),
            Self::INT_OPAQUE_WHITE => Some("INT_OPAQUE_WHITE"),
            Self::FLOAT_CUSTOM_EXT => Some("FLOAT_CUSTOM_EXT"),
            Self::INT_CUSTOM_EXT => Some("INT_CUSTOM_EXT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for BufferCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (BufferCreateFlags::SPARSE_BINDING.0, "SPARSE_BINDING"),
            (BufferCreateFlags::SPARSE_RESIDENCY.0, "SPARSE_RESIDENCY"),
            (BufferCreateFlags::SPARSE_ALIASED.0, "SPARSE_ALIASED"),
            (
                BufferCreateFlags::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0,
                "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT",
            ),
            (
                BufferCreateFlags::VIDEO_PROFILE_INDEPENDENT_KHR.0,
                "VIDEO_PROFILE_INDEPENDENT_KHR",
            ),
            (BufferCreateFlags::PROTECTED.0, "PROTECTED"),
            (
                BufferCreateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY.0,
                "DEVICE_ADDRESS_CAPTURE_REPLAY",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for BufferUsageFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (BufferUsageFlags::TRANSFER_SRC.0, "TRANSFER_SRC"),
            (BufferUsageFlags::TRANSFER_DST.0, "TRANSFER_DST"),
            (
                BufferUsageFlags::UNIFORM_TEXEL_BUFFER.0,
                "UNIFORM_TEXEL_BUFFER",
            ),
            (
                BufferUsageFlags::STORAGE_TEXEL_BUFFER.0,
                "STORAGE_TEXEL_BUFFER",
            ),
            (BufferUsageFlags::UNIFORM_BUFFER.0, "UNIFORM_BUFFER"),
            (BufferUsageFlags::STORAGE_BUFFER.0, "STORAGE_BUFFER"),
            (BufferUsageFlags::INDEX_BUFFER.0, "INDEX_BUFFER"),
            (BufferUsageFlags::VERTEX_BUFFER.0, "VERTEX_BUFFER"),
            (BufferUsageFlags::INDIRECT_BUFFER.0, "INDIRECT_BUFFER"),
            (
                BufferUsageFlags::VIDEO_DECODE_SRC_KHR.0,
                "VIDEO_DECODE_SRC_KHR",
            ),
            (
                BufferUsageFlags::VIDEO_DECODE_DST_KHR.0,
                "VIDEO_DECODE_DST_KHR",
            ),
            (
                BufferUsageFlags::TRANSFORM_FEEDBACK_BUFFER_EXT.0,
                "TRANSFORM_FEEDBACK_BUFFER_EXT",
            ),
            (
                BufferUsageFlags::TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT.0,
                "TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT",
            ),
            (
                BufferUsageFlags::CONDITIONAL_RENDERING_EXT.0,
                "CONDITIONAL_RENDERING_EXT",
            ),
            #[cfg(feature = "provisional")]
            (
                BufferUsageFlags::EXECUTION_GRAPH_SCRATCH_AMDX.0,
                "EXECUTION_GRAPH_SCRATCH_AMDX",
            ),
            (
                BufferUsageFlags::DESCRIPTOR_HEAP_EXT.0,
                "DESCRIPTOR_HEAP_EXT",
            ),
            (
                BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR.0,
                "ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR",
            ),
            (
                BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE_KHR.0,
                "ACCELERATION_STRUCTURE_STORAGE_KHR",
            ),
            (
                BufferUsageFlags::SHADER_BINDING_TABLE_KHR.0,
                "SHADER_BINDING_TABLE_KHR",
            ),
            (
                BufferUsageFlags::VIDEO_ENCODE_DST_KHR.0,
                "VIDEO_ENCODE_DST_KHR",
            ),
            (
                BufferUsageFlags::VIDEO_ENCODE_SRC_KHR.0,
                "VIDEO_ENCODE_SRC_KHR",
            ),
            (
                BufferUsageFlags::SAMPLER_DESCRIPTOR_BUFFER_EXT.0,
                "SAMPLER_DESCRIPTOR_BUFFER_EXT",
            ),
            (
                BufferUsageFlags::RESOURCE_DESCRIPTOR_BUFFER_EXT.0,
                "RESOURCE_DESCRIPTOR_BUFFER_EXT",
            ),
            (
                BufferUsageFlags::PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT.0,
                "PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT",
            ),
            (
                BufferUsageFlags::MICROMAP_BUILD_INPUT_READ_ONLY_EXT.0,
                "MICROMAP_BUILD_INPUT_READ_ONLY_EXT",
            ),
            (
                BufferUsageFlags::MICROMAP_STORAGE_EXT.0,
                "MICROMAP_STORAGE_EXT",
            ),
            (BufferUsageFlags::TILE_MEMORY_QCOM.0, "TILE_MEMORY_QCOM"),
            (
                BufferUsageFlags::SHADER_DEVICE_ADDRESS.0,
                "SHADER_DEVICE_ADDRESS",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for BufferUsageFlags2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags64, &str)] = &[
            (BufferUsageFlags2::TRANSFER_SRC.0, "TRANSFER_SRC"),
            (BufferUsageFlags2::TRANSFER_DST.0, "TRANSFER_DST"),
            (
                BufferUsageFlags2::UNIFORM_TEXEL_BUFFER.0,
                "UNIFORM_TEXEL_BUFFER",
            ),
            (
                BufferUsageFlags2::STORAGE_TEXEL_BUFFER.0,
                "STORAGE_TEXEL_BUFFER",
            ),
            (BufferUsageFlags2::UNIFORM_BUFFER.0, "UNIFORM_BUFFER"),
            (BufferUsageFlags2::STORAGE_BUFFER.0, "STORAGE_BUFFER"),
            (BufferUsageFlags2::INDEX_BUFFER.0, "INDEX_BUFFER"),
            (BufferUsageFlags2::VERTEX_BUFFER.0, "VERTEX_BUFFER"),
            (BufferUsageFlags2::INDIRECT_BUFFER.0, "INDIRECT_BUFFER"),
            #[cfg(feature = "provisional")]
            (
                BufferUsageFlags2::EXECUTION_GRAPH_SCRATCH_AMDX.0,
                "EXECUTION_GRAPH_SCRATCH_AMDX",
            ),
            (
                BufferUsageFlags2::DESCRIPTOR_HEAP_EXT.0,
                "DESCRIPTOR_HEAP_EXT",
            ),
            (
                BufferUsageFlags2::MICROMAP_BUILD_INPUT_READ_ONLY_EXT.0,
                "MICROMAP_BUILD_INPUT_READ_ONLY_EXT",
            ),
            (
                BufferUsageFlags2::MICROMAP_STORAGE_EXT.0,
                "MICROMAP_STORAGE_EXT",
            ),
            (
                BufferUsageFlags2::CONDITIONAL_RENDERING_EXT.0,
                "CONDITIONAL_RENDERING_EXT",
            ),
            (
                BufferUsageFlags2::SHADER_BINDING_TABLE_KHR.0,
                "SHADER_BINDING_TABLE_KHR",
            ),
            (
                BufferUsageFlags2::TRANSFORM_FEEDBACK_BUFFER_EXT.0,
                "TRANSFORM_FEEDBACK_BUFFER_EXT",
            ),
            (
                BufferUsageFlags2::TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT.0,
                "TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT",
            ),
            (
                BufferUsageFlags2::VIDEO_DECODE_SRC_KHR.0,
                "VIDEO_DECODE_SRC_KHR",
            ),
            (
                BufferUsageFlags2::VIDEO_DECODE_DST_KHR.0,
                "VIDEO_DECODE_DST_KHR",
            ),
            (
                BufferUsageFlags2::VIDEO_ENCODE_DST_KHR.0,
                "VIDEO_ENCODE_DST_KHR",
            ),
            (
                BufferUsageFlags2::VIDEO_ENCODE_SRC_KHR.0,
                "VIDEO_ENCODE_SRC_KHR",
            ),
            (
                BufferUsageFlags2::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR.0,
                "ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR",
            ),
            (
                BufferUsageFlags2::ACCELERATION_STRUCTURE_STORAGE_KHR.0,
                "ACCELERATION_STRUCTURE_STORAGE_KHR",
            ),
            (
                BufferUsageFlags2::SAMPLER_DESCRIPTOR_BUFFER_EXT.0,
                "SAMPLER_DESCRIPTOR_BUFFER_EXT",
            ),
            (
                BufferUsageFlags2::RESOURCE_DESCRIPTOR_BUFFER_EXT.0,
                "RESOURCE_DESCRIPTOR_BUFFER_EXT",
            ),
            (
                BufferUsageFlags2::PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT.0,
                "PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT",
            ),
            #[cfg(feature = "provisional")]
            (
                BufferUsageFlags2::COMPRESSED_DATA_DGF1_AMDX.0,
                "COMPRESSED_DATA_DGF1_AMDX",
            ),
            (
                BufferUsageFlags2::DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM.0,
                "DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM",
            ),
            (BufferUsageFlags2::TILE_MEMORY_QCOM.0, "TILE_MEMORY_QCOM"),
            (
                BufferUsageFlags2::MEMORY_DECOMPRESSION_EXT.0,
                "MEMORY_DECOMPRESSION_EXT",
            ),
            (
                BufferUsageFlags2::PREPROCESS_BUFFER_EXT.0,
                "PREPROCESS_BUFFER_EXT",
            ),
            (
                BufferUsageFlags2::SHADER_DEVICE_ADDRESS.0,
                "SHADER_DEVICE_ADDRESS",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for BufferViewCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for BuildAccelerationStructureFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                BuildAccelerationStructureFlagsKHR::ALLOW_UPDATE.0,
                "ALLOW_UPDATE",
            ),
            (
                BuildAccelerationStructureFlagsKHR::ALLOW_COMPACTION.0,
                "ALLOW_COMPACTION",
            ),
            (
                BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE.0,
                "PREFER_FAST_TRACE",
            ),
            (
                BuildAccelerationStructureFlagsKHR::PREFER_FAST_BUILD.0,
                "PREFER_FAST_BUILD",
            ),
            (
                BuildAccelerationStructureFlagsKHR::LOW_MEMORY.0,
                "LOW_MEMORY",
            ),
            (BuildAccelerationStructureFlagsKHR::MOTION_NV.0, "MOTION_NV"),
            (
                BuildAccelerationStructureFlagsKHR::ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT.0,
                "ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT",
            ),
            #[cfg(feature = "provisional")]
            (
                BuildAccelerationStructureFlagsKHR::ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV.0,
                "ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV",
            ),
            (
                BuildAccelerationStructureFlagsKHR::ALLOW_DATA_ACCESS.0,
                "ALLOW_DATA_ACCESS",
            ),
            (
                BuildAccelerationStructureFlagsKHR::ALLOW_CLUSTER_OPACITY_MICROMAPS_NV.0,
                "ALLOW_CLUSTER_OPACITY_MICROMAPS_NV",
            ),
            (
                BuildAccelerationStructureFlagsKHR::ALLOW_OPACITY_MICROMAP_UPDATE.0,
                "ALLOW_OPACITY_MICROMAP_UPDATE",
            ),
            (
                BuildAccelerationStructureFlagsKHR::ALLOW_DISABLE_OPACITY_MICROMAPS.0,
                "ALLOW_DISABLE_OPACITY_MICROMAPS",
            ),
            (
                BuildAccelerationStructureFlagsKHR::MICROMAP_LOSSY.0,
                "MICROMAP_LOSSY",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for BuildAccelerationStructureModeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::BUILD => Some("BUILD"),
            Self::UPDATE => Some("UPDATE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for ChromaLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::COSITED_EVEN => Some("COSITED_EVEN"),
            Self::MIDPOINT => Some("MIDPOINT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for ColorComponentFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ColorComponentFlags::R.0, "R"),
            (ColorComponentFlags::G.0, "G"),
            (ColorComponentFlags::B.0, "B"),
            (ColorComponentFlags::A.0, "A"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ColorSpaceKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::SRGB_NONLINEAR => Some("SRGB_NONLINEAR"),
            Self::DISPLAY_P3_NONLINEAR_EXT => Some("DISPLAY_P3_NONLINEAR_EXT"),
            Self::EXTENDED_SRGB_LINEAR_EXT => Some("EXTENDED_SRGB_LINEAR_EXT"),
            Self::DISPLAY_P3_LINEAR_EXT => Some("DISPLAY_P3_LINEAR_EXT"),
            Self::DCI_P3_NONLINEAR_EXT => Some("DCI_P3_NONLINEAR_EXT"),
            Self::BT709_LINEAR_EXT => Some("BT709_LINEAR_EXT"),
            Self::BT709_NONLINEAR_EXT => Some("BT709_NONLINEAR_EXT"),
            Self::BT2020_LINEAR_EXT => Some("BT2020_LINEAR_EXT"),
            Self::HDR10_ST2084_EXT => Some("HDR10_ST2084_EXT"),
            #[allow(deprecated)]
            Self::DOLBYVISION_EXT => Some("DOLBYVISION_EXT"),
            Self::HDR10_HLG_EXT => Some("HDR10_HLG_EXT"),
            Self::ADOBERGB_LINEAR_EXT => Some("ADOBERGB_LINEAR_EXT"),
            Self::ADOBERGB_NONLINEAR_EXT => Some("ADOBERGB_NONLINEAR_EXT"),
            Self::PASS_THROUGH_EXT => Some("PASS_THROUGH_EXT"),
            Self::EXTENDED_SRGB_NONLINEAR_EXT => Some("EXTENDED_SRGB_NONLINEAR_EXT"),
            Self::DISPLAY_NATIVE_AMD => Some("DISPLAY_NATIVE_AMD"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for CommandBufferLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::PRIMARY => Some("PRIMARY"),
            Self::SECONDARY => Some("SECONDARY"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for CommandBufferResetFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(
            CommandBufferResetFlags::RELEASE_RESOURCES.0,
            "RELEASE_RESOURCES",
        )];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for CommandBufferUsageFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                CommandBufferUsageFlags::ONE_TIME_SUBMIT.0,
                "ONE_TIME_SUBMIT",
            ),
            (
                CommandBufferUsageFlags::RENDER_PASS_CONTINUE.0,
                "RENDER_PASS_CONTINUE",
            ),
            (
                CommandBufferUsageFlags::SIMULTANEOUS_USE.0,
                "SIMULTANEOUS_USE",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for CommandPoolCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (CommandPoolCreateFlags::TRANSIENT.0, "TRANSIENT"),
            (
                CommandPoolCreateFlags::RESET_COMMAND_BUFFER.0,
                "RESET_COMMAND_BUFFER",
            ),
            (CommandPoolCreateFlags::PROTECTED.0, "PROTECTED"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for CommandPoolResetFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(
            CommandPoolResetFlags::RELEASE_RESOURCES.0,
            "RELEASE_RESOURCES",
        )];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for CommandPoolTrimFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for CompareOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::NEVER => Some("NEVER"),
            Self::LESS => Some("LESS"),
            Self::EQUAL => Some("EQUAL"),
            Self::LESS_OR_EQUAL => Some("LESS_OR_EQUAL"),
            Self::GREATER => Some("GREATER"),
            Self::NOT_EQUAL => Some("NOT_EQUAL"),
            Self::GREATER_OR_EQUAL => Some("GREATER_OR_EQUAL"),
            Self::ALWAYS => Some("ALWAYS"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for ComponentSwizzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::IDENTITY => Some("IDENTITY"),
            Self::ZERO => Some("ZERO"),
            Self::ONE => Some("ONE"),
            Self::R => Some("R"),
            Self::G => Some("G"),
            Self::B => Some("B"),
            Self::A => Some("A"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for CompositeAlphaFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (CompositeAlphaFlagsKHR::OPAQUE.0, "OPAQUE"),
            (CompositeAlphaFlagsKHR::PRE_MULTIPLIED.0, "PRE_MULTIPLIED"),
            (CompositeAlphaFlagsKHR::POST_MULTIPLIED.0, "POST_MULTIPLIED"),
            (CompositeAlphaFlagsKHR::INHERIT.0, "INHERIT"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ConservativeRasterizationModeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::DISABLED => Some("DISABLED"),
            Self::OVERESTIMATE => Some("OVERESTIMATE"),
            Self::UNDERESTIMATE => Some("UNDERESTIMATE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for CopyAccelerationStructureModeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::CLONE => Some("CLONE"),
            Self::COMPACT => Some("COMPACT"),
            Self::SERIALIZE => Some("SERIALIZE"),
            Self::DESERIALIZE => Some("DESERIALIZE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for CoverageModulationModeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::NONE => Some("NONE"),
            Self::RGB => Some("RGB"),
            Self::ALPHA => Some("ALPHA"),
            Self::RGBA => Some("RGBA"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for CoverageReductionModeNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::MERGE => Some("MERGE"),
            Self::TRUNCATE => Some("TRUNCATE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for CullModeFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (CullModeFlags::NONE.0, "NONE"),
            (CullModeFlags::FRONT.0, "FRONT"),
            (CullModeFlags::BACK.0, "BACK"),
            (CullModeFlags::FRONT_AND_BACK.0, "FRONT_AND_BACK"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DebugUtilsMessageSeverityFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (DebugUtilsMessageSeverityFlagsEXT::VERBOSE.0, "VERBOSE"),
            (DebugUtilsMessageSeverityFlagsEXT::INFO.0, "INFO"),
            (DebugUtilsMessageSeverityFlagsEXT::WARNING.0, "WARNING"),
            (DebugUtilsMessageSeverityFlagsEXT::ERROR.0, "ERROR"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DebugUtilsMessageTypeFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (DebugUtilsMessageTypeFlagsEXT::GENERAL.0, "GENERAL"),
            (DebugUtilsMessageTypeFlagsEXT::VALIDATION.0, "VALIDATION"),
            (DebugUtilsMessageTypeFlagsEXT::PERFORMANCE.0, "PERFORMANCE"),
            (
                DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING.0,
                "DEVICE_ADDRESS_BINDING",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DebugUtilsMessengerCallbackDataFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DebugUtilsMessengerCreateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DependencyFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (DependencyFlags::BY_REGION.0, "BY_REGION"),
            (DependencyFlags::FEEDBACK_LOOP_EXT.0, "FEEDBACK_LOOP_EXT"),
            (
                DependencyFlags::QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_KHR.0,
                "QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_KHR",
            ),
            (
                DependencyFlags::ASYMMETRIC_EVENT_KHR.0,
                "ASYMMETRIC_EVENT_KHR",
            ),
            (DependencyFlags::DEVICE_GROUP.0, "DEVICE_GROUP"),
            (DependencyFlags::VIEW_LOCAL.0, "VIEW_LOCAL"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DepthClampModeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::VIEWPORT_RANGE => Some("VIEWPORT_RANGE"),
            Self::USER_DEFINED_RANGE => Some("USER_DEFINED_RANGE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for DescriptorBindingFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                DescriptorBindingFlags::UPDATE_AFTER_BIND.0,
                "UPDATE_AFTER_BIND",
            ),
            (
                DescriptorBindingFlags::UPDATE_UNUSED_WHILE_PENDING.0,
                "UPDATE_UNUSED_WHILE_PENDING",
            ),
            (DescriptorBindingFlags::PARTIALLY_BOUND.0, "PARTIALLY_BOUND"),
            (
                DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT.0,
                "VARIABLE_DESCRIPTOR_COUNT",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DescriptorMappingSourceEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::HEAP_WITH_CONSTANT_OFFSET => Some("HEAP_WITH_CONSTANT_OFFSET"),
            Self::HEAP_WITH_PUSH_INDEX => Some("HEAP_WITH_PUSH_INDEX"),
            Self::HEAP_WITH_INDIRECT_INDEX => Some("HEAP_WITH_INDIRECT_INDEX"),
            Self::HEAP_WITH_INDIRECT_INDEX_ARRAY => Some("HEAP_WITH_INDIRECT_INDEX_ARRAY"),
            Self::RESOURCE_HEAP_DATA => Some("RESOURCE_HEAP_DATA"),
            Self::PUSH_DATA => Some("PUSH_DATA"),
            Self::PUSH_ADDRESS => Some("PUSH_ADDRESS"),
            Self::INDIRECT_ADDRESS => Some("INDIRECT_ADDRESS"),
            Self::HEAP_WITH_SHADER_RECORD_INDEX => Some("HEAP_WITH_SHADER_RECORD_INDEX"),
            Self::SHADER_RECORD_DATA => Some("SHADER_RECORD_DATA"),
            Self::SHADER_RECORD_ADDRESS => Some("SHADER_RECORD_ADDRESS"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for DescriptorPoolCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET.0,
                "FREE_DESCRIPTOR_SET",
            ),
            (DescriptorPoolCreateFlags::HOST_ONLY_EXT.0, "HOST_ONLY_EXT"),
            (
                DescriptorPoolCreateFlags::ALLOW_OVERALLOCATION_SETS_NV.0,
                "ALLOW_OVERALLOCATION_SETS_NV",
            ),
            (
                DescriptorPoolCreateFlags::ALLOW_OVERALLOCATION_POOLS_NV.0,
                "ALLOW_OVERALLOCATION_POOLS_NV",
            ),
            (
                DescriptorPoolCreateFlags::UPDATE_AFTER_BIND.0,
                "UPDATE_AFTER_BIND",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DescriptorPoolResetFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DescriptorSetLayoutCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                DescriptorSetLayoutCreateFlags::DESCRIPTOR_BUFFER_EXT.0,
                "DESCRIPTOR_BUFFER_EXT",
            ),
            (
                DescriptorSetLayoutCreateFlags::EMBEDDED_IMMUTABLE_SAMPLERS_EXT.0,
                "EMBEDDED_IMMUTABLE_SAMPLERS_EXT",
            ),
            (
                DescriptorSetLayoutCreateFlags::INDIRECT_BINDABLE_NV.0,
                "INDIRECT_BINDABLE_NV",
            ),
            (
                DescriptorSetLayoutCreateFlags::HOST_ONLY_POOL_EXT.0,
                "HOST_ONLY_POOL_EXT",
            ),
            (
                DescriptorSetLayoutCreateFlags::PER_STAGE_NV.0,
                "PER_STAGE_NV",
            ),
            (
                DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL.0,
                "UPDATE_AFTER_BIND_POOL",
            ),
            (
                DescriptorSetLayoutCreateFlags::PUSH_DESCRIPTOR.0,
                "PUSH_DESCRIPTOR",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DescriptorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::SAMPLER => Some("SAMPLER"),
            Self::COMBINED_IMAGE_SAMPLER => Some("COMBINED_IMAGE_SAMPLER"),
            Self::SAMPLED_IMAGE => Some("SAMPLED_IMAGE"),
            Self::STORAGE_IMAGE => Some("STORAGE_IMAGE"),
            Self::UNIFORM_TEXEL_BUFFER => Some("UNIFORM_TEXEL_BUFFER"),
            Self::STORAGE_TEXEL_BUFFER => Some("STORAGE_TEXEL_BUFFER"),
            Self::UNIFORM_BUFFER => Some("UNIFORM_BUFFER"),
            Self::STORAGE_BUFFER => Some("STORAGE_BUFFER"),
            Self::UNIFORM_BUFFER_DYNAMIC => Some("UNIFORM_BUFFER_DYNAMIC"),
            Self::STORAGE_BUFFER_DYNAMIC => Some("STORAGE_BUFFER_DYNAMIC"),
            Self::INPUT_ATTACHMENT => Some("INPUT_ATTACHMENT"),
            Self::ACCELERATION_STRUCTURE_KHR => Some("ACCELERATION_STRUCTURE_KHR"),
            Self::ACCELERATION_STRUCTURE_NV => Some("ACCELERATION_STRUCTURE_NV"),
            Self::SAMPLE_WEIGHT_IMAGE_QCOM => Some("SAMPLE_WEIGHT_IMAGE_QCOM"),
            Self::BLOCK_MATCH_IMAGE_QCOM => Some("BLOCK_MATCH_IMAGE_QCOM"),
            Self::TENSOR_ARM => Some("TENSOR_ARM"),
            Self::MUTABLE_EXT => Some("MUTABLE_EXT"),
            Self::PARTITIONED_ACCELERATION_STRUCTURE_NV => {
                Some("PARTITIONED_ACCELERATION_STRUCTURE_NV")
            }
            Self::INLINE_UNIFORM_BLOCK => Some("INLINE_UNIFORM_BLOCK"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for DescriptorUpdateTemplateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DescriptorUpdateTemplateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::DESCRIPTOR_SET => Some("DESCRIPTOR_SET"),
            Self::PUSH_DESCRIPTORS => Some("PUSH_DESCRIPTORS"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for DeviceCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DeviceGroupPresentModeFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (DeviceGroupPresentModeFlagsKHR::LOCAL.0, "LOCAL"),
            (DeviceGroupPresentModeFlagsKHR::REMOTE.0, "REMOTE"),
            (DeviceGroupPresentModeFlagsKHR::SUM.0, "SUM"),
            (
                DeviceGroupPresentModeFlagsKHR::LOCAL_MULTI_DEVICE.0,
                "LOCAL_MULTI_DEVICE",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DeviceQueueCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                DeviceQueueCreateFlags::INTERNALLY_SYNCHRONIZED_KHR.0,
                "INTERNALLY_SYNCHRONIZED_KHR",
            ),
            (DeviceQueueCreateFlags::PROTECTED.0, "PROTECTED"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DisplayModeCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DisplayPlaneAlphaFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (DisplayPlaneAlphaFlagsKHR::OPAQUE.0, "OPAQUE"),
            (DisplayPlaneAlphaFlagsKHR::GLOBAL.0, "GLOBAL"),
            (DisplayPlaneAlphaFlagsKHR::PER_PIXEL.0, "PER_PIXEL"),
            (
                DisplayPlaneAlphaFlagsKHR::PER_PIXEL_PREMULTIPLIED.0,
                "PER_PIXEL_PREMULTIPLIED",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DisplaySurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for DriverId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::AMD_PROPRIETARY => Some("AMD_PROPRIETARY"),
            Self::AMD_OPEN_SOURCE => Some("AMD_OPEN_SOURCE"),
            Self::MESA_RADV => Some("MESA_RADV"),
            Self::NVIDIA_PROPRIETARY => Some("NVIDIA_PROPRIETARY"),
            Self::INTEL_PROPRIETARY_WINDOWS => Some("INTEL_PROPRIETARY_WINDOWS"),
            Self::INTEL_OPEN_SOURCE_MESA => Some("INTEL_OPEN_SOURCE_MESA"),
            Self::IMAGINATION_PROPRIETARY => Some("IMAGINATION_PROPRIETARY"),
            Self::QUALCOMM_PROPRIETARY => Some("QUALCOMM_PROPRIETARY"),
            Self::ARM_PROPRIETARY => Some("ARM_PROPRIETARY"),
            Self::GOOGLE_SWIFTSHADER => Some("GOOGLE_SWIFTSHADER"),
            Self::GGP_PROPRIETARY => Some("GGP_PROPRIETARY"),
            Self::BROADCOM_PROPRIETARY => Some("BROADCOM_PROPRIETARY"),
            Self::MESA_LLVMPIPE => Some("MESA_LLVMPIPE"),
            Self::MOLTENVK => Some("MOLTENVK"),
            Self::COREAVI_PROPRIETARY => Some("COREAVI_PROPRIETARY"),
            Self::JUICE_PROPRIETARY => Some("JUICE_PROPRIETARY"),
            Self::VERISILICON_PROPRIETARY => Some("VERISILICON_PROPRIETARY"),
            Self::MESA_TURNIP => Some("MESA_TURNIP"),
            Self::MESA_V3DV => Some("MESA_V3DV"),
            Self::MESA_PANVK => Some("MESA_PANVK"),
            Self::SAMSUNG_PROPRIETARY => Some("SAMSUNG_PROPRIETARY"),
            Self::MESA_VENUS => Some("MESA_VENUS"),
            Self::MESA_DOZEN => Some("MESA_DOZEN"),
            Self::MESA_NVK => Some("MESA_NVK"),
            Self::IMAGINATION_OPEN_SOURCE_MESA => Some("IMAGINATION_OPEN_SOURCE_MESA"),
            Self::MESA_HONEYKRISP => Some("MESA_HONEYKRISP"),
            Self::VULKAN_SC_EMULATION_ON_VULKAN => Some("VULKAN_SC_EMULATION_ON_VULKAN"),
            Self::MESA_KOSMICKRISP => Some("MESA_KOSMICKRISP"),
            Self::MESA_GFXSTREAM => Some("MESA_GFXSTREAM"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for DynamicState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::VIEWPORT => Some("VIEWPORT"),
            Self::SCISSOR => Some("SCISSOR"),
            Self::LINE_WIDTH => Some("LINE_WIDTH"),
            Self::DEPTH_BIAS => Some("DEPTH_BIAS"),
            Self::BLEND_CONSTANTS => Some("BLEND_CONSTANTS"),
            Self::DEPTH_BOUNDS => Some("DEPTH_BOUNDS"),
            Self::STENCIL_COMPARE_MASK => Some("STENCIL_COMPARE_MASK"),
            Self::STENCIL_WRITE_MASK => Some("STENCIL_WRITE_MASK"),
            Self::STENCIL_REFERENCE => Some("STENCIL_REFERENCE"),
            Self::VIEWPORT_W_SCALING_NV => Some("VIEWPORT_W_SCALING_NV"),
            Self::DISCARD_RECTANGLE_EXT => Some("DISCARD_RECTANGLE_EXT"),
            Self::DISCARD_RECTANGLE_ENABLE_EXT => Some("DISCARD_RECTANGLE_ENABLE_EXT"),
            Self::DISCARD_RECTANGLE_MODE_EXT => Some("DISCARD_RECTANGLE_MODE_EXT"),
            Self::SAMPLE_LOCATIONS_EXT => Some("SAMPLE_LOCATIONS_EXT"),
            Self::RAY_TRACING_PIPELINE_STACK_SIZE_KHR => {
                Some("RAY_TRACING_PIPELINE_STACK_SIZE_KHR")
            }
            Self::VIEWPORT_SHADING_RATE_PALETTE_NV => Some("VIEWPORT_SHADING_RATE_PALETTE_NV"),
            Self::VIEWPORT_COARSE_SAMPLE_ORDER_NV => Some("VIEWPORT_COARSE_SAMPLE_ORDER_NV"),
            Self::EXCLUSIVE_SCISSOR_ENABLE_NV => Some("EXCLUSIVE_SCISSOR_ENABLE_NV"),
            Self::EXCLUSIVE_SCISSOR_NV => Some("EXCLUSIVE_SCISSOR_NV"),
            Self::FRAGMENT_SHADING_RATE_KHR => Some("FRAGMENT_SHADING_RATE_KHR"),
            Self::VERTEX_INPUT_EXT => Some("VERTEX_INPUT_EXT"),
            Self::PATCH_CONTROL_POINTS_EXT => Some("PATCH_CONTROL_POINTS_EXT"),
            Self::LOGIC_OP_EXT => Some("LOGIC_OP_EXT"),
            Self::COLOR_WRITE_ENABLE_EXT => Some("COLOR_WRITE_ENABLE_EXT"),
            Self::DEPTH_CLAMP_ENABLE_EXT => Some("DEPTH_CLAMP_ENABLE_EXT"),
            Self::POLYGON_MODE_EXT => Some("POLYGON_MODE_EXT"),
            Self::RASTERIZATION_SAMPLES_EXT => Some("RASTERIZATION_SAMPLES_EXT"),
            Self::SAMPLE_MASK_EXT => Some("SAMPLE_MASK_EXT"),
            Self::ALPHA_TO_COVERAGE_ENABLE_EXT => Some("ALPHA_TO_COVERAGE_ENABLE_EXT"),
            Self::ALPHA_TO_ONE_ENABLE_EXT => Some("ALPHA_TO_ONE_ENABLE_EXT"),
            Self::LOGIC_OP_ENABLE_EXT => Some("LOGIC_OP_ENABLE_EXT"),
            Self::COLOR_BLEND_ENABLE_EXT => Some("COLOR_BLEND_ENABLE_EXT"),
            Self::COLOR_BLEND_EQUATION_EXT => Some("COLOR_BLEND_EQUATION_EXT"),
            Self::COLOR_WRITE_MASK_EXT => Some("COLOR_WRITE_MASK_EXT"),
            Self::TESSELLATION_DOMAIN_ORIGIN_EXT => Some("TESSELLATION_DOMAIN_ORIGIN_EXT"),
            Self::RASTERIZATION_STREAM_EXT => Some("RASTERIZATION_STREAM_EXT"),
            Self::CONSERVATIVE_RASTERIZATION_MODE_EXT => {
                Some("CONSERVATIVE_RASTERIZATION_MODE_EXT")
            }
            Self::EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT => {
                Some("EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT")
            }
            Self::DEPTH_CLIP_ENABLE_EXT => Some("DEPTH_CLIP_ENABLE_EXT"),
            Self::SAMPLE_LOCATIONS_ENABLE_EXT => Some("SAMPLE_LOCATIONS_ENABLE_EXT"),
            Self::COLOR_BLEND_ADVANCED_EXT => Some("COLOR_BLEND_ADVANCED_EXT"),
            Self::PROVOKING_VERTEX_MODE_EXT => Some("PROVOKING_VERTEX_MODE_EXT"),
            Self::LINE_RASTERIZATION_MODE_EXT => Some("LINE_RASTERIZATION_MODE_EXT"),
            Self::LINE_STIPPLE_ENABLE_EXT => Some("LINE_STIPPLE_ENABLE_EXT"),
            Self::DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT => Some("DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT"),
            Self::VIEWPORT_W_SCALING_ENABLE_NV => Some("VIEWPORT_W_SCALING_ENABLE_NV"),
            Self::VIEWPORT_SWIZZLE_NV => Some("VIEWPORT_SWIZZLE_NV"),
            Self::COVERAGE_TO_COLOR_ENABLE_NV => Some("COVERAGE_TO_COLOR_ENABLE_NV"),
            Self::COVERAGE_TO_COLOR_LOCATION_NV => Some("COVERAGE_TO_COLOR_LOCATION_NV"),
            Self::COVERAGE_MODULATION_MODE_NV => Some("COVERAGE_MODULATION_MODE_NV"),
            Self::COVERAGE_MODULATION_TABLE_ENABLE_NV => {
                Some("COVERAGE_MODULATION_TABLE_ENABLE_NV")
            }
            Self::COVERAGE_MODULATION_TABLE_NV => Some("COVERAGE_MODULATION_TABLE_NV"),
            Self::SHADING_RATE_IMAGE_ENABLE_NV => Some("SHADING_RATE_IMAGE_ENABLE_NV"),
            Self::REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV => {
                Some("REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV")
            }
            Self::COVERAGE_REDUCTION_MODE_NV => Some("COVERAGE_REDUCTION_MODE_NV"),
            Self::ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT => {
                Some("ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT")
            }
            Self::DEPTH_CLAMP_RANGE_EXT => Some("DEPTH_CLAMP_RANGE_EXT"),
            Self::CULL_MODE => Some("CULL_MODE"),
            Self::FRONT_FACE => Some("FRONT_FACE"),
            Self::PRIMITIVE_TOPOLOGY => Some("PRIMITIVE_TOPOLOGY"),
            Self::VIEWPORT_WITH_COUNT => Some("VIEWPORT_WITH_COUNT"),
            Self::SCISSOR_WITH_COUNT => Some("SCISSOR_WITH_COUNT"),
            Self::VERTEX_INPUT_BINDING_STRIDE => Some("VERTEX_INPUT_BINDING_STRIDE"),
            Self::DEPTH_TEST_ENABLE => Some("DEPTH_TEST_ENABLE"),
            Self::DEPTH_WRITE_ENABLE => Some("DEPTH_WRITE_ENABLE"),
            Self::DEPTH_COMPARE_OP => Some("DEPTH_COMPARE_OP"),
            Self::DEPTH_BOUNDS_TEST_ENABLE => Some("DEPTH_BOUNDS_TEST_ENABLE"),
            Self::STENCIL_TEST_ENABLE => Some("STENCIL_TEST_ENABLE"),
            Self::STENCIL_OP => Some("STENCIL_OP"),
            Self::RASTERIZER_DISCARD_ENABLE => Some("RASTERIZER_DISCARD_ENABLE"),
            Self::DEPTH_BIAS_ENABLE => Some("DEPTH_BIAS_ENABLE"),
            Self::PRIMITIVE_RESTART_ENABLE => Some("PRIMITIVE_RESTART_ENABLE"),
            Self::LINE_STIPPLE => Some("LINE_STIPPLE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for EventCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(EventCreateFlags::DEVICE_ONLY.0, "DEVICE_ONLY")];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ExternalFenceFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ExternalFenceFeatureFlags::EXPORTABLE.0, "EXPORTABLE"),
            (ExternalFenceFeatureFlags::IMPORTABLE.0, "IMPORTABLE"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ExternalFenceHandleTypeFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ExternalFenceHandleTypeFlags::OPAQUE_FD.0, "OPAQUE_FD"),
            (ExternalFenceHandleTypeFlags::OPAQUE_WIN32.0, "OPAQUE_WIN32"),
            (
                ExternalFenceHandleTypeFlags::OPAQUE_WIN32_KMT.0,
                "OPAQUE_WIN32_KMT",
            ),
            (ExternalFenceHandleTypeFlags::SYNC_FD.0, "SYNC_FD"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ExternalMemoryFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                ExternalMemoryFeatureFlags::DEDICATED_ONLY.0,
                "DEDICATED_ONLY",
            ),
            (ExternalMemoryFeatureFlags::EXPORTABLE.0, "EXPORTABLE"),
            (ExternalMemoryFeatureFlags::IMPORTABLE.0, "IMPORTABLE"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ExternalMemoryHandleTypeFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ExternalMemoryHandleTypeFlags::OPAQUE_FD.0, "OPAQUE_FD"),
            (
                ExternalMemoryHandleTypeFlags::OPAQUE_WIN32.0,
                "OPAQUE_WIN32",
            ),
            (
                ExternalMemoryHandleTypeFlags::OPAQUE_WIN32_KMT.0,
                "OPAQUE_WIN32_KMT",
            ),
            (
                ExternalMemoryHandleTypeFlags::D3D11_TEXTURE.0,
                "D3D11_TEXTURE",
            ),
            (
                ExternalMemoryHandleTypeFlags::D3D11_TEXTURE_KMT.0,
                "D3D11_TEXTURE_KMT",
            ),
            (ExternalMemoryHandleTypeFlags::D3D12_HEAP.0, "D3D12_HEAP"),
            (
                ExternalMemoryHandleTypeFlags::D3D12_RESOURCE.0,
                "D3D12_RESOURCE",
            ),
            (ExternalMemoryHandleTypeFlags::DMA_BUF_EXT.0, "DMA_BUF_EXT"),
            (
                ExternalMemoryHandleTypeFlags::ANDROID_HARDWARE_BUFFER_ANDROID.0,
                "ANDROID_HARDWARE_BUFFER_ANDROID",
            ),
            (
                ExternalMemoryHandleTypeFlags::HOST_ALLOCATION_EXT.0,
                "HOST_ALLOCATION_EXT",
            ),
            (
                ExternalMemoryHandleTypeFlags::HOST_MAPPED_FOREIGN_MEMORY_EXT.0,
                "HOST_MAPPED_FOREIGN_MEMORY_EXT",
            ),
            (
                ExternalMemoryHandleTypeFlags::ZIRCON_VMO_FUCHSIA.0,
                "ZIRCON_VMO_FUCHSIA",
            ),
            (
                ExternalMemoryHandleTypeFlags::RDMA_ADDRESS_NV.0,
                "RDMA_ADDRESS_NV",
            ),
            (
                ExternalMemoryHandleTypeFlags::OH_NATIVE_BUFFER_OHOS.0,
                "OH_NATIVE_BUFFER_OHOS",
            ),
            (
                ExternalMemoryHandleTypeFlags::SCREEN_BUFFER_QNX.0,
                "SCREEN_BUFFER_QNX",
            ),
            (
                ExternalMemoryHandleTypeFlags::MTLBUFFER_EXT.0,
                "MTLBUFFER_EXT",
            ),
            (
                ExternalMemoryHandleTypeFlags::MTLTEXTURE_EXT.0,
                "MTLTEXTURE_EXT",
            ),
            (ExternalMemoryHandleTypeFlags::MTLHEAP_EXT.0, "MTLHEAP_EXT"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ExternalSemaphoreFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ExternalSemaphoreFeatureFlags::EXPORTABLE.0, "EXPORTABLE"),
            (ExternalSemaphoreFeatureFlags::IMPORTABLE.0, "IMPORTABLE"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ExternalSemaphoreHandleTypeFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ExternalSemaphoreHandleTypeFlags::OPAQUE_FD.0, "OPAQUE_FD"),
            (
                ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32.0,
                "OPAQUE_WIN32",
            ),
            (
                ExternalSemaphoreHandleTypeFlags::OPAQUE_WIN32_KMT.0,
                "OPAQUE_WIN32_KMT",
            ),
            (
                ExternalSemaphoreHandleTypeFlags::D3D12_FENCE.0,
                "D3D12_FENCE",
            ),
            (ExternalSemaphoreHandleTypeFlags::SYNC_FD.0, "SYNC_FD"),
            (
                ExternalSemaphoreHandleTypeFlags::ZIRCON_EVENT_FUCHSIA.0,
                "ZIRCON_EVENT_FUCHSIA",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for FenceCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(FenceCreateFlags::SIGNALED.0, "SIGNALED")];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for FenceImportFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(FenceImportFlags::TEMPORARY.0, "TEMPORARY")];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::NEAREST => Some("NEAREST"),
            Self::LINEAR => Some("LINEAR"),
            Self::CUBIC_EXT => Some("CUBIC_EXT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::UNDEFINED => Some("UNDEFINED"),
            Self::R4G4_UNORM_PACK8 => Some("R4G4_UNORM_PACK8"),
            Self::R4G4B4A4_UNORM_PACK16 => Some("R4G4B4A4_UNORM_PACK16"),
            Self::B4G4R4A4_UNORM_PACK16 => Some("B4G4R4A4_UNORM_PACK16"),
            Self::R5G6B5_UNORM_PACK16 => Some("R5G6B5_UNORM_PACK16"),
            Self::B5G6R5_UNORM_PACK16 => Some("B5G6R5_UNORM_PACK16"),
            Self::R5G5B5A1_UNORM_PACK16 => Some("R5G5B5A1_UNORM_PACK16"),
            Self::B5G5R5A1_UNORM_PACK16 => Some("B5G5R5A1_UNORM_PACK16"),
            Self::A1R5G5B5_UNORM_PACK16 => Some("A1R5G5B5_UNORM_PACK16"),
            Self::R8_UNORM => Some("R8_UNORM"),
            Self::R8_SNORM => Some("R8_SNORM"),
            Self::R8_USCALED => Some("R8_USCALED"),
            Self::R8_SSCALED => Some("R8_SSCALED"),
            Self::R8_UINT => Some("R8_UINT"),
            Self::R8_SINT => Some("R8_SINT"),
            Self::R8_SRGB => Some("R8_SRGB"),
            Self::R8G8_UNORM => Some("R8G8_UNORM"),
            Self::R8G8_SNORM => Some("R8G8_SNORM"),
            Self::R8G8_USCALED => Some("R8G8_USCALED"),
            Self::R8G8_SSCALED => Some("R8G8_SSCALED"),
            Self::R8G8_UINT => Some("R8G8_UINT"),
            Self::R8G8_SINT => Some("R8G8_SINT"),
            Self::R8G8_SRGB => Some("R8G8_SRGB"),
            Self::R8G8B8_UNORM => Some("R8G8B8_UNORM"),
            Self::R8G8B8_SNORM => Some("R8G8B8_SNORM"),
            Self::R8G8B8_USCALED => Some("R8G8B8_USCALED"),
            Self::R8G8B8_SSCALED => Some("R8G8B8_SSCALED"),
            Self::R8G8B8_UINT => Some("R8G8B8_UINT"),
            Self::R8G8B8_SINT => Some("R8G8B8_SINT"),
            Self::R8G8B8_SRGB => Some("R8G8B8_SRGB"),
            Self::B8G8R8_UNORM => Some("B8G8R8_UNORM"),
            Self::B8G8R8_SNORM => Some("B8G8R8_SNORM"),
            Self::B8G8R8_USCALED => Some("B8G8R8_USCALED"),
            Self::B8G8R8_SSCALED => Some("B8G8R8_SSCALED"),
            Self::B8G8R8_UINT => Some("B8G8R8_UINT"),
            Self::B8G8R8_SINT => Some("B8G8R8_SINT"),
            Self::B8G8R8_SRGB => Some("B8G8R8_SRGB"),
            Self::R8G8B8A8_UNORM => Some("R8G8B8A8_UNORM"),
            Self::R8G8B8A8_SNORM => Some("R8G8B8A8_SNORM"),
            Self::R8G8B8A8_USCALED => Some("R8G8B8A8_USCALED"),
            Self::R8G8B8A8_SSCALED => Some("R8G8B8A8_SSCALED"),
            Self::R8G8B8A8_UINT => Some("R8G8B8A8_UINT"),
            Self::R8G8B8A8_SINT => Some("R8G8B8A8_SINT"),
            Self::R8G8B8A8_SRGB => Some("R8G8B8A8_SRGB"),
            Self::B8G8R8A8_UNORM => Some("B8G8R8A8_UNORM"),
            Self::B8G8R8A8_SNORM => Some("B8G8R8A8_SNORM"),
            Self::B8G8R8A8_USCALED => Some("B8G8R8A8_USCALED"),
            Self::B8G8R8A8_SSCALED => Some("B8G8R8A8_SSCALED"),
            Self::B8G8R8A8_UINT => Some("B8G8R8A8_UINT"),
            Self::B8G8R8A8_SINT => Some("B8G8R8A8_SINT"),
            Self::B8G8R8A8_SRGB => Some("B8G8R8A8_SRGB"),
            Self::A8B8G8R8_UNORM_PACK32 => Some("A8B8G8R8_UNORM_PACK32"),
            Self::A8B8G8R8_SNORM_PACK32 => Some("A8B8G8R8_SNORM_PACK32"),
            Self::A8B8G8R8_USCALED_PACK32 => Some("A8B8G8R8_USCALED_PACK32"),
            Self::A8B8G8R8_SSCALED_PACK32 => Some("A8B8G8R8_SSCALED_PACK32"),
            Self::A8B8G8R8_UINT_PACK32 => Some("A8B8G8R8_UINT_PACK32"),
            Self::A8B8G8R8_SINT_PACK32 => Some("A8B8G8R8_SINT_PACK32"),
            Self::A8B8G8R8_SRGB_PACK32 => Some("A8B8G8R8_SRGB_PACK32"),
            Self::A2R10G10B10_UNORM_PACK32 => Some("A2R10G10B10_UNORM_PACK32"),
            Self::A2R10G10B10_SNORM_PACK32 => Some("A2R10G10B10_SNORM_PACK32"),
            Self::A2R10G10B10_USCALED_PACK32 => Some("A2R10G10B10_USCALED_PACK32"),
            Self::A2R10G10B10_SSCALED_PACK32 => Some("A2R10G10B10_SSCALED_PACK32"),
            Self::A2R10G10B10_UINT_PACK32 => Some("A2R10G10B10_UINT_PACK32"),
            Self::A2R10G10B10_SINT_PACK32 => Some("A2R10G10B10_SINT_PACK32"),
            Self::A2B10G10R10_UNORM_PACK32 => Some("A2B10G10R10_UNORM_PACK32"),
            Self::A2B10G10R10_SNORM_PACK32 => Some("A2B10G10R10_SNORM_PACK32"),
            Self::A2B10G10R10_USCALED_PACK32 => Some("A2B10G10R10_USCALED_PACK32"),
            Self::A2B10G10R10_SSCALED_PACK32 => Some("A2B10G10R10_SSCALED_PACK32"),
            Self::A2B10G10R10_UINT_PACK32 => Some("A2B10G10R10_UINT_PACK32"),
            Self::A2B10G10R10_SINT_PACK32 => Some("A2B10G10R10_SINT_PACK32"),
            Self::R16_UNORM => Some("R16_UNORM"),
            Self::R16_SNORM => Some("R16_SNORM"),
            Self::R16_USCALED => Some("R16_USCALED"),
            Self::R16_SSCALED => Some("R16_SSCALED"),
            Self::R16_UINT => Some("R16_UINT"),
            Self::R16_SINT => Some("R16_SINT"),
            Self::R16_SFLOAT => Some("R16_SFLOAT"),
            Self::R16G16_UNORM => Some("R16G16_UNORM"),
            Self::R16G16_SNORM => Some("R16G16_SNORM"),
            Self::R16G16_USCALED => Some("R16G16_USCALED"),
            Self::R16G16_SSCALED => Some("R16G16_SSCALED"),
            Self::R16G16_UINT => Some("R16G16_UINT"),
            Self::R16G16_SINT => Some("R16G16_SINT"),
            Self::R16G16_SFLOAT => Some("R16G16_SFLOAT"),
            Self::R16G16B16_UNORM => Some("R16G16B16_UNORM"),
            Self::R16G16B16_SNORM => Some("R16G16B16_SNORM"),
            Self::R16G16B16_USCALED => Some("R16G16B16_USCALED"),
            Self::R16G16B16_SSCALED => Some("R16G16B16_SSCALED"),
            Self::R16G16B16_UINT => Some("R16G16B16_UINT"),
            Self::R16G16B16_SINT => Some("R16G16B16_SINT"),
            Self::R16G16B16_SFLOAT => Some("R16G16B16_SFLOAT"),
            Self::R16G16B16A16_UNORM => Some("R16G16B16A16_UNORM"),
            Self::R16G16B16A16_SNORM => Some("R16G16B16A16_SNORM"),
            Self::R16G16B16A16_USCALED => Some("R16G16B16A16_USCALED"),
            Self::R16G16B16A16_SSCALED => Some("R16G16B16A16_SSCALED"),
            Self::R16G16B16A16_UINT => Some("R16G16B16A16_UINT"),
            Self::R16G16B16A16_SINT => Some("R16G16B16A16_SINT"),
            Self::R16G16B16A16_SFLOAT => Some("R16G16B16A16_SFLOAT"),
            Self::R32_UINT => Some("R32_UINT"),
            Self::R32_SINT => Some("R32_SINT"),
            Self::R32_SFLOAT => Some("R32_SFLOAT"),
            Self::R32G32_UINT => Some("R32G32_UINT"),
            Self::R32G32_SINT => Some("R32G32_SINT"),
            Self::R32G32_SFLOAT => Some("R32G32_SFLOAT"),
            Self::R32G32B32_UINT => Some("R32G32B32_UINT"),
            Self::R32G32B32_SINT => Some("R32G32B32_SINT"),
            Self::R32G32B32_SFLOAT => Some("R32G32B32_SFLOAT"),
            Self::R32G32B32A32_UINT => Some("R32G32B32A32_UINT"),
            Self::R32G32B32A32_SINT => Some("R32G32B32A32_SINT"),
            Self::R32G32B32A32_SFLOAT => Some("R32G32B32A32_SFLOAT"),
            Self::R64_UINT => Some("R64_UINT"),
            Self::R64_SINT => Some("R64_SINT"),
            Self::R64_SFLOAT => Some("R64_SFLOAT"),
            Self::R64G64_UINT => Some("R64G64_UINT"),
            Self::R64G64_SINT => Some("R64G64_SINT"),
            Self::R64G64_SFLOAT => Some("R64G64_SFLOAT"),
            Self::R64G64B64_UINT => Some("R64G64B64_UINT"),
            Self::R64G64B64_SINT => Some("R64G64B64_SINT"),
            Self::R64G64B64_SFLOAT => Some("R64G64B64_SFLOAT"),
            Self::R64G64B64A64_UINT => Some("R64G64B64A64_UINT"),
            Self::R64G64B64A64_SINT => Some("R64G64B64A64_SINT"),
            Self::R64G64B64A64_SFLOAT => Some("R64G64B64A64_SFLOAT"),
            Self::B10G11R11_UFLOAT_PACK32 => Some("B10G11R11_UFLOAT_PACK32"),
            Self::E5B9G9R9_UFLOAT_PACK32 => Some("E5B9G9R9_UFLOAT_PACK32"),
            Self::D16_UNORM => Some("D16_UNORM"),
            Self::X8_D24_UNORM_PACK32 => Some("X8_D24_UNORM_PACK32"),
            Self::D32_SFLOAT => Some("D32_SFLOAT"),
            Self::S8_UINT => Some("S8_UINT"),
            Self::D16_UNORM_S8_UINT => Some("D16_UNORM_S8_UINT"),
            Self::D24_UNORM_S8_UINT => Some("D24_UNORM_S8_UINT"),
            Self::D32_SFLOAT_S8_UINT => Some("D32_SFLOAT_S8_UINT"),
            Self::BC1_RGB_UNORM_BLOCK => Some("BC1_RGB_UNORM_BLOCK"),
            Self::BC1_RGB_SRGB_BLOCK => Some("BC1_RGB_SRGB_BLOCK"),
            Self::BC1_RGBA_UNORM_BLOCK => Some("BC1_RGBA_UNORM_BLOCK"),
            Self::BC1_RGBA_SRGB_BLOCK => Some("BC1_RGBA_SRGB_BLOCK"),
            Self::BC2_UNORM_BLOCK => Some("BC2_UNORM_BLOCK"),
            Self::BC2_SRGB_BLOCK => Some("BC2_SRGB_BLOCK"),
            Self::BC3_UNORM_BLOCK => Some("BC3_UNORM_BLOCK"),
            Self::BC3_SRGB_BLOCK => Some("BC3_SRGB_BLOCK"),
            Self::BC4_UNORM_BLOCK => Some("BC4_UNORM_BLOCK"),
            Self::BC4_SNORM_BLOCK => Some("BC4_SNORM_BLOCK"),
            Self::BC5_UNORM_BLOCK => Some("BC5_UNORM_BLOCK"),
            Self::BC5_SNORM_BLOCK => Some("BC5_SNORM_BLOCK"),
            Self::BC6H_UFLOAT_BLOCK => Some("BC6H_UFLOAT_BLOCK"),
            Self::BC6H_SFLOAT_BLOCK => Some("BC6H_SFLOAT_BLOCK"),
            Self::BC7_UNORM_BLOCK => Some("BC7_UNORM_BLOCK"),
            Self::BC7_SRGB_BLOCK => Some("BC7_SRGB_BLOCK"),
            Self::ETC2_R8G8B8_UNORM_BLOCK => Some("ETC2_R8G8B8_UNORM_BLOCK"),
            Self::ETC2_R8G8B8_SRGB_BLOCK => Some("ETC2_R8G8B8_SRGB_BLOCK"),
            Self::ETC2_R8G8B8A1_UNORM_BLOCK => Some("ETC2_R8G8B8A1_UNORM_BLOCK"),
            Self::ETC2_R8G8B8A1_SRGB_BLOCK => Some("ETC2_R8G8B8A1_SRGB_BLOCK"),
            Self::ETC2_R8G8B8A8_UNORM_BLOCK => Some("ETC2_R8G8B8A8_UNORM_BLOCK"),
            Self::ETC2_R8G8B8A8_SRGB_BLOCK => Some("ETC2_R8G8B8A8_SRGB_BLOCK"),
            Self::EAC_R11_UNORM_BLOCK => Some("EAC_R11_UNORM_BLOCK"),
            Self::EAC_R11_SNORM_BLOCK => Some("EAC_R11_SNORM_BLOCK"),
            Self::EAC_R11G11_UNORM_BLOCK => Some("EAC_R11G11_UNORM_BLOCK"),
            Self::EAC_R11G11_SNORM_BLOCK => Some("EAC_R11G11_SNORM_BLOCK"),
            Self::ASTC_4X4_UNORM_BLOCK => Some("ASTC_4X4_UNORM_BLOCK"),
            Self::ASTC_4X4_SRGB_BLOCK => Some("ASTC_4X4_SRGB_BLOCK"),
            Self::ASTC_5X4_UNORM_BLOCK => Some("ASTC_5X4_UNORM_BLOCK"),
            Self::ASTC_5X4_SRGB_BLOCK => Some("ASTC_5X4_SRGB_BLOCK"),
            Self::ASTC_5X5_UNORM_BLOCK => Some("ASTC_5X5_UNORM_BLOCK"),
            Self::ASTC_5X5_SRGB_BLOCK => Some("ASTC_5X5_SRGB_BLOCK"),
            Self::ASTC_6X5_UNORM_BLOCK => Some("ASTC_6X5_UNORM_BLOCK"),
            Self::ASTC_6X5_SRGB_BLOCK => Some("ASTC_6X5_SRGB_BLOCK"),
            Self::ASTC_6X6_UNORM_BLOCK => Some("ASTC_6X6_UNORM_BLOCK"),
            Self::ASTC_6X6_SRGB_BLOCK => Some("ASTC_6X6_SRGB_BLOCK"),
            Self::ASTC_8X5_UNORM_BLOCK => Some("ASTC_8X5_UNORM_BLOCK"),
            Self::ASTC_8X5_SRGB_BLOCK => Some("ASTC_8X5_SRGB_BLOCK"),
            Self::ASTC_8X6_UNORM_BLOCK => Some("ASTC_8X6_UNORM_BLOCK"),
            Self::ASTC_8X6_SRGB_BLOCK => Some("ASTC_8X6_SRGB_BLOCK"),
            Self::ASTC_8X8_UNORM_BLOCK => Some("ASTC_8X8_UNORM_BLOCK"),
            Self::ASTC_8X8_SRGB_BLOCK => Some("ASTC_8X8_SRGB_BLOCK"),
            Self::ASTC_10X5_UNORM_BLOCK => Some("ASTC_10X5_UNORM_BLOCK"),
            Self::ASTC_10X5_SRGB_BLOCK => Some("ASTC_10X5_SRGB_BLOCK"),
            Self::ASTC_10X6_UNORM_BLOCK => Some("ASTC_10X6_UNORM_BLOCK"),
            Self::ASTC_10X6_SRGB_BLOCK => Some("ASTC_10X6_SRGB_BLOCK"),
            Self::ASTC_10X8_UNORM_BLOCK => Some("ASTC_10X8_UNORM_BLOCK"),
            Self::ASTC_10X8_SRGB_BLOCK => Some("ASTC_10X8_SRGB_BLOCK"),
            Self::ASTC_10X10_UNORM_BLOCK => Some("ASTC_10X10_UNORM_BLOCK"),
            Self::ASTC_10X10_SRGB_BLOCK => Some("ASTC_10X10_SRGB_BLOCK"),
            Self::ASTC_12X10_UNORM_BLOCK => Some("ASTC_12X10_UNORM_BLOCK"),
            Self::ASTC_12X10_SRGB_BLOCK => Some("ASTC_12X10_SRGB_BLOCK"),
            Self::ASTC_12X12_UNORM_BLOCK => Some("ASTC_12X12_UNORM_BLOCK"),
            Self::ASTC_12X12_SRGB_BLOCK => Some("ASTC_12X12_SRGB_BLOCK"),
            Self::PVRTC1_2BPP_UNORM_BLOCK_IMG => Some("PVRTC1_2BPP_UNORM_BLOCK_IMG"),
            Self::PVRTC1_4BPP_UNORM_BLOCK_IMG => Some("PVRTC1_4BPP_UNORM_BLOCK_IMG"),
            Self::PVRTC2_2BPP_UNORM_BLOCK_IMG => Some("PVRTC2_2BPP_UNORM_BLOCK_IMG"),
            Self::PVRTC2_4BPP_UNORM_BLOCK_IMG => Some("PVRTC2_4BPP_UNORM_BLOCK_IMG"),
            Self::PVRTC1_2BPP_SRGB_BLOCK_IMG => Some("PVRTC1_2BPP_SRGB_BLOCK_IMG"),
            Self::PVRTC1_4BPP_SRGB_BLOCK_IMG => Some("PVRTC1_4BPP_SRGB_BLOCK_IMG"),
            Self::PVRTC2_2BPP_SRGB_BLOCK_IMG => Some("PVRTC2_2BPP_SRGB_BLOCK_IMG"),
            Self::PVRTC2_4BPP_SRGB_BLOCK_IMG => Some("PVRTC2_4BPP_SRGB_BLOCK_IMG"),
            Self::ASTC_3X3X3_UNORM_BLOCK_EXT => Some("ASTC_3X3X3_UNORM_BLOCK_EXT"),
            Self::ASTC_3X3X3_SRGB_BLOCK_EXT => Some("ASTC_3X3X3_SRGB_BLOCK_EXT"),
            Self::ASTC_3X3X3_SFLOAT_BLOCK_EXT => Some("ASTC_3X3X3_SFLOAT_BLOCK_EXT"),
            Self::ASTC_4X3X3_UNORM_BLOCK_EXT => Some("ASTC_4X3X3_UNORM_BLOCK_EXT"),
            Self::ASTC_4X3X3_SRGB_BLOCK_EXT => Some("ASTC_4X3X3_SRGB_BLOCK_EXT"),
            Self::ASTC_4X3X3_SFLOAT_BLOCK_EXT => Some("ASTC_4X3X3_SFLOAT_BLOCK_EXT"),
            Self::ASTC_4X4X3_UNORM_BLOCK_EXT => Some("ASTC_4X4X3_UNORM_BLOCK_EXT"),
            Self::ASTC_4X4X3_SRGB_BLOCK_EXT => Some("ASTC_4X4X3_SRGB_BLOCK_EXT"),
            Self::ASTC_4X4X3_SFLOAT_BLOCK_EXT => Some("ASTC_4X4X3_SFLOAT_BLOCK_EXT"),
            Self::ASTC_4X4X4_UNORM_BLOCK_EXT => Some("ASTC_4X4X4_UNORM_BLOCK_EXT"),
            Self::ASTC_4X4X4_SRGB_BLOCK_EXT => Some("ASTC_4X4X4_SRGB_BLOCK_EXT"),
            Self::ASTC_4X4X4_SFLOAT_BLOCK_EXT => Some("ASTC_4X4X4_SFLOAT_BLOCK_EXT"),
            Self::ASTC_5X4X4_UNORM_BLOCK_EXT => Some("ASTC_5X4X4_UNORM_BLOCK_EXT"),
            Self::ASTC_5X4X4_SRGB_BLOCK_EXT => Some("ASTC_5X4X4_SRGB_BLOCK_EXT"),
            Self::ASTC_5X4X4_SFLOAT_BLOCK_EXT => Some("ASTC_5X4X4_SFLOAT_BLOCK_EXT"),
            Self::ASTC_5X5X4_UNORM_BLOCK_EXT => Some("ASTC_5X5X4_UNORM_BLOCK_EXT"),
            Self::ASTC_5X5X4_SRGB_BLOCK_EXT => Some("ASTC_5X5X4_SRGB_BLOCK_EXT"),
            Self::ASTC_5X5X4_SFLOAT_BLOCK_EXT => Some("ASTC_5X5X4_SFLOAT_BLOCK_EXT"),
            Self::ASTC_5X5X5_UNORM_BLOCK_EXT => Some("ASTC_5X5X5_UNORM_BLOCK_EXT"),
            Self::ASTC_5X5X5_SRGB_BLOCK_EXT => Some("ASTC_5X5X5_SRGB_BLOCK_EXT"),
            Self::ASTC_5X5X5_SFLOAT_BLOCK_EXT => Some("ASTC_5X5X5_SFLOAT_BLOCK_EXT"),
            Self::ASTC_6X5X5_UNORM_BLOCK_EXT => Some("ASTC_6X5X5_UNORM_BLOCK_EXT"),
            Self::ASTC_6X5X5_SRGB_BLOCK_EXT => Some("ASTC_6X5X5_SRGB_BLOCK_EXT"),
            Self::ASTC_6X5X5_SFLOAT_BLOCK_EXT => Some("ASTC_6X5X5_SFLOAT_BLOCK_EXT"),
            Self::ASTC_6X6X5_UNORM_BLOCK_EXT => Some("ASTC_6X6X5_UNORM_BLOCK_EXT"),
            Self::ASTC_6X6X5_SRGB_BLOCK_EXT => Some("ASTC_6X6X5_SRGB_BLOCK_EXT"),
            Self::ASTC_6X6X5_SFLOAT_BLOCK_EXT => Some("ASTC_6X6X5_SFLOAT_BLOCK_EXT"),
            Self::ASTC_6X6X6_UNORM_BLOCK_EXT => Some("ASTC_6X6X6_UNORM_BLOCK_EXT"),
            Self::ASTC_6X6X6_SRGB_BLOCK_EXT => Some("ASTC_6X6X6_SRGB_BLOCK_EXT"),
            Self::ASTC_6X6X6_SFLOAT_BLOCK_EXT => Some("ASTC_6X6X6_SFLOAT_BLOCK_EXT"),
            Self::R8_BOOL_ARM => Some("R8_BOOL_ARM"),
            Self::R16_SFLOAT_FPENCODING_BFLOAT16_ARM => Some("R16_SFLOAT_FPENCODING_BFLOAT16_ARM"),
            Self::R8_SFLOAT_FPENCODING_FLOAT8E4M3_ARM => {
                Some("R8_SFLOAT_FPENCODING_FLOAT8E4M3_ARM")
            }
            Self::R8_SFLOAT_FPENCODING_FLOAT8E5M2_ARM => {
                Some("R8_SFLOAT_FPENCODING_FLOAT8E5M2_ARM")
            }
            Self::R16G16_SFIXED5_NV => Some("R16G16_SFIXED5_NV"),
            Self::R10X6_UINT_PACK16_ARM => Some("R10X6_UINT_PACK16_ARM"),
            Self::R10X6G10X6_UINT_2PACK16_ARM => Some("R10X6G10X6_UINT_2PACK16_ARM"),
            Self::R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM => {
                Some("R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM")
            }
            Self::R12X4_UINT_PACK16_ARM => Some("R12X4_UINT_PACK16_ARM"),
            Self::R12X4G12X4_UINT_2PACK16_ARM => Some("R12X4G12X4_UINT_2PACK16_ARM"),
            Self::R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM => {
                Some("R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM")
            }
            Self::R14X2_UINT_PACK16_ARM => Some("R14X2_UINT_PACK16_ARM"),
            Self::R14X2G14X2_UINT_2PACK16_ARM => Some("R14X2G14X2_UINT_2PACK16_ARM"),
            Self::R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM => {
                Some("R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM")
            }
            Self::R14X2_UNORM_PACK16_ARM => Some("R14X2_UNORM_PACK16_ARM"),
            Self::R14X2G14X2_UNORM_2PACK16_ARM => Some("R14X2G14X2_UNORM_2PACK16_ARM"),
            Self::R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM => {
                Some("R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM")
            }
            Self::G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM => {
                Some("G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM")
            }
            Self::G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM => {
                Some("G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM")
            }
            Self::G8B8G8R8_422_UNORM => Some("G8B8G8R8_422_UNORM"),
            Self::B8G8R8G8_422_UNORM => Some("B8G8R8G8_422_UNORM"),
            Self::G8_B8_R8_3PLANE_420_UNORM => Some("G8_B8_R8_3PLANE_420_UNORM"),
            Self::G8_B8R8_2PLANE_420_UNORM => Some("G8_B8R8_2PLANE_420_UNORM"),
            Self::G8_B8_R8_3PLANE_422_UNORM => Some("G8_B8_R8_3PLANE_422_UNORM"),
            Self::G8_B8R8_2PLANE_422_UNORM => Some("G8_B8R8_2PLANE_422_UNORM"),
            Self::G8_B8_R8_3PLANE_444_UNORM => Some("G8_B8_R8_3PLANE_444_UNORM"),
            Self::R10X6_UNORM_PACK16 => Some("R10X6_UNORM_PACK16"),
            Self::R10X6G10X6_UNORM_2PACK16 => Some("R10X6G10X6_UNORM_2PACK16"),
            Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => Some("R10X6G10X6B10X6A10X6_UNORM_4PACK16"),
            Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => {
                Some("G10X6B10X6G10X6R10X6_422_UNORM_4PACK16")
            }
            Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => {
                Some("B10X6G10X6R10X6G10X6_422_UNORM_4PACK16")
            }
            Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => {
                Some("G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16")
            }
            Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => {
                Some("G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16")
            }
            Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => {
                Some("G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16")
            }
            Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => {
                Some("G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16")
            }
            Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => {
                Some("G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16")
            }
            Self::R12X4_UNORM_PACK16 => Some("R12X4_UNORM_PACK16"),
            Self::R12X4G12X4_UNORM_2PACK16 => Some("R12X4G12X4_UNORM_2PACK16"),
            Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => Some("R12X4G12X4B12X4A12X4_UNORM_4PACK16"),
            Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => {
                Some("G12X4B12X4G12X4R12X4_422_UNORM_4PACK16")
            }
            Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => {
                Some("B12X4G12X4R12X4G12X4_422_UNORM_4PACK16")
            }
            Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => {
                Some("G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16")
            }
            Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => {
                Some("G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16")
            }
            Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => {
                Some("G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16")
            }
            Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => {
                Some("G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16")
            }
            Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => {
                Some("G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16")
            }
            Self::G16B16G16R16_422_UNORM => Some("G16B16G16R16_422_UNORM"),
            Self::B16G16R16G16_422_UNORM => Some("B16G16R16G16_422_UNORM"),
            Self::G16_B16_R16_3PLANE_420_UNORM => Some("G16_B16_R16_3PLANE_420_UNORM"),
            Self::G16_B16R16_2PLANE_420_UNORM => Some("G16_B16R16_2PLANE_420_UNORM"),
            Self::G16_B16_R16_3PLANE_422_UNORM => Some("G16_B16_R16_3PLANE_422_UNORM"),
            Self::G16_B16R16_2PLANE_422_UNORM => Some("G16_B16R16_2PLANE_422_UNORM"),
            Self::G16_B16_R16_3PLANE_444_UNORM => Some("G16_B16_R16_3PLANE_444_UNORM"),
            Self::G8_B8R8_2PLANE_444_UNORM => Some("G8_B8R8_2PLANE_444_UNORM"),
            Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16 => {
                Some("G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16")
            }
            Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16 => {
                Some("G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16")
            }
            Self::G16_B16R16_2PLANE_444_UNORM => Some("G16_B16R16_2PLANE_444_UNORM"),
            Self::A4R4G4B4_UNORM_PACK16 => Some("A4R4G4B4_UNORM_PACK16"),
            Self::A4B4G4R4_UNORM_PACK16 => Some("A4B4G4R4_UNORM_PACK16"),
            Self::ASTC_4X4_SFLOAT_BLOCK => Some("ASTC_4X4_SFLOAT_BLOCK"),
            Self::ASTC_5X4_SFLOAT_BLOCK => Some("ASTC_5X4_SFLOAT_BLOCK"),
            Self::ASTC_5X5_SFLOAT_BLOCK => Some("ASTC_5X5_SFLOAT_BLOCK"),
            Self::ASTC_6X5_SFLOAT_BLOCK => Some("ASTC_6X5_SFLOAT_BLOCK"),
            Self::ASTC_6X6_SFLOAT_BLOCK => Some("ASTC_6X6_SFLOAT_BLOCK"),
            Self::ASTC_8X5_SFLOAT_BLOCK => Some("ASTC_8X5_SFLOAT_BLOCK"),
            Self::ASTC_8X6_SFLOAT_BLOCK => Some("ASTC_8X6_SFLOAT_BLOCK"),
            Self::ASTC_8X8_SFLOAT_BLOCK => Some("ASTC_8X8_SFLOAT_BLOCK"),
            Self::ASTC_10X5_SFLOAT_BLOCK => Some("ASTC_10X5_SFLOAT_BLOCK"),
            Self::ASTC_10X6_SFLOAT_BLOCK => Some("ASTC_10X6_SFLOAT_BLOCK"),
            Self::ASTC_10X8_SFLOAT_BLOCK => Some("ASTC_10X8_SFLOAT_BLOCK"),
            Self::ASTC_10X10_SFLOAT_BLOCK => Some("ASTC_10X10_SFLOAT_BLOCK"),
            Self::ASTC_12X10_SFLOAT_BLOCK => Some("ASTC_12X10_SFLOAT_BLOCK"),
            Self::ASTC_12X12_SFLOAT_BLOCK => Some("ASTC_12X12_SFLOAT_BLOCK"),
            Self::A1B5G5R5_UNORM_PACK16 => Some("A1B5G5R5_UNORM_PACK16"),
            Self::A8_UNORM => Some("A8_UNORM"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for FormatFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN : & [(Flags , & str)] = & [(FormatFeatureFlags :: SAMPLED_IMAGE . 0 , "SAMPLED_IMAGE") , (FormatFeatureFlags :: STORAGE_IMAGE . 0 , "STORAGE_IMAGE") , (FormatFeatureFlags :: STORAGE_IMAGE_ATOMIC . 0 , "STORAGE_IMAGE_ATOMIC") , (FormatFeatureFlags :: UNIFORM_TEXEL_BUFFER . 0 , "UNIFORM_TEXEL_BUFFER") , (FormatFeatureFlags :: STORAGE_TEXEL_BUFFER . 0 , "STORAGE_TEXEL_BUFFER") , (FormatFeatureFlags :: STORAGE_TEXEL_BUFFER_ATOMIC . 0 , "STORAGE_TEXEL_BUFFER_ATOMIC") , (FormatFeatureFlags :: VERTEX_BUFFER . 0 , "VERTEX_BUFFER") , (FormatFeatureFlags :: COLOR_ATTACHMENT . 0 , "COLOR_ATTACHMENT") , (FormatFeatureFlags :: COLOR_ATTACHMENT_BLEND . 0 , "COLOR_ATTACHMENT_BLEND") , (FormatFeatureFlags :: DEPTH_STENCIL_ATTACHMENT . 0 , "DEPTH_STENCIL_ATTACHMENT") , (FormatFeatureFlags :: BLIT_SRC . 0 , "BLIT_SRC") , (FormatFeatureFlags :: BLIT_DST . 0 , "BLIT_DST") , (FormatFeatureFlags :: SAMPLED_IMAGE_FILTER_LINEAR . 0 , "SAMPLED_IMAGE_FILTER_LINEAR") , (FormatFeatureFlags :: VIDEO_DECODE_OUTPUT_KHR . 0 , "VIDEO_DECODE_OUTPUT_KHR") , (FormatFeatureFlags :: VIDEO_DECODE_DPB_KHR . 0 , "VIDEO_DECODE_DPB_KHR") , (FormatFeatureFlags :: ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR . 0 , "ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR") , (FormatFeatureFlags :: SAMPLED_IMAGE_FILTER_CUBIC_EXT . 0 , "SAMPLED_IMAGE_FILTER_CUBIC_EXT") , (FormatFeatureFlags :: FRAGMENT_DENSITY_MAP_EXT . 0 , "FRAGMENT_DENSITY_MAP_EXT") , (FormatFeatureFlags :: FRAGMENT_SHADING_RATE_ATTACHMENT_KHR . 0 , "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR") , (FormatFeatureFlags :: VIDEO_ENCODE_INPUT_KHR . 0 , "VIDEO_ENCODE_INPUT_KHR") , (FormatFeatureFlags :: VIDEO_ENCODE_DPB_KHR . 0 , "VIDEO_ENCODE_DPB_KHR") , (FormatFeatureFlags :: TRANSFER_SRC . 0 , "TRANSFER_SRC") , (FormatFeatureFlags :: TRANSFER_DST . 0 , "TRANSFER_DST") , (FormatFeatureFlags :: MIDPOINT_CHROMA_SAMPLES . 0 , "MIDPOINT_CHROMA_SAMPLES") , (FormatFeatureFlags :: SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER . 0 , "SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER") , (FormatFeatureFlags :: SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER . 0 , "SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER") , (FormatFeatureFlags :: SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT . 0 , "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT") , (FormatFeatureFlags :: SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE . 0 , "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE") , (FormatFeatureFlags :: DISJOINT . 0 , "DISJOINT") , (FormatFeatureFlags :: COSITED_CHROMA_SAMPLES . 0 , "COSITED_CHROMA_SAMPLES") , (FormatFeatureFlags :: SAMPLED_IMAGE_FILTER_MINMAX . 0 , "SAMPLED_IMAGE_FILTER_MINMAX")] ;
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for FormatFeatureFlags2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN : & [(Flags64 , & str)] = & [(FormatFeatureFlags2 :: SAMPLED_IMAGE . 0 , "SAMPLED_IMAGE") , (FormatFeatureFlags2 :: STORAGE_IMAGE . 0 , "STORAGE_IMAGE") , (FormatFeatureFlags2 :: STORAGE_IMAGE_ATOMIC . 0 , "STORAGE_IMAGE_ATOMIC") , (FormatFeatureFlags2 :: UNIFORM_TEXEL_BUFFER . 0 , "UNIFORM_TEXEL_BUFFER") , (FormatFeatureFlags2 :: STORAGE_TEXEL_BUFFER . 0 , "STORAGE_TEXEL_BUFFER") , (FormatFeatureFlags2 :: STORAGE_TEXEL_BUFFER_ATOMIC . 0 , "STORAGE_TEXEL_BUFFER_ATOMIC") , (FormatFeatureFlags2 :: VERTEX_BUFFER . 0 , "VERTEX_BUFFER") , (FormatFeatureFlags2 :: COLOR_ATTACHMENT . 0 , "COLOR_ATTACHMENT") , (FormatFeatureFlags2 :: COLOR_ATTACHMENT_BLEND . 0 , "COLOR_ATTACHMENT_BLEND") , (FormatFeatureFlags2 :: DEPTH_STENCIL_ATTACHMENT . 0 , "DEPTH_STENCIL_ATTACHMENT") , (FormatFeatureFlags2 :: BLIT_SRC . 0 , "BLIT_SRC") , (FormatFeatureFlags2 :: BLIT_DST . 0 , "BLIT_DST") , (FormatFeatureFlags2 :: SAMPLED_IMAGE_FILTER_LINEAR . 0 , "SAMPLED_IMAGE_FILTER_LINEAR") , (FormatFeatureFlags2 :: TRANSFER_SRC . 0 , "TRANSFER_SRC") , (FormatFeatureFlags2 :: TRANSFER_DST . 0 , "TRANSFER_DST") , (FormatFeatureFlags2 :: SAMPLED_IMAGE_FILTER_MINMAX . 0 , "SAMPLED_IMAGE_FILTER_MINMAX") , (FormatFeatureFlags2 :: MIDPOINT_CHROMA_SAMPLES . 0 , "MIDPOINT_CHROMA_SAMPLES") , (FormatFeatureFlags2 :: SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER . 0 , "SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER") , (FormatFeatureFlags2 :: SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER . 0 , "SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER") , (FormatFeatureFlags2 :: SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT . 0 , "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT") , (FormatFeatureFlags2 :: SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE . 0 , "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE") , (FormatFeatureFlags2 :: DISJOINT . 0 , "DISJOINT") , (FormatFeatureFlags2 :: COSITED_CHROMA_SAMPLES . 0 , "COSITED_CHROMA_SAMPLES") , (FormatFeatureFlags2 :: STORAGE_READ_WITHOUT_FORMAT . 0 , "STORAGE_READ_WITHOUT_FORMAT") , (FormatFeatureFlags2 :: STORAGE_WRITE_WITHOUT_FORMAT . 0 , "STORAGE_WRITE_WITHOUT_FORMAT") , (FormatFeatureFlags2 :: SAMPLED_IMAGE_DEPTH_COMPARISON . 0 , "SAMPLED_IMAGE_DEPTH_COMPARISON") , (FormatFeatureFlags2 :: VIDEO_DECODE_OUTPUT_KHR . 0 , "VIDEO_DECODE_OUTPUT_KHR") , (FormatFeatureFlags2 :: VIDEO_DECODE_DPB_KHR . 0 , "VIDEO_DECODE_DPB_KHR") , (FormatFeatureFlags2 :: ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR . 0 , "ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR") , (FormatFeatureFlags2 :: FRAGMENT_DENSITY_MAP_EXT . 0 , "FRAGMENT_DENSITY_MAP_EXT") , (FormatFeatureFlags2 :: FRAGMENT_SHADING_RATE_ATTACHMENT_KHR . 0 , "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR") , (FormatFeatureFlags2 :: VIDEO_ENCODE_INPUT_KHR . 0 , "VIDEO_ENCODE_INPUT_KHR") , (FormatFeatureFlags2 :: VIDEO_ENCODE_DPB_KHR . 0 , "VIDEO_ENCODE_DPB_KHR") , (FormatFeatureFlags2 :: BLOCK_MATCHING_SXD_QCOM . 0 , "BLOCK_MATCHING_SXD_QCOM") , (FormatFeatureFlags2 :: ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV . 0 , "ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV") , (FormatFeatureFlags2 :: LINEAR_COLOR_ATTACHMENT_NV . 0 , "LINEAR_COLOR_ATTACHMENT_NV") , (FormatFeatureFlags2 :: WEIGHT_IMAGE_QCOM . 0 , "WEIGHT_IMAGE_QCOM") , (FormatFeatureFlags2 :: WEIGHT_SAMPLED_IMAGE_QCOM . 0 , "WEIGHT_SAMPLED_IMAGE_QCOM") , (FormatFeatureFlags2 :: BLOCK_MATCHING_QCOM . 0 , "BLOCK_MATCHING_QCOM") , (FormatFeatureFlags2 :: BOX_FILTER_SAMPLED_QCOM . 0 , "BOX_FILTER_SAMPLED_QCOM") , (FormatFeatureFlags2 :: TENSOR_SHADER_ARM . 0 , "TENSOR_SHADER_ARM") , (FormatFeatureFlags2 :: TENSOR_IMAGE_ALIASING_ARM . 0 , "TENSOR_IMAGE_ALIASING_ARM") , (FormatFeatureFlags2 :: OPTICAL_FLOW_IMAGE_NV . 0 , "OPTICAL_FLOW_IMAGE_NV") , (FormatFeatureFlags2 :: OPTICAL_FLOW_VECTOR_NV . 0 , "OPTICAL_FLOW_VECTOR_NV") , (FormatFeatureFlags2 :: OPTICAL_FLOW_COST_NV . 0 , "OPTICAL_FLOW_COST_NV") , (FormatFeatureFlags2 :: TENSOR_DATA_GRAPH_ARM . 0 , "TENSOR_DATA_GRAPH_ARM") , (FormatFeatureFlags2 :: COPY_IMAGE_INDIRECT_DST_KHR . 0 , "COPY_IMAGE_INDIRECT_DST_KHR") , (FormatFeatureFlags2 :: VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR . 0 , "VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR") , (FormatFeatureFlags2 :: VIDEO_ENCODE_EMPHASIS_MAP_KHR . 0 , "VIDEO_ENCODE_EMPHASIS_MAP_KHR") , (FormatFeatureFlags2 :: DEPTH_COPY_ON_COMPUTE_QUEUE_KHR . 0 , "DEPTH_COPY_ON_COMPUTE_QUEUE_KHR") , (FormatFeatureFlags2 :: DEPTH_COPY_ON_TRANSFER_QUEUE_KHR . 0 , "DEPTH_COPY_ON_TRANSFER_QUEUE_KHR") , (FormatFeatureFlags2 :: STENCIL_COPY_ON_COMPUTE_QUEUE_KHR . 0 , "STENCIL_COPY_ON_COMPUTE_QUEUE_KHR") , (FormatFeatureFlags2 :: STENCIL_COPY_ON_TRANSFER_QUEUE_KHR . 0 , "STENCIL_COPY_ON_TRANSFER_QUEUE_KHR") , (FormatFeatureFlags2 :: DATA_GRAPH_OPTICAL_FLOW_IMAGE_ARM . 0 , "DATA_GRAPH_OPTICAL_FLOW_IMAGE_ARM") , (FormatFeatureFlags2 :: DATA_GRAPH_OPTICAL_FLOW_VECTOR_ARM . 0 , "DATA_GRAPH_OPTICAL_FLOW_VECTOR_ARM") , (FormatFeatureFlags2 :: DATA_GRAPH_OPTICAL_FLOW_COST_ARM . 0 , "DATA_GRAPH_OPTICAL_FLOW_COST_ARM") , (FormatFeatureFlags2 :: SAMPLED_IMAGE_FILTER_CUBIC . 0 , "SAMPLED_IMAGE_FILTER_CUBIC") , (FormatFeatureFlags2 :: HOST_IMAGE_TRANSFER . 0 , "HOST_IMAGE_TRANSFER")] ;
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for FramebufferCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(FramebufferCreateFlags::IMAGELESS.0, "IMAGELESS")];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for FrontFace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::COUNTER_CLOCKWISE => Some("COUNTER_CLOCKWISE"),
            Self::CLOCKWISE => Some("CLOCKWISE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for GeometryFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (GeometryFlagsKHR::OPAQUE.0, "OPAQUE"),
            (
                GeometryFlagsKHR::NO_DUPLICATE_ANY_HIT_INVOCATION.0,
                "NO_DUPLICATE_ANY_HIT_INVOCATION",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for GeometryInstanceFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                GeometryInstanceFlagsKHR::TRIANGLE_FACING_CULL_DISABLE.0,
                "TRIANGLE_FACING_CULL_DISABLE",
            ),
            (
                GeometryInstanceFlagsKHR::TRIANGLE_FLIP_FACING.0,
                "TRIANGLE_FLIP_FACING",
            ),
            (GeometryInstanceFlagsKHR::FORCE_OPAQUE.0, "FORCE_OPAQUE"),
            (
                GeometryInstanceFlagsKHR::FORCE_NO_OPAQUE.0,
                "FORCE_NO_OPAQUE",
            ),
            (
                GeometryInstanceFlagsKHR::FORCE_OPACITY_MICROMAP_2_STATE.0,
                "FORCE_OPACITY_MICROMAP_2_STATE",
            ),
            (
                GeometryInstanceFlagsKHR::DISABLE_OPACITY_MICROMAPS.0,
                "DISABLE_OPACITY_MICROMAPS",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for GeometryTypeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::TRIANGLES => Some("TRIANGLES"),
            Self::AABBS => Some("AABBS"),
            Self::INSTANCES => Some("INSTANCES"),
            Self::SPHERES_NV => Some("SPHERES_NV"),
            Self::LINEAR_SWEPT_SPHERES_NV => Some("LINEAR_SWEPT_SPHERES_NV"),
            #[cfg(feature = "provisional")]
            Self::DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX => {
                Some("DENSE_GEOMETRY_FORMAT_TRIANGLES_AMDX")
            }
            Self::MICROMAP => Some("MICROMAP"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for GraphicsPipelineLibraryFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                GraphicsPipelineLibraryFlagsEXT::VERTEX_INPUT_INTERFACE.0,
                "VERTEX_INPUT_INTERFACE",
            ),
            (
                GraphicsPipelineLibraryFlagsEXT::PRE_RASTERIZATION_SHADERS.0,
                "PRE_RASTERIZATION_SHADERS",
            ),
            (
                GraphicsPipelineLibraryFlagsEXT::FRAGMENT_SHADER.0,
                "FRAGMENT_SHADER",
            ),
            (
                GraphicsPipelineLibraryFlagsEXT::FRAGMENT_OUTPUT_INTERFACE.0,
                "FRAGMENT_OUTPUT_INTERFACE",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for HostImageCopyFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(HostImageCopyFlags::MEMCPY.0, "MEMCPY")];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ImageAspectFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ImageAspectFlags::COLOR.0, "COLOR"),
            (ImageAspectFlags::DEPTH.0, "DEPTH"),
            (ImageAspectFlags::STENCIL.0, "STENCIL"),
            (ImageAspectFlags::METADATA.0, "METADATA"),
            (ImageAspectFlags::MEMORY_PLANE_0_EXT.0, "MEMORY_PLANE_0_EXT"),
            (ImageAspectFlags::MEMORY_PLANE_1_EXT.0, "MEMORY_PLANE_1_EXT"),
            (ImageAspectFlags::MEMORY_PLANE_2_EXT.0, "MEMORY_PLANE_2_EXT"),
            (ImageAspectFlags::MEMORY_PLANE_3_EXT.0, "MEMORY_PLANE_3_EXT"),
            (ImageAspectFlags::PLANE_0.0, "PLANE_0"),
            (ImageAspectFlags::PLANE_1.0, "PLANE_1"),
            (ImageAspectFlags::PLANE_2.0, "PLANE_2"),
            (ImageAspectFlags::NONE.0, "NONE"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ImageCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ImageCreateFlags::SPARSE_BINDING.0, "SPARSE_BINDING"),
            (ImageCreateFlags::SPARSE_RESIDENCY.0, "SPARSE_RESIDENCY"),
            (ImageCreateFlags::SPARSE_ALIASED.0, "SPARSE_ALIASED"),
            (ImageCreateFlags::MUTABLE_FORMAT.0, "MUTABLE_FORMAT"),
            (ImageCreateFlags::CUBE_COMPATIBLE.0, "CUBE_COMPATIBLE"),
            (ImageCreateFlags::CORNER_SAMPLED_NV.0, "CORNER_SAMPLED_NV"),
            (
                ImageCreateFlags::DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT.0,
                "DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT",
            ),
            (
                ImageCreateFlags::SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT.0,
                "SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT",
            ),
            (ImageCreateFlags::SUBSAMPLED_EXT.0, "SUBSAMPLED_EXT"),
            (
                ImageCreateFlags::MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT.0,
                "MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT",
            ),
            (
                ImageCreateFlags::TYPE_2D_VIEW_COMPATIBLE_EXT.0,
                "TYPE_2D_VIEW_COMPATIBLE_EXT",
            ),
            (
                ImageCreateFlags::VIDEO_PROFILE_INDEPENDENT_KHR.0,
                "VIDEO_PROFILE_INDEPENDENT_KHR",
            ),
            (
                ImageCreateFlags::FRAGMENT_DENSITY_MAP_OFFSET_EXT.0,
                "FRAGMENT_DENSITY_MAP_OFFSET_EXT",
            ),
            (
                ImageCreateFlags::ALIAS_SINGLE_LAYER_DESCRIPTOR_KHR.0,
                "ALIAS_SINGLE_LAYER_DESCRIPTOR_KHR",
            ),
            (ImageCreateFlags::ALIAS.0, "ALIAS"),
            (
                ImageCreateFlags::SPLIT_INSTANCE_BIND_REGIONS.0,
                "SPLIT_INSTANCE_BIND_REGIONS",
            ),
            (
                ImageCreateFlags::TYPE_2D_ARRAY_COMPATIBLE.0,
                "TYPE_2D_ARRAY_COMPATIBLE",
            ),
            (
                ImageCreateFlags::BLOCK_TEXEL_VIEW_COMPATIBLE.0,
                "BLOCK_TEXEL_VIEW_COMPATIBLE",
            ),
            (ImageCreateFlags::EXTENDED_USAGE.0, "EXTENDED_USAGE"),
            (ImageCreateFlags::PROTECTED.0, "PROTECTED"),
            (ImageCreateFlags::DISJOINT.0, "DISJOINT"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ImageLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::UNDEFINED => Some("UNDEFINED"),
            Self::GENERAL => Some("GENERAL"),
            Self::COLOR_ATTACHMENT_OPTIMAL => Some("COLOR_ATTACHMENT_OPTIMAL"),
            Self::DEPTH_STENCIL_ATTACHMENT_OPTIMAL => Some("DEPTH_STENCIL_ATTACHMENT_OPTIMAL"),
            Self::DEPTH_STENCIL_READ_ONLY_OPTIMAL => Some("DEPTH_STENCIL_READ_ONLY_OPTIMAL"),
            Self::SHADER_READ_ONLY_OPTIMAL => Some("SHADER_READ_ONLY_OPTIMAL"),
            Self::TRANSFER_SRC_OPTIMAL => Some("TRANSFER_SRC_OPTIMAL"),
            Self::TRANSFER_DST_OPTIMAL => Some("TRANSFER_DST_OPTIMAL"),
            Self::PREINITIALIZED => Some("PREINITIALIZED"),
            Self::PRESENT_SRC_KHR => Some("PRESENT_SRC_KHR"),
            Self::VIDEO_DECODE_DST_KHR => Some("VIDEO_DECODE_DST_KHR"),
            Self::VIDEO_DECODE_SRC_KHR => Some("VIDEO_DECODE_SRC_KHR"),
            Self::VIDEO_DECODE_DPB_KHR => Some("VIDEO_DECODE_DPB_KHR"),
            Self::SHARED_PRESENT_KHR => Some("SHARED_PRESENT_KHR"),
            Self::FRAGMENT_DENSITY_MAP_OPTIMAL_EXT => Some("FRAGMENT_DENSITY_MAP_OPTIMAL_EXT"),
            Self::FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR => {
                Some("FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR")
            }
            Self::VIDEO_ENCODE_DST_KHR => Some("VIDEO_ENCODE_DST_KHR"),
            Self::VIDEO_ENCODE_SRC_KHR => Some("VIDEO_ENCODE_SRC_KHR"),
            Self::VIDEO_ENCODE_DPB_KHR => Some("VIDEO_ENCODE_DPB_KHR"),
            Self::ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT => {
                Some("ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT")
            }
            Self::TENSOR_ALIASING_ARM => Some("TENSOR_ALIASING_ARM"),
            Self::VIDEO_ENCODE_QUANTIZATION_MAP_KHR => Some("VIDEO_ENCODE_QUANTIZATION_MAP_KHR"),
            Self::ZERO_INITIALIZED_EXT => Some("ZERO_INITIALIZED_EXT"),
            Self::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL => {
                Some("DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL")
            }
            Self::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL => {
                Some("DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL")
            }
            Self::DEPTH_ATTACHMENT_OPTIMAL => Some("DEPTH_ATTACHMENT_OPTIMAL"),
            Self::DEPTH_READ_ONLY_OPTIMAL => Some("DEPTH_READ_ONLY_OPTIMAL"),
            Self::STENCIL_ATTACHMENT_OPTIMAL => Some("STENCIL_ATTACHMENT_OPTIMAL"),
            Self::STENCIL_READ_ONLY_OPTIMAL => Some("STENCIL_READ_ONLY_OPTIMAL"),
            Self::READ_ONLY_OPTIMAL => Some("READ_ONLY_OPTIMAL"),
            Self::ATTACHMENT_OPTIMAL => Some("ATTACHMENT_OPTIMAL"),
            Self::RENDERING_LOCAL_READ => Some("RENDERING_LOCAL_READ"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for ImageTiling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::OPTIMAL => Some("OPTIMAL"),
            Self::LINEAR => Some("LINEAR"),
            Self::DRM_FORMAT_MODIFIER_EXT => Some("DRM_FORMAT_MODIFIER_EXT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::TYPE_1D => Some("TYPE_1D"),
            Self::TYPE_2D => Some("TYPE_2D"),
            Self::TYPE_3D => Some("TYPE_3D"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for ImageUsageFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ImageUsageFlags::TRANSFER_SRC.0, "TRANSFER_SRC"),
            (ImageUsageFlags::TRANSFER_DST.0, "TRANSFER_DST"),
            (ImageUsageFlags::SAMPLED.0, "SAMPLED"),
            (ImageUsageFlags::STORAGE.0, "STORAGE"),
            (ImageUsageFlags::COLOR_ATTACHMENT.0, "COLOR_ATTACHMENT"),
            (
                ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT.0,
                "DEPTH_STENCIL_ATTACHMENT",
            ),
            (
                ImageUsageFlags::TRANSIENT_ATTACHMENT.0,
                "TRANSIENT_ATTACHMENT",
            ),
            (ImageUsageFlags::INPUT_ATTACHMENT.0, "INPUT_ATTACHMENT"),
            (
                ImageUsageFlags::VIDEO_DECODE_DST_KHR.0,
                "VIDEO_DECODE_DST_KHR",
            ),
            (
                ImageUsageFlags::VIDEO_DECODE_SRC_KHR.0,
                "VIDEO_DECODE_SRC_KHR",
            ),
            (
                ImageUsageFlags::VIDEO_DECODE_DPB_KHR.0,
                "VIDEO_DECODE_DPB_KHR",
            ),
            (
                ImageUsageFlags::FRAGMENT_DENSITY_MAP_EXT.0,
                "FRAGMENT_DENSITY_MAP_EXT",
            ),
            (
                ImageUsageFlags::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0,
                "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR",
            ),
            (
                ImageUsageFlags::VIDEO_ENCODE_DST_KHR.0,
                "VIDEO_ENCODE_DST_KHR",
            ),
            (
                ImageUsageFlags::VIDEO_ENCODE_SRC_KHR.0,
                "VIDEO_ENCODE_SRC_KHR",
            ),
            (
                ImageUsageFlags::VIDEO_ENCODE_DPB_KHR.0,
                "VIDEO_ENCODE_DPB_KHR",
            ),
            (
                ImageUsageFlags::ATTACHMENT_FEEDBACK_LOOP_EXT.0,
                "ATTACHMENT_FEEDBACK_LOOP_EXT",
            ),
            (
                ImageUsageFlags::INVOCATION_MASK_HUAWEI.0,
                "INVOCATION_MASK_HUAWEI",
            ),
            (ImageUsageFlags::SAMPLE_WEIGHT_QCOM.0, "SAMPLE_WEIGHT_QCOM"),
            (
                ImageUsageFlags::SAMPLE_BLOCK_MATCH_QCOM.0,
                "SAMPLE_BLOCK_MATCH_QCOM",
            ),
            (
                ImageUsageFlags::TENSOR_ALIASING_ARM.0,
                "TENSOR_ALIASING_ARM",
            ),
            (ImageUsageFlags::TILE_MEMORY_QCOM.0, "TILE_MEMORY_QCOM"),
            (
                ImageUsageFlags::VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR.0,
                "VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR",
            ),
            (
                ImageUsageFlags::VIDEO_ENCODE_EMPHASIS_MAP_KHR.0,
                "VIDEO_ENCODE_EMPHASIS_MAP_KHR",
            ),
            (ImageUsageFlags::HOST_TRANSFER.0, "HOST_TRANSFER"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ImageViewCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                ImageViewCreateFlags::FRAGMENT_DENSITY_MAP_DYNAMIC_EXT.0,
                "FRAGMENT_DENSITY_MAP_DYNAMIC_EXT",
            ),
            (
                ImageViewCreateFlags::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0,
                "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT",
            ),
            (
                ImageViewCreateFlags::FRAGMENT_DENSITY_MAP_DEFERRED_EXT.0,
                "FRAGMENT_DENSITY_MAP_DEFERRED_EXT",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ImageViewType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::TYPE_1D => Some("TYPE_1D"),
            Self::TYPE_2D => Some("TYPE_2D"),
            Self::TYPE_3D => Some("TYPE_3D"),
            Self::CUBE => Some("CUBE"),
            Self::TYPE_1D_ARRAY => Some("TYPE_1D_ARRAY"),
            Self::TYPE_2D_ARRAY => Some("TYPE_2D_ARRAY"),
            Self::CUBE_ARRAY => Some("CUBE_ARRAY"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for IndexType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::UINT16 => Some("UINT16"),
            Self::UINT32 => Some("UINT32"),
            Self::NONE_KHR => Some("NONE_KHR"),
            Self::UINT8 => Some("UINT8"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for InstanceCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(
            InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR.0,
            "ENUMERATE_PORTABILITY_KHR",
        )];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for InternalAllocationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::EXECUTABLE => Some("EXECUTABLE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for LineRasterizationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::DEFAULT => Some("DEFAULT"),
            Self::RECTANGULAR => Some("RECTANGULAR"),
            Self::BRESENHAM => Some("BRESENHAM"),
            Self::RECTANGULAR_SMOOTH => Some("RECTANGULAR_SMOOTH"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for LogicOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::CLEAR => Some("CLEAR"),
            Self::AND => Some("AND"),
            Self::AND_REVERSE => Some("AND_REVERSE"),
            Self::COPY => Some("COPY"),
            Self::AND_INVERTED => Some("AND_INVERTED"),
            Self::NO_OP => Some("NO_OP"),
            Self::XOR => Some("XOR"),
            Self::OR => Some("OR"),
            Self::NOR => Some("NOR"),
            Self::EQUIVALENT => Some("EQUIVALENT"),
            Self::INVERT => Some("INVERT"),
            Self::OR_REVERSE => Some("OR_REVERSE"),
            Self::COPY_INVERTED => Some("COPY_INVERTED"),
            Self::OR_INVERTED => Some("OR_INVERTED"),
            Self::NAND => Some("NAND"),
            Self::SET => Some("SET"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for MemoryAllocateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (MemoryAllocateFlags::DEVICE_MASK.0, "DEVICE_MASK"),
            (
                MemoryAllocateFlags::ZERO_INITIALIZE_EXT.0,
                "ZERO_INITIALIZE_EXT",
            ),
            (MemoryAllocateFlags::DEVICE_ADDRESS.0, "DEVICE_ADDRESS"),
            (
                MemoryAllocateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY.0,
                "DEVICE_ADDRESS_CAPTURE_REPLAY",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for MemoryHeapFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (MemoryHeapFlags::DEVICE_LOCAL.0, "DEVICE_LOCAL"),
            (MemoryHeapFlags::TILE_MEMORY_QCOM.0, "TILE_MEMORY_QCOM"),
            (MemoryHeapFlags::MULTI_INSTANCE.0, "MULTI_INSTANCE"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for MemoryMapFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(MemoryMapFlags::PLACED_EXT.0, "PLACED_EXT")];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for MemoryPropertyFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (MemoryPropertyFlags::DEVICE_LOCAL.0, "DEVICE_LOCAL"),
            (MemoryPropertyFlags::HOST_VISIBLE.0, "HOST_VISIBLE"),
            (MemoryPropertyFlags::HOST_COHERENT.0, "HOST_COHERENT"),
            (MemoryPropertyFlags::HOST_CACHED.0, "HOST_CACHED"),
            (MemoryPropertyFlags::LAZILY_ALLOCATED.0, "LAZILY_ALLOCATED"),
            (
                MemoryPropertyFlags::DEVICE_COHERENT_AMD.0,
                "DEVICE_COHERENT_AMD",
            ),
            (
                MemoryPropertyFlags::DEVICE_UNCACHED_AMD.0,
                "DEVICE_UNCACHED_AMD",
            ),
            (MemoryPropertyFlags::RDMA_CAPABLE_NV.0, "RDMA_CAPABLE_NV"),
            (MemoryPropertyFlags::PROTECTED.0, "PROTECTED"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for MemoryUnmapFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(MemoryUnmapFlags::RESERVE_EXT.0, "RESERVE_EXT")];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for MetalSurfaceCreateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PeerMemoryFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (PeerMemoryFeatureFlags::COPY_SRC.0, "COPY_SRC"),
            (PeerMemoryFeatureFlags::COPY_DST.0, "COPY_DST"),
            (PeerMemoryFeatureFlags::GENERIC_SRC.0, "GENERIC_SRC"),
            (PeerMemoryFeatureFlags::GENERIC_DST.0, "GENERIC_DST"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PhysicalDeviceLayeredApiKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::VULKAN => Some("VULKAN"),
            Self::D3D12 => Some("D3D12"),
            Self::METAL => Some("METAL"),
            Self::OPENGL => Some("OPENGL"),
            Self::OPENGLES => Some("OPENGLES"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for PhysicalDeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::OTHER => Some("OTHER"),
            Self::INTEGRATED_GPU => Some("INTEGRATED_GPU"),
            Self::DISCRETE_GPU => Some("DISCRETE_GPU"),
            Self::VIRTUAL_GPU => Some("VIRTUAL_GPU"),
            Self::CPU => Some("CPU"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for PipelineBindPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::GRAPHICS => Some("GRAPHICS"),
            Self::COMPUTE => Some("COMPUTE"),
            #[cfg(feature = "provisional")]
            Self::EXECUTION_GRAPH_AMDX => Some("EXECUTION_GRAPH_AMDX"),
            Self::RAY_TRACING_KHR => Some("RAY_TRACING_KHR"),
            Self::SUBPASS_SHADING_HUAWEI => Some("SUBPASS_SHADING_HUAWEI"),
            Self::DATA_GRAPH_ARM => Some("DATA_GRAPH_ARM"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for PipelineCacheCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                PipelineCacheCreateFlags::INTERNALLY_SYNCHRONIZED_MERGE_KHR.0,
                "INTERNALLY_SYNCHRONIZED_MERGE_KHR",
            ),
            (
                PipelineCacheCreateFlags::EXTERNALLY_SYNCHRONIZED.0,
                "EXTERNALLY_SYNCHRONIZED",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineCacheHeaderVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::ONE => Some("ONE"),
            Self::DATA_GRAPH_QCOM => Some("DATA_GRAPH_QCOM"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for PipelineColorBlendStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(
            PipelineColorBlendStateCreateFlags::RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT.0,
            "RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT",
        )];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                PipelineCreateFlags::DISABLE_OPTIMIZATION.0,
                "DISABLE_OPTIMIZATION",
            ),
            (
                PipelineCreateFlags::ALLOW_DERIVATIVES.0,
                "ALLOW_DERIVATIVES",
            ),
            (PipelineCreateFlags::DERIVATIVE.0, "DERIVATIVE"),
            (
                PipelineCreateFlags::RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR.0,
                "RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR",
            ),
            (
                PipelineCreateFlags::RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR.0,
                "RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR",
            ),
            (
                PipelineCreateFlags::RAY_TRACING_NO_NULL_MISS_SHADERS_KHR.0,
                "RAY_TRACING_NO_NULL_MISS_SHADERS_KHR",
            ),
            (
                PipelineCreateFlags::RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR.0,
                "RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR",
            ),
            (
                PipelineCreateFlags::RAY_TRACING_SKIP_TRIANGLES_KHR.0,
                "RAY_TRACING_SKIP_TRIANGLES_KHR",
            ),
            (
                PipelineCreateFlags::RAY_TRACING_SKIP_AABBS_KHR.0,
                "RAY_TRACING_SKIP_AABBS_KHR",
            ),
            (
                PipelineCreateFlags::RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR.0,
                "RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR",
            ),
            (PipelineCreateFlags::DEFER_COMPILE_NV.0, "DEFER_COMPILE_NV"),
            (
                PipelineCreateFlags::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT.0,
                "RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT",
            ),
            (
                PipelineCreateFlags::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0,
                "RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR",
            ),
            (
                PipelineCreateFlags::CAPTURE_STATISTICS_KHR.0,
                "CAPTURE_STATISTICS_KHR",
            ),
            (
                PipelineCreateFlags::CAPTURE_INTERNAL_REPRESENTATIONS_KHR.0,
                "CAPTURE_INTERNAL_REPRESENTATIONS_KHR",
            ),
            (
                PipelineCreateFlags::INDIRECT_BINDABLE_NV.0,
                "INDIRECT_BINDABLE_NV",
            ),
            (PipelineCreateFlags::LIBRARY_KHR.0, "LIBRARY_KHR"),
            (
                PipelineCreateFlags::DESCRIPTOR_BUFFER_EXT.0,
                "DESCRIPTOR_BUFFER_EXT",
            ),
            (
                PipelineCreateFlags::RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT.0,
                "RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT",
            ),
            (
                PipelineCreateFlags::LINK_TIME_OPTIMIZATION_EXT.0,
                "LINK_TIME_OPTIMIZATION_EXT",
            ),
            (
                PipelineCreateFlags::RAY_TRACING_ALLOW_MOTION_NV.0,
                "RAY_TRACING_ALLOW_MOTION_NV",
            ),
            (
                PipelineCreateFlags::COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT.0,
                "COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT",
            ),
            (
                PipelineCreateFlags::DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT.0,
                "DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT",
            ),
            #[cfg(feature = "provisional")]
            (
                PipelineCreateFlags::RAY_TRACING_DISPLACEMENT_MICROMAP_NV.0,
                "RAY_TRACING_DISPLACEMENT_MICROMAP_NV",
            ),
            (
                PipelineCreateFlags::RAY_TRACING_OPACITY_MICROMAP_KHR.0,
                "RAY_TRACING_OPACITY_MICROMAP_KHR",
            ),
            (PipelineCreateFlags::DISPATCH_BASE.0, "DISPATCH_BASE"),
            (
                PipelineCreateFlags::VIEW_INDEX_FROM_DEVICE_INDEX.0,
                "VIEW_INDEX_FROM_DEVICE_INDEX",
            ),
            (
                PipelineCreateFlags::FAIL_ON_PIPELINE_COMPILE_REQUIRED.0,
                "FAIL_ON_PIPELINE_COMPILE_REQUIRED",
            ),
            (
                PipelineCreateFlags::EARLY_RETURN_ON_FAILURE.0,
                "EARLY_RETURN_ON_FAILURE",
            ),
            (
                PipelineCreateFlags::NO_PROTECTED_ACCESS.0,
                "NO_PROTECTED_ACCESS",
            ),
            (
                PipelineCreateFlags::PROTECTED_ACCESS_ONLY.0,
                "PROTECTED_ACCESS_ONLY",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineCreateFlags2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags64, &str)] = &[
            (
                PipelineCreateFlags2::DISABLE_OPTIMIZATION.0,
                "DISABLE_OPTIMIZATION",
            ),
            (
                PipelineCreateFlags2::ALLOW_DERIVATIVES.0,
                "ALLOW_DERIVATIVES",
            ),
            (PipelineCreateFlags2::DERIVATIVE.0, "DERIVATIVE"),
            (
                PipelineCreateFlags2::VIEW_INDEX_FROM_DEVICE_INDEX.0,
                "VIEW_INDEX_FROM_DEVICE_INDEX",
            ),
            (PipelineCreateFlags2::DISPATCH_BASE.0, "DISPATCH_BASE"),
            (
                PipelineCreateFlags2::FAIL_ON_PIPELINE_COMPILE_REQUIRED.0,
                "FAIL_ON_PIPELINE_COMPILE_REQUIRED",
            ),
            (
                PipelineCreateFlags2::EARLY_RETURN_ON_FAILURE.0,
                "EARLY_RETURN_ON_FAILURE",
            ),
            (
                PipelineCreateFlags2::NO_PROTECTED_ACCESS.0,
                "NO_PROTECTED_ACCESS",
            ),
            (
                PipelineCreateFlags2::PROTECTED_ACCESS_ONLY.0,
                "PROTECTED_ACCESS_ONLY",
            ),
            #[cfg(feature = "provisional")]
            (
                PipelineCreateFlags2::EXECUTION_GRAPH_AMDX.0,
                "EXECUTION_GRAPH_AMDX",
            ),
            (
                PipelineCreateFlags2::DESCRIPTOR_HEAP_EXT.0,
                "DESCRIPTOR_HEAP_EXT",
            ),
            (
                PipelineCreateFlags2::RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV.0,
                "RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV",
            ),
            (
                PipelineCreateFlags2::ENABLE_LEGACY_DITHERING_EXT.0,
                "ENABLE_LEGACY_DITHERING_EXT",
            ),
            (PipelineCreateFlags2::DEFER_COMPILE_NV.0, "DEFER_COMPILE_NV"),
            (
                PipelineCreateFlags2::CAPTURE_STATISTICS_KHR.0,
                "CAPTURE_STATISTICS_KHR",
            ),
            (
                PipelineCreateFlags2::CAPTURE_INTERNAL_REPRESENTATIONS_KHR.0,
                "CAPTURE_INTERNAL_REPRESENTATIONS_KHR",
            ),
            (
                PipelineCreateFlags2::LINK_TIME_OPTIMIZATION_EXT.0,
                "LINK_TIME_OPTIMIZATION_EXT",
            ),
            (
                PipelineCreateFlags2::RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT.0,
                "RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT",
            ),
            (PipelineCreateFlags2::LIBRARY_KHR.0, "LIBRARY_KHR"),
            (
                PipelineCreateFlags2::RAY_TRACING_SKIP_TRIANGLES_KHR.0,
                "RAY_TRACING_SKIP_TRIANGLES_KHR",
            ),
            (
                PipelineCreateFlags2::RAY_TRACING_SKIP_AABBS_KHR.0,
                "RAY_TRACING_SKIP_AABBS_KHR",
            ),
            (
                PipelineCreateFlags2::RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR.0,
                "RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR",
            ),
            (
                PipelineCreateFlags2::RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR.0,
                "RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR",
            ),
            (
                PipelineCreateFlags2::RAY_TRACING_NO_NULL_MISS_SHADERS_KHR.0,
                "RAY_TRACING_NO_NULL_MISS_SHADERS_KHR",
            ),
            (
                PipelineCreateFlags2::RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR.0,
                "RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR",
            ),
            (
                PipelineCreateFlags2::RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR.0,
                "RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR",
            ),
            (
                PipelineCreateFlags2::INDIRECT_BINDABLE_NV.0,
                "INDIRECT_BINDABLE_NV",
            ),
            (
                PipelineCreateFlags2::RAY_TRACING_ALLOW_MOTION_NV.0,
                "RAY_TRACING_ALLOW_MOTION_NV",
            ),
            (
                PipelineCreateFlags2::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0,
                "RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR",
            ),
            (
                PipelineCreateFlags2::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT.0,
                "RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT",
            ),
            (
                PipelineCreateFlags2::COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT.0,
                "COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT",
            ),
            (
                PipelineCreateFlags2::DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT.0,
                "DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT",
            ),
            (
                PipelineCreateFlags2::RAY_TRACING_DISPLACEMENT_MICROMAP_NV.0,
                "RAY_TRACING_DISPLACEMENT_MICROMAP_NV",
            ),
            (
                PipelineCreateFlags2::DESCRIPTOR_BUFFER_EXT.0,
                "DESCRIPTOR_BUFFER_EXT",
            ),
            (
                PipelineCreateFlags2::DISALLOW_OPACITY_MICROMAP_ARM.0,
                "DISALLOW_OPACITY_MICROMAP_ARM",
            ),
            (
                PipelineCreateFlags2::INSTRUMENT_SHADERS_ARM.0,
                "INSTRUMENT_SHADERS_ARM",
            ),
            (PipelineCreateFlags2::CAPTURE_DATA_KHR.0, "CAPTURE_DATA_KHR"),
            (
                PipelineCreateFlags2::INDIRECT_BINDABLE_EXT.0,
                "INDIRECT_BINDABLE_EXT",
            ),
            (
                PipelineCreateFlags2::PER_LAYER_FRAGMENT_DENSITY_VALVE.0,
                "PER_LAYER_FRAGMENT_DENSITY_VALVE",
            ),
            (
                PipelineCreateFlags2::RAY_TRACING_OPACITY_MICROMAP_KHR.0,
                "RAY_TRACING_OPACITY_MICROMAP_KHR",
            ),
            (
                PipelineCreateFlags2::OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_KHR.0,
                "OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX_KHR",
            ),
            (
                PipelineCreateFlags2::TYPE_64_INDEXING_EXT.0,
                "TYPE_64_INDEXING_EXT",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineCreationFeedbackFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (PipelineCreationFeedbackFlags::VALID.0, "VALID"),
            (
                PipelineCreationFeedbackFlags::APPLICATION_PIPELINE_CACHE_HIT.0,
                "APPLICATION_PIPELINE_CACHE_HIT",
            ),
            (
                PipelineCreationFeedbackFlags::BASE_PIPELINE_ACCELERATION.0,
                "BASE_PIPELINE_ACCELERATION",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineDepthStencilStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN : & [(Flags , & str)] = & [(PipelineDepthStencilStateCreateFlags :: RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT . 0 , "RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT") , (PipelineDepthStencilStateCreateFlags :: RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT . 0 , "RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT")] ;
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineDynamicStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineInputAssemblyStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineLayoutCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                PipelineLayoutCreateFlags::INDEPENDENT_SETS_EXT.0,
                "INDEPENDENT_SETS_EXT",
            ),
            (
                PipelineLayoutCreateFlags::NO_TASK_SHADER_KHR.0,
                "NO_TASK_SHADER_KHR",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineMultisampleStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineRasterizationStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineRobustnessBufferBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::DEVICE_DEFAULT => Some("DEVICE_DEFAULT"),
            Self::DISABLED => Some("DISABLED"),
            Self::ROBUST_BUFFER_ACCESS => Some("ROBUST_BUFFER_ACCESS"),
            Self::ROBUST_BUFFER_ACCESS_2 => Some("ROBUST_BUFFER_ACCESS_2"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for PipelineRobustnessImageBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::DEVICE_DEFAULT => Some("DEVICE_DEFAULT"),
            Self::DISABLED => Some("DISABLED"),
            Self::ROBUST_IMAGE_ACCESS => Some("ROBUST_IMAGE_ACCESS"),
            Self::ROBUST_IMAGE_ACCESS_2 => Some("ROBUST_IMAGE_ACCESS_2"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for PipelineShaderStageCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                PipelineShaderStageCreateFlags::ALLOW_VARYING_SUBGROUP_SIZE.0,
                "ALLOW_VARYING_SUBGROUP_SIZE",
            ),
            (
                PipelineShaderStageCreateFlags::REQUIRE_FULL_SUBGROUPS.0,
                "REQUIRE_FULL_SUBGROUPS",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineStageFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (PipelineStageFlags::TOP_OF_PIPE.0, "TOP_OF_PIPE"),
            (PipelineStageFlags::DRAW_INDIRECT.0, "DRAW_INDIRECT"),
            (PipelineStageFlags::VERTEX_INPUT.0, "VERTEX_INPUT"),
            (PipelineStageFlags::VERTEX_SHADER.0, "VERTEX_SHADER"),
            (
                PipelineStageFlags::TESSELLATION_CONTROL_SHADER.0,
                "TESSELLATION_CONTROL_SHADER",
            ),
            (
                PipelineStageFlags::TESSELLATION_EVALUATION_SHADER.0,
                "TESSELLATION_EVALUATION_SHADER",
            ),
            (PipelineStageFlags::GEOMETRY_SHADER.0, "GEOMETRY_SHADER"),
            (PipelineStageFlags::FRAGMENT_SHADER.0, "FRAGMENT_SHADER"),
            (
                PipelineStageFlags::EARLY_FRAGMENT_TESTS.0,
                "EARLY_FRAGMENT_TESTS",
            ),
            (
                PipelineStageFlags::LATE_FRAGMENT_TESTS.0,
                "LATE_FRAGMENT_TESTS",
            ),
            (
                PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT.0,
                "COLOR_ATTACHMENT_OUTPUT",
            ),
            (PipelineStageFlags::COMPUTE_SHADER.0, "COMPUTE_SHADER"),
            (PipelineStageFlags::TRANSFER.0, "TRANSFER"),
            (PipelineStageFlags::BOTTOM_OF_PIPE.0, "BOTTOM_OF_PIPE"),
            (PipelineStageFlags::HOST.0, "HOST"),
            (PipelineStageFlags::ALL_GRAPHICS.0, "ALL_GRAPHICS"),
            (PipelineStageFlags::ALL_COMMANDS.0, "ALL_COMMANDS"),
            (
                PipelineStageFlags::TRANSFORM_FEEDBACK_EXT.0,
                "TRANSFORM_FEEDBACK_EXT",
            ),
            (
                PipelineStageFlags::CONDITIONAL_RENDERING_EXT.0,
                "CONDITIONAL_RENDERING_EXT",
            ),
            (
                PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR.0,
                "ACCELERATION_STRUCTURE_BUILD_KHR",
            ),
            (
                PipelineStageFlags::RAY_TRACING_SHADER_KHR.0,
                "RAY_TRACING_SHADER_KHR",
            ),
            (
                PipelineStageFlags::FRAGMENT_DENSITY_PROCESS_EXT.0,
                "FRAGMENT_DENSITY_PROCESS_EXT",
            ),
            (
                PipelineStageFlags::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0,
                "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR",
            ),
            (PipelineStageFlags::TASK_SHADER_EXT.0, "TASK_SHADER_EXT"),
            (PipelineStageFlags::MESH_SHADER_EXT.0, "MESH_SHADER_EXT"),
            (
                PipelineStageFlags::COMMAND_PREPROCESS_EXT.0,
                "COMMAND_PREPROCESS_EXT",
            ),
            (PipelineStageFlags::NONE.0, "NONE"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineStageFlags2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags64, &str)] = &[
            (PipelineStageFlags2::NONE.0, "NONE"),
            (PipelineStageFlags2::TOP_OF_PIPE.0, "TOP_OF_PIPE"),
            (PipelineStageFlags2::DRAW_INDIRECT.0, "DRAW_INDIRECT"),
            (PipelineStageFlags2::VERTEX_INPUT.0, "VERTEX_INPUT"),
            (PipelineStageFlags2::VERTEX_SHADER.0, "VERTEX_SHADER"),
            (
                PipelineStageFlags2::TESSELLATION_CONTROL_SHADER.0,
                "TESSELLATION_CONTROL_SHADER",
            ),
            (
                PipelineStageFlags2::TESSELLATION_EVALUATION_SHADER.0,
                "TESSELLATION_EVALUATION_SHADER",
            ),
            (PipelineStageFlags2::GEOMETRY_SHADER.0, "GEOMETRY_SHADER"),
            (PipelineStageFlags2::FRAGMENT_SHADER.0, "FRAGMENT_SHADER"),
            (
                PipelineStageFlags2::EARLY_FRAGMENT_TESTS.0,
                "EARLY_FRAGMENT_TESTS",
            ),
            (
                PipelineStageFlags2::LATE_FRAGMENT_TESTS.0,
                "LATE_FRAGMENT_TESTS",
            ),
            (
                PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT.0,
                "COLOR_ATTACHMENT_OUTPUT",
            ),
            (PipelineStageFlags2::COMPUTE_SHADER.0, "COMPUTE_SHADER"),
            (PipelineStageFlags2::ALL_TRANSFER.0, "ALL_TRANSFER"),
            (PipelineStageFlags2::BOTTOM_OF_PIPE.0, "BOTTOM_OF_PIPE"),
            (PipelineStageFlags2::HOST.0, "HOST"),
            (PipelineStageFlags2::ALL_GRAPHICS.0, "ALL_GRAPHICS"),
            (PipelineStageFlags2::ALL_COMMANDS.0, "ALL_COMMANDS"),
            (PipelineStageFlags2::COPY.0, "COPY"),
            (PipelineStageFlags2::RESOLVE.0, "RESOLVE"),
            (PipelineStageFlags2::BLIT.0, "BLIT"),
            (PipelineStageFlags2::CLEAR.0, "CLEAR"),
            (PipelineStageFlags2::INDEX_INPUT.0, "INDEX_INPUT"),
            (
                PipelineStageFlags2::VERTEX_ATTRIBUTE_INPUT.0,
                "VERTEX_ATTRIBUTE_INPUT",
            ),
            (
                PipelineStageFlags2::PRE_RASTERIZATION_SHADERS.0,
                "PRE_RASTERIZATION_SHADERS",
            ),
            (PipelineStageFlags2::VIDEO_DECODE_KHR.0, "VIDEO_DECODE_KHR"),
            (PipelineStageFlags2::VIDEO_ENCODE_KHR.0, "VIDEO_ENCODE_KHR"),
            (
                PipelineStageFlags2::TRANSFORM_FEEDBACK_EXT.0,
                "TRANSFORM_FEEDBACK_EXT",
            ),
            (
                PipelineStageFlags2::CONDITIONAL_RENDERING_EXT.0,
                "CONDITIONAL_RENDERING_EXT",
            ),
            (
                PipelineStageFlags2::COMMAND_PREPROCESS_EXT.0,
                "COMMAND_PREPROCESS_EXT",
            ),
            (
                PipelineStageFlags2::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0,
                "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR",
            ),
            (
                PipelineStageFlags2::ACCELERATION_STRUCTURE_BUILD_KHR.0,
                "ACCELERATION_STRUCTURE_BUILD_KHR",
            ),
            (
                PipelineStageFlags2::RAY_TRACING_SHADER_KHR.0,
                "RAY_TRACING_SHADER_KHR",
            ),
            (
                PipelineStageFlags2::FRAGMENT_DENSITY_PROCESS_EXT.0,
                "FRAGMENT_DENSITY_PROCESS_EXT",
            ),
            (PipelineStageFlags2::TASK_SHADER_EXT.0, "TASK_SHADER_EXT"),
            (PipelineStageFlags2::MESH_SHADER_EXT.0, "MESH_SHADER_EXT"),
            (
                PipelineStageFlags2::SUBPASS_SHADER_HUAWEI.0,
                "SUBPASS_SHADER_HUAWEI",
            ),
            (
                PipelineStageFlags2::INVOCATION_MASK_HUAWEI.0,
                "INVOCATION_MASK_HUAWEI",
            ),
            (
                PipelineStageFlags2::ACCELERATION_STRUCTURE_COPY_KHR.0,
                "ACCELERATION_STRUCTURE_COPY_KHR",
            ),
            (
                PipelineStageFlags2::MICROMAP_BUILD_EXT.0,
                "MICROMAP_BUILD_EXT",
            ),
            (
                PipelineStageFlags2::CLUSTER_CULLING_SHADER_HUAWEI.0,
                "CLUSTER_CULLING_SHADER_HUAWEI",
            ),
            (PipelineStageFlags2::OPTICAL_FLOW_NV.0, "OPTICAL_FLOW_NV"),
            (
                PipelineStageFlags2::CONVERT_COOPERATIVE_VECTOR_MATRIX_NV.0,
                "CONVERT_COOPERATIVE_VECTOR_MATRIX_NV",
            ),
            (PipelineStageFlags2::DATA_GRAPH_ARM.0, "DATA_GRAPH_ARM"),
            (
                PipelineStageFlags2::COPY_INDIRECT_KHR.0,
                "COPY_INDIRECT_KHR",
            ),
            (
                PipelineStageFlags2::MEMORY_DECOMPRESSION_EXT.0,
                "MEMORY_DECOMPRESSION_EXT",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineTessellationStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineVertexInputStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PipelineViewportStateCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for PointClippingBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::ALL_CLIP_PLANES => Some("ALL_CLIP_PLANES"),
            Self::USER_CLIP_PLANES_ONLY => Some("USER_CLIP_PLANES_ONLY"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for PolygonMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::FILL => Some("FILL"),
            Self::LINE => Some("LINE"),
            Self::POINT => Some("POINT"),
            Self::FILL_RECTANGLE_NV => Some("FILL_RECTANGLE_NV"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for PresentModeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::IMMEDIATE => Some("IMMEDIATE"),
            Self::MAILBOX => Some("MAILBOX"),
            Self::FIFO => Some("FIFO"),
            Self::FIFO_RELAXED => Some("FIFO_RELAXED"),
            Self::SHARED_DEMAND_REFRESH => Some("SHARED_DEMAND_REFRESH"),
            Self::SHARED_CONTINUOUS_REFRESH => Some("SHARED_CONTINUOUS_REFRESH"),
            Self::FIFO_LATEST_READY => Some("FIFO_LATEST_READY"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for PrimitiveTopology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::POINT_LIST => Some("POINT_LIST"),
            Self::LINE_LIST => Some("LINE_LIST"),
            Self::LINE_STRIP => Some("LINE_STRIP"),
            Self::TRIANGLE_LIST => Some("TRIANGLE_LIST"),
            Self::TRIANGLE_STRIP => Some("TRIANGLE_STRIP"),
            Self::TRIANGLE_FAN => Some("TRIANGLE_FAN"),
            Self::LINE_LIST_WITH_ADJACENCY => Some("LINE_LIST_WITH_ADJACENCY"),
            Self::LINE_STRIP_WITH_ADJACENCY => Some("LINE_STRIP_WITH_ADJACENCY"),
            Self::TRIANGLE_LIST_WITH_ADJACENCY => Some("TRIANGLE_LIST_WITH_ADJACENCY"),
            Self::TRIANGLE_STRIP_WITH_ADJACENCY => Some("TRIANGLE_STRIP_WITH_ADJACENCY"),
            Self::PATCH_LIST => Some("PATCH_LIST"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for PrivateDataSlotCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ProvokingVertexModeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::FIRST_VERTEX => Some("FIRST_VERTEX"),
            Self::LAST_VERTEX => Some("LAST_VERTEX"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for QueryControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(QueryControlFlags::PRECISE.0, "PRECISE")];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for QueryPipelineStatisticFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                QueryPipelineStatisticFlags::INPUT_ASSEMBLY_VERTICES.0,
                "INPUT_ASSEMBLY_VERTICES",
            ),
            (
                QueryPipelineStatisticFlags::INPUT_ASSEMBLY_PRIMITIVES.0,
                "INPUT_ASSEMBLY_PRIMITIVES",
            ),
            (
                QueryPipelineStatisticFlags::VERTEX_SHADER_INVOCATIONS.0,
                "VERTEX_SHADER_INVOCATIONS",
            ),
            (
                QueryPipelineStatisticFlags::GEOMETRY_SHADER_INVOCATIONS.0,
                "GEOMETRY_SHADER_INVOCATIONS",
            ),
            (
                QueryPipelineStatisticFlags::GEOMETRY_SHADER_PRIMITIVES.0,
                "GEOMETRY_SHADER_PRIMITIVES",
            ),
            (
                QueryPipelineStatisticFlags::CLIPPING_INVOCATIONS.0,
                "CLIPPING_INVOCATIONS",
            ),
            (
                QueryPipelineStatisticFlags::CLIPPING_PRIMITIVES.0,
                "CLIPPING_PRIMITIVES",
            ),
            (
                QueryPipelineStatisticFlags::FRAGMENT_SHADER_INVOCATIONS.0,
                "FRAGMENT_SHADER_INVOCATIONS",
            ),
            (
                QueryPipelineStatisticFlags::TESSELLATION_CONTROL_SHADER_PATCHES.0,
                "TESSELLATION_CONTROL_SHADER_PATCHES",
            ),
            (
                QueryPipelineStatisticFlags::TESSELLATION_EVALUATION_SHADER_INVOCATIONS.0,
                "TESSELLATION_EVALUATION_SHADER_INVOCATIONS",
            ),
            (
                QueryPipelineStatisticFlags::COMPUTE_SHADER_INVOCATIONS.0,
                "COMPUTE_SHADER_INVOCATIONS",
            ),
            (
                QueryPipelineStatisticFlags::TASK_SHADER_INVOCATIONS_EXT.0,
                "TASK_SHADER_INVOCATIONS_EXT",
            ),
            (
                QueryPipelineStatisticFlags::MESH_SHADER_INVOCATIONS_EXT.0,
                "MESH_SHADER_INVOCATIONS_EXT",
            ),
            (
                QueryPipelineStatisticFlags::CLUSTER_CULLING_SHADER_INVOCATIONS_HUAWEI.0,
                "CLUSTER_CULLING_SHADER_INVOCATIONS_HUAWEI",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for QueryPoolCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(QueryPoolCreateFlags::RESET_KHR.0, "RESET_KHR")];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for QueryResultFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (QueryResultFlags::TYPE_64.0, "TYPE_64"),
            (QueryResultFlags::WAIT.0, "WAIT"),
            (QueryResultFlags::WITH_AVAILABILITY.0, "WITH_AVAILABILITY"),
            (QueryResultFlags::PARTIAL.0, "PARTIAL"),
            (QueryResultFlags::WITH_STATUS_KHR.0, "WITH_STATUS_KHR"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for QueryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::OCCLUSION => Some("OCCLUSION"),
            Self::PIPELINE_STATISTICS => Some("PIPELINE_STATISTICS"),
            Self::TIMESTAMP => Some("TIMESTAMP"),
            Self::RESULT_STATUS_ONLY_KHR => Some("RESULT_STATUS_ONLY_KHR"),
            Self::TRANSFORM_FEEDBACK_STREAM_EXT => Some("TRANSFORM_FEEDBACK_STREAM_EXT"),
            Self::PERFORMANCE_QUERY_KHR => Some("PERFORMANCE_QUERY_KHR"),
            Self::ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR => {
                Some("ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR")
            }
            Self::ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR => {
                Some("ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR")
            }
            Self::ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV => {
                Some("ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV")
            }
            Self::TIME_ELAPSED_QCOM => Some("TIME_ELAPSED_QCOM"),
            Self::PERFORMANCE_QUERY_INTEL => Some("PERFORMANCE_QUERY_INTEL"),
            Self::VIDEO_ENCODE_FEEDBACK_KHR => Some("VIDEO_ENCODE_FEEDBACK_KHR"),
            Self::MESH_PRIMITIVES_GENERATED_EXT => Some("MESH_PRIMITIVES_GENERATED_EXT"),
            Self::PRIMITIVES_GENERATED_EXT => Some("PRIMITIVES_GENERATED_EXT"),
            Self::ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR => {
                Some("ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR")
            }
            Self::ACCELERATION_STRUCTURE_SIZE_KHR => Some("ACCELERATION_STRUCTURE_SIZE_KHR"),
            Self::MICROMAP_SERIALIZATION_SIZE_EXT => Some("MICROMAP_SERIALIZATION_SIZE_EXT"),
            Self::MICROMAP_COMPACTED_SIZE_EXT => Some("MICROMAP_COMPACTED_SIZE_EXT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for QueueFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (QueueFlags::GRAPHICS.0, "GRAPHICS"),
            (QueueFlags::COMPUTE.0, "COMPUTE"),
            (QueueFlags::TRANSFER.0, "TRANSFER"),
            (QueueFlags::SPARSE_BINDING.0, "SPARSE_BINDING"),
            (QueueFlags::VIDEO_DECODE_KHR.0, "VIDEO_DECODE_KHR"),
            (QueueFlags::VIDEO_ENCODE_KHR.0, "VIDEO_ENCODE_KHR"),
            (QueueFlags::OPTICAL_FLOW_NV.0, "OPTICAL_FLOW_NV"),
            (QueueFlags::DATA_GRAPH_ARM.0, "DATA_GRAPH_ARM"),
            (QueueFlags::PROTECTED.0, "PROTECTED"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for QueueGlobalPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::LOW => Some("LOW"),
            Self::MEDIUM => Some("MEDIUM"),
            Self::HIGH => Some("HIGH"),
            Self::REALTIME => Some("REALTIME"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for RayTracingShaderGroupTypeKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::GENERAL => Some("GENERAL"),
            Self::TRIANGLES_HIT_GROUP => Some("TRIANGLES_HIT_GROUP"),
            Self::PROCEDURAL_HIT_GROUP => Some("PROCEDURAL_HIT_GROUP"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for RenderPassCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (RenderPassCreateFlags::TRANSFORM_QCOM.0, "TRANSFORM_QCOM"),
            (
                RenderPassCreateFlags::PER_LAYER_FRAGMENT_DENSITY_VALVE.0,
                "PER_LAYER_FRAGMENT_DENSITY_VALVE",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for RenderingFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                RenderingFlags::CONTENTS_SECONDARY_COMMAND_BUFFERS.0,
                "CONTENTS_SECONDARY_COMMAND_BUFFERS",
            ),
            (RenderingFlags::SUSPENDING.0, "SUSPENDING"),
            (RenderingFlags::RESUMING.0, "RESUMING"),
            (
                RenderingFlags::ENABLE_LEGACY_DITHERING_EXT.0,
                "ENABLE_LEGACY_DITHERING_EXT",
            ),
            (RenderingFlags::CONTENTS_INLINE_KHR.0, "CONTENTS_INLINE_KHR"),
            (
                RenderingFlags::PER_LAYER_FRAGMENT_DENSITY_VALVE.0,
                "PER_LAYER_FRAGMENT_DENSITY_VALVE",
            ),
            (RenderingFlags::FRAGMENT_REGION_EXT.0, "FRAGMENT_REGION_EXT"),
            (RenderingFlags::CUSTOM_RESOLVE_EXT.0, "CUSTOM_RESOLVE_EXT"),
            (
                RenderingFlags::LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR.0,
                "LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ResolveModeFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ResolveModeFlags::NONE.0, "NONE"),
            (ResolveModeFlags::SAMPLE_ZERO.0, "SAMPLE_ZERO"),
            (ResolveModeFlags::AVERAGE.0, "AVERAGE"),
            (ResolveModeFlags::MIN.0, "MIN"),
            (ResolveModeFlags::MAX.0, "MAX"),
            (
                ResolveModeFlags::EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID.0,
                "EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID",
            ),
            (ResolveModeFlags::CUSTOM_EXT.0, "CUSTOM_EXT"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for SampleCountFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (SampleCountFlags::TYPE_1.0, "TYPE_1"),
            (SampleCountFlags::TYPE_2.0, "TYPE_2"),
            (SampleCountFlags::TYPE_4.0, "TYPE_4"),
            (SampleCountFlags::TYPE_8.0, "TYPE_8"),
            (SampleCountFlags::TYPE_16.0, "TYPE_16"),
            (SampleCountFlags::TYPE_32.0, "TYPE_32"),
            (SampleCountFlags::TYPE_64.0, "TYPE_64"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for SamplerAddressMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::REPEAT => Some("REPEAT"),
            Self::MIRRORED_REPEAT => Some("MIRRORED_REPEAT"),
            Self::CLAMP_TO_EDGE => Some("CLAMP_TO_EDGE"),
            Self::CLAMP_TO_BORDER => Some("CLAMP_TO_BORDER"),
            Self::MIRROR_CLAMP_TO_EDGE => Some("MIRROR_CLAMP_TO_EDGE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for SamplerCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (SamplerCreateFlags::SUBSAMPLED_EXT.0, "SUBSAMPLED_EXT"),
            (
                SamplerCreateFlags::SUBSAMPLED_COARSE_RECONSTRUCTION_EXT.0,
                "SUBSAMPLED_COARSE_RECONSTRUCTION_EXT",
            ),
            (
                SamplerCreateFlags::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0,
                "DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT",
            ),
            (
                SamplerCreateFlags::NON_SEAMLESS_CUBE_MAP_EXT.0,
                "NON_SEAMLESS_CUBE_MAP_EXT",
            ),
            (
                SamplerCreateFlags::IMAGE_PROCESSING_QCOM.0,
                "IMAGE_PROCESSING_QCOM",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for SamplerMipmapMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::NEAREST => Some("NEAREST"),
            Self::LINEAR => Some("LINEAR"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for SamplerReductionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::WEIGHTED_AVERAGE => Some("WEIGHTED_AVERAGE"),
            Self::MIN => Some("MIN"),
            Self::MAX => Some("MAX"),
            Self::WEIGHTED_AVERAGE_RANGECLAMP_QCOM => Some("WEIGHTED_AVERAGE_RANGECLAMP_QCOM"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for SamplerYcbcrModelConversion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::RGB_IDENTITY => Some("RGB_IDENTITY"),
            Self::YCBCR_IDENTITY => Some("YCBCR_IDENTITY"),
            Self::YCBCR_709 => Some("YCBCR_709"),
            Self::YCBCR_601 => Some("YCBCR_601"),
            Self::YCBCR_2020 => Some("YCBCR_2020"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for SamplerYcbcrRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::ITU_FULL => Some("ITU_FULL"),
            Self::ITU_NARROW => Some("ITU_NARROW"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for SemaphoreCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for SemaphoreImportFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(SemaphoreImportFlags::TEMPORARY.0, "TEMPORARY")];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for SemaphoreType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::BINARY => Some("BINARY"),
            Self::TIMELINE => Some("TIMELINE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for SemaphoreWaitFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(SemaphoreWaitFlags::ANY.0, "ANY")];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ShaderCodeTypeEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::BINARY => Some("BINARY"),
            Self::SPIRV => Some("SPIRV"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for ShaderCreateFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ShaderCreateFlagsEXT::LINK_STAGE.0, "LINK_STAGE"),
            (ShaderCreateFlagsEXT::DESCRIPTOR_HEAP.0, "DESCRIPTOR_HEAP"),
            (
                ShaderCreateFlagsEXT::INSTRUMENT_SHADER_ARM.0,
                "INSTRUMENT_SHADER_ARM",
            ),
            (
                ShaderCreateFlagsEXT::ALLOW_VARYING_SUBGROUP_SIZE.0,
                "ALLOW_VARYING_SUBGROUP_SIZE",
            ),
            (
                ShaderCreateFlagsEXT::REQUIRE_FULL_SUBGROUPS.0,
                "REQUIRE_FULL_SUBGROUPS",
            ),
            (ShaderCreateFlagsEXT::NO_TASK_SHADER.0, "NO_TASK_SHADER"),
            (ShaderCreateFlagsEXT::DISPATCH_BASE.0, "DISPATCH_BASE"),
            (
                ShaderCreateFlagsEXT::FRAGMENT_SHADING_RATE_ATTACHMENT.0,
                "FRAGMENT_SHADING_RATE_ATTACHMENT",
            ),
            (
                ShaderCreateFlagsEXT::FRAGMENT_DENSITY_MAP_ATTACHMENT.0,
                "FRAGMENT_DENSITY_MAP_ATTACHMENT",
            ),
            (
                ShaderCreateFlagsEXT::INDIRECT_BINDABLE.0,
                "INDIRECT_BINDABLE",
            ),
            (
                ShaderCreateFlagsEXT::OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX.0,
                "OPACITY_MICROMAP_DISALLOW_MIXED_SPECIAL_INDEX",
            ),
            (ShaderCreateFlagsEXT::TYPE_64_INDEXING.0, "TYPE_64_INDEXING"),
            (
                ShaderCreateFlagsEXT::INDEPENDENT_SETS_KHR.0,
                "INDEPENDENT_SETS_KHR",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ShaderFloatControlsIndependence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::TYPE_32_ONLY => Some("TYPE_32_ONLY"),
            Self::ALL => Some("ALL"),
            Self::NONE => Some("NONE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for ShaderGroupShaderKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::GENERAL => Some("GENERAL"),
            Self::CLOSEST_HIT => Some("CLOSEST_HIT"),
            Self::ANY_HIT => Some("ANY_HIT"),
            Self::INTERSECTION => Some("INTERSECTION"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for ShaderModuleCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for ShaderStageFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ShaderStageFlags::VERTEX.0, "VERTEX"),
            (
                ShaderStageFlags::TESSELLATION_CONTROL.0,
                "TESSELLATION_CONTROL",
            ),
            (
                ShaderStageFlags::TESSELLATION_EVALUATION.0,
                "TESSELLATION_EVALUATION",
            ),
            (ShaderStageFlags::GEOMETRY.0, "GEOMETRY"),
            (ShaderStageFlags::FRAGMENT.0, "FRAGMENT"),
            (ShaderStageFlags::COMPUTE.0, "COMPUTE"),
            (ShaderStageFlags::ALL_GRAPHICS.0, "ALL_GRAPHICS"),
            (ShaderStageFlags::ALL.0, "ALL"),
            (ShaderStageFlags::RAYGEN_KHR.0, "RAYGEN_KHR"),
            (ShaderStageFlags::ANY_HIT_KHR.0, "ANY_HIT_KHR"),
            (ShaderStageFlags::CLOSEST_HIT_KHR.0, "CLOSEST_HIT_KHR"),
            (ShaderStageFlags::MISS_KHR.0, "MISS_KHR"),
            (ShaderStageFlags::INTERSECTION_KHR.0, "INTERSECTION_KHR"),
            (ShaderStageFlags::CALLABLE_KHR.0, "CALLABLE_KHR"),
            (ShaderStageFlags::TASK_EXT.0, "TASK_EXT"),
            (ShaderStageFlags::MESH_EXT.0, "MESH_EXT"),
            (
                ShaderStageFlags::SUBPASS_SHADING_HUAWEI.0,
                "SUBPASS_SHADING_HUAWEI",
            ),
            (
                ShaderStageFlags::CLUSTER_CULLING_HUAWEI.0,
                "CLUSTER_CULLING_HUAWEI",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for SharingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::EXCLUSIVE => Some("EXCLUSIVE"),
            Self::CONCURRENT => Some("CONCURRENT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for SparseImageFormatFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (SparseImageFormatFlags::SINGLE_MIPTAIL.0, "SINGLE_MIPTAIL"),
            (
                SparseImageFormatFlags::ALIGNED_MIP_SIZE.0,
                "ALIGNED_MIP_SIZE",
            ),
            (
                SparseImageFormatFlags::NONSTANDARD_BLOCK_SIZE.0,
                "NONSTANDARD_BLOCK_SIZE",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for SparseMemoryBindFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(SparseMemoryBindFlags::METADATA.0, "METADATA")];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for SpirvResourceTypeFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (SpirvResourceTypeFlagsEXT::ALL.0, "ALL"),
            (SpirvResourceTypeFlagsEXT::SAMPLER.0, "SAMPLER"),
            (SpirvResourceTypeFlagsEXT::SAMPLED_IMAGE.0, "SAMPLED_IMAGE"),
            (
                SpirvResourceTypeFlagsEXT::READ_ONLY_IMAGE.0,
                "READ_ONLY_IMAGE",
            ),
            (
                SpirvResourceTypeFlagsEXT::READ_WRITE_IMAGE.0,
                "READ_WRITE_IMAGE",
            ),
            (
                SpirvResourceTypeFlagsEXT::COMBINED_SAMPLED_IMAGE.0,
                "COMBINED_SAMPLED_IMAGE",
            ),
            (
                SpirvResourceTypeFlagsEXT::UNIFORM_BUFFER.0,
                "UNIFORM_BUFFER",
            ),
            (
                SpirvResourceTypeFlagsEXT::READ_ONLY_STORAGE_BUFFER.0,
                "READ_ONLY_STORAGE_BUFFER",
            ),
            (
                SpirvResourceTypeFlagsEXT::READ_WRITE_STORAGE_BUFFER.0,
                "READ_WRITE_STORAGE_BUFFER",
            ),
            (
                SpirvResourceTypeFlagsEXT::ACCELERATION_STRUCTURE.0,
                "ACCELERATION_STRUCTURE",
            ),
            (SpirvResourceTypeFlagsEXT::TENSOR_ARM.0, "TENSOR_ARM"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for StencilFaceFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (StencilFaceFlags::FRONT.0, "FRONT"),
            (StencilFaceFlags::BACK.0, "BACK"),
            (StencilFaceFlags::FRONT_AND_BACK.0, "FRONT_AND_BACK"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for StencilOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::KEEP => Some("KEEP"),
            Self::ZERO => Some("ZERO"),
            Self::REPLACE => Some("REPLACE"),
            Self::INCREMENT_AND_CLAMP => Some("INCREMENT_AND_CLAMP"),
            Self::DECREMENT_AND_CLAMP => Some("DECREMENT_AND_CLAMP"),
            Self::INVERT => Some("INVERT"),
            Self::INCREMENT_AND_WRAP => Some("INCREMENT_AND_WRAP"),
            Self::DECREMENT_AND_WRAP => Some("DECREMENT_AND_WRAP"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for StructureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::APPLICATION_INFO => Some("APPLICATION_INFO"),
            Self::INSTANCE_CREATE_INFO => Some("INSTANCE_CREATE_INFO"),
            Self::DEVICE_QUEUE_CREATE_INFO => Some("DEVICE_QUEUE_CREATE_INFO"),
            Self::DEVICE_CREATE_INFO => Some("DEVICE_CREATE_INFO"),
            Self::SUBMIT_INFO => Some("SUBMIT_INFO"),
            Self::MEMORY_ALLOCATE_INFO => Some("MEMORY_ALLOCATE_INFO"),
            Self::MAPPED_MEMORY_RANGE => Some("MAPPED_MEMORY_RANGE"),
            Self::BIND_SPARSE_INFO => Some("BIND_SPARSE_INFO"),
            Self::FENCE_CREATE_INFO => Some("FENCE_CREATE_INFO"),
            Self::SEMAPHORE_CREATE_INFO => Some("SEMAPHORE_CREATE_INFO"),
            Self::EVENT_CREATE_INFO => Some("EVENT_CREATE_INFO"),
            Self::QUERY_POOL_CREATE_INFO => Some("QUERY_POOL_CREATE_INFO"),
            Self::BUFFER_CREATE_INFO => Some("BUFFER_CREATE_INFO"),
            Self::BUFFER_VIEW_CREATE_INFO => Some("BUFFER_VIEW_CREATE_INFO"),
            Self::IMAGE_CREATE_INFO => Some("IMAGE_CREATE_INFO"),
            Self::IMAGE_VIEW_CREATE_INFO => Some("IMAGE_VIEW_CREATE_INFO"),
            Self::SHADER_MODULE_CREATE_INFO => Some("SHADER_MODULE_CREATE_INFO"),
            Self::PIPELINE_CACHE_CREATE_INFO => Some("PIPELINE_CACHE_CREATE_INFO"),
            Self::PIPELINE_SHADER_STAGE_CREATE_INFO => Some("PIPELINE_SHADER_STAGE_CREATE_INFO"),
            Self::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO => {
                Some("PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO")
            }
            Self::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO => {
                Some("PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO")
            }
            Self::PIPELINE_TESSELLATION_STATE_CREATE_INFO => {
                Some("PIPELINE_TESSELLATION_STATE_CREATE_INFO")
            }
            Self::PIPELINE_VIEWPORT_STATE_CREATE_INFO => {
                Some("PIPELINE_VIEWPORT_STATE_CREATE_INFO")
            }
            Self::PIPELINE_RASTERIZATION_STATE_CREATE_INFO => {
                Some("PIPELINE_RASTERIZATION_STATE_CREATE_INFO")
            }
            Self::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO => {
                Some("PIPELINE_MULTISAMPLE_STATE_CREATE_INFO")
            }
            Self::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO => {
                Some("PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO")
            }
            Self::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO => {
                Some("PIPELINE_COLOR_BLEND_STATE_CREATE_INFO")
            }
            Self::PIPELINE_DYNAMIC_STATE_CREATE_INFO => Some("PIPELINE_DYNAMIC_STATE_CREATE_INFO"),
            Self::GRAPHICS_PIPELINE_CREATE_INFO => Some("GRAPHICS_PIPELINE_CREATE_INFO"),
            Self::COMPUTE_PIPELINE_CREATE_INFO => Some("COMPUTE_PIPELINE_CREATE_INFO"),
            Self::PIPELINE_LAYOUT_CREATE_INFO => Some("PIPELINE_LAYOUT_CREATE_INFO"),
            Self::SAMPLER_CREATE_INFO => Some("SAMPLER_CREATE_INFO"),
            Self::DESCRIPTOR_SET_LAYOUT_CREATE_INFO => Some("DESCRIPTOR_SET_LAYOUT_CREATE_INFO"),
            Self::DESCRIPTOR_POOL_CREATE_INFO => Some("DESCRIPTOR_POOL_CREATE_INFO"),
            Self::DESCRIPTOR_SET_ALLOCATE_INFO => Some("DESCRIPTOR_SET_ALLOCATE_INFO"),
            Self::WRITE_DESCRIPTOR_SET => Some("WRITE_DESCRIPTOR_SET"),
            Self::COPY_DESCRIPTOR_SET => Some("COPY_DESCRIPTOR_SET"),
            Self::FRAMEBUFFER_CREATE_INFO => Some("FRAMEBUFFER_CREATE_INFO"),
            Self::RENDER_PASS_CREATE_INFO => Some("RENDER_PASS_CREATE_INFO"),
            Self::COMMAND_POOL_CREATE_INFO => Some("COMMAND_POOL_CREATE_INFO"),
            Self::COMMAND_BUFFER_ALLOCATE_INFO => Some("COMMAND_BUFFER_ALLOCATE_INFO"),
            Self::COMMAND_BUFFER_INHERITANCE_INFO => Some("COMMAND_BUFFER_INHERITANCE_INFO"),
            Self::COMMAND_BUFFER_BEGIN_INFO => Some("COMMAND_BUFFER_BEGIN_INFO"),
            Self::RENDER_PASS_BEGIN_INFO => Some("RENDER_PASS_BEGIN_INFO"),
            Self::BUFFER_MEMORY_BARRIER => Some("BUFFER_MEMORY_BARRIER"),
            Self::IMAGE_MEMORY_BARRIER => Some("IMAGE_MEMORY_BARRIER"),
            Self::MEMORY_BARRIER => Some("MEMORY_BARRIER"),
            Self::LOADER_INSTANCE_CREATE_INFO => Some("LOADER_INSTANCE_CREATE_INFO"),
            Self::LOADER_DEVICE_CREATE_INFO => Some("LOADER_DEVICE_CREATE_INFO"),
            Self::SWAPCHAIN_CREATE_INFO_KHR => Some("SWAPCHAIN_CREATE_INFO_KHR"),
            Self::PRESENT_INFO_KHR => Some("PRESENT_INFO_KHR"),
            Self::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR => {
                Some("DEVICE_GROUP_PRESENT_CAPABILITIES_KHR")
            }
            Self::IMAGE_SWAPCHAIN_CREATE_INFO_KHR => Some("IMAGE_SWAPCHAIN_CREATE_INFO_KHR"),
            Self::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR => {
                Some("BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR")
            }
            Self::ACQUIRE_NEXT_IMAGE_INFO_KHR => Some("ACQUIRE_NEXT_IMAGE_INFO_KHR"),
            Self::DEVICE_GROUP_PRESENT_INFO_KHR => Some("DEVICE_GROUP_PRESENT_INFO_KHR"),
            Self::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR => {
                Some("DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR")
            }
            Self::DISPLAY_MODE_CREATE_INFO_KHR => Some("DISPLAY_MODE_CREATE_INFO_KHR"),
            Self::DISPLAY_SURFACE_CREATE_INFO_KHR => Some("DISPLAY_SURFACE_CREATE_INFO_KHR"),
            Self::DISPLAY_PRESENT_INFO_KHR => Some("DISPLAY_PRESENT_INFO_KHR"),
            Self::XLIB_SURFACE_CREATE_INFO_KHR => Some("XLIB_SURFACE_CREATE_INFO_KHR"),
            Self::XCB_SURFACE_CREATE_INFO_KHR => Some("XCB_SURFACE_CREATE_INFO_KHR"),
            Self::WAYLAND_SURFACE_CREATE_INFO_KHR => Some("WAYLAND_SURFACE_CREATE_INFO_KHR"),
            Self::ANDROID_SURFACE_CREATE_INFO_KHR => Some("ANDROID_SURFACE_CREATE_INFO_KHR"),
            Self::WIN32_SURFACE_CREATE_INFO_KHR => Some("WIN32_SURFACE_CREATE_INFO_KHR"),
            Self::NATIVE_BUFFER_ANDROID => Some("NATIVE_BUFFER_ANDROID"),
            Self::SWAPCHAIN_IMAGE_CREATE_INFO_ANDROID => {
                Some("SWAPCHAIN_IMAGE_CREATE_INFO_ANDROID")
            }
            Self::PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_ANDROID => {
                Some("PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_ANDROID")
            }
            Self::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT => {
                Some("DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT")
            }
            Self::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD => {
                Some("PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD")
            }
            Self::DEBUG_MARKER_OBJECT_NAME_INFO_EXT => Some("DEBUG_MARKER_OBJECT_NAME_INFO_EXT"),
            Self::DEBUG_MARKER_OBJECT_TAG_INFO_EXT => Some("DEBUG_MARKER_OBJECT_TAG_INFO_EXT"),
            Self::DEBUG_MARKER_MARKER_INFO_EXT => Some("DEBUG_MARKER_MARKER_INFO_EXT"),
            Self::VIDEO_PROFILE_INFO_KHR => Some("VIDEO_PROFILE_INFO_KHR"),
            Self::VIDEO_CAPABILITIES_KHR => Some("VIDEO_CAPABILITIES_KHR"),
            Self::VIDEO_PICTURE_RESOURCE_INFO_KHR => Some("VIDEO_PICTURE_RESOURCE_INFO_KHR"),
            Self::VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR => {
                Some("VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR")
            }
            Self::BIND_VIDEO_SESSION_MEMORY_INFO_KHR => Some("BIND_VIDEO_SESSION_MEMORY_INFO_KHR"),
            Self::VIDEO_SESSION_CREATE_INFO_KHR => Some("VIDEO_SESSION_CREATE_INFO_KHR"),
            Self::VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                Some("VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR => {
                Some("VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR")
            }
            Self::VIDEO_BEGIN_CODING_INFO_KHR => Some("VIDEO_BEGIN_CODING_INFO_KHR"),
            Self::VIDEO_END_CODING_INFO_KHR => Some("VIDEO_END_CODING_INFO_KHR"),
            Self::VIDEO_CODING_CONTROL_INFO_KHR => Some("VIDEO_CODING_CONTROL_INFO_KHR"),
            Self::VIDEO_REFERENCE_SLOT_INFO_KHR => Some("VIDEO_REFERENCE_SLOT_INFO_KHR"),
            Self::QUEUE_FAMILY_VIDEO_PROPERTIES_KHR => Some("QUEUE_FAMILY_VIDEO_PROPERTIES_KHR"),
            Self::VIDEO_PROFILE_LIST_INFO_KHR => Some("VIDEO_PROFILE_LIST_INFO_KHR"),
            Self::PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR => {
                Some("PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR")
            }
            Self::VIDEO_FORMAT_PROPERTIES_KHR => Some("VIDEO_FORMAT_PROPERTIES_KHR"),
            Self::QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR => {
                Some("QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR")
            }
            Self::VIDEO_DECODE_INFO_KHR => Some("VIDEO_DECODE_INFO_KHR"),
            Self::VIDEO_DECODE_CAPABILITIES_KHR => Some("VIDEO_DECODE_CAPABILITIES_KHR"),
            Self::VIDEO_DECODE_USAGE_INFO_KHR => Some("VIDEO_DECODE_USAGE_INFO_KHR"),
            Self::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV => {
                Some("DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV")
            }
            Self::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV => {
                Some("DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV")
            }
            Self::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV => {
                Some("DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT")
            }
            Self::PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT => {
                Some("PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT")
            }
            Self::CU_MODULE_CREATE_INFO_NVX => Some("CU_MODULE_CREATE_INFO_NVX"),
            Self::CU_FUNCTION_CREATE_INFO_NVX => Some("CU_FUNCTION_CREATE_INFO_NVX"),
            Self::CU_LAUNCH_INFO_NVX => Some("CU_LAUNCH_INFO_NVX"),
            Self::CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX => {
                Some("CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX")
            }
            Self::IMAGE_VIEW_HANDLE_INFO_NVX => Some("IMAGE_VIEW_HANDLE_INFO_NVX"),
            Self::IMAGE_VIEW_ADDRESS_PROPERTIES_NVX => Some("IMAGE_VIEW_ADDRESS_PROPERTIES_NVX"),
            Self::VIDEO_ENCODE_H264_CAPABILITIES_KHR => Some("VIDEO_ENCODE_H264_CAPABILITIES_KHR"),
            Self::VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                Some("VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR => {
                Some("VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_PICTURE_INFO_KHR => Some("VIDEO_ENCODE_H264_PICTURE_INFO_KHR"),
            Self::VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR => {
                Some("VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR => {
                Some("VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR => {
                Some("VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_PROFILE_INFO_KHR => Some("VIDEO_ENCODE_H264_PROFILE_INFO_KHR"),
            Self::VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR => {
                Some("VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR => {
                Some("VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR => {
                Some("VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR => {
                Some("VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR")
            }
            Self::VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR => {
                Some("VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR => {
                Some("VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_CAPABILITIES_KHR => Some("VIDEO_ENCODE_H265_CAPABILITIES_KHR"),
            Self::VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                Some("VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR => {
                Some("VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_PICTURE_INFO_KHR => Some("VIDEO_ENCODE_H265_PICTURE_INFO_KHR"),
            Self::VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR => {
                Some("VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR => {
                Some("VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR => {
                Some("VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_PROFILE_INFO_KHR => Some("VIDEO_ENCODE_H265_PROFILE_INFO_KHR"),
            Self::VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR => {
                Some("VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR => {
                Some("VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR => {
                Some("VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR => {
                Some("VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR")
            }
            Self::VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR => {
                Some("VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR")
            }
            Self::VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR => {
                Some("VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR")
            }
            Self::VIDEO_DECODE_H264_CAPABILITIES_KHR => Some("VIDEO_DECODE_H264_CAPABILITIES_KHR"),
            Self::VIDEO_DECODE_H264_PICTURE_INFO_KHR => Some("VIDEO_DECODE_H264_PICTURE_INFO_KHR"),
            Self::VIDEO_DECODE_H264_PROFILE_INFO_KHR => Some("VIDEO_DECODE_H264_PROFILE_INFO_KHR"),
            Self::VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                Some("VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR => {
                Some("VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR")
            }
            Self::VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR => {
                Some("VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR")
            }
            Self::TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD => {
                Some("TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD")
            }
            Self::STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP => {
                Some("STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP")
            }
            Self::PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV")
            }
            Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV => {
                Some("EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV")
            }
            Self::EXPORT_MEMORY_ALLOCATE_INFO_NV => Some("EXPORT_MEMORY_ALLOCATE_INFO_NV"),
            Self::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV => Some("IMPORT_MEMORY_WIN32_HANDLE_INFO_NV"),
            Self::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV => Some("EXPORT_MEMORY_WIN32_HANDLE_INFO_NV"),
            Self::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV => {
                Some("WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV")
            }
            Self::VALIDATION_FLAGS_EXT => Some("VALIDATION_FLAGS_EXT"),
            Self::VI_SURFACE_CREATE_INFO_NN => Some("VI_SURFACE_CREATE_INFO_NN"),
            Self::IMAGE_VIEW_ASTC_DECODE_MODE_EXT => Some("IMAGE_VIEW_ASTC_DECODE_MODE_EXT"),
            Self::PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT")
            }
            Self::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR => {
                Some("IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR")
            }
            Self::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR => {
                Some("EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR")
            }
            Self::MEMORY_WIN32_HANDLE_PROPERTIES_KHR => Some("MEMORY_WIN32_HANDLE_PROPERTIES_KHR"),
            Self::MEMORY_GET_WIN32_HANDLE_INFO_KHR => Some("MEMORY_GET_WIN32_HANDLE_INFO_KHR"),
            Self::IMPORT_MEMORY_FD_INFO_KHR => Some("IMPORT_MEMORY_FD_INFO_KHR"),
            Self::MEMORY_FD_PROPERTIES_KHR => Some("MEMORY_FD_PROPERTIES_KHR"),
            Self::MEMORY_GET_FD_INFO_KHR => Some("MEMORY_GET_FD_INFO_KHR"),
            Self::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR => {
                Some("WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR")
            }
            Self::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR => {
                Some("IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR")
            }
            Self::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR => {
                Some("EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR")
            }
            Self::D3D12_FENCE_SUBMIT_INFO_KHR => Some("D3D12_FENCE_SUBMIT_INFO_KHR"),
            Self::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR => {
                Some("SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR")
            }
            Self::IMPORT_SEMAPHORE_FD_INFO_KHR => Some("IMPORT_SEMAPHORE_FD_INFO_KHR"),
            Self::SEMAPHORE_GET_FD_INFO_KHR => Some("SEMAPHORE_GET_FD_INFO_KHR"),
            Self::COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT => {
                Some("COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT")
            }
            Self::CONDITIONAL_RENDERING_BEGIN_INFO_EXT => {
                Some("CONDITIONAL_RENDERING_BEGIN_INFO_EXT")
            }
            Self::PRESENT_REGIONS_KHR => Some("PRESENT_REGIONS_KHR"),
            Self::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV => {
                Some("PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV")
            }
            Self::SURFACE_CAPABILITIES_2_EXT => Some("SURFACE_CAPABILITIES_2_EXT"),
            Self::DISPLAY_POWER_INFO_EXT => Some("DISPLAY_POWER_INFO_EXT"),
            Self::DEVICE_EVENT_INFO_EXT => Some("DEVICE_EVENT_INFO_EXT"),
            Self::DISPLAY_EVENT_INFO_EXT => Some("DISPLAY_EVENT_INFO_EXT"),
            Self::SWAPCHAIN_COUNTER_CREATE_INFO_EXT => Some("SWAPCHAIN_COUNTER_CREATE_INFO_EXT"),
            Self::PRESENT_TIMES_INFO_GOOGLE => Some("PRESENT_TIMES_INFO_GOOGLE"),
            Self::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX => {
                Some("PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX")
            }
            Self::MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX => {
                Some("MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX")
            }
            Self::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV => {
                Some("PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT")
            }
            Self::PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT => {
                Some("PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT")
            }
            Self::PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT => {
                Some("PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT")
            }
            Self::PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT => {
                Some("PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT")
            }
            Self::HDR_METADATA_EXT => Some("HDR_METADATA_EXT"),
            Self::PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG => {
                Some("PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG")
            }
            Self::SHARED_PRESENT_SURFACE_CAPABILITIES_KHR => {
                Some("SHARED_PRESENT_SURFACE_CAPABILITIES_KHR")
            }
            Self::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR => Some("IMPORT_FENCE_WIN32_HANDLE_INFO_KHR"),
            Self::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR => Some("EXPORT_FENCE_WIN32_HANDLE_INFO_KHR"),
            Self::FENCE_GET_WIN32_HANDLE_INFO_KHR => Some("FENCE_GET_WIN32_HANDLE_INFO_KHR"),
            Self::IMPORT_FENCE_FD_INFO_KHR => Some("IMPORT_FENCE_FD_INFO_KHR"),
            Self::FENCE_GET_FD_INFO_KHR => Some("FENCE_GET_FD_INFO_KHR"),
            Self::PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR")
            }
            Self::QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR => {
                Some("QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR")
            }
            Self::PERFORMANCE_QUERY_SUBMIT_INFO_KHR => Some("PERFORMANCE_QUERY_SUBMIT_INFO_KHR"),
            Self::ACQUIRE_PROFILING_LOCK_INFO_KHR => Some("ACQUIRE_PROFILING_LOCK_INFO_KHR"),
            Self::PERFORMANCE_COUNTER_KHR => Some("PERFORMANCE_COUNTER_KHR"),
            Self::PERFORMANCE_COUNTER_DESCRIPTION_KHR => {
                Some("PERFORMANCE_COUNTER_DESCRIPTION_KHR")
            }
            Self::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR => Some("PHYSICAL_DEVICE_SURFACE_INFO_2_KHR"),
            Self::SURFACE_CAPABILITIES_2_KHR => Some("SURFACE_CAPABILITIES_2_KHR"),
            Self::SURFACE_FORMAT_2_KHR => Some("SURFACE_FORMAT_2_KHR"),
            Self::DISPLAY_PROPERTIES_2_KHR => Some("DISPLAY_PROPERTIES_2_KHR"),
            Self::DISPLAY_PLANE_PROPERTIES_2_KHR => Some("DISPLAY_PLANE_PROPERTIES_2_KHR"),
            Self::DISPLAY_MODE_PROPERTIES_2_KHR => Some("DISPLAY_MODE_PROPERTIES_2_KHR"),
            Self::DISPLAY_PLANE_INFO_2_KHR => Some("DISPLAY_PLANE_INFO_2_KHR"),
            Self::DISPLAY_PLANE_CAPABILITIES_2_KHR => Some("DISPLAY_PLANE_CAPABILITIES_2_KHR"),
            Self::IOS_SURFACE_CREATE_INFO_MVK => Some("IOS_SURFACE_CREATE_INFO_MVK"),
            Self::MACOS_SURFACE_CREATE_INFO_MVK => Some("MACOS_SURFACE_CREATE_INFO_MVK"),
            Self::DEBUG_UTILS_OBJECT_NAME_INFO_EXT => Some("DEBUG_UTILS_OBJECT_NAME_INFO_EXT"),
            Self::DEBUG_UTILS_OBJECT_TAG_INFO_EXT => Some("DEBUG_UTILS_OBJECT_TAG_INFO_EXT"),
            Self::DEBUG_UTILS_LABEL_EXT => Some("DEBUG_UTILS_LABEL_EXT"),
            Self::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT => {
                Some("DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT")
            }
            Self::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT => {
                Some("DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT")
            }
            Self::ANDROID_HARDWARE_BUFFER_USAGE_ANDROID => {
                Some("ANDROID_HARDWARE_BUFFER_USAGE_ANDROID")
            }
            Self::ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID => {
                Some("ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID")
            }
            Self::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID => {
                Some("ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID")
            }
            Self::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => {
                Some("IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID")
            }
            Self::MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => {
                Some("MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID")
            }
            Self::EXTERNAL_FORMAT_ANDROID => Some("EXTERNAL_FORMAT_ANDROID"),
            Self::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID => {
                Some("ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID")
            }
            Self::PHYSICAL_DEVICE_GPA_FEATURES_AMD => Some("PHYSICAL_DEVICE_GPA_FEATURES_AMD"),
            Self::PHYSICAL_DEVICE_GPA_PROPERTIES_AMD => Some("PHYSICAL_DEVICE_GPA_PROPERTIES_AMD"),
            Self::GPA_SAMPLE_BEGIN_INFO_AMD => Some("GPA_SAMPLE_BEGIN_INFO_AMD"),
            Self::GPA_SESSION_CREATE_INFO_AMD => Some("GPA_SESSION_CREATE_INFO_AMD"),
            Self::GPA_DEVICE_CLOCK_MODE_INFO_AMD => Some("GPA_DEVICE_CLOCK_MODE_INFO_AMD"),
            Self::PHYSICAL_DEVICE_GPA_PROPERTIES_2_AMD => {
                Some("PHYSICAL_DEVICE_GPA_PROPERTIES_2_AMD")
            }
            Self::GPA_DEVICE_GET_CLOCK_INFO_AMD => Some("GPA_DEVICE_GET_CLOCK_INFO_AMD"),
            #[cfg(feature = "provisional")]
            Self::PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX => {
                Some("PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX")
            }
            #[cfg(feature = "provisional")]
            Self::PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX => {
                Some("PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX")
            }
            #[cfg(feature = "provisional")]
            Self::EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX => {
                Some("EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX")
            }
            #[cfg(feature = "provisional")]
            Self::EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX => {
                Some("EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX")
            }
            #[cfg(feature = "provisional")]
            Self::PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX => {
                Some("PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX")
            }
            Self::TEXEL_BUFFER_DESCRIPTOR_INFO_EXT => Some("TEXEL_BUFFER_DESCRIPTOR_INFO_EXT"),
            Self::IMAGE_DESCRIPTOR_INFO_EXT => Some("IMAGE_DESCRIPTOR_INFO_EXT"),
            Self::RESOURCE_DESCRIPTOR_INFO_EXT => Some("RESOURCE_DESCRIPTOR_INFO_EXT"),
            Self::BIND_HEAP_INFO_EXT => Some("BIND_HEAP_INFO_EXT"),
            Self::PUSH_DATA_INFO_EXT => Some("PUSH_DATA_INFO_EXT"),
            Self::DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT => {
                Some("DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT")
            }
            Self::SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT => {
                Some("SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT")
            }
            Self::OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT => {
                Some("OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT")
            }
            Self::COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT => {
                Some("COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT")
            }
            Self::SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT => {
                Some("SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT")
            }
            Self::INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV => {
                Some("INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV")
            }
            Self::SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT => {
                Some("SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM => {
                Some("PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM")
            }
            Self::ATTACHMENT_SAMPLE_COUNT_INFO_AMD => Some("ATTACHMENT_SAMPLE_COUNT_INFO_AMD"),
            Self::PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR")
            }
            Self::SAMPLE_LOCATIONS_INFO_EXT => Some("SAMPLE_LOCATIONS_INFO_EXT"),
            Self::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT => {
                Some("RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT")
            }
            Self::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT => {
                Some("PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT")
            }
            Self::MULTISAMPLE_PROPERTIES_EXT => Some("MULTISAMPLE_PROPERTIES_EXT"),
            Self::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT")
            }
            Self::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT => {
                Some("PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT")
            }
            Self::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV => {
                Some("PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV")
            }
            Self::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR => {
                Some("WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR")
            }
            Self::ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR => {
                Some("ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR")
            }
            Self::ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR => {
                Some("ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR")
            }
            Self::ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR => {
                Some("ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR")
            }
            Self::ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR => {
                Some("ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR")
            }
            Self::ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR => {
                Some("ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR")
            }
            Self::ACCELERATION_STRUCTURE_GEOMETRY_KHR => {
                Some("ACCELERATION_STRUCTURE_GEOMETRY_KHR")
            }
            Self::ACCELERATION_STRUCTURE_VERSION_INFO_KHR => {
                Some("ACCELERATION_STRUCTURE_VERSION_INFO_KHR")
            }
            Self::COPY_ACCELERATION_STRUCTURE_INFO_KHR => {
                Some("COPY_ACCELERATION_STRUCTURE_INFO_KHR")
            }
            Self::COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR => {
                Some("COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR")
            }
            Self::COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR => {
                Some("COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR")
            }
            Self::ACCELERATION_STRUCTURE_CREATE_INFO_KHR => {
                Some("ACCELERATION_STRUCTURE_CREATE_INFO_KHR")
            }
            Self::ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR => {
                Some("ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR")
            }
            Self::RAY_TRACING_PIPELINE_CREATE_INFO_KHR => {
                Some("RAY_TRACING_PIPELINE_CREATE_INFO_KHR")
            }
            Self::RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR => {
                Some("RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR")
            }
            Self::RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR => {
                Some("RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR")
            }
            Self::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV => {
                Some("PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV")
            }
            Self::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT => {
                Some("DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT")
            }
            Self::PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT => {
                Some("PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT")
            }
            Self::IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT => {
                Some("IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT")
            }
            Self::IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT => {
                Some("IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT")
            }
            Self::IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT => {
                Some("IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT")
            }
            Self::DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT => {
                Some("DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT")
            }
            Self::VALIDATION_CACHE_CREATE_INFO_EXT => Some("VALIDATION_CACHE_CREATE_INFO_EXT"),
            Self::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT => {
                Some("SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT")
            }
            #[cfg(feature = "provisional")]
            Self::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR")
            }
            #[cfg(feature = "provisional")]
            Self::PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR")
            }
            Self::PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV => {
                Some("PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV")
            }
            Self::PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV => {
                Some("PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV")
            }
            Self::RAY_TRACING_PIPELINE_CREATE_INFO_NV => {
                Some("RAY_TRACING_PIPELINE_CREATE_INFO_NV")
            }
            Self::ACCELERATION_STRUCTURE_CREATE_INFO_NV => {
                Some("ACCELERATION_STRUCTURE_CREATE_INFO_NV")
            }
            Self::GEOMETRY_NV => Some("GEOMETRY_NV"),
            Self::GEOMETRY_TRIANGLES_NV => Some("GEOMETRY_TRIANGLES_NV"),
            Self::GEOMETRY_AABB_NV => Some("GEOMETRY_AABB_NV"),
            Self::BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV => {
                Some("BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV")
            }
            Self::WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV => {
                Some("WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV")
            }
            Self::ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV => {
                Some("ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV")
            }
            Self::RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV => {
                Some("RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV")
            }
            Self::ACCELERATION_STRUCTURE_INFO_NV => Some("ACCELERATION_STRUCTURE_INFO_NV"),
            Self::PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV")
            }
            Self::PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV => {
                Some("PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT => {
                Some("PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT")
            }
            Self::FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT => {
                Some("FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_CONVERSION_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_COOPERATIVE_MATRIX_CONVERSION_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_ELAPSED_TIMER_QUERY_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_ELAPSED_TIMER_QUERY_FEATURES_QCOM")
            }
            Self::IMPORT_MEMORY_HOST_POINTER_INFO_EXT => {
                Some("IMPORT_MEMORY_HOST_POINTER_INFO_EXT")
            }
            Self::MEMORY_HOST_POINTER_PROPERTIES_EXT => Some("MEMORY_HOST_POINTER_PROPERTIES_EXT"),
            Self::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR")
            }
            Self::PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD => {
                Some("PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD")
            }
            Self::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD => {
                Some("PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD")
            }
            Self::VIDEO_DECODE_H265_CAPABILITIES_KHR => Some("VIDEO_DECODE_H265_CAPABILITIES_KHR"),
            Self::VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                Some("VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR => {
                Some("VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR")
            }
            Self::VIDEO_DECODE_H265_PROFILE_INFO_KHR => Some("VIDEO_DECODE_H265_PROFILE_INFO_KHR"),
            Self::VIDEO_DECODE_H265_PICTURE_INFO_KHR => Some("VIDEO_DECODE_H265_PICTURE_INFO_KHR"),
            Self::VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR => {
                Some("VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR")
            }
            Self::DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD => {
                Some("DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD")
            }
            Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT")
            }
            Self::PRESENT_FRAME_TOKEN_GGP => Some("PRESENT_FRAME_TOKEN_GGP"),
            Self::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV")
            }
            Self::PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV => {
                Some("PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV")
            }
            Self::CHECKPOINT_DATA_NV => Some("CHECKPOINT_DATA_NV"),
            Self::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV => {
                Some("QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV")
            }
            Self::QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV => {
                Some("QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV")
            }
            Self::CHECKPOINT_DATA_2_NV => Some("CHECKPOINT_DATA_2_NV"),
            Self::PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT")
            }
            Self::SWAPCHAIN_TIMING_PROPERTIES_EXT => Some("SWAPCHAIN_TIMING_PROPERTIES_EXT"),
            Self::SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT => {
                Some("SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT")
            }
            Self::PRESENT_TIMINGS_INFO_EXT => Some("PRESENT_TIMINGS_INFO_EXT"),
            Self::PRESENT_TIMING_INFO_EXT => Some("PRESENT_TIMING_INFO_EXT"),
            Self::PAST_PRESENTATION_TIMING_INFO_EXT => Some("PAST_PRESENTATION_TIMING_INFO_EXT"),
            Self::PAST_PRESENTATION_TIMING_PROPERTIES_EXT => {
                Some("PAST_PRESENTATION_TIMING_PROPERTIES_EXT")
            }
            Self::PAST_PRESENTATION_TIMING_EXT => Some("PAST_PRESENTATION_TIMING_EXT"),
            Self::PRESENT_TIMING_SURFACE_CAPABILITIES_EXT => {
                Some("PRESENT_TIMING_SURFACE_CAPABILITIES_EXT")
            }
            Self::SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT => {
                Some("SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL => {
                Some("PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL")
            }
            Self::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL => {
                Some("QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL")
            }
            Self::INITIALIZE_PERFORMANCE_API_INFO_INTEL => {
                Some("INITIALIZE_PERFORMANCE_API_INFO_INTEL")
            }
            Self::PERFORMANCE_MARKER_INFO_INTEL => Some("PERFORMANCE_MARKER_INFO_INTEL"),
            Self::PERFORMANCE_STREAM_MARKER_INFO_INTEL => {
                Some("PERFORMANCE_STREAM_MARKER_INFO_INTEL")
            }
            Self::PERFORMANCE_OVERRIDE_INFO_INTEL => Some("PERFORMANCE_OVERRIDE_INFO_INTEL"),
            Self::PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL => {
                Some("PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL")
            }
            Self::PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT")
            }
            Self::DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD => {
                Some("DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD")
            }
            Self::SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD => {
                Some("SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD")
            }
            Self::IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA => {
                Some("IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA")
            }
            Self::METAL_SURFACE_CREATE_INFO_EXT => Some("METAL_SURFACE_CREATE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT")
            }
            Self::RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT => {
                Some("RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT")
            }
            Self::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT => {
                Some("RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT")
            }
            Self::FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR => {
                Some("FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR")
            }
            Self::PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR => {
                Some("PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR => {
                Some("PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR")
            }
            Self::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR => {
                Some("RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD => {
                Some("PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD")
            }
            Self::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD => {
                Some("PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD")
            }
            Self::PHYSICAL_DEVICE_SHADER_CONSTANT_DATA_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_SHADER_CONSTANT_DATA_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_ABORT_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_SHADER_ABORT_FEATURES_KHR")
            }
            Self::DEVICE_FAULT_SHADER_ABORT_MESSAGE_INFO_KHR => {
                Some("DEVICE_FAULT_SHADER_ABORT_MESSAGE_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_ABORT_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_SHADER_ABORT_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT")
            }
            Self::MEMORY_PRIORITY_ALLOCATE_INFO_EXT => Some("MEMORY_PRIORITY_ALLOCATE_INFO_EXT"),
            Self::SURFACE_PROTECTED_CAPABILITIES_KHR => Some("SURFACE_PROTECTED_CAPABILITIES_KHR"),
            Self::PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT")
            }
            Self::BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT => {
                Some("BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT")
            }
            Self::VALIDATION_FEATURES_EXT => Some("VALIDATION_FEATURES_EXT"),
            Self::PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV")
            }
            Self::COOPERATIVE_MATRIX_PROPERTIES_NV => Some("COOPERATIVE_MATRIX_PROPERTIES_NV"),
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV")
            }
            Self::PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV => {
                Some("PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV")
            }
            Self::FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV => {
                Some("FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT")
            }
            Self::PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT => {
                Some("PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT")
            }
            Self::SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT => {
                Some("SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT")
            }
            Self::SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT => {
                Some("SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT")
            }
            Self::SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT => {
                Some("SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT")
            }
            Self::HEADLESS_SURFACE_CREATE_INFO_EXT => Some("HEADLESS_SURFACE_CREATE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR")
            }
            Self::PIPELINE_INFO_KHR => Some("PIPELINE_INFO_KHR"),
            Self::PIPELINE_EXECUTABLE_PROPERTIES_KHR => Some("PIPELINE_EXECUTABLE_PROPERTIES_KHR"),
            Self::PIPELINE_EXECUTABLE_INFO_KHR => Some("PIPELINE_EXECUTABLE_INFO_KHR"),
            Self::PIPELINE_EXECUTABLE_STATISTIC_KHR => Some("PIPELINE_EXECUTABLE_STATISTIC_KHR"),
            Self::PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR => {
                Some("PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR")
            }
            Self::PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT")
            }
            Self::MEMORY_MAP_PLACED_INFO_EXT => Some("MEMORY_MAP_PLACED_INFO_EXT"),
            Self::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV")
            }
            Self::GRAPHICS_SHADER_GROUP_CREATE_INFO_NV => {
                Some("GRAPHICS_SHADER_GROUP_CREATE_INFO_NV")
            }
            Self::GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV => {
                Some("GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV")
            }
            Self::INDIRECT_COMMANDS_LAYOUT_TOKEN_NV => Some("INDIRECT_COMMANDS_LAYOUT_TOKEN_NV"),
            Self::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV => {
                Some("INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV")
            }
            Self::GENERATED_COMMANDS_INFO_NV => Some("GENERATED_COMMANDS_INFO_NV"),
            Self::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV => {
                Some("GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV")
            }
            Self::COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV => {
                Some("COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT")
            }
            Self::COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM => {
                Some("COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM")
            }
            Self::RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM => {
                Some("RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM")
            }
            Self::PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT")
            }
            Self::DEPTH_BIAS_INFO_EXT => Some("DEPTH_BIAS_INFO_EXT"),
            Self::DEPTH_BIAS_REPRESENTATION_INFO_EXT => Some("DEPTH_BIAS_REPRESENTATION_INFO_EXT"),
            Self::PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT")
            }
            Self::DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT => {
                Some("DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT")
            }
            Self::DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT => {
                Some("DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT")
            }
            Self::SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT => {
                Some("SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT")
            }
            Self::PIPELINE_LIBRARY_CREATE_INFO_KHR => Some("PIPELINE_LIBRARY_CREATE_INFO_KHR"),
            Self::PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV")
            }
            Self::SURFACE_CAPABILITIES_PRESENT_BARRIER_NV => {
                Some("SURFACE_CAPABILITIES_PRESENT_BARRIER_NV")
            }
            Self::SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV => {
                Some("SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV")
            }
            Self::PRESENT_ID_KHR => Some("PRESENT_ID_KHR"),
            Self::PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR")
            }
            Self::VIDEO_ENCODE_INFO_KHR => Some("VIDEO_ENCODE_INFO_KHR"),
            Self::VIDEO_ENCODE_RATE_CONTROL_INFO_KHR => Some("VIDEO_ENCODE_RATE_CONTROL_INFO_KHR"),
            Self::VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR => {
                Some("VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR")
            }
            Self::VIDEO_ENCODE_CAPABILITIES_KHR => Some("VIDEO_ENCODE_CAPABILITIES_KHR"),
            Self::VIDEO_ENCODE_USAGE_INFO_KHR => Some("VIDEO_ENCODE_USAGE_INFO_KHR"),
            Self::QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR => {
                Some("QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR => {
                Some("PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR")
            }
            Self::VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR => {
                Some("VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR")
            }
            Self::VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR => {
                Some("VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR")
            }
            Self::VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR => {
                Some("VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR")
            }
            Self::VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR => {
                Some("VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV")
            }
            Self::DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV => {
                Some("DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV")
            }
            Self::PERF_HINT_INFO_QCOM => Some("PERF_HINT_INFO_QCOM"),
            Self::PHYSICAL_DEVICE_QUEUE_PERF_HINT_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_QUEUE_PERF_HINT_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_QUEUE_PERF_HINT_PROPERTIES_QCOM => {
                Some("PHYSICAL_DEVICE_QUEUE_PERF_HINT_PROPERTIES_QCOM")
            }
            Self::PHYSICAL_DEVICE_IMAGE_PROCESSING_3_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_IMAGE_PROCESSING_3_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_PROPERTIES_QCOM => {
                Some("PHYSICAL_DEVICE_SHADER_MULTIPLE_WAIT_QUEUES_PROPERTIES_QCOM")
            }
            Self::PHYSICAL_DEVICE_SHADER_SPLIT_BARRIER_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_SPLIT_BARRIER_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_SPLIT_BARRIER_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_SPLIT_BARRIER_PROPERTIES_EXT")
            }
            #[cfg(feature = "provisional")]
            Self::CUDA_MODULE_CREATE_INFO_NV => Some("CUDA_MODULE_CREATE_INFO_NV"),
            #[cfg(feature = "provisional")]
            Self::CUDA_FUNCTION_CREATE_INFO_NV => Some("CUDA_FUNCTION_CREATE_INFO_NV"),
            #[cfg(feature = "provisional")]
            Self::CUDA_LAUNCH_INFO_NV => Some("CUDA_LAUNCH_INFO_NV"),
            #[cfg(feature = "provisional")]
            Self::PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV")
            }
            #[cfg(feature = "provisional")]
            Self::PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM => {
                Some("PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM")
            }
            Self::RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM => {
                Some("RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM")
            }
            Self::PER_TILE_BEGIN_INFO_QCOM => Some("PER_TILE_BEGIN_INFO_QCOM"),
            Self::PER_TILE_END_INFO_QCOM => Some("PER_TILE_END_INFO_QCOM"),
            Self::DISPATCH_TILE_INFO_QCOM => Some("DISPATCH_TILE_INFO_QCOM"),
            Self::QUERY_LOW_LATENCY_SUPPORT_NV => Some("QUERY_LOW_LATENCY_SUPPORT_NV"),
            Self::EXPORT_METAL_OBJECT_CREATE_INFO_EXT => {
                Some("EXPORT_METAL_OBJECT_CREATE_INFO_EXT")
            }
            Self::EXPORT_METAL_OBJECTS_INFO_EXT => Some("EXPORT_METAL_OBJECTS_INFO_EXT"),
            Self::EXPORT_METAL_DEVICE_INFO_EXT => Some("EXPORT_METAL_DEVICE_INFO_EXT"),
            Self::EXPORT_METAL_COMMAND_QUEUE_INFO_EXT => {
                Some("EXPORT_METAL_COMMAND_QUEUE_INFO_EXT")
            }
            Self::EXPORT_METAL_BUFFER_INFO_EXT => Some("EXPORT_METAL_BUFFER_INFO_EXT"),
            Self::IMPORT_METAL_BUFFER_INFO_EXT => Some("IMPORT_METAL_BUFFER_INFO_EXT"),
            Self::EXPORT_METAL_TEXTURE_INFO_EXT => Some("EXPORT_METAL_TEXTURE_INFO_EXT"),
            Self::IMPORT_METAL_TEXTURE_INFO_EXT => Some("IMPORT_METAL_TEXTURE_INFO_EXT"),
            Self::EXPORT_METAL_IO_SURFACE_INFO_EXT => Some("EXPORT_METAL_IO_SURFACE_INFO_EXT"),
            Self::IMPORT_METAL_IO_SURFACE_INFO_EXT => Some("IMPORT_METAL_IO_SURFACE_INFO_EXT"),
            Self::EXPORT_METAL_SHARED_EVENT_INFO_EXT => Some("EXPORT_METAL_SHARED_EVENT_INFO_EXT"),
            Self::IMPORT_METAL_SHARED_EVENT_INFO_EXT => Some("IMPORT_METAL_SHARED_EVENT_INFO_EXT"),
            Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT")
            }
            Self::DESCRIPTOR_ADDRESS_INFO_EXT => Some("DESCRIPTOR_ADDRESS_INFO_EXT"),
            Self::DESCRIPTOR_GET_INFO_EXT => Some("DESCRIPTOR_GET_INFO_EXT"),
            Self::BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                Some("BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
            }
            Self::IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                Some("IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
            }
            Self::IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                Some("IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
            }
            Self::SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                Some("SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
            }
            Self::OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT => {
                Some("OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT")
            }
            Self::DESCRIPTOR_BUFFER_BINDING_INFO_EXT => Some("DESCRIPTOR_BUFFER_BINDING_INFO_EXT"),
            Self::DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT => {
                Some("DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT")
            }
            Self::ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT => {
                Some("ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT")
            }
            Self::DEVICE_MEMORY_COPY_KHR => Some("DEVICE_MEMORY_COPY_KHR"),
            Self::COPY_DEVICE_MEMORY_INFO_KHR => Some("COPY_DEVICE_MEMORY_INFO_KHR"),
            Self::DEVICE_MEMORY_IMAGE_COPY_KHR => Some("DEVICE_MEMORY_IMAGE_COPY_KHR"),
            Self::COPY_DEVICE_MEMORY_IMAGE_INFO_KHR => Some("COPY_DEVICE_MEMORY_IMAGE_INFO_KHR"),
            Self::MEMORY_RANGE_BARRIERS_INFO_KHR => Some("MEMORY_RANGE_BARRIERS_INFO_KHR"),
            Self::MEMORY_RANGE_BARRIER_KHR => Some("MEMORY_RANGE_BARRIER_KHR"),
            Self::PHYSICAL_DEVICE_DEVICE_ADDRESS_COMMANDS_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_DEVICE_ADDRESS_COMMANDS_FEATURES_KHR")
            }
            Self::BIND_INDEX_BUFFER_3_INFO_KHR => Some("BIND_INDEX_BUFFER_3_INFO_KHR"),
            Self::BIND_VERTEX_BUFFER_3_INFO_KHR => Some("BIND_VERTEX_BUFFER_3_INFO_KHR"),
            Self::DRAW_INDIRECT_2_INFO_KHR => Some("DRAW_INDIRECT_2_INFO_KHR"),
            Self::DRAW_INDIRECT_COUNT_2_INFO_KHR => Some("DRAW_INDIRECT_COUNT_2_INFO_KHR"),
            Self::DISPATCH_INDIRECT_2_INFO_KHR => Some("DISPATCH_INDIRECT_2_INFO_KHR"),
            Self::CONDITIONAL_RENDERING_BEGIN_INFO_2_EXT => {
                Some("CONDITIONAL_RENDERING_BEGIN_INFO_2_EXT")
            }
            Self::BIND_TRANSFORM_FEEDBACK_BUFFER_2_INFO_EXT => {
                Some("BIND_TRANSFORM_FEEDBACK_BUFFER_2_INFO_EXT")
            }
            Self::MEMORY_MARKER_INFO_AMD => Some("MEMORY_MARKER_INFO_AMD"),
            Self::ACCELERATION_STRUCTURE_CREATE_INFO_2_KHR => {
                Some("ACCELERATION_STRUCTURE_CREATE_INFO_2_KHR")
            }
            Self::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT")
            }
            Self::GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT => {
                Some("GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD => {
                Some("PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV")
            }
            Self::PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV => {
                Some("PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV")
            }
            Self::ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV => {
                Some("ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV")
            }
            Self::ACCELERATION_STRUCTURE_MOTION_INFO_NV => {
                Some("ACCELERATION_STRUCTURE_MOTION_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT")
            }
            Self::COPY_COMMAND_TRANSFORM_INFO_QCOM => Some("COPY_COMMAND_TRANSFORM_INFO_QCOM"),
            Self::PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT")
            }
            Self::IMAGE_COMPRESSION_CONTROL_EXT => Some("IMAGE_COMPRESSION_CONTROL_EXT"),
            Self::IMAGE_COMPRESSION_PROPERTIES_EXT => Some("IMAGE_COMPRESSION_PROPERTIES_EXT"),
            Self::PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_FAULT_FEATURES_EXT => Some("PHYSICAL_DEVICE_FAULT_FEATURES_EXT"),
            Self::DEVICE_FAULT_COUNTS_EXT => Some("DEVICE_FAULT_COUNTS_EXT"),
            Self::DEVICE_FAULT_INFO_EXT => Some("DEVICE_FAULT_INFO_EXT"),
            Self::PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT")
            }
            Self::DIRECTFB_SURFACE_CREATE_INFO_EXT => Some("DIRECTFB_SURFACE_CREATE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT")
            }
            Self::VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT => {
                Some("VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT")
            }
            Self::VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT => {
                Some("VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT")
            }
            Self::PHYSICAL_DEVICE_DRM_PROPERTIES_EXT => Some("PHYSICAL_DEVICE_DRM_PROPERTIES_EXT"),
            Self::PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT")
            }
            Self::DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT => {
                Some("DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT")
            }
            Self::PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT")
            }
            Self::PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT => {
                Some("PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT")
            }
            Self::IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA => {
                Some("IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA")
            }
            Self::MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA => {
                Some("MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA")
            }
            Self::MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA => {
                Some("MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA")
            }
            Self::IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA => {
                Some("IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA")
            }
            Self::SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA => {
                Some("SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA")
            }
            Self::BUFFER_COLLECTION_CREATE_INFO_FUCHSIA => {
                Some("BUFFER_COLLECTION_CREATE_INFO_FUCHSIA")
            }
            Self::IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA => {
                Some("IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA")
            }
            Self::BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA => {
                Some("BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA")
            }
            Self::BUFFER_COLLECTION_PROPERTIES_FUCHSIA => {
                Some("BUFFER_COLLECTION_PROPERTIES_FUCHSIA")
            }
            Self::BUFFER_CONSTRAINTS_INFO_FUCHSIA => Some("BUFFER_CONSTRAINTS_INFO_FUCHSIA"),
            Self::BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA => {
                Some("BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA")
            }
            Self::IMAGE_CONSTRAINTS_INFO_FUCHSIA => Some("IMAGE_CONSTRAINTS_INFO_FUCHSIA"),
            Self::IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA => {
                Some("IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA")
            }
            Self::SYSMEM_COLOR_SPACE_FUCHSIA => Some("SYSMEM_COLOR_SPACE_FUCHSIA"),
            Self::BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA => {
                Some("BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA")
            }
            Self::SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI => {
                Some("SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI")
            }
            Self::PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI => {
                Some("PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI")
            }
            Self::PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI => {
                Some("PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI")
            }
            Self::PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI => {
                Some("PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI")
            }
            Self::MEMORY_GET_REMOTE_ADDRESS_INFO_NV => Some("MEMORY_GET_REMOTE_ADDRESS_INFO_NV"),
            Self::PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV")
            }
            Self::PIPELINE_PROPERTIES_IDENTIFIER_EXT => Some("PIPELINE_PROPERTIES_IDENTIFIER_EXT"),
            Self::PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT")
            }
            Self::FRAME_BOUNDARY_EXT => Some("FRAME_BOUNDARY_EXT"),
            Self::PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT")
            }
            Self::SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT => {
                Some("SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT")
            }
            Self::MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT => {
                Some("MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT")
            }
            Self::SCREEN_SURFACE_CREATE_INFO_QNX => Some("SCREEN_SURFACE_CREATE_INFO_QNX"),
            Self::PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT")
            }
            Self::PIPELINE_COLOR_WRITE_CREATE_INFO_EXT => {
                Some("PIPELINE_COLOR_WRITE_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_UNTYPED_POINTERS_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_SHADER_UNTYPED_POINTERS_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE => {
                Some("PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE")
            }
            Self::VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE => {
                Some("VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE")
            }
            Self::VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE => {
                Some("VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE")
            }
            Self::VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE => {
                Some("VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE")
            }
            Self::PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT")
            }
            Self::IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT => Some("IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT")
            }
            Self::MICROMAP_BUILD_INFO_EXT => Some("MICROMAP_BUILD_INFO_EXT"),
            Self::MICROMAP_VERSION_INFO_EXT => Some("MICROMAP_VERSION_INFO_EXT"),
            Self::COPY_MICROMAP_INFO_EXT => Some("COPY_MICROMAP_INFO_EXT"),
            Self::COPY_MICROMAP_TO_MEMORY_INFO_EXT => Some("COPY_MICROMAP_TO_MEMORY_INFO_EXT"),
            Self::COPY_MEMORY_TO_MICROMAP_INFO_EXT => Some("COPY_MEMORY_TO_MICROMAP_INFO_EXT"),
            Self::PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT")
            }
            Self::MICROMAP_CREATE_INFO_EXT => Some("MICROMAP_CREATE_INFO_EXT"),
            Self::MICROMAP_BUILD_SIZES_INFO_EXT => Some("MICROMAP_BUILD_SIZES_INFO_EXT"),
            Self::ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT => {
                Some("ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT")
            }
            #[cfg(feature = "provisional")]
            Self::PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV")
            }
            #[cfg(feature = "provisional")]
            Self::PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV")
            }
            #[cfg(feature = "provisional")]
            Self::ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV => {
                Some("ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV")
            }
            Self::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI => {
                Some("PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI")
            }
            Self::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI => {
                Some("PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI")
            }
            Self::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI => {
                Some("PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI")
            }
            Self::PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT")
            }
            Self::SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT => {
                Some("SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM => {
                Some("PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM")
            }
            Self::DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM => {
                Some("DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM")
            }
            Self::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM => {
                Some("PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM")
            }
            Self::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM => {
                Some("PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM")
            }
            Self::DISPATCH_PARAMETERS_ARM => Some("DISPATCH_PARAMETERS_ARM"),
            Self::PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_PROPERTIES_ARM => {
                Some("PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_PROPERTIES_ARM")
            }
            Self::PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT")
            }
            Self::IMAGE_VIEW_SLICED_CREATE_INFO_EXT => Some("IMAGE_VIEW_SLICED_CREATE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE => {
                Some("PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE")
            }
            Self::DESCRIPTOR_SET_BINDING_REFERENCE_VALVE => {
                Some("DESCRIPTOR_SET_BINDING_REFERENCE_VALVE")
            }
            Self::DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE => {
                Some("DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE")
            }
            Self::PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM => {
                Some("PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM")
            }
            Self::PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM => {
                Some("PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM")
            }
            Self::RENDER_PASS_STRIPE_BEGIN_INFO_ARM => Some("RENDER_PASS_STRIPE_BEGIN_INFO_ARM"),
            Self::RENDER_PASS_STRIPE_INFO_ARM => Some("RENDER_PASS_STRIPE_INFO_ARM"),
            Self::RENDER_PASS_STRIPE_SUBMIT_INFO_ARM => Some("RENDER_PASS_STRIPE_SUBMIT_INFO_ARM"),
            Self::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV")
            }
            Self::COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV => {
                Some("COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV")
            }
            Self::PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV => {
                Some("PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV")
            }
            Self::ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV => {
                Some("ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV")
            }
            Self::ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV => {
                Some("ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV")
            }
            Self::PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM => {
                Some("PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM")
            }
            Self::IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM => {
                Some("IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM")
            }
            Self::PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT")
            }
            Self::NATIVE_BUFFER_USAGE_OHOS => Some("NATIVE_BUFFER_USAGE_OHOS"),
            Self::NATIVE_BUFFER_PROPERTIES_OHOS => Some("NATIVE_BUFFER_PROPERTIES_OHOS"),
            Self::NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS => {
                Some("NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS")
            }
            Self::IMPORT_NATIVE_BUFFER_INFO_OHOS => Some("IMPORT_NATIVE_BUFFER_INFO_OHOS"),
            Self::MEMORY_GET_NATIVE_BUFFER_INFO_OHOS => Some("MEMORY_GET_NATIVE_BUFFER_INFO_OHOS"),
            Self::EXTERNAL_FORMAT_OHOS => Some("EXTERNAL_FORMAT_OHOS"),
            Self::EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT => {
                Some("EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT")
            }
            Self::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT")
            }
            Self::RENDER_PASS_CREATION_CONTROL_EXT => Some("RENDER_PASS_CREATION_CONTROL_EXT"),
            Self::RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT => {
                Some("RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT")
            }
            Self::RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT => {
                Some("RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT")
            }
            Self::DIRECT_DRIVER_LOADING_INFO_LUNARG => Some("DIRECT_DRIVER_LOADING_INFO_LUNARG"),
            Self::DIRECT_DRIVER_LOADING_LIST_LUNARG => Some("DIRECT_DRIVER_LOADING_LIST_LUNARG"),
            Self::TENSOR_CREATE_INFO_ARM => Some("TENSOR_CREATE_INFO_ARM"),
            Self::TENSOR_VIEW_CREATE_INFO_ARM => Some("TENSOR_VIEW_CREATE_INFO_ARM"),
            Self::BIND_TENSOR_MEMORY_INFO_ARM => Some("BIND_TENSOR_MEMORY_INFO_ARM"),
            Self::WRITE_DESCRIPTOR_SET_TENSOR_ARM => Some("WRITE_DESCRIPTOR_SET_TENSOR_ARM"),
            Self::PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM => {
                Some("PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM")
            }
            Self::TENSOR_FORMAT_PROPERTIES_ARM => Some("TENSOR_FORMAT_PROPERTIES_ARM"),
            Self::TENSOR_DESCRIPTION_ARM => Some("TENSOR_DESCRIPTION_ARM"),
            Self::TENSOR_MEMORY_REQUIREMENTS_INFO_ARM => {
                Some("TENSOR_MEMORY_REQUIREMENTS_INFO_ARM")
            }
            Self::TENSOR_MEMORY_BARRIER_ARM => Some("TENSOR_MEMORY_BARRIER_ARM"),
            Self::PHYSICAL_DEVICE_TENSOR_FEATURES_ARM => {
                Some("PHYSICAL_DEVICE_TENSOR_FEATURES_ARM")
            }
            Self::DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM => {
                Some("DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM")
            }
            Self::COPY_TENSOR_INFO_ARM => Some("COPY_TENSOR_INFO_ARM"),
            Self::TENSOR_COPY_ARM => Some("TENSOR_COPY_ARM"),
            Self::TENSOR_DEPENDENCY_INFO_ARM => Some("TENSOR_DEPENDENCY_INFO_ARM"),
            Self::MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM => {
                Some("MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM")
            }
            Self::PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM => {
                Some("PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM")
            }
            Self::EXTERNAL_TENSOR_PROPERTIES_ARM => Some("EXTERNAL_TENSOR_PROPERTIES_ARM"),
            Self::EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM => {
                Some("EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM => {
                Some("PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM => {
                Some("PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM")
            }
            Self::DESCRIPTOR_GET_TENSOR_INFO_ARM => Some("DESCRIPTOR_GET_TENSOR_INFO_ARM"),
            Self::TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM => {
                Some("TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM")
            }
            Self::TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM => {
                Some("TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM")
            }
            Self::FRAME_BOUNDARY_TENSORS_ARM => Some("FRAME_BOUNDARY_TENSORS_ARM"),
            Self::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT")
            }
            Self::PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT => {
                Some("PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT")
            }
            Self::SHADER_MODULE_IDENTIFIER_EXT => Some("SHADER_MODULE_IDENTIFIER_EXT"),
            Self::PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV")
            }
            Self::OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV => Some("OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV"),
            Self::OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV => {
                Some("OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV")
            }
            Self::OPTICAL_FLOW_SESSION_CREATE_INFO_NV => {
                Some("OPTICAL_FLOW_SESSION_CREATE_INFO_NV")
            }
            Self::OPTICAL_FLOW_EXECUTE_INFO_NV => Some("OPTICAL_FLOW_EXECUTE_INFO_NV"),
            Self::OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV => {
                Some("OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID => {
                Some("PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID")
            }
            Self::PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID => {
                Some("PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID")
            }
            Self::ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID => {
                Some("ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID")
            }
            Self::PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD => {
                Some("PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD")
            }
            Self::ANTI_LAG_DATA_AMD => Some("ANTI_LAG_DATA_AMD"),
            Self::ANTI_LAG_PRESENTATION_INFO_AMD => Some("ANTI_LAG_PRESENTATION_INFO_AMD"),
            #[cfg(feature = "provisional")]
            Self::PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX => {
                Some("PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX")
            }
            #[cfg(feature = "provisional")]
            Self::ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX => {
                Some("ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX")
            }
            Self::SURFACE_CAPABILITIES_PRESENT_ID_2_KHR => {
                Some("SURFACE_CAPABILITIES_PRESENT_ID_2_KHR")
            }
            Self::PRESENT_ID_2_KHR => Some("PRESENT_ID_2_KHR"),
            Self::PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR")
            }
            Self::SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR => {
                Some("SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR")
            }
            Self::PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR")
            }
            Self::PRESENT_WAIT_2_INFO_KHR => Some("PRESENT_WAIT_2_INFO_KHR"),
            Self::PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT")
            }
            Self::SHADER_CREATE_INFO_EXT => Some("SHADER_CREATE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR")
            }
            Self::PIPELINE_BINARY_CREATE_INFO_KHR => Some("PIPELINE_BINARY_CREATE_INFO_KHR"),
            Self::PIPELINE_BINARY_INFO_KHR => Some("PIPELINE_BINARY_INFO_KHR"),
            Self::PIPELINE_BINARY_KEY_KHR => Some("PIPELINE_BINARY_KEY_KHR"),
            Self::PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR")
            }
            Self::RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR => {
                Some("RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR")
            }
            Self::PIPELINE_BINARY_DATA_INFO_KHR => Some("PIPELINE_BINARY_DATA_INFO_KHR"),
            Self::PIPELINE_CREATE_INFO_KHR => Some("PIPELINE_CREATE_INFO_KHR"),
            Self::DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR => {
                Some("DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR")
            }
            Self::PIPELINE_BINARY_HANDLES_INFO_KHR => Some("PIPELINE_BINARY_HANDLES_INFO_KHR"),
            Self::PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM")
            }
            Self::TILE_PROPERTIES_QCOM => Some("TILE_PROPERTIES_QCOM"),
            Self::PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC => {
                Some("PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC")
            }
            Self::AMIGO_PROFILING_SUBMIT_INFO_SEC => Some("AMIGO_PROFILING_SUBMIT_INFO_SEC"),
            Self::SURFACE_PRESENT_MODE_KHR => Some("SURFACE_PRESENT_MODE_KHR"),
            Self::SURFACE_PRESENT_SCALING_CAPABILITIES_KHR => {
                Some("SURFACE_PRESENT_SCALING_CAPABILITIES_KHR")
            }
            Self::SURFACE_PRESENT_MODE_COMPATIBILITY_KHR => {
                Some("SURFACE_PRESENT_MODE_COMPATIBILITY_KHR")
            }
            Self::PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR")
            }
            Self::SWAPCHAIN_PRESENT_FENCE_INFO_KHR => Some("SWAPCHAIN_PRESENT_FENCE_INFO_KHR"),
            Self::SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR => {
                Some("SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR")
            }
            Self::SWAPCHAIN_PRESENT_MODE_INFO_KHR => Some("SWAPCHAIN_PRESENT_MODE_INFO_KHR"),
            Self::SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR => {
                Some("SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR")
            }
            Self::RELEASE_SWAPCHAIN_IMAGES_INFO_KHR => Some("RELEASE_SWAPCHAIN_IMAGES_INFO_KHR"),
            Self::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV")
            }
            Self::COOPERATIVE_VECTOR_PROPERTIES_NV => Some("COOPERATIVE_VECTOR_PROPERTIES_NV"),
            Self::CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV => {
                Some("CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT")
            }
            Self::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT => {
                Some("MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_PROPERTIES_EXT")
            }
            Self::LAYER_SETTINGS_CREATE_INFO_EXT => Some("LAYER_SETTINGS_CREATE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM => {
                Some("PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM")
            }
            Self::PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM => {
                Some("PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM")
            }
            Self::PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_INTERNALLY_SYNCHRONIZED_QUEUES_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_INTERNALLY_SYNCHRONIZED_QUEUES_FEATURES_KHR")
            }
            Self::LATENCY_SLEEP_MODE_INFO_NV => Some("LATENCY_SLEEP_MODE_INFO_NV"),
            Self::LATENCY_SLEEP_INFO_NV => Some("LATENCY_SLEEP_INFO_NV"),
            Self::SET_LATENCY_MARKER_INFO_NV => Some("SET_LATENCY_MARKER_INFO_NV"),
            Self::GET_LATENCY_MARKER_INFO_NV => Some("GET_LATENCY_MARKER_INFO_NV"),
            Self::LATENCY_TIMINGS_FRAME_REPORT_NV => Some("LATENCY_TIMINGS_FRAME_REPORT_NV"),
            Self::LATENCY_SUBMISSION_PRESENT_ID_NV => Some("LATENCY_SUBMISSION_PRESENT_ID_NV"),
            Self::OUT_OF_BAND_QUEUE_TYPE_INFO_NV => Some("OUT_OF_BAND_QUEUE_TYPE_INFO_NV"),
            Self::SWAPCHAIN_LATENCY_CREATE_INFO_NV => Some("SWAPCHAIN_LATENCY_CREATE_INFO_NV"),
            Self::LATENCY_SURFACE_CAPABILITIES_NV => Some("LATENCY_SURFACE_CAPABILITIES_NV"),
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR")
            }
            Self::COOPERATIVE_MATRIX_PROPERTIES_KHR => Some("COOPERATIVE_MATRIX_PROPERTIES_KHR"),
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR")
            }
            Self::DATA_GRAPH_PIPELINE_CREATE_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_CONSTANT_ARM => Some("DATA_GRAPH_PIPELINE_CONSTANT_ARM"),
            Self::DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM")
            }
            Self::BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM => {
                Some("BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM")
            }
            Self::PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM => {
                Some("PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM => {
                Some("DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_INFO_ARM => Some("DATA_GRAPH_PIPELINE_INFO_ARM"),
            Self::DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM => {
                Some("DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM")
            }
            Self::DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM => {
                Some("DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM")
            }
            Self::QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM => {
                Some("QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM")
            }
            Self::QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM => {
                Some("QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM")
            }
            Self::PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM => {
                Some("PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM")
            }
            Self::QUEUE_FAMILY_DATA_GRAPH_TOSA_PROPERTIES_ARM => {
                Some("QUEUE_FAMILY_DATA_GRAPH_TOSA_PROPERTIES_ARM")
            }
            Self::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM")
            }
            Self::MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM => {
                Some("MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM")
            }
            Self::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR")
            }
            Self::VIDEO_DECODE_AV1_CAPABILITIES_KHR => Some("VIDEO_DECODE_AV1_CAPABILITIES_KHR"),
            Self::VIDEO_DECODE_AV1_PICTURE_INFO_KHR => Some("VIDEO_DECODE_AV1_PICTURE_INFO_KHR"),
            Self::VIDEO_DECODE_AV1_PROFILE_INFO_KHR => Some("VIDEO_DECODE_AV1_PROFILE_INFO_KHR"),
            Self::VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                Some("VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR => Some("VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR"),
            Self::VIDEO_ENCODE_AV1_CAPABILITIES_KHR => Some("VIDEO_ENCODE_AV1_CAPABILITIES_KHR"),
            Self::VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                Some("VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_AV1_PICTURE_INFO_KHR => Some("VIDEO_ENCODE_AV1_PICTURE_INFO_KHR"),
            Self::VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR => Some("VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR"),
            Self::PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR")
            }
            Self::VIDEO_ENCODE_AV1_PROFILE_INFO_KHR => Some("VIDEO_ENCODE_AV1_PROFILE_INFO_KHR"),
            Self::VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR => {
                Some("VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR")
            }
            Self::VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR => {
                Some("VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR")
            }
            Self::VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR => {
                Some("VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR")
            }
            Self::VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR => {
                Some("VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR => {
                Some("VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR")
            }
            Self::VIDEO_DECODE_VP9_CAPABILITIES_KHR => Some("VIDEO_DECODE_VP9_CAPABILITIES_KHR"),
            Self::VIDEO_DECODE_VP9_PICTURE_INFO_KHR => Some("VIDEO_DECODE_VP9_PICTURE_INFO_KHR"),
            Self::VIDEO_DECODE_VP9_PROFILE_INFO_KHR => Some("VIDEO_DECODE_VP9_PROFILE_INFO_KHR"),
            Self::PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR")
            }
            Self::VIDEO_INLINE_QUERY_INFO_KHR => Some("VIDEO_INLINE_QUERY_INFO_KHR"),
            Self::PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM => {
                Some("PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM")
            }
            Self::SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM => {
                Some("SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM")
            }
            Self::SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM => {
                Some("SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM")
            }
            Self::PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM")
            }
            Self::BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM => Some("BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM"),
            Self::PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM")
            }
            Self::SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM => {
                Some("SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM")
            }
            Self::PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR")
            }
            Self::ATTACHMENT_FEEDBACK_LOOP_INFO_EXT => Some("ATTACHMENT_FEEDBACK_LOOP_INFO_EXT"),
            Self::SCREEN_BUFFER_PROPERTIES_QNX => Some("SCREEN_BUFFER_PROPERTIES_QNX"),
            Self::SCREEN_BUFFER_FORMAT_PROPERTIES_QNX => {
                Some("SCREEN_BUFFER_FORMAT_PROPERTIES_QNX")
            }
            Self::IMPORT_SCREEN_BUFFER_INFO_QNX => Some("IMPORT_SCREEN_BUFFER_INFO_QNX"),
            Self::EXTERNAL_FORMAT_QNX => Some("EXTERNAL_FORMAT_QNX"),
            Self::PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX => {
                Some("PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX")
            }
            Self::PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT => {
                Some("PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT")
            }
            Self::CALIBRATED_TIMESTAMP_INFO_KHR => Some("CALIBRATED_TIMESTAMP_INFO_KHR"),
            Self::SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT => {
                Some("SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT")
            }
            Self::BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT => {
                Some("BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM")
            }
            Self::PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM => {
                Some("PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM")
            }
            Self::TILE_MEMORY_REQUIREMENTS_QCOM => Some("TILE_MEMORY_REQUIREMENTS_QCOM"),
            Self::TILE_MEMORY_BIND_INFO_QCOM => Some("TILE_MEMORY_BIND_INFO_QCOM"),
            Self::TILE_MEMORY_SIZE_INFO_QCOM => Some("TILE_MEMORY_SIZE_INFO_QCOM"),
            Self::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR")
            }
            Self::COPY_MEMORY_INDIRECT_INFO_KHR => Some("COPY_MEMORY_INDIRECT_INFO_KHR"),
            Self::COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR => {
                Some("COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT")
            }
            Self::DECOMPRESS_MEMORY_INFO_EXT => Some("DECOMPRESS_MEMORY_INFO_EXT"),
            Self::DISPLAY_SURFACE_STEREO_CREATE_INFO_NV => {
                Some("DISPLAY_SURFACE_STEREO_CREATE_INFO_NV")
            }
            Self::DISPLAY_MODE_STEREO_PROPERTIES_NV => Some("DISPLAY_MODE_STEREO_PROPERTIES_NV"),
            Self::VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR => {
                Some("VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR")
            }
            Self::VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR => {
                Some("VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR")
            }
            Self::VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR => {
                Some("VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR")
            }
            Self::VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR => {
                Some("VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR")
            }
            Self::VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR => {
                Some("VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR")
            }
            Self::VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR => {
                Some("VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR")
            }
            Self::VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR => {
                Some("VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR")
            }
            Self::VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR => {
                Some("VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR")
            }
            Self::PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR")
            }
            Self::VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR => {
                Some("VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR")
            }
            Self::VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR => {
                Some("VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR")
            }
            Self::VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR => {
                Some("VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR")
            }
            Self::VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR => {
                Some("VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR")
            }
            Self::VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR => {
                Some("VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV")
            }
            Self::EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV => {
                Some("EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV")
            }
            Self::EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV => {
                Some("EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV")
            }
            Self::EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV => {
                Some("EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV")
            }
            Self::PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_SHADER_RELAXED_EXTENDED_INSTRUCTION_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_SHADER_RELAXED_EXTENDED_INSTRUCTION_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR => {
                Some("PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR")
            }
            Self::PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_SHADER_REPLICATED_COMPOSITES_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_REPLICATED_COMPOSITES_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV")
            }
            Self::CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV => {
                Some("CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV")
            }
            Self::CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV => {
                Some("CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV")
            }
            Self::CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV => {
                Some("CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV")
            }
            Self::CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV => {
                Some("CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV")
            }
            Self::CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV => {
                Some("CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV")
            }
            Self::RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV => {
                Some("RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV")
            }
            Self::WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV => {
                Some("WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV")
            }
            Self::PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV => {
                Some("PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV")
            }
            Self::BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV => {
                Some("BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV")
            }
            Self::PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV => {
                Some("PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV")
            }
            Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT")
            }
            Self::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT => {
                Some("GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT")
            }
            Self::INDIRECT_EXECUTION_SET_CREATE_INFO_EXT => {
                Some("INDIRECT_EXECUTION_SET_CREATE_INFO_EXT")
            }
            Self::GENERATED_COMMANDS_INFO_EXT => Some("GENERATED_COMMANDS_INFO_EXT"),
            Self::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT => {
                Some("INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT")
            }
            Self::INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT => Some("INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT"),
            Self::WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT => {
                Some("WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT")
            }
            Self::WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT => {
                Some("WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT")
            }
            Self::INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT => {
                Some("INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT")
            }
            Self::INDIRECT_EXECUTION_SET_SHADER_INFO_EXT => {
                Some("INDIRECT_EXECUTION_SET_SHADER_INFO_EXT")
            }
            Self::INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT => {
                Some("INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT")
            }
            Self::GENERATED_COMMANDS_PIPELINE_INFO_EXT => {
                Some("GENERATED_COMMANDS_PIPELINE_INFO_EXT")
            }
            Self::GENERATED_COMMANDS_SHADER_INFO_EXT => Some("GENERATED_COMMANDS_SHADER_INFO_EXT"),
            Self::PHYSICAL_DEVICE_FAULT_FEATURES_KHR => Some("PHYSICAL_DEVICE_FAULT_FEATURES_KHR"),
            Self::PHYSICAL_DEVICE_FAULT_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_FAULT_PROPERTIES_KHR")
            }
            Self::DEVICE_FAULT_INFO_KHR => Some("DEVICE_FAULT_INFO_KHR"),
            Self::DEVICE_FAULT_DEBUG_INFO_KHR => Some("DEVICE_FAULT_DEBUG_INFO_KHR"),
            Self::PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR")
            }
            Self::MEMORY_BARRIER_ACCESS_FLAGS_3_KHR => Some("MEMORY_BARRIER_ACCESS_FLAGS_3_KHR"),
            Self::PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA => {
                Some("PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA")
            }
            Self::PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA => {
                Some("PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA")
            }
            Self::IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA => {
                Some("IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA")
            }
            Self::PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR")
            }
            Self::PUSH_CONSTANT_BANK_INFO_NV => Some("PUSH_CONSTANT_BANK_INFO_NV"),
            Self::PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT")
            }
            Self::PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT => {
                Some("PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR")
            }
            Self::QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR => {
                Some("QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_VIDEO_MAINTENANCE_2_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_VIDEO_MAINTENANCE_2_FEATURES_KHR")
            }
            Self::VIDEO_DECODE_H264_INLINE_SESSION_PARAMETERS_INFO_KHR => {
                Some("VIDEO_DECODE_H264_INLINE_SESSION_PARAMETERS_INFO_KHR")
            }
            Self::VIDEO_DECODE_H265_INLINE_SESSION_PARAMETERS_INFO_KHR => {
                Some("VIDEO_DECODE_H265_INLINE_SESSION_PARAMETERS_INFO_KHR")
            }
            Self::VIDEO_DECODE_AV1_INLINE_SESSION_PARAMETERS_INFO_KHR => {
                Some("VIDEO_DECODE_AV1_INLINE_SESSION_PARAMETERS_INFO_KHR")
            }
            Self::SURFACE_CREATE_INFO_OHOS => Some("SURFACE_CREATE_INFO_OHOS"),
            Self::PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI => {
                Some("PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI")
            }
            Self::HDR_VIVID_DYNAMIC_METADATA_HUAWEI => Some("HDR_VIVID_DYNAMIC_METADATA_HUAWEI"),
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV")
            }
            Self::COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV => {
                Some("COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV => {
                Some("PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV")
            }
            Self::PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM => {
                Some("PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM")
            }
            Self::IMPORT_MEMORY_METAL_HANDLE_INFO_EXT => {
                Some("IMPORT_MEMORY_METAL_HANDLE_INFO_EXT")
            }
            Self::MEMORY_METAL_HANDLE_PROPERTIES_EXT => Some("MEMORY_METAL_HANDLE_PROPERTIES_EXT"),
            Self::MEMORY_GET_METAL_HANDLE_INFO_EXT => Some("MEMORY_GET_METAL_HANDLE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM => {
                Some("PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM")
            }
            Self::PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM => {
                Some("PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM")
            }
            Self::PERFORMANCE_COUNTER_ARM => Some("PERFORMANCE_COUNTER_ARM"),
            Self::PERFORMANCE_COUNTER_DESCRIPTION_ARM => {
                Some("PERFORMANCE_COUNTER_DESCRIPTION_ARM")
            }
            Self::RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM => {
                Some("RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM")
            }
            Self::PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_FEATURES_ARM => {
                Some("PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_FEATURES_ARM")
            }
            Self::PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_PROPERTIES_ARM => {
                Some("PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_PROPERTIES_ARM")
            }
            Self::SHADER_INSTRUMENTATION_CREATE_INFO_ARM => {
                Some("SHADER_INSTRUMENTATION_CREATE_INFO_ARM")
            }
            Self::SHADER_INSTRUMENTATION_METRIC_DESCRIPTION_ARM => {
                Some("SHADER_INSTRUMENTATION_METRIC_DESCRIPTION_ARM")
            }
            Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_ROBUSTNESS_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_ROBUSTNESS_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM => {
                Some("PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE => {
                Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE => {
                Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE")
            }
            Self::PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE => {
                Some("PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE")
            }
            Self::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR")
            }
            Self::SET_PRESENT_CONFIG_NV => Some("SET_PRESENT_CONFIG_NV"),
            Self::PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT")
            }
            Self::RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT => {
                Some("RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_KHR")
            }
            Self::ACCELERATION_STRUCTURE_GEOMETRY_MICROMAP_DATA_KHR => {
                Some("ACCELERATION_STRUCTURE_GEOMETRY_MICROMAP_DATA_KHR")
            }
            Self::ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_KHR => {
                Some("ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_64_INDEXING_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_64_INDEXING_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT")
            }
            Self::BEGIN_CUSTOM_RESOLVE_INFO_EXT => Some("BEGIN_CUSTOM_RESOLVE_INFO_EXT"),
            Self::CUSTOM_RESOLVE_CREATE_INFO_EXT => Some("CUSTOM_RESOLVE_CREATE_INFO_EXT"),
            Self::PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM => {
                Some("PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM")
            }
            Self::DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM => {
                Some("DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR")
            }
            Self::RENDERING_ATTACHMENT_FLAGS_INFO_KHR => {
                Some("RENDERING_ATTACHMENT_FLAGS_INFO_KHR")
            }
            Self::RENDERING_END_INFO_KHR => Some("RENDERING_END_INFO_KHR"),
            Self::RESOLVE_IMAGE_MODE_INFO_KHR => Some("RESOLVE_IMAGE_MODE_INFO_KHR"),
            Self::PHYSICAL_DEVICE_DATA_GRAPH_OPTICAL_FLOW_FEATURES_ARM => {
                Some("PHYSICAL_DEVICE_DATA_GRAPH_OPTICAL_FLOW_FEATURES_ARM")
            }
            Self::QUEUE_FAMILY_DATA_GRAPH_OPTICAL_FLOW_PROPERTIES_ARM => {
                Some("QUEUE_FAMILY_DATA_GRAPH_OPTICAL_FLOW_PROPERTIES_ARM")
            }
            Self::DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_INFO_ARM => {
                Some("DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_INFO_ARM")
            }
            Self::DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_ARM => {
                Some("DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_OPTICAL_FLOW_DISPATCH_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_OPTICAL_FLOW_DISPATCH_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_OPTICAL_FLOW_CREATE_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_OPTICAL_FLOW_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_RESOURCE_INFO_IMAGE_LAYOUT_ARM => {
                Some("DATA_GRAPH_PIPELINE_RESOURCE_INFO_IMAGE_LAYOUT_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_SINGLE_NODE_CREATE_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_SINGLE_NODE_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_SINGLE_NODE_CONNECTION_ARM => {
                Some("DATA_GRAPH_PIPELINE_SINGLE_NODE_CONNECTION_ARM")
            }
            Self::PHYSICAL_DEVICE_SHADER_LONG_VECTOR_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_LONG_VECTOR_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_LONG_VECTOR_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_LONG_VECTOR_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC => {
                Some("PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC")
            }
            Self::PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT")
            }
            Self::COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV => {
                Some("COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV")
            }
            Self::PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_11_FEATURES_KHR => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_11_FEATURES_KHR")
            }
            Self::QUEUE_FAMILY_OPTIMAL_IMAGE_TRANSFER_GRANULARITY_PROPERTIES_KHR => {
                Some("QUEUE_FAMILY_OPTIMAL_IMAGE_TRANSFER_GRANULARITY_PROPERTIES_KHR")
            }
            Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT")
            }
            Self::UBM_SURFACE_CREATE_INFO_SEC => Some("UBM_SURFACE_CREATE_INFO_SEC"),
            Self::PHYSICAL_DEVICE_SHADER_MIXED_FLOAT_DOT_PRODUCT_FEATURES_VALVE => {
                Some("PHYSICAL_DEVICE_SHADER_MIXED_FLOAT_DOT_PRODUCT_FEATURES_VALVE")
            }
            Self::PHYSICAL_DEVICE_THROTTLE_HINT_FEATURES_SEC => {
                Some("PHYSICAL_DEVICE_THROTTLE_HINT_FEATURES_SEC")
            }
            Self::THROTTLE_HINT_SUBMIT_INFO_SEC => Some("THROTTLE_HINT_SUBMIT_INFO_SEC"),
            Self::DATA_GRAPH_PIPELINE_NEURAL_STATISTICS_CREATE_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_NEURAL_STATISTICS_CREATE_INFO_ARM")
            }
            Self::DATA_GRAPH_PIPELINE_SESSION_NEURAL_STATISTICS_CREATE_INFO_ARM => {
                Some("DATA_GRAPH_PIPELINE_SESSION_NEURAL_STATISTICS_CREATE_INFO_ARM")
            }
            Self::PHYSICAL_DEVICE_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_FEATURES_ARM => {
                Some("PHYSICAL_DEVICE_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_FEATURES_ARM")
            }
            Self::PHYSICAL_DEVICE_PRIMITIVE_RESTART_INDEX_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_PRIMITIVE_RESTART_INDEX_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_DECODE_VECTOR_FEATURES_NV => {
                Some("PHYSICAL_DEVICE_COOPERATIVE_MATRIX_DECODE_VECTOR_FEATURES_NV")
            }
            Self::BIND_BUFFER_MEMORY_INFO => Some("BIND_BUFFER_MEMORY_INFO"),
            Self::BIND_IMAGE_MEMORY_INFO => Some("BIND_IMAGE_MEMORY_INFO"),
            Self::MEMORY_DEDICATED_REQUIREMENTS => Some("MEMORY_DEDICATED_REQUIREMENTS"),
            Self::MEMORY_DEDICATED_ALLOCATE_INFO => Some("MEMORY_DEDICATED_ALLOCATE_INFO"),
            Self::MEMORY_ALLOCATE_FLAGS_INFO => Some("MEMORY_ALLOCATE_FLAGS_INFO"),
            Self::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO => {
                Some("DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO")
            }
            Self::DEVICE_GROUP_SUBMIT_INFO => Some("DEVICE_GROUP_SUBMIT_INFO"),
            Self::DEVICE_GROUP_BIND_SPARSE_INFO => Some("DEVICE_GROUP_BIND_SPARSE_INFO"),
            Self::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO => {
                Some("BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO")
            }
            Self::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO => {
                Some("BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO")
            }
            Self::PHYSICAL_DEVICE_GROUP_PROPERTIES => Some("PHYSICAL_DEVICE_GROUP_PROPERTIES"),
            Self::DEVICE_GROUP_DEVICE_CREATE_INFO => Some("DEVICE_GROUP_DEVICE_CREATE_INFO"),
            Self::BUFFER_MEMORY_REQUIREMENTS_INFO_2 => Some("BUFFER_MEMORY_REQUIREMENTS_INFO_2"),
            Self::IMAGE_MEMORY_REQUIREMENTS_INFO_2 => Some("IMAGE_MEMORY_REQUIREMENTS_INFO_2"),
            Self::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2 => {
                Some("IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2")
            }
            Self::MEMORY_REQUIREMENTS_2 => Some("MEMORY_REQUIREMENTS_2"),
            Self::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2 => Some("SPARSE_IMAGE_MEMORY_REQUIREMENTS_2"),
            Self::PHYSICAL_DEVICE_FEATURES_2 => Some("PHYSICAL_DEVICE_FEATURES_2"),
            Self::PHYSICAL_DEVICE_PROPERTIES_2 => Some("PHYSICAL_DEVICE_PROPERTIES_2"),
            Self::FORMAT_PROPERTIES_2 => Some("FORMAT_PROPERTIES_2"),
            Self::IMAGE_FORMAT_PROPERTIES_2 => Some("IMAGE_FORMAT_PROPERTIES_2"),
            Self::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2 => {
                Some("PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2")
            }
            Self::QUEUE_FAMILY_PROPERTIES_2 => Some("QUEUE_FAMILY_PROPERTIES_2"),
            Self::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2 => {
                Some("PHYSICAL_DEVICE_MEMORY_PROPERTIES_2")
            }
            Self::SPARSE_IMAGE_FORMAT_PROPERTIES_2 => Some("SPARSE_IMAGE_FORMAT_PROPERTIES_2"),
            Self::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2 => {
                Some("PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2")
            }
            Self::IMAGE_VIEW_USAGE_CREATE_INFO => Some("IMAGE_VIEW_USAGE_CREATE_INFO"),
            Self::PROTECTED_SUBMIT_INFO => Some("PROTECTED_SUBMIT_INFO"),
            Self::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES => {
                Some("PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES")
            }
            Self::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES => {
                Some("PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES")
            }
            Self::DEVICE_QUEUE_INFO_2 => Some("DEVICE_QUEUE_INFO_2"),
            Self::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO => {
                Some("PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO")
            }
            Self::EXTERNAL_IMAGE_FORMAT_PROPERTIES => Some("EXTERNAL_IMAGE_FORMAT_PROPERTIES"),
            Self::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO => {
                Some("PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO")
            }
            Self::EXTERNAL_BUFFER_PROPERTIES => Some("EXTERNAL_BUFFER_PROPERTIES"),
            Self::PHYSICAL_DEVICE_ID_PROPERTIES => Some("PHYSICAL_DEVICE_ID_PROPERTIES"),
            Self::EXTERNAL_MEMORY_BUFFER_CREATE_INFO => Some("EXTERNAL_MEMORY_BUFFER_CREATE_INFO"),
            Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO => Some("EXTERNAL_MEMORY_IMAGE_CREATE_INFO"),
            Self::EXPORT_MEMORY_ALLOCATE_INFO => Some("EXPORT_MEMORY_ALLOCATE_INFO"),
            Self::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO => {
                Some("PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO")
            }
            Self::EXTERNAL_FENCE_PROPERTIES => Some("EXTERNAL_FENCE_PROPERTIES"),
            Self::EXPORT_FENCE_CREATE_INFO => Some("EXPORT_FENCE_CREATE_INFO"),
            Self::EXPORT_SEMAPHORE_CREATE_INFO => Some("EXPORT_SEMAPHORE_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO => {
                Some("PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO")
            }
            Self::EXTERNAL_SEMAPHORE_PROPERTIES => Some("EXTERNAL_SEMAPHORE_PROPERTIES"),
            Self::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES => {
                Some("PHYSICAL_DEVICE_SUBGROUP_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES => {
                Some("PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES")
            }
            Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES => {
                Some("PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES")
            }
            Self::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO => {
                Some("DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES")
            }
            Self::DESCRIPTOR_SET_LAYOUT_SUPPORT => Some("DESCRIPTOR_SET_LAYOUT_SUPPORT"),
            Self::SAMPLER_YCBCR_CONVERSION_CREATE_INFO => {
                Some("SAMPLER_YCBCR_CONVERSION_CREATE_INFO")
            }
            Self::SAMPLER_YCBCR_CONVERSION_INFO => Some("SAMPLER_YCBCR_CONVERSION_INFO"),
            Self::BIND_IMAGE_PLANE_MEMORY_INFO => Some("BIND_IMAGE_PLANE_MEMORY_INFO"),
            Self::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO => {
                Some("IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO")
            }
            Self::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES => {
                Some("PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES")
            }
            Self::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES => {
                Some("SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES")
            }
            Self::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO => {
                Some("DEVICE_GROUP_RENDER_PASS_BEGIN_INFO")
            }
            Self::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES => {
                Some("PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES")
            }
            Self::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO => {
                Some("RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO")
            }
            Self::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO => {
                Some("PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO")
            }
            Self::RENDER_PASS_MULTIVIEW_CREATE_INFO => Some("RENDER_PASS_MULTIVIEW_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_MULTIVIEW_FEATURES => Some("PHYSICAL_DEVICE_MULTIVIEW_FEATURES"),
            Self::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES => {
                Some("PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES => {
                Some("PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES")
            }
            Self::PHYSICAL_DEVICE_DRIVER_PROPERTIES => Some("PHYSICAL_DEVICE_DRIVER_PROPERTIES"),
            Self::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES => {
                Some("PHYSICAL_DEVICE_VULKAN_1_1_FEATURES")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES => {
                Some("PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES => {
                Some("PHYSICAL_DEVICE_VULKAN_1_2_FEATURES")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES => {
                Some("PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES")
            }
            Self::IMAGE_FORMAT_LIST_CREATE_INFO => Some("IMAGE_FORMAT_LIST_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES => {
                Some("PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES")
            }
            Self::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES => {
                Some("PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES")
            }
            Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES => {
                Some("PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES")
            }
            Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES => {
                Some("PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES")
            }
            Self::SEMAPHORE_TYPE_CREATE_INFO => Some("SEMAPHORE_TYPE_CREATE_INFO"),
            Self::TIMELINE_SEMAPHORE_SUBMIT_INFO => Some("TIMELINE_SEMAPHORE_SUBMIT_INFO"),
            Self::SEMAPHORE_WAIT_INFO => Some("SEMAPHORE_WAIT_INFO"),
            Self::SEMAPHORE_SIGNAL_INFO => Some("SEMAPHORE_SIGNAL_INFO"),
            Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES => {
                Some("PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES")
            }
            Self::BUFFER_DEVICE_ADDRESS_INFO => Some("BUFFER_DEVICE_ADDRESS_INFO"),
            Self::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO => {
                Some("BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO")
            }
            Self::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO => {
                Some("MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO")
            }
            Self::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO => {
                Some("DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO")
            }
            Self::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES => {
                Some("PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES => {
                Some("PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES => {
                Some("PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES")
            }
            Self::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES => {
                Some("PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES")
            }
            Self::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO => {
                Some("DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES => {
                Some("PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES => {
                Some("PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES")
            }
            Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO => {
                Some("DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO")
            }
            Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT => {
                Some("DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT")
            }
            Self::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES => {
                Some("PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES => {
                Some("PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES")
            }
            Self::SAMPLER_REDUCTION_MODE_CREATE_INFO => Some("SAMPLER_REDUCTION_MODE_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES => {
                Some("PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES => {
                Some("PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES")
            }
            Self::ATTACHMENT_DESCRIPTION_2 => Some("ATTACHMENT_DESCRIPTION_2"),
            Self::ATTACHMENT_REFERENCE_2 => Some("ATTACHMENT_REFERENCE_2"),
            Self::SUBPASS_DESCRIPTION_2 => Some("SUBPASS_DESCRIPTION_2"),
            Self::SUBPASS_DEPENDENCY_2 => Some("SUBPASS_DEPENDENCY_2"),
            Self::RENDER_PASS_CREATE_INFO_2 => Some("RENDER_PASS_CREATE_INFO_2"),
            Self::SUBPASS_BEGIN_INFO => Some("SUBPASS_BEGIN_INFO"),
            Self::SUBPASS_END_INFO => Some("SUBPASS_END_INFO"),
            Self::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES => {
                Some("PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES")
            }
            Self::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE => {
                Some("SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE")
            }
            Self::IMAGE_STENCIL_USAGE_CREATE_INFO => Some("IMAGE_STENCIL_USAGE_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES => {
                Some("PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES")
            }
            Self::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO => {
                Some("FRAMEBUFFER_ATTACHMENTS_CREATE_INFO")
            }
            Self::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO => Some("FRAMEBUFFER_ATTACHMENT_IMAGE_INFO"),
            Self::RENDER_PASS_ATTACHMENT_BEGIN_INFO => Some("RENDER_PASS_ATTACHMENT_BEGIN_INFO"),
            Self::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES => {
                Some("PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES")
            }
            Self::ATTACHMENT_REFERENCE_STENCIL_LAYOUT => {
                Some("ATTACHMENT_REFERENCE_STENCIL_LAYOUT")
            }
            Self::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT => {
                Some("ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES => {
                Some("PHYSICAL_DEVICE_VULKAN_1_3_FEATURES")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES => {
                Some("PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_TOOL_PROPERTIES => Some("PHYSICAL_DEVICE_TOOL_PROPERTIES"),
            Self::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES => {
                Some("PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES")
            }
            Self::DEVICE_PRIVATE_DATA_CREATE_INFO => Some("DEVICE_PRIVATE_DATA_CREATE_INFO"),
            Self::PRIVATE_DATA_SLOT_CREATE_INFO => Some("PRIVATE_DATA_SLOT_CREATE_INFO"),
            Self::MEMORY_BARRIER_2 => Some("MEMORY_BARRIER_2"),
            Self::BUFFER_MEMORY_BARRIER_2 => Some("BUFFER_MEMORY_BARRIER_2"),
            Self::IMAGE_MEMORY_BARRIER_2 => Some("IMAGE_MEMORY_BARRIER_2"),
            Self::DEPENDENCY_INFO => Some("DEPENDENCY_INFO"),
            Self::SUBMIT_INFO_2 => Some("SUBMIT_INFO_2"),
            Self::SEMAPHORE_SUBMIT_INFO => Some("SEMAPHORE_SUBMIT_INFO"),
            Self::COMMAND_BUFFER_SUBMIT_INFO => Some("COMMAND_BUFFER_SUBMIT_INFO"),
            Self::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES => {
                Some("PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES")
            }
            Self::COPY_BUFFER_INFO_2 => Some("COPY_BUFFER_INFO_2"),
            Self::COPY_IMAGE_INFO_2 => Some("COPY_IMAGE_INFO_2"),
            Self::COPY_BUFFER_TO_IMAGE_INFO_2 => Some("COPY_BUFFER_TO_IMAGE_INFO_2"),
            Self::COPY_IMAGE_TO_BUFFER_INFO_2 => Some("COPY_IMAGE_TO_BUFFER_INFO_2"),
            Self::BUFFER_COPY_2 => Some("BUFFER_COPY_2"),
            Self::IMAGE_COPY_2 => Some("IMAGE_COPY_2"),
            Self::BUFFER_IMAGE_COPY_2 => Some("BUFFER_IMAGE_COPY_2"),
            Self::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES => {
                Some("PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES")
            }
            Self::FORMAT_PROPERTIES_3 => Some("FORMAT_PROPERTIES_3"),
            Self::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES")
            }
            Self::DEVICE_BUFFER_MEMORY_REQUIREMENTS => Some("DEVICE_BUFFER_MEMORY_REQUIREMENTS"),
            Self::DEVICE_IMAGE_MEMORY_REQUIREMENTS => Some("DEVICE_IMAGE_MEMORY_REQUIREMENTS"),
            Self::PIPELINE_CREATION_FEEDBACK_CREATE_INFO => {
                Some("PIPELINE_CREATION_FEEDBACK_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES => {
                Some("PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES => {
                Some("PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES")
            }
            Self::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES => {
                Some("PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES")
            }
            Self::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES => {
                Some("PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES")
            }
            Self::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES => {
                Some("PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES => {
                Some("PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES")
            }
            Self::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO => {
                Some("PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES => {
                Some("PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES")
            }
            Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES => {
                Some("PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES")
            }
            Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES => {
                Some("PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES")
            }
            Self::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK => {
                Some("WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK")
            }
            Self::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO => {
                Some("DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES => {
                Some("PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES => {
                Some("PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES => {
                Some("PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES")
            }
            Self::BLIT_IMAGE_INFO_2 => Some("BLIT_IMAGE_INFO_2"),
            Self::RESOLVE_IMAGE_INFO_2 => Some("RESOLVE_IMAGE_INFO_2"),
            Self::IMAGE_BLIT_2 => Some("IMAGE_BLIT_2"),
            Self::IMAGE_RESOLVE_2 => Some("IMAGE_RESOLVE_2"),
            Self::RENDERING_INFO => Some("RENDERING_INFO"),
            Self::RENDERING_ATTACHMENT_INFO => Some("RENDERING_ATTACHMENT_INFO"),
            Self::PIPELINE_RENDERING_CREATE_INFO => Some("PIPELINE_RENDERING_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES => {
                Some("PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES")
            }
            Self::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO => {
                Some("COMMAND_BUFFER_INHERITANCE_RENDERING_INFO")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_4_FEATURES => {
                Some("PHYSICAL_DEVICE_VULKAN_1_4_FEATURES")
            }
            Self::PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES => {
                Some("PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES")
            }
            Self::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO => {
                Some("DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES => {
                Some("PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES")
            }
            Self::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES => {
                Some("QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES => {
                Some("PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES")
            }
            Self::MEMORY_MAP_INFO => Some("MEMORY_MAP_INFO"),
            Self::MEMORY_UNMAP_INFO => Some("MEMORY_UNMAP_INFO"),
            Self::PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES")
            }
            Self::DEVICE_IMAGE_SUBRESOURCE_INFO => Some("DEVICE_IMAGE_SUBRESOURCE_INFO"),
            Self::SUBRESOURCE_LAYOUT_2 => Some("SUBRESOURCE_LAYOUT_2"),
            Self::IMAGE_SUBRESOURCE_2 => Some("IMAGE_SUBRESOURCE_2"),
            Self::BUFFER_USAGE_FLAGS_2_CREATE_INFO => Some("BUFFER_USAGE_FLAGS_2_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES")
            }
            Self::PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES")
            }
            Self::BIND_MEMORY_STATUS => Some("BIND_MEMORY_STATUS"),
            Self::PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES => {
                Some("PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES")
            }
            Self::PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES => {
                Some("PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES")
            }
            Self::MEMORY_TO_IMAGE_COPY => Some("MEMORY_TO_IMAGE_COPY"),
            Self::IMAGE_TO_MEMORY_COPY => Some("IMAGE_TO_MEMORY_COPY"),
            Self::COPY_IMAGE_TO_MEMORY_INFO => Some("COPY_IMAGE_TO_MEMORY_INFO"),
            Self::COPY_MEMORY_TO_IMAGE_INFO => Some("COPY_MEMORY_TO_IMAGE_INFO"),
            Self::HOST_IMAGE_LAYOUT_TRANSITION_INFO => Some("HOST_IMAGE_LAYOUT_TRANSITION_INFO"),
            Self::COPY_IMAGE_TO_IMAGE_INFO => Some("COPY_IMAGE_TO_IMAGE_INFO"),
            Self::SUBRESOURCE_HOST_MEMCPY_SIZE => Some("SUBRESOURCE_HOST_MEMCPY_SIZE"),
            Self::HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY => {
                Some("HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY")
            }
            Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES => {
                Some("PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES => {
                Some("PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES")
            }
            Self::PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES => {
                Some("PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES")
            }
            Self::PIPELINE_CREATE_FLAGS_2_CREATE_INFO => {
                Some("PIPELINE_CREATE_FLAGS_2_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES => {
                Some("PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES")
            }
            Self::BIND_DESCRIPTOR_SETS_INFO => Some("BIND_DESCRIPTOR_SETS_INFO"),
            Self::PUSH_CONSTANTS_INFO => Some("PUSH_CONSTANTS_INFO"),
            Self::PUSH_DESCRIPTOR_SET_INFO => Some("PUSH_DESCRIPTOR_SET_INFO"),
            Self::PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO => {
                Some("PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO")
            }
            Self::PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES => {
                Some("PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES")
            }
            Self::PIPELINE_ROBUSTNESS_CREATE_INFO => Some("PIPELINE_ROBUSTNESS_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES => {
                Some("PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES")
            }
            Self::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES => {
                Some("PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES => {
                Some("PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES")
            }
            Self::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO => {
                Some("PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES => {
                Some("PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES => {
                Some("PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES")
            }
            Self::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO => {
                Some("PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES => {
                Some("PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES")
            }
            Self::RENDERING_AREA_INFO => Some("RENDERING_AREA_INFO"),
            Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES => {
                Some("PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES")
            }
            Self::RENDERING_ATTACHMENT_LOCATION_INFO => Some("RENDERING_ATTACHMENT_LOCATION_INFO"),
            Self::RENDERING_INPUT_ATTACHMENT_INDEX_INFO => {
                Some("RENDERING_INPUT_ATTACHMENT_INDEX_INFO")
            }
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for SubgroupFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (SubgroupFeatureFlags::BASIC.0, "BASIC"),
            (SubgroupFeatureFlags::VOTE.0, "VOTE"),
            (SubgroupFeatureFlags::ARITHMETIC.0, "ARITHMETIC"),
            (SubgroupFeatureFlags::BALLOT.0, "BALLOT"),
            (SubgroupFeatureFlags::SHUFFLE.0, "SHUFFLE"),
            (SubgroupFeatureFlags::SHUFFLE_RELATIVE.0, "SHUFFLE_RELATIVE"),
            (SubgroupFeatureFlags::CLUSTERED.0, "CLUSTERED"),
            (SubgroupFeatureFlags::QUAD.0, "QUAD"),
            (SubgroupFeatureFlags::PARTITIONED_EXT.0, "PARTITIONED_EXT"),
            (SubgroupFeatureFlags::ROTATE.0, "ROTATE"),
            (SubgroupFeatureFlags::ROTATE_CLUSTERED.0, "ROTATE_CLUSTERED"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for SubmitFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(SubmitFlags::PROTECTED.0, "PROTECTED")];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for SubpassContents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::INLINE => Some("INLINE"),
            Self::SECONDARY_COMMAND_BUFFERS => Some("SECONDARY_COMMAND_BUFFERS"),
            Self::INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR => {
                Some("INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR")
            }
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for SubpassDescriptionFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                SubpassDescriptionFlags::PER_VIEW_ATTRIBUTES_NVX.0,
                "PER_VIEW_ATTRIBUTES_NVX",
            ),
            (
                SubpassDescriptionFlags::PER_VIEW_POSITION_X_ONLY_NVX.0,
                "PER_VIEW_POSITION_X_ONLY_NVX",
            ),
            (
                SubpassDescriptionFlags::TILE_SHADING_APRON_QCOM.0,
                "TILE_SHADING_APRON_QCOM",
            ),
            (
                SubpassDescriptionFlags::RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT.0,
                "RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT",
            ),
            (
                SubpassDescriptionFlags::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT.0,
                "RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT",
            ),
            (
                SubpassDescriptionFlags::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT.0,
                "RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT",
            ),
            (
                SubpassDescriptionFlags::ENABLE_LEGACY_DITHERING_EXT.0,
                "ENABLE_LEGACY_DITHERING_EXT",
            ),
            (
                SubpassDescriptionFlags::FRAGMENT_REGION_EXT.0,
                "FRAGMENT_REGION_EXT",
            ),
            (
                SubpassDescriptionFlags::CUSTOM_RESOLVE_EXT.0,
                "CUSTOM_RESOLVE_EXT",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for SurfaceTransformFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (SurfaceTransformFlagsKHR::IDENTITY.0, "IDENTITY"),
            (SurfaceTransformFlagsKHR::ROTATE_90.0, "ROTATE_90"),
            (SurfaceTransformFlagsKHR::ROTATE_180.0, "ROTATE_180"),
            (SurfaceTransformFlagsKHR::ROTATE_270.0, "ROTATE_270"),
            (
                SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR.0,
                "HORIZONTAL_MIRROR",
            ),
            (
                SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_90.0,
                "HORIZONTAL_MIRROR_ROTATE_90",
            ),
            (
                SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_180.0,
                "HORIZONTAL_MIRROR_ROTATE_180",
            ),
            (
                SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_270.0,
                "HORIZONTAL_MIRROR_ROTATE_270",
            ),
            (SurfaceTransformFlagsKHR::INHERIT.0, "INHERIT"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for SwapchainCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                SwapchainCreateFlagsKHR::SPLIT_INSTANCE_BIND_REGIONS.0,
                "SPLIT_INSTANCE_BIND_REGIONS",
            ),
            (SwapchainCreateFlagsKHR::PROTECTED.0, "PROTECTED"),
            (SwapchainCreateFlagsKHR::MUTABLE_FORMAT.0, "MUTABLE_FORMAT"),
            (
                SwapchainCreateFlagsKHR::PRESENT_TIMING_EXT.0,
                "PRESENT_TIMING_EXT",
            ),
            (SwapchainCreateFlagsKHR::PRESENT_ID_2.0, "PRESENT_ID_2"),
            (SwapchainCreateFlagsKHR::PRESENT_WAIT_2.0, "PRESENT_WAIT_2"),
            (
                SwapchainCreateFlagsKHR::DEFERRED_MEMORY_ALLOCATION.0,
                "DEFERRED_MEMORY_ALLOCATION",
            ),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for SystemAllocationScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::COMMAND => Some("COMMAND"),
            Self::OBJECT => Some("OBJECT"),
            Self::CACHE => Some("CACHE"),
            Self::DEVICE => Some("DEVICE"),
            Self::INSTANCE => Some("INSTANCE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for TensorViewCreateFlagsARM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags64, &str)] = &[(
            TensorViewCreateFlagsARM::DESCRIPTOR_BUFFER_CAPTURE_REPLAY.0,
            "DESCRIPTOR_BUFFER_CAPTURE_REPLAY",
        )];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for TessellationDomainOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::UPPER_LEFT => Some("UPPER_LEFT"),
            Self::LOWER_LEFT => Some("LOWER_LEFT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for ToolPurposeFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ToolPurposeFlags::VALIDATION.0, "VALIDATION"),
            (ToolPurposeFlags::PROFILING.0, "PROFILING"),
            (ToolPurposeFlags::TRACING.0, "TRACING"),
            (
                ToolPurposeFlags::ADDITIONAL_FEATURES.0,
                "ADDITIONAL_FEATURES",
            ),
            (ToolPurposeFlags::MODIFYING_FEATURES.0, "MODIFYING_FEATURES"),
            (
                ToolPurposeFlags::DEBUG_REPORTING_EXT.0,
                "DEBUG_REPORTING_EXT",
            ),
            (ToolPurposeFlags::DEBUG_MARKERS_EXT.0, "DEBUG_MARKERS_EXT"),
        ];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for VendorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::KHRONOS => Some("KHRONOS"),
            Self::VIV => Some("VIV"),
            Self::VSI => Some("VSI"),
            Self::KAZAN => Some("KAZAN"),
            Self::CODEPLAY => Some("CODEPLAY"),
            Self::MESA => Some("MESA"),
            Self::POCL => Some("POCL"),
            Self::MOBILEYE => Some("MOBILEYE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for VertexInputRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::VERTEX => Some("VERTEX"),
            Self::INSTANCE => Some("INSTANCE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for ViewportCoordinateSwizzleNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Self::POSITIVE_X => Some("POSITIVE_X"),
            Self::NEGATIVE_X => Some("NEGATIVE_X"),
            Self::POSITIVE_Y => Some("POSITIVE_Y"),
            Self::NEGATIVE_Y => Some("NEGATIVE_Y"),
            Self::POSITIVE_Z => Some("POSITIVE_Z"),
            Self::NEGATIVE_Z => Some("NEGATIVE_Z"),
            Self::POSITIVE_W => Some("POSITIVE_W"),
            Self::NEGATIVE_W => Some("NEGATIVE_W"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            self.0.fmt(f)
        }
    }
}
impl fmt::Debug for WaylandSurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for Win32SurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for XcbSurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
impl fmt::Debug for XlibSurfaceCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[];
        debug_flags(f, KNOWN, self.0)
    }
}
