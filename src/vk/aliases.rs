use super::bitflags::*;
use super::definitions::*;
use super::enums::*;
pub type PipelineCreationFeedbackFlagsEXT = PipelineCreationFeedbackFlags;
pub type AccessFlags2KHR = AccessFlags2;
pub type PipelineStageFlags2KHR = PipelineStageFlags2;
pub type RenderingFlagsKHR = RenderingFlags;
pub type PipelineCreateFlags2KHR = PipelineCreateFlags2;
pub type BufferUsageFlags2KHR = BufferUsageFlags2;
pub type CommandPoolTrimFlagsKHR = CommandPoolTrimFlags;
pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;
pub type SubmitFlagsKHR = SubmitFlags;
pub type HostImageCopyFlagsEXT = HostImageCopyFlags;
pub type PointClippingBehaviorKHR = PointClippingBehavior;
pub type LineRasterizationModeEXT = LineRasterizationMode;
pub type TessellationDomainOriginKHR = TessellationDomainOrigin;
pub type BufferUsageFlags2CreateInfoKHR<'a> = BufferUsageFlags2CreateInfo<'a>;
pub type PipelineCreateFlags2CreateInfoKHR<'a> = PipelineCreateFlags2CreateInfo<'a>;
pub type PhysicalDeviceFeatures2KHR<'a> = PhysicalDeviceFeatures2<'a>;
pub type PhysicalDeviceProperties2KHR<'a> = PhysicalDeviceProperties2<'a>;
pub type FormatProperties2KHR<'a> = FormatProperties2<'a>;
pub type ImageFormatProperties2KHR<'a> = ImageFormatProperties2<'a>;
pub type PhysicalDeviceImageFormatInfo2KHR<'a> = PhysicalDeviceImageFormatInfo2<'a>;
pub type QueueFamilyProperties2KHR<'a> = QueueFamilyProperties2<'a>;
pub type PhysicalDeviceMemoryProperties2KHR<'a> = PhysicalDeviceMemoryProperties2<'a>;
pub type SparseImageFormatProperties2KHR<'a> = SparseImageFormatProperties2<'a>;
pub type PhysicalDeviceSparseImageFormatInfo2KHR<'a> = PhysicalDeviceSparseImageFormatInfo2<'a>;
pub type PhysicalDeviceVariablePointerFeatures<'a> = PhysicalDeviceVariablePointersFeatures<'a>;
pub type InputAttachmentAspectReferenceKHR = InputAttachmentAspectReference;
pub type RenderPassInputAttachmentAspectCreateInfoKHR<'a> =
    RenderPassInputAttachmentAspectCreateInfo<'a>;
pub type DeviceBufferMemoryRequirementsKHR<'a> = DeviceBufferMemoryRequirements<'a>;
pub type DeviceImageMemoryRequirementsKHR<'a> = DeviceImageMemoryRequirements<'a>;
pub type PhysicalDevicePointClippingPropertiesKHR<'a> = PhysicalDevicePointClippingProperties<'a>;
pub type ImageViewUsageCreateInfoKHR<'a> = ImageViewUsageCreateInfo<'a>;
pub type PipelineTessellationDomainOriginStateCreateInfoKHR<'a> =
    PipelineTessellationDomainOriginStateCreateInfo<'a>;
pub type PhysicalDeviceMaintenance3PropertiesKHR<'a> = PhysicalDeviceMaintenance3Properties<'a>;
pub type PhysicalDeviceMaintenance4FeaturesKHR<'a> = PhysicalDeviceMaintenance4Features<'a>;
pub type PhysicalDeviceMaintenance4PropertiesKHR<'a> = PhysicalDeviceMaintenance4Properties<'a>;
pub type PhysicalDeviceMaintenance5FeaturesKHR<'a> = PhysicalDeviceMaintenance5Features<'a>;
pub type PhysicalDeviceMaintenance5PropertiesKHR<'a> = PhysicalDeviceMaintenance5Properties<'a>;
pub type PhysicalDeviceMaintenance6FeaturesKHR<'a> = PhysicalDeviceMaintenance6Features<'a>;
pub type PhysicalDeviceMaintenance6PropertiesKHR<'a> = PhysicalDeviceMaintenance6Properties<'a>;
pub type RenderingAreaInfoKHR<'a> = RenderingAreaInfo<'a>;
pub type DescriptorSetLayoutSupportKHR<'a> = DescriptorSetLayoutSupport<'a>;
pub type PhysicalDeviceShaderDrawParameterFeatures<'a> =
    PhysicalDeviceShaderDrawParametersFeatures<'a>;
pub type PhysicalDeviceDescriptorIndexingFeaturesEXT<'a> =
    PhysicalDeviceDescriptorIndexingFeatures<'a>;
pub type PhysicalDeviceDescriptorIndexingPropertiesEXT<'a> =
    PhysicalDeviceDescriptorIndexingProperties<'a>;
pub type DescriptorSetLayoutBindingFlagsCreateInfoEXT<'a> =
    DescriptorSetLayoutBindingFlagsCreateInfo<'a>;
pub type DescriptorSetVariableDescriptorCountAllocateInfoEXT<'a> =
    DescriptorSetVariableDescriptorCountAllocateInfo<'a>;
pub type DescriptorSetVariableDescriptorCountLayoutSupportEXT<'a> =
    DescriptorSetVariableDescriptorCountLayoutSupport<'a>;
pub type PhysicalDeviceBufferDeviceAddressFeaturesKHR<'a> =
    PhysicalDeviceBufferDeviceAddressFeatures<'a>;
