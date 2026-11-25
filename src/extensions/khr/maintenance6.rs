//! <https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance6.html>

use crate::vk;

impl crate::khr::maintenance6::Device {

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2KHR.html>
    ///
    /// Deprecated: <https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-descriptor-sets>
    #[inline]
    #[doc(alias = "vkCmdPushConstants2KHR")]
    pub unsafe fn cmd_push_constants2(
        &self,
        command_buffer: vk::CommandBuffer,
        push_constants_info: &vk::PushConstantsInfoKHR<'_>,
    ) {
        (self.fp.cmd_push_constants2_khr)(command_buffer, push_constants_info)
    }




}