pub type BufferDeviceAddressInfoKHR<'a> = BufferDeviceAddressInfo<'a>;
pub type BufferOpaqueCaptureAddressCreateInfoKHR<'a> = BufferOpaqueCaptureAddressCreateInfo<'a>;
pub type PipelineCreationFeedbackEXT = PipelineCreationFeedback;
pub type PipelineCreationFeedbackCreateInfoEXT<'a> = PipelineCreationFeedbackCreateInfo<'a>;
pub type ShaderRequiredSubgroupSizeCreateInfoEXT<'a> =
    PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a>;
pub type MemoryOpaqueCaptureAddressAllocateInfoKHR<'a> = MemoryOpaqueCaptureAddressAllocateInfo<'a>;
pub type DeviceMemoryOpaqueCaptureAddressInfoKHR<'a> = DeviceMemoryOpaqueCaptureAddressInfo<'a>;
pub type PhysicalDevicePipelineCreationCacheControlFeaturesEXT<'a> =
    PhysicalDevicePipelineCreationCacheControlFeatures<'a>;
pub type BufferCopy2KHR<'a> = BufferCopy2<'a>;
pub type ImageCopy2KHR<'a> = ImageCopy2<'a>;
pub type ImageBlit2KHR<'a> = ImageBlit2<'a>;
pub type BufferImageCopy2KHR<'a> = BufferImageCopy2<'a>;
pub type ImageResolve2KHR<'a> = ImageResolve2<'a>;
pub type CopyBufferInfo2KHR<'a> = CopyBufferInfo2<'a>;
pub type CopyImageInfo2KHR<'a> = CopyImageInfo2<'a>;
pub type BlitImageInfo2KHR<'a> = BlitImageInfo2<'a>;
pub type CopyBufferToImageInfo2KHR<'a> = CopyBufferToImageInfo2<'a>;
pub type CopyImageToBufferInfo2KHR<'a> = CopyImageToBufferInfo2<'a>;
pub type ResolveImageInfo2KHR<'a> = ResolveImageInfo2<'a>;
pub type MemoryBarrier2KHR<'a> = MemoryBarrier2<'a>;
pub type ImageMemoryBarrier2KHR<'a> = ImageMemoryBarrier2<'a>;
pub type BufferMemoryBarrier2KHR<'a> = BufferMemoryBarrier2<'a>;
pub type DependencyInfoKHR<'a> = DependencyInfo<'a>;
pub type SemaphoreSubmitInfoKHR<'a> = SemaphoreSubmitInfo<'a>;
pub type CommandBufferSubmitInfoKHR<'a> = CommandBufferSubmitInfo<'a>;
pub type SubmitInfo2KHR<'a> = SubmitInfo2<'a>;
pub type PhysicalDeviceSynchronization2FeaturesKHR<'a> = PhysicalDeviceSynchronization2Features<'a>;
pub type PhysicalDeviceHostImageCopyFeaturesEXT<'a> = PhysicalDeviceHostImageCopyFeatures<'a>;
pub type PhysicalDeviceHostImageCopyPropertiesEXT<'a> = PhysicalDeviceHostImageCopyProperties<'a>;
pub type MemoryToImageCopyEXT<'a> = MemoryToImageCopy<'a>;
pub type ImageToMemoryCopyEXT<'a> = ImageToMemoryCopy<'a>;
pub type CopyMemoryToImageInfoEXT<'a> = CopyMemoryToImageInfo<'a>;
pub type CopyImageToMemoryInfoEXT<'a> = CopyImageToMemoryInfo<'a>;
pub type CopyImageToImageInfoEXT<'a> = CopyImageToImageInfo<'a>;
pub type HostImageLayoutTransitionInfoEXT<'a> = HostImageLayoutTransitionInfo<'a>;
pub type SubresourceHostMemcpySizeEXT<'a> = SubresourceHostMemcpySize<'a>;
pub type HostImageCopyDevicePerformanceQueryEXT<'a> = HostImageCopyDevicePerformanceQuery<'a>;
pub type PipelineRenderingCreateInfoKHR<'a> = PipelineRenderingCreateInfo<'a>;
pub type RenderingInfoKHR<'a> = RenderingInfo<'a>;
pub type RenderingAttachmentInfoKHR<'a> = RenderingAttachmentInfo<'a>;
pub type PhysicalDeviceDynamicRenderingFeaturesKHR<'a> = PhysicalDeviceDynamicRenderingFeatures<'a>;
pub type CommandBufferInheritanceRenderingInfoKHR<'a> = CommandBufferInheritanceRenderingInfo<'a>;
pub type ImageSubresource2KHR<'a> = ImageSubresource2<'a>;
pub type ImageSubresource2EXT<'a> = ImageSubresource2<'a>;
pub type SubresourceLayout2KHR<'a> = SubresourceLayout2<'a>;
pub type SubresourceLayout2EXT<'a> = SubresourceLayout2<'a>;
pub type DeviceImageSubresourceInfoKHR<'a> = DeviceImageSubresourceInfo<'a>;
pub type BindMemoryStatusKHR<'a> = BindMemoryStatus<'a>;
pub type BindDescriptorSetsInfoKHR<'a> = BindDescriptorSetsInfo<'a>;
pub type PushConstantsInfoKHR<'a> = PushConstantsInfo<'a>;
pub type PushDescriptorSetInfoKHR<'a> = PushDescriptorSetInfo<'a>;
pub type PushDescriptorSetWithTemplateInfoKHR<'a> = PushDescriptorSetWithTemplateInfo<'a>;
pub type PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR<'a> =
    PhysicalDeviceDynamicRenderingLocalReadFeatures<'a>;
pub type RenderingAttachmentLocationInfoKHR<'a> = RenderingAttachmentLocationInfo<'a>;
pub type RenderingInputAttachmentIndexInfoKHR<'a> = RenderingInputAttachmentIndexInfo<'a>;
pub type DeviceAddressRangeEXT = DeviceAddressRangeKHR;
