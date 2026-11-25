#[doc = "Extensions tagged EXT"]
pub mod ext {
    #[doc = "VK_EXT_debug_utils"]
    pub mod debug_utils {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::EXT_DEBUG_UTILS_NAME as NAME,
            crate::vk::EXT_DEBUG_UTILS_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_debug_utils instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn load(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle;
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_debug_utils instance-level function pointers"]
        pub struct InstanceFn {
            pub create_debug_utils_messenger_ext: PFN_vkCreateDebugUtilsMessengerEXT,
            pub destroy_debug_utils_messenger_ext: PFN_vkDestroyDebugUtilsMessengerEXT,
            pub submit_debug_utils_message_ext: PFN_vkSubmitDebugUtilsMessageEXT,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_debug_utils_messenger_ext: unsafe {
                        unsafe extern "system" fn create_debug_utils_messenger_ext(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const DebugUtilsMessengerCreateInfoEXT<'_>,
                            _p_allocator: *const AllocationCallbacks,
                            _p_messenger: *mut DebugUtilsMessengerEXT,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_debug_utils_messenger_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateDebugUtilsMessengerEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_debug_utils_messenger_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_debug_utils_messenger_ext: unsafe {
                        unsafe extern "system" fn destroy_debug_utils_messenger_ext(
                            _instance: crate::vk::Instance,
                            _messenger: DebugUtilsMessengerEXT,
                            _p_allocator: *const AllocationCallbacks,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_debug_utils_messenger_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkDestroyDebugUtilsMessengerEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_debug_utils_messenger_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    submit_debug_utils_message_ext: unsafe {
                        unsafe extern "system" fn submit_debug_utils_message_ext(
                            _instance: crate::vk::Instance,
                            _message_severity: DebugUtilsMessageSeverityFlagsEXT,
                            _message_types: DebugUtilsMessageTypeFlagsEXT,
                            _p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(submit_debug_utils_message_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkSubmitDebugUtilsMessageEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            submit_debug_utils_message_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_EXT_debug_utils device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_debug_utils device-level function pointers"]
        pub struct DeviceFn {
            pub set_debug_utils_object_name_ext: PFN_vkSetDebugUtilsObjectNameEXT,
            pub set_debug_utils_object_tag_ext: PFN_vkSetDebugUtilsObjectTagEXT,
            pub queue_begin_debug_utils_label_ext: PFN_vkQueueBeginDebugUtilsLabelEXT,
            pub queue_end_debug_utils_label_ext: PFN_vkQueueEndDebugUtilsLabelEXT,
            pub queue_insert_debug_utils_label_ext: PFN_vkQueueInsertDebugUtilsLabelEXT,
            pub cmd_begin_debug_utils_label_ext: PFN_vkCmdBeginDebugUtilsLabelEXT,
            pub cmd_end_debug_utils_label_ext: PFN_vkCmdEndDebugUtilsLabelEXT,
            pub cmd_insert_debug_utils_label_ext: PFN_vkCmdInsertDebugUtilsLabelEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    set_debug_utils_object_name_ext: unsafe {
                        unsafe extern "system" fn set_debug_utils_object_name_ext(
                            _device: crate::vk::Device,
                            _p_name_info: *const DebugUtilsObjectNameInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(set_debug_utils_object_name_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkSetDebugUtilsObjectNameEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            set_debug_utils_object_name_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    set_debug_utils_object_tag_ext: unsafe {
                        unsafe extern "system" fn set_debug_utils_object_tag_ext(
                            _device: crate::vk::Device,
                            _p_tag_info: *const DebugUtilsObjectTagInfoEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(set_debug_utils_object_tag_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkSetDebugUtilsObjectTagEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            set_debug_utils_object_tag_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    queue_begin_debug_utils_label_ext: unsafe {
                        unsafe extern "system" fn queue_begin_debug_utils_label_ext(
                            _queue: Queue,
                            _p_label_info: *const DebugUtilsLabelEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(queue_begin_debug_utils_label_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkQueueBeginDebugUtilsLabelEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            queue_begin_debug_utils_label_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    queue_end_debug_utils_label_ext: unsafe {
                        unsafe extern "system" fn queue_end_debug_utils_label_ext(_queue: Queue) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(queue_end_debug_utils_label_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkQueueEndDebugUtilsLabelEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            queue_end_debug_utils_label_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    queue_insert_debug_utils_label_ext: unsafe {
                        unsafe extern "system" fn queue_insert_debug_utils_label_ext(
                            _queue: Queue,
                            _p_label_info: *const DebugUtilsLabelEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(queue_insert_debug_utils_label_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkQueueInsertDebugUtilsLabelEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            queue_insert_debug_utils_label_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_begin_debug_utils_label_ext: unsafe {
                        unsafe extern "system" fn cmd_begin_debug_utils_label_ext(
                            _command_buffer: CommandBuffer,
                            _p_label_info: *const DebugUtilsLabelEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_begin_debug_utils_label_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginDebugUtilsLabelEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_begin_debug_utils_label_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_end_debug_utils_label_ext: unsafe {
                        unsafe extern "system" fn cmd_end_debug_utils_label_ext(
                            _command_buffer: CommandBuffer,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_end_debug_utils_label_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdEndDebugUtilsLabelEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_end_debug_utils_label_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_insert_debug_utils_label_ext: unsafe {
                        unsafe extern "system" fn cmd_insert_debug_utils_label_ext(
                            _command_buffer: CommandBuffer,
                            _p_label_info: *const DebugUtilsLabelEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_insert_debug_utils_label_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdInsertDebugUtilsLabelEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_insert_debug_utils_label_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_descriptor_heap"]
    pub mod descriptor_heap {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::EXT_DESCRIPTOR_HEAP_NAME as NAME,
            crate::vk::EXT_DESCRIPTOR_HEAP_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_descriptor_heap instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn load(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle;
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_descriptor_heap instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_descriptor_size_ext: PFN_vkGetPhysicalDeviceDescriptorSizeEXT,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_descriptor_size_ext: unsafe {
                        unsafe extern "system" fn get_physical_device_descriptor_size_ext(
                            _physical_device: PhysicalDevice,
                            _descriptor_type: DescriptorType,
                        ) -> DeviceSize {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_descriptor_size_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceDescriptorSizeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_descriptor_size_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_EXT_descriptor_heap device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_descriptor_heap device-level function pointers"]
        pub struct DeviceFn {
            pub write_sampler_descriptors_ext: PFN_vkWriteSamplerDescriptorsEXT,
            pub write_resource_descriptors_ext: PFN_vkWriteResourceDescriptorsEXT,
            pub cmd_bind_sampler_heap_ext: PFN_vkCmdBindSamplerHeapEXT,
            pub cmd_bind_resource_heap_ext: PFN_vkCmdBindResourceHeapEXT,
            pub cmd_push_data_ext: PFN_vkCmdPushDataEXT,
            pub get_image_opaque_capture_data_ext: PFN_vkGetImageOpaqueCaptureDataEXT,
            pub register_custom_border_color_ext: PFN_vkRegisterCustomBorderColorEXT,
            pub unregister_custom_border_color_ext: PFN_vkUnregisterCustomBorderColorEXT,
            pub get_tensor_opaque_capture_data_arm: PFN_vkGetTensorOpaqueCaptureDataARM,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    write_sampler_descriptors_ext: unsafe {
                        unsafe extern "system" fn write_sampler_descriptors_ext(
                            _device: crate::vk::Device,
                            _sampler_count: u32,
                            _p_samplers: *const SamplerCreateInfo<'_>,
                            _p_descriptors: *const HostAddressRangeEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(write_sampler_descriptors_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkWriteSamplerDescriptorsEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            write_sampler_descriptors_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    write_resource_descriptors_ext: unsafe {
                        unsafe extern "system" fn write_resource_descriptors_ext(
                            _device: crate::vk::Device,
                            _resource_count: u32,
                            _p_resources: *const ResourceDescriptorInfoEXT<'_>,
                            _p_descriptors: *const HostAddressRangeEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(write_resource_descriptors_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkWriteResourceDescriptorsEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            write_resource_descriptors_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_bind_sampler_heap_ext: unsafe {
                        unsafe extern "system" fn cmd_bind_sampler_heap_ext(
                            _command_buffer: CommandBuffer,
                            _p_bind_info: *const BindHeapInfoEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_bind_sampler_heap_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBindSamplerHeapEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_sampler_heap_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_bind_resource_heap_ext: unsafe {
                        unsafe extern "system" fn cmd_bind_resource_heap_ext(
                            _command_buffer: CommandBuffer,
                            _p_bind_info: *const BindHeapInfoEXT<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_bind_resource_heap_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBindResourceHeapEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_resource_heap_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_push_data_ext: unsafe {
                        unsafe extern "system" fn cmd_push_data_ext(
                            _command_buffer: CommandBuffer,
                            _p_push_data_info: *const PushDataInfoEXT<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_push_data_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdPushDataEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_push_data_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_image_opaque_capture_data_ext: unsafe {
                        unsafe extern "system" fn get_image_opaque_capture_data_ext(
                            _device: crate::vk::Device,
                            _image_count: u32,
                            _p_images: *const Image,
                            _p_datas: *mut HostAddressRangeEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_image_opaque_capture_data_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetImageOpaqueCaptureDataEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_image_opaque_capture_data_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    register_custom_border_color_ext: unsafe {
                        unsafe extern "system" fn register_custom_border_color_ext(
                            _device: crate::vk::Device,
                            _p_border_color: *const SamplerCustomBorderColorCreateInfoEXT<'_>,
                            _request_index: Bool32,
                            _p_index: *mut u32,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(register_custom_border_color_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkRegisterCustomBorderColorEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            register_custom_border_color_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    unregister_custom_border_color_ext: unsafe {
                        unsafe extern "system" fn unregister_custom_border_color_ext(
                            _device: crate::vk::Device,
                            _index: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(unregister_custom_border_color_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkUnregisterCustomBorderColorEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            unregister_custom_border_color_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_tensor_opaque_capture_data_arm: unsafe {
                        unsafe extern "system" fn get_tensor_opaque_capture_data_arm(
                            _device: crate::vk::Device,
                            _tensor_count: u32,
                            _p_tensors: *const TensorARM,
                            _p_datas: *mut HostAddressRangeEXT<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_tensor_opaque_capture_data_arm)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetTensorOpaqueCaptureDataARM\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_tensor_opaque_capture_data_arm
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_descriptor_indexing"]
    pub mod descriptor_indexing {
        pub use {
            crate::vk::EXT_DESCRIPTOR_INDEXING_NAME as NAME,
            crate::vk::EXT_DESCRIPTOR_INDEXING_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_pipeline_creation_feedback"]
    pub mod pipeline_creation_feedback {
        pub use {
            crate::vk::EXT_PIPELINE_CREATION_FEEDBACK_NAME as NAME,
            crate::vk::EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_metal_surface"]
    pub mod metal_surface {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::EXT_METAL_SURFACE_NAME as NAME,
            crate::vk::EXT_METAL_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_metal_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn load(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle;
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_metal_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_metal_surface_ext: PFN_vkCreateMetalSurfaceEXT,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_metal_surface_ext: unsafe {
                        unsafe extern "system" fn create_metal_surface_ext(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const MetalSurfaceCreateInfoEXT<'_>,
                            _p_allocator: *const AllocationCallbacks,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_metal_surface_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateMetalSurfaceEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_metal_surface_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_memory_budget"]
    pub mod memory_budget {
        pub use {
            crate::vk::EXT_MEMORY_BUDGET_NAME as NAME,
            crate::vk::EXT_MEMORY_BUDGET_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_extended_dynamic_state"]
    pub mod extended_dynamic_state {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::EXT_EXTENDED_DYNAMIC_STATE_NAME as NAME,
            crate::vk::EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_extended_dynamic_state device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_extended_dynamic_state device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_cull_mode_ext: PFN_vkCmdSetCullMode,
            pub cmd_set_front_face_ext: PFN_vkCmdSetFrontFace,
            pub cmd_set_primitive_topology_ext: PFN_vkCmdSetPrimitiveTopology,
            pub cmd_set_viewport_with_count_ext: PFN_vkCmdSetViewportWithCount,
            pub cmd_set_scissor_with_count_ext: PFN_vkCmdSetScissorWithCount,
            pub cmd_bind_vertex_buffers2_ext: PFN_vkCmdBindVertexBuffers2,
            pub cmd_set_depth_test_enable_ext: PFN_vkCmdSetDepthTestEnable,
            pub cmd_set_depth_write_enable_ext: PFN_vkCmdSetDepthWriteEnable,
            pub cmd_set_depth_compare_op_ext: PFN_vkCmdSetDepthCompareOp,
            pub cmd_set_depth_bounds_test_enable_ext: PFN_vkCmdSetDepthBoundsTestEnable,
            pub cmd_set_stencil_test_enable_ext: PFN_vkCmdSetStencilTestEnable,
            pub cmd_set_stencil_op_ext: PFN_vkCmdSetStencilOp,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_cull_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_cull_mode_ext(
                            _command_buffer: CommandBuffer,
                            _cull_mode: CullModeFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_cull_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCullModeEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_cull_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_front_face_ext: unsafe {
                        unsafe extern "system" fn cmd_set_front_face_ext(
                            _command_buffer: CommandBuffer,
                            _front_face: FrontFace,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_front_face_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetFrontFaceEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_front_face_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_primitive_topology_ext: unsafe {
                        unsafe extern "system" fn cmd_set_primitive_topology_ext(
                            _command_buffer: CommandBuffer,
                            _primitive_topology: PrimitiveTopology,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_primitive_topology_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPrimitiveTopologyEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_primitive_topology_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_viewport_with_count_ext: unsafe {
                        unsafe extern "system" fn cmd_set_viewport_with_count_ext(
                            _command_buffer: CommandBuffer,
                            _viewport_count: u32,
                            _p_viewports: *const Viewport,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_viewport_with_count_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportWithCountEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_viewport_with_count_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_scissor_with_count_ext: unsafe {
                        unsafe extern "system" fn cmd_set_scissor_with_count_ext(
                            _command_buffer: CommandBuffer,
                            _scissor_count: u32,
                            _p_scissors: *const Rect2D,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_scissor_with_count_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetScissorWithCountEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_scissor_with_count_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_bind_vertex_buffers2_ext: unsafe {
                        unsafe extern "system" fn cmd_bind_vertex_buffers2_ext(
                            _command_buffer: CommandBuffer,
                            _first_binding: u32,
                            _binding_count: u32,
                            _p_buffers: *const Buffer,
                            _p_offsets: *const DeviceSize,
                            _p_sizes: *const DeviceSize,
                            _p_strides: *const DeviceSize,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_bind_vertex_buffers2_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBindVertexBuffers2EXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_vertex_buffers2_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_test_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_test_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_test_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthTestEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_test_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_write_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_write_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_write_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_write_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthWriteEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_write_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_compare_op_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_compare_op_ext(
                            _command_buffer: CommandBuffer,
                            _depth_compare_op: CompareOp,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_compare_op_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthCompareOpEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_compare_op_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_bounds_test_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_bounds_test_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_bounds_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_bounds_test_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetDepthBoundsTestEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_bounds_test_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_stencil_test_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_stencil_test_enable_ext(
                            _command_buffer: CommandBuffer,
                            _stencil_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_stencil_test_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilTestEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_stencil_test_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_stencil_op_ext: unsafe {
                        unsafe extern "system" fn cmd_set_stencil_op_ext(
                            _command_buffer: CommandBuffer,
                            _face_mask: StencilFaceFlags,
                            _fail_op: StencilOp,
                            _pass_op: StencilOp,
                            _depth_fail_op: StencilOp,
                            _compare_op: CompareOp,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_stencil_op_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilOpEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_stencil_op_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_host_image_copy"]
    pub mod host_image_copy {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::EXT_HOST_IMAGE_COPY_NAME as NAME,
            crate::vk::EXT_HOST_IMAGE_COPY_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_host_image_copy device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_host_image_copy device-level function pointers"]
        pub struct DeviceFn {
            pub copy_memory_to_image_ext: PFN_vkCopyMemoryToImage,
            pub copy_image_to_memory_ext: PFN_vkCopyImageToMemory,
            pub copy_image_to_image_ext: PFN_vkCopyImageToImage,
            pub transition_image_layout_ext: PFN_vkTransitionImageLayout,
            pub get_image_subresource_layout2_ext: PFN_vkGetImageSubresourceLayout2,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    copy_memory_to_image_ext: unsafe {
                        unsafe extern "system" fn copy_memory_to_image_ext(
                            _device: crate::vk::Device,
                            _p_copy_memory_to_image_info: *const CopyMemoryToImageInfo<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(copy_memory_to_image_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCopyMemoryToImageEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            copy_memory_to_image_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    copy_image_to_memory_ext: unsafe {
                        unsafe extern "system" fn copy_image_to_memory_ext(
                            _device: crate::vk::Device,
                            _p_copy_image_to_memory_info: *const CopyImageToMemoryInfo<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(copy_image_to_memory_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCopyImageToMemoryEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            copy_image_to_memory_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    copy_image_to_image_ext: unsafe {
                        unsafe extern "system" fn copy_image_to_image_ext(
                            _device: crate::vk::Device,
                            _p_copy_image_to_image_info: *const CopyImageToImageInfo<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(copy_image_to_image_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCopyImageToImageEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            copy_image_to_image_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    transition_image_layout_ext: unsafe {
                        unsafe extern "system" fn transition_image_layout_ext(
                            _device: crate::vk::Device,
                            _transition_count: u32,
                            _p_transitions: *const HostImageLayoutTransitionInfo<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(transition_image_layout_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkTransitionImageLayoutEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            transition_image_layout_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_image_subresource_layout2_ext: unsafe {
                        unsafe extern "system" fn get_image_subresource_layout2_ext(
                            _device: crate::vk::Device,
                            _image: Image,
                            _p_subresource: *const ImageSubresource2<'_>,
                            _p_layout: *mut SubresourceLayout2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_image_subresource_layout2_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetImageSubresourceLayout2EXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_image_subresource_layout2_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_pipeline_creation_cache_control"]
    pub mod pipeline_creation_cache_control {
        pub use {
            crate::vk::EXT_PIPELINE_CREATION_CACHE_CONTROL_NAME as NAME,
            crate::vk::EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_graphics_pipeline_library"]
    pub mod graphics_pipeline_library {
        pub use {
            crate::vk::EXT_GRAPHICS_PIPELINE_LIBRARY_NAME as NAME,
            crate::vk::EXT_GRAPHICS_PIPELINE_LIBRARY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_EXT_mesh_shader"]
    pub mod mesh_shader {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::EXT_MESH_SHADER_NAME as NAME,
            crate::vk::EXT_MESH_SHADER_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_mesh_shader device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_mesh_shader device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_draw_mesh_tasks_ext: PFN_vkCmdDrawMeshTasksEXT,
            #[doc = "Deprecated: <https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-buffer-commands>"]
            pub cmd_draw_mesh_tasks_indirect_ext: PFN_vkCmdDrawMeshTasksIndirectEXT,
            #[doc = "Deprecated: <https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-buffer-commands>"]
            pub cmd_draw_mesh_tasks_indirect_count_ext: PFN_vkCmdDrawMeshTasksIndirectCountEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_draw_mesh_tasks_ext: unsafe {
                        unsafe extern "system" fn cmd_draw_mesh_tasks_ext(
                            _command_buffer: CommandBuffer,
                            _group_count_x: u32,
                            _group_count_y: u32,
                            _group_count_z: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_draw_mesh_tasks_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMeshTasksEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_mesh_tasks_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_draw_mesh_tasks_indirect_ext: unsafe {
                        unsafe extern "system" fn cmd_draw_mesh_tasks_indirect_ext(
                            _command_buffer: CommandBuffer,
                            _buffer: Buffer,
                            _offset: DeviceSize,
                            _draw_count: u32,
                            _stride: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_draw_mesh_tasks_indirect_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMeshTasksIndirectEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_mesh_tasks_indirect_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_draw_mesh_tasks_indirect_count_ext: unsafe {
                        unsafe extern "system" fn cmd_draw_mesh_tasks_indirect_count_ext(
                            _command_buffer: CommandBuffer,
                            _buffer: Buffer,
                            _offset: DeviceSize,
                            _count_buffer: Buffer,
                            _count_buffer_offset: DeviceSize,
                            _max_draw_count: u32,
                            _stride: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_draw_mesh_tasks_indirect_count_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdDrawMeshTasksIndirectCountEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_mesh_tasks_indirect_count_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_extended_dynamic_state2"]
    pub mod extended_dynamic_state2 {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::EXT_EXTENDED_DYNAMIC_STATE2_NAME as NAME,
            crate::vk::EXT_EXTENDED_DYNAMIC_STATE2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_extended_dynamic_state2 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_extended_dynamic_state2 device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_patch_control_points_ext: PFN_vkCmdSetPatchControlPointsEXT,
            pub cmd_set_rasterizer_discard_enable_ext: PFN_vkCmdSetRasterizerDiscardEnable,
            pub cmd_set_depth_bias_enable_ext: PFN_vkCmdSetDepthBiasEnable,
            pub cmd_set_logic_op_ext: PFN_vkCmdSetLogicOpEXT,
            pub cmd_set_primitive_restart_enable_ext: PFN_vkCmdSetPrimitiveRestartEnable,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_patch_control_points_ext: unsafe {
                        unsafe extern "system" fn cmd_set_patch_control_points_ext(
                            _command_buffer: CommandBuffer,
                            _patch_control_points: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_patch_control_points_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPatchControlPointsEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_patch_control_points_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_rasterizer_discard_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_rasterizer_discard_enable_ext(
                            _command_buffer: CommandBuffer,
                            _rasterizer_discard_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rasterizer_discard_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRasterizerDiscardEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rasterizer_discard_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_bias_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_bias_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_bias_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_bias_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBiasEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_bias_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_logic_op_ext: unsafe {
                        unsafe extern "system" fn cmd_set_logic_op_ext(
                            _command_buffer: CommandBuffer,
                            _logic_op: LogicOp,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_set_logic_op_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLogicOpEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_logic_op_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_primitive_restart_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_primitive_restart_enable_ext(
                            _command_buffer: CommandBuffer,
                            _primitive_restart_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_primitive_restart_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetPrimitiveRestartEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_primitive_restart_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_multi_draw"]
    pub mod multi_draw {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::EXT_MULTI_DRAW_NAME as NAME,
            crate::vk::EXT_MULTI_DRAW_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_multi_draw device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_multi_draw device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_draw_multi_ext: PFN_vkCmdDrawMultiEXT,
            pub cmd_draw_multi_indexed_ext: PFN_vkCmdDrawMultiIndexedEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_draw_multi_ext: unsafe {
                        unsafe extern "system" fn cmd_draw_multi_ext(
                            _command_buffer: CommandBuffer,
                            _draw_count: u32,
                            _p_vertex_info: *const MultiDrawInfoEXT,
                            _instance_count: u32,
                            _first_instance: u32,
                            _stride: u32,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_draw_multi_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMultiEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_multi_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_draw_multi_indexed_ext: unsafe {
                        unsafe extern "system" fn cmd_draw_multi_indexed_ext(
                            _command_buffer: CommandBuffer,
                            _draw_count: u32,
                            _p_index_info: *const MultiDrawIndexedInfoEXT,
                            _instance_count: u32,
                            _first_instance: u32,
                            _stride: u32,
                            _p_vertex_offset: *const i32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_draw_multi_indexed_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMultiIndexedEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_multi_indexed_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_extended_dynamic_state3"]
    pub mod extended_dynamic_state3 {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::EXT_EXTENDED_DYNAMIC_STATE3_NAME as NAME,
            crate::vk::EXT_EXTENDED_DYNAMIC_STATE3_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_extended_dynamic_state3 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_extended_dynamic_state3 device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_depth_clamp_enable_ext: PFN_vkCmdSetDepthClampEnableEXT,
            pub cmd_set_polygon_mode_ext: PFN_vkCmdSetPolygonModeEXT,
            pub cmd_set_rasterization_samples_ext: PFN_vkCmdSetRasterizationSamplesEXT,
            pub cmd_set_sample_mask_ext: PFN_vkCmdSetSampleMaskEXT,
            pub cmd_set_alpha_to_coverage_enable_ext: PFN_vkCmdSetAlphaToCoverageEnableEXT,
            pub cmd_set_alpha_to_one_enable_ext: PFN_vkCmdSetAlphaToOneEnableEXT,
            pub cmd_set_logic_op_enable_ext: PFN_vkCmdSetLogicOpEnableEXT,
            pub cmd_set_color_blend_enable_ext: PFN_vkCmdSetColorBlendEnableEXT,
            pub cmd_set_color_blend_equation_ext: PFN_vkCmdSetColorBlendEquationEXT,
            pub cmd_set_color_write_mask_ext: PFN_vkCmdSetColorWriteMaskEXT,
            pub cmd_set_tessellation_domain_origin_ext: PFN_vkCmdSetTessellationDomainOriginEXT,
            pub cmd_set_rasterization_stream_ext: PFN_vkCmdSetRasterizationStreamEXT,
            pub cmd_set_conservative_rasterization_mode_ext:
                PFN_vkCmdSetConservativeRasterizationModeEXT,
            pub cmd_set_extra_primitive_overestimation_size_ext:
                PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT,
            pub cmd_set_depth_clip_enable_ext: PFN_vkCmdSetDepthClipEnableEXT,
            pub cmd_set_sample_locations_enable_ext: PFN_vkCmdSetSampleLocationsEnableEXT,
            pub cmd_set_color_blend_advanced_ext: PFN_vkCmdSetColorBlendAdvancedEXT,
            pub cmd_set_provoking_vertex_mode_ext: PFN_vkCmdSetProvokingVertexModeEXT,
            pub cmd_set_line_rasterization_mode_ext: PFN_vkCmdSetLineRasterizationModeEXT,
            pub cmd_set_line_stipple_enable_ext: PFN_vkCmdSetLineStippleEnableEXT,
            pub cmd_set_depth_clip_negative_one_to_one_ext:
                PFN_vkCmdSetDepthClipNegativeOneToOneEXT,
            pub cmd_set_viewport_w_scaling_enable_nv: PFN_vkCmdSetViewportWScalingEnableNV,
            pub cmd_set_viewport_swizzle_nv: PFN_vkCmdSetViewportSwizzleNV,
            pub cmd_set_coverage_to_color_enable_nv: PFN_vkCmdSetCoverageToColorEnableNV,
            pub cmd_set_coverage_to_color_location_nv: PFN_vkCmdSetCoverageToColorLocationNV,
            pub cmd_set_coverage_modulation_mode_nv: PFN_vkCmdSetCoverageModulationModeNV,
            pub cmd_set_coverage_modulation_table_enable_nv:
                PFN_vkCmdSetCoverageModulationTableEnableNV,
            pub cmd_set_coverage_modulation_table_nv: PFN_vkCmdSetCoverageModulationTableNV,
            pub cmd_set_shading_rate_image_enable_nv: PFN_vkCmdSetShadingRateImageEnableNV,
            pub cmd_set_representative_fragment_test_enable_nv:
                PFN_vkCmdSetRepresentativeFragmentTestEnableNV,
            pub cmd_set_coverage_reduction_mode_nv: PFN_vkCmdSetCoverageReductionModeNV,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_depth_clamp_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_clamp_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_clamp_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_clamp_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthClampEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_clamp_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_polygon_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_polygon_mode_ext(
                            _command_buffer: CommandBuffer,
                            _polygon_mode: PolygonMode,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_polygon_mode_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPolygonModeEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_polygon_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_rasterization_samples_ext: unsafe {
                        unsafe extern "system" fn cmd_set_rasterization_samples_ext(
                            _command_buffer: CommandBuffer,
                            _rasterization_samples: SampleCountFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rasterization_samples_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRasterizationSamplesEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rasterization_samples_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_sample_mask_ext: unsafe {
                        unsafe extern "system" fn cmd_set_sample_mask_ext(
                            _command_buffer: CommandBuffer,
                            _samples: SampleCountFlags,
                            _p_sample_mask: *const SampleMask,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_sample_mask_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetSampleMaskEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_sample_mask_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_alpha_to_coverage_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_alpha_to_coverage_enable_ext(
                            _command_buffer: CommandBuffer,
                            _alpha_to_coverage_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_alpha_to_coverage_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetAlphaToCoverageEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_alpha_to_coverage_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_alpha_to_one_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_alpha_to_one_enable_ext(
                            _command_buffer: CommandBuffer,
                            _alpha_to_one_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_alpha_to_one_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetAlphaToOneEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_alpha_to_one_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_logic_op_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_logic_op_enable_ext(
                            _command_buffer: CommandBuffer,
                            _logic_op_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_logic_op_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLogicOpEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_logic_op_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_blend_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_blend_enable_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_blend_enables: *const Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_blend_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorBlendEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_blend_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_blend_equation_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_blend_equation_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_blend_equations: *const ColorBlendEquationEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_blend_equation_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorBlendEquationEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_blend_equation_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_write_mask_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_write_mask_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_write_masks: *const ColorComponentFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_write_mask_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorWriteMaskEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_write_mask_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_tessellation_domain_origin_ext: unsafe {
                        unsafe extern "system" fn cmd_set_tessellation_domain_origin_ext(
                            _command_buffer: CommandBuffer,
                            _domain_origin: TessellationDomainOrigin,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_tessellation_domain_origin_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetTessellationDomainOriginEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_tessellation_domain_origin_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_rasterization_stream_ext: unsafe {
                        unsafe extern "system" fn cmd_set_rasterization_stream_ext(
                            _command_buffer: CommandBuffer,
                            _rasterization_stream: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rasterization_stream_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRasterizationStreamEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rasterization_stream_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_conservative_rasterization_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_conservative_rasterization_mode_ext(
                            _command_buffer: CommandBuffer,
                            _conservative_rasterization_mode: ConservativeRasterizationModeEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_conservative_rasterization_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetConservativeRasterizationModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_conservative_rasterization_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_extra_primitive_overestimation_size_ext: unsafe {
                        unsafe extern "system" fn cmd_set_extra_primitive_overestimation_size_ext(
                            _command_buffer: CommandBuffer,
                            _extra_primitive_overestimation_size: f32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_extra_primitive_overestimation_size_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetExtraPrimitiveOverestimationSizeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_extra_primitive_overestimation_size_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_clip_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_clip_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_clip_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_clip_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthClipEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_clip_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_sample_locations_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_sample_locations_enable_ext(
                            _command_buffer: CommandBuffer,
                            _sample_locations_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_sample_locations_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetSampleLocationsEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_sample_locations_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_blend_advanced_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_blend_advanced_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_blend_advanced: *const ColorBlendAdvancedEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_blend_advanced_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorBlendAdvancedEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_blend_advanced_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_provoking_vertex_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_provoking_vertex_mode_ext(
                            _command_buffer: CommandBuffer,
                            _provoking_vertex_mode: ProvokingVertexModeEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_provoking_vertex_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetProvokingVertexModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_provoking_vertex_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_line_rasterization_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_line_rasterization_mode_ext(
                            _command_buffer: CommandBuffer,
                            _line_rasterization_mode: LineRasterizationModeEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_line_rasterization_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetLineRasterizationModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_line_rasterization_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_line_stipple_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_line_stipple_enable_ext(
                            _command_buffer: CommandBuffer,
                            _stippled_line_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_line_stipple_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLineStippleEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_line_stipple_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_clip_negative_one_to_one_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_clip_negative_one_to_one_ext(
                            _command_buffer: CommandBuffer,
                            _negative_one_to_one: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_clip_negative_one_to_one_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetDepthClipNegativeOneToOneEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_clip_negative_one_to_one_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_viewport_w_scaling_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_viewport_w_scaling_enable_nv(
                            _command_buffer: CommandBuffer,
                            _viewport_w_scaling_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_viewport_w_scaling_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetViewportWScalingEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_viewport_w_scaling_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_viewport_swizzle_nv: unsafe {
                        unsafe extern "system" fn cmd_set_viewport_swizzle_nv(
                            _command_buffer: CommandBuffer,
                            _first_viewport: u32,
                            _viewport_count: u32,
                            _p_viewport_swizzles: *const ViewportSwizzleNV,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_viewport_swizzle_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportSwizzleNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_viewport_swizzle_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_to_color_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_to_color_enable_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_to_color_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_to_color_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageToColorEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_to_color_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_to_color_location_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_to_color_location_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_to_color_location: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_to_color_location_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageToColorLocationNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_to_color_location_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_modulation_mode_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_modulation_mode_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_modulation_mode: CoverageModulationModeNV,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_modulation_mode_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageModulationModeNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_modulation_mode_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_modulation_table_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_modulation_table_enable_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_modulation_table_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_modulation_table_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageModulationTableEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_modulation_table_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_modulation_table_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_modulation_table_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_modulation_table_count: u32,
                            _p_coverage_modulation_table: *const f32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_modulation_table_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageModulationTableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_modulation_table_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_shading_rate_image_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_shading_rate_image_enable_nv(
                            _command_buffer: CommandBuffer,
                            _shading_rate_image_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_shading_rate_image_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetShadingRateImageEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_shading_rate_image_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_representative_fragment_test_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_representative_fragment_test_enable_nv(
                            _command_buffer: CommandBuffer,
                            _representative_fragment_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_representative_fragment_test_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRepresentativeFragmentTestEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_representative_fragment_test_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_reduction_mode_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_reduction_mode_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_reduction_mode: CoverageReductionModeNV,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_reduction_mode_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageReductionModeNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_reduction_mode_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_EXT_shader_object"]
    pub mod shader_object {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::EXT_SHADER_OBJECT_NAME as NAME,
            crate::vk::EXT_SHADER_OBJECT_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_EXT_shader_object device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_EXT_shader_object device-level function pointers"]
        pub struct DeviceFn {
            pub create_shaders_ext: PFN_vkCreateShadersEXT,
            pub destroy_shader_ext: PFN_vkDestroyShaderEXT,
            pub get_shader_binary_data_ext: PFN_vkGetShaderBinaryDataEXT,
            pub cmd_bind_shaders_ext: PFN_vkCmdBindShadersEXT,
            pub cmd_set_cull_mode_ext: PFN_vkCmdSetCullMode,
            pub cmd_set_front_face_ext: PFN_vkCmdSetFrontFace,
            pub cmd_set_primitive_topology_ext: PFN_vkCmdSetPrimitiveTopology,
            pub cmd_set_viewport_with_count_ext: PFN_vkCmdSetViewportWithCount,
            pub cmd_set_scissor_with_count_ext: PFN_vkCmdSetScissorWithCount,
            pub cmd_bind_vertex_buffers2_ext: PFN_vkCmdBindVertexBuffers2,
            pub cmd_set_depth_test_enable_ext: PFN_vkCmdSetDepthTestEnable,
            pub cmd_set_depth_write_enable_ext: PFN_vkCmdSetDepthWriteEnable,
            pub cmd_set_depth_compare_op_ext: PFN_vkCmdSetDepthCompareOp,
            pub cmd_set_depth_bounds_test_enable_ext: PFN_vkCmdSetDepthBoundsTestEnable,
            pub cmd_set_stencil_test_enable_ext: PFN_vkCmdSetStencilTestEnable,
            pub cmd_set_stencil_op_ext: PFN_vkCmdSetStencilOp,
            pub cmd_set_vertex_input_ext: PFN_vkCmdSetVertexInputEXT,
            pub cmd_set_patch_control_points_ext: PFN_vkCmdSetPatchControlPointsEXT,
            pub cmd_set_rasterizer_discard_enable_ext: PFN_vkCmdSetRasterizerDiscardEnable,
            pub cmd_set_depth_bias_enable_ext: PFN_vkCmdSetDepthBiasEnable,
            pub cmd_set_logic_op_ext: PFN_vkCmdSetLogicOpEXT,
            pub cmd_set_primitive_restart_enable_ext: PFN_vkCmdSetPrimitiveRestartEnable,
            pub cmd_set_tessellation_domain_origin_ext: PFN_vkCmdSetTessellationDomainOriginEXT,
            pub cmd_set_depth_clamp_enable_ext: PFN_vkCmdSetDepthClampEnableEXT,
            pub cmd_set_polygon_mode_ext: PFN_vkCmdSetPolygonModeEXT,
            pub cmd_set_rasterization_samples_ext: PFN_vkCmdSetRasterizationSamplesEXT,
            pub cmd_set_sample_mask_ext: PFN_vkCmdSetSampleMaskEXT,
            pub cmd_set_alpha_to_coverage_enable_ext: PFN_vkCmdSetAlphaToCoverageEnableEXT,
            pub cmd_set_alpha_to_one_enable_ext: PFN_vkCmdSetAlphaToOneEnableEXT,
            pub cmd_set_logic_op_enable_ext: PFN_vkCmdSetLogicOpEnableEXT,
            pub cmd_set_color_blend_enable_ext: PFN_vkCmdSetColorBlendEnableEXT,
            pub cmd_set_color_blend_equation_ext: PFN_vkCmdSetColorBlendEquationEXT,
            pub cmd_set_color_write_mask_ext: PFN_vkCmdSetColorWriteMaskEXT,
            pub cmd_set_rasterization_stream_ext: PFN_vkCmdSetRasterizationStreamEXT,
            pub cmd_set_conservative_rasterization_mode_ext:
                PFN_vkCmdSetConservativeRasterizationModeEXT,
            pub cmd_set_extra_primitive_overestimation_size_ext:
                PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT,
            pub cmd_set_depth_clip_enable_ext: PFN_vkCmdSetDepthClipEnableEXT,
            pub cmd_set_sample_locations_enable_ext: PFN_vkCmdSetSampleLocationsEnableEXT,
            pub cmd_set_color_blend_advanced_ext: PFN_vkCmdSetColorBlendAdvancedEXT,
            pub cmd_set_provoking_vertex_mode_ext: PFN_vkCmdSetProvokingVertexModeEXT,
            pub cmd_set_line_rasterization_mode_ext: PFN_vkCmdSetLineRasterizationModeEXT,
            pub cmd_set_line_stipple_enable_ext: PFN_vkCmdSetLineStippleEnableEXT,
            pub cmd_set_depth_clip_negative_one_to_one_ext:
                PFN_vkCmdSetDepthClipNegativeOneToOneEXT,
            pub cmd_set_viewport_w_scaling_enable_nv: PFN_vkCmdSetViewportWScalingEnableNV,
            pub cmd_set_viewport_swizzle_nv: PFN_vkCmdSetViewportSwizzleNV,
            pub cmd_set_coverage_to_color_enable_nv: PFN_vkCmdSetCoverageToColorEnableNV,
            pub cmd_set_coverage_to_color_location_nv: PFN_vkCmdSetCoverageToColorLocationNV,
            pub cmd_set_coverage_modulation_mode_nv: PFN_vkCmdSetCoverageModulationModeNV,
            pub cmd_set_coverage_modulation_table_enable_nv:
                PFN_vkCmdSetCoverageModulationTableEnableNV,
            pub cmd_set_coverage_modulation_table_nv: PFN_vkCmdSetCoverageModulationTableNV,
            pub cmd_set_shading_rate_image_enable_nv: PFN_vkCmdSetShadingRateImageEnableNV,
            pub cmd_set_representative_fragment_test_enable_nv:
                PFN_vkCmdSetRepresentativeFragmentTestEnableNV,
            pub cmd_set_coverage_reduction_mode_nv: PFN_vkCmdSetCoverageReductionModeNV,
            pub cmd_set_depth_clamp_range_ext: PFN_vkCmdSetDepthClampRangeEXT,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_shaders_ext: unsafe {
                        unsafe extern "system" fn create_shaders_ext(
                            _device: crate::vk::Device,
                            _create_info_count: u32,
                            _p_create_infos: *const ShaderCreateInfoEXT<'_>,
                            _p_allocator: *const AllocationCallbacks,
                            _p_shaders: *mut ShaderEXT,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(create_shaders_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateShadersEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_shaders_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_shader_ext: unsafe {
                        unsafe extern "system" fn destroy_shader_ext(
                            _device: crate::vk::Device,
                            _shader: ShaderEXT,
                            _p_allocator: *const AllocationCallbacks,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(destroy_shader_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroyShaderEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_shader_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_shader_binary_data_ext: unsafe {
                        unsafe extern "system" fn get_shader_binary_data_ext(
                            _device: crate::vk::Device,
                            _shader: ShaderEXT,
                            _p_data_size: *mut usize,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_shader_binary_data_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetShaderBinaryDataEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_shader_binary_data_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_bind_shaders_ext: unsafe {
                        unsafe extern "system" fn cmd_bind_shaders_ext(
                            _command_buffer: CommandBuffer,
                            _stage_count: u32,
                            _p_stages: *const ShaderStageFlags,
                            _p_shaders: *const ShaderEXT,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_bind_shaders_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBindShadersEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_shaders_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_cull_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_cull_mode_ext(
                            _command_buffer: CommandBuffer,
                            _cull_mode: CullModeFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_cull_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCullModeEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_cull_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_front_face_ext: unsafe {
                        unsafe extern "system" fn cmd_set_front_face_ext(
                            _command_buffer: CommandBuffer,
                            _front_face: FrontFace,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_front_face_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetFrontFaceEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_front_face_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_primitive_topology_ext: unsafe {
                        unsafe extern "system" fn cmd_set_primitive_topology_ext(
                            _command_buffer: CommandBuffer,
                            _primitive_topology: PrimitiveTopology,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_primitive_topology_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPrimitiveTopologyEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_primitive_topology_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_viewport_with_count_ext: unsafe {
                        unsafe extern "system" fn cmd_set_viewport_with_count_ext(
                            _command_buffer: CommandBuffer,
                            _viewport_count: u32,
                            _p_viewports: *const Viewport,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_viewport_with_count_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportWithCountEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_viewport_with_count_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_scissor_with_count_ext: unsafe {
                        unsafe extern "system" fn cmd_set_scissor_with_count_ext(
                            _command_buffer: CommandBuffer,
                            _scissor_count: u32,
                            _p_scissors: *const Rect2D,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_scissor_with_count_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetScissorWithCountEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_scissor_with_count_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_bind_vertex_buffers2_ext: unsafe {
                        unsafe extern "system" fn cmd_bind_vertex_buffers2_ext(
                            _command_buffer: CommandBuffer,
                            _first_binding: u32,
                            _binding_count: u32,
                            _p_buffers: *const Buffer,
                            _p_offsets: *const DeviceSize,
                            _p_sizes: *const DeviceSize,
                            _p_strides: *const DeviceSize,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_bind_vertex_buffers2_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBindVertexBuffers2EXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_vertex_buffers2_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_test_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_test_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_test_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthTestEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_test_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_write_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_write_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_write_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_write_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthWriteEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_write_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_compare_op_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_compare_op_ext(
                            _command_buffer: CommandBuffer,
                            _depth_compare_op: CompareOp,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_compare_op_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthCompareOpEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_compare_op_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_bounds_test_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_bounds_test_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_bounds_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_bounds_test_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetDepthBoundsTestEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_bounds_test_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_stencil_test_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_stencil_test_enable_ext(
                            _command_buffer: CommandBuffer,
                            _stencil_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_stencil_test_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilTestEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_stencil_test_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_stencil_op_ext: unsafe {
                        unsafe extern "system" fn cmd_set_stencil_op_ext(
                            _command_buffer: CommandBuffer,
                            _face_mask: StencilFaceFlags,
                            _fail_op: StencilOp,
                            _pass_op: StencilOp,
                            _depth_fail_op: StencilOp,
                            _compare_op: CompareOp,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_stencil_op_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilOpEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_stencil_op_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_vertex_input_ext: unsafe {
                        unsafe extern "system" fn cmd_set_vertex_input_ext(
                            _command_buffer: CommandBuffer,
                            _vertex_binding_description_count: u32,
                            _p_vertex_binding_descriptions : * const VertexInputBindingDescription2EXT < '_ >,
                            _vertex_attribute_description_count: u32,
                            _p_vertex_attribute_descriptions : * const VertexInputAttributeDescription2EXT < '_ >,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_vertex_input_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetVertexInputEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_vertex_input_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_patch_control_points_ext: unsafe {
                        unsafe extern "system" fn cmd_set_patch_control_points_ext(
                            _command_buffer: CommandBuffer,
                            _patch_control_points: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_patch_control_points_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPatchControlPointsEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_patch_control_points_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_rasterizer_discard_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_rasterizer_discard_enable_ext(
                            _command_buffer: CommandBuffer,
                            _rasterizer_discard_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rasterizer_discard_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRasterizerDiscardEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rasterizer_discard_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_bias_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_bias_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_bias_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_bias_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBiasEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_bias_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_logic_op_ext: unsafe {
                        unsafe extern "system" fn cmd_set_logic_op_ext(
                            _command_buffer: CommandBuffer,
                            _logic_op: LogicOp,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_set_logic_op_ext)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLogicOpEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_logic_op_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_primitive_restart_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_primitive_restart_enable_ext(
                            _command_buffer: CommandBuffer,
                            _primitive_restart_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_primitive_restart_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetPrimitiveRestartEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_primitive_restart_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_tessellation_domain_origin_ext: unsafe {
                        unsafe extern "system" fn cmd_set_tessellation_domain_origin_ext(
                            _command_buffer: CommandBuffer,
                            _domain_origin: TessellationDomainOrigin,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_tessellation_domain_origin_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetTessellationDomainOriginEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_tessellation_domain_origin_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_clamp_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_clamp_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_clamp_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_clamp_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthClampEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_clamp_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_polygon_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_polygon_mode_ext(
                            _command_buffer: CommandBuffer,
                            _polygon_mode: PolygonMode,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_polygon_mode_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPolygonModeEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_polygon_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_rasterization_samples_ext: unsafe {
                        unsafe extern "system" fn cmd_set_rasterization_samples_ext(
                            _command_buffer: CommandBuffer,
                            _rasterization_samples: SampleCountFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rasterization_samples_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRasterizationSamplesEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rasterization_samples_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_sample_mask_ext: unsafe {
                        unsafe extern "system" fn cmd_set_sample_mask_ext(
                            _command_buffer: CommandBuffer,
                            _samples: SampleCountFlags,
                            _p_sample_mask: *const SampleMask,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_sample_mask_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetSampleMaskEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_sample_mask_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_alpha_to_coverage_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_alpha_to_coverage_enable_ext(
                            _command_buffer: CommandBuffer,
                            _alpha_to_coverage_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_alpha_to_coverage_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetAlphaToCoverageEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_alpha_to_coverage_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_alpha_to_one_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_alpha_to_one_enable_ext(
                            _command_buffer: CommandBuffer,
                            _alpha_to_one_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_alpha_to_one_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetAlphaToOneEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_alpha_to_one_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_logic_op_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_logic_op_enable_ext(
                            _command_buffer: CommandBuffer,
                            _logic_op_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_logic_op_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLogicOpEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_logic_op_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_blend_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_blend_enable_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_blend_enables: *const Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_blend_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorBlendEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_blend_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_blend_equation_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_blend_equation_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_blend_equations: *const ColorBlendEquationEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_blend_equation_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorBlendEquationEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_blend_equation_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_write_mask_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_write_mask_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_write_masks: *const ColorComponentFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_write_mask_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorWriteMaskEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_write_mask_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_rasterization_stream_ext: unsafe {
                        unsafe extern "system" fn cmd_set_rasterization_stream_ext(
                            _command_buffer: CommandBuffer,
                            _rasterization_stream: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rasterization_stream_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRasterizationStreamEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rasterization_stream_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_conservative_rasterization_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_conservative_rasterization_mode_ext(
                            _command_buffer: CommandBuffer,
                            _conservative_rasterization_mode: ConservativeRasterizationModeEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_conservative_rasterization_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetConservativeRasterizationModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_conservative_rasterization_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_extra_primitive_overestimation_size_ext: unsafe {
                        unsafe extern "system" fn cmd_set_extra_primitive_overestimation_size_ext(
                            _command_buffer: CommandBuffer,
                            _extra_primitive_overestimation_size: f32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_extra_primitive_overestimation_size_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetExtraPrimitiveOverestimationSizeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_extra_primitive_overestimation_size_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_clip_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_clip_enable_ext(
                            _command_buffer: CommandBuffer,
                            _depth_clip_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_clip_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthClipEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_clip_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_sample_locations_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_sample_locations_enable_ext(
                            _command_buffer: CommandBuffer,
                            _sample_locations_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_sample_locations_enable_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetSampleLocationsEnableEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_sample_locations_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_color_blend_advanced_ext: unsafe {
                        unsafe extern "system" fn cmd_set_color_blend_advanced_ext(
                            _command_buffer: CommandBuffer,
                            _first_attachment: u32,
                            _attachment_count: u32,
                            _p_color_blend_advanced: *const ColorBlendAdvancedEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_color_blend_advanced_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorBlendAdvancedEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_color_blend_advanced_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_provoking_vertex_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_provoking_vertex_mode_ext(
                            _command_buffer: CommandBuffer,
                            _provoking_vertex_mode: ProvokingVertexModeEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_provoking_vertex_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetProvokingVertexModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_provoking_vertex_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_line_rasterization_mode_ext: unsafe {
                        unsafe extern "system" fn cmd_set_line_rasterization_mode_ext(
                            _command_buffer: CommandBuffer,
                            _line_rasterization_mode: LineRasterizationModeEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_line_rasterization_mode_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetLineRasterizationModeEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_line_rasterization_mode_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_line_stipple_enable_ext: unsafe {
                        unsafe extern "system" fn cmd_set_line_stipple_enable_ext(
                            _command_buffer: CommandBuffer,
                            _stippled_line_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_line_stipple_enable_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLineStippleEnableEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_line_stipple_enable_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_clip_negative_one_to_one_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_clip_negative_one_to_one_ext(
                            _command_buffer: CommandBuffer,
                            _negative_one_to_one: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_clip_negative_one_to_one_ext)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetDepthClipNegativeOneToOneEXT\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_clip_negative_one_to_one_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_viewport_w_scaling_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_viewport_w_scaling_enable_nv(
                            _command_buffer: CommandBuffer,
                            _viewport_w_scaling_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_viewport_w_scaling_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetViewportWScalingEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_viewport_w_scaling_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_viewport_swizzle_nv: unsafe {
                        unsafe extern "system" fn cmd_set_viewport_swizzle_nv(
                            _command_buffer: CommandBuffer,
                            _first_viewport: u32,
                            _viewport_count: u32,
                            _p_viewport_swizzles: *const ViewportSwizzleNV,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_viewport_swizzle_nv)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportSwizzleNV\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_viewport_swizzle_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_to_color_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_to_color_enable_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_to_color_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_to_color_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageToColorEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_to_color_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_to_color_location_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_to_color_location_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_to_color_location: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_to_color_location_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageToColorLocationNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_to_color_location_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_modulation_mode_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_modulation_mode_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_modulation_mode: CoverageModulationModeNV,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_modulation_mode_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageModulationModeNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_modulation_mode_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_modulation_table_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_modulation_table_enable_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_modulation_table_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_modulation_table_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageModulationTableEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_modulation_table_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_modulation_table_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_modulation_table_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_modulation_table_count: u32,
                            _p_coverage_modulation_table: *const f32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_modulation_table_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageModulationTableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_modulation_table_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_shading_rate_image_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_shading_rate_image_enable_nv(
                            _command_buffer: CommandBuffer,
                            _shading_rate_image_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_shading_rate_image_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetShadingRateImageEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_shading_rate_image_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_representative_fragment_test_enable_nv: unsafe {
                        unsafe extern "system" fn cmd_set_representative_fragment_test_enable_nv(
                            _command_buffer: CommandBuffer,
                            _representative_fragment_test_enable: Bool32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_representative_fragment_test_enable_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRepresentativeFragmentTestEnableNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_representative_fragment_test_enable_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_coverage_reduction_mode_nv: unsafe {
                        unsafe extern "system" fn cmd_set_coverage_reduction_mode_nv(
                            _command_buffer: CommandBuffer,
                            _coverage_reduction_mode: CoverageReductionModeNV,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_coverage_reduction_mode_nv)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetCoverageReductionModeNV\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_coverage_reduction_mode_nv
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_depth_clamp_range_ext: unsafe {
                        unsafe extern "system" fn cmd_set_depth_clamp_range_ext(
                            _command_buffer: CommandBuffer,
                            _depth_clamp_mode: DepthClampModeEXT,
                            _p_depth_clamp_range: *const DepthClampRangeEXT,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_depth_clamp_range_ext)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthClampRangeEXT\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_depth_clamp_range_ext
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
}
#[doc = "Extensions tagged KHR"]
pub mod khr {
    #[doc = "VK_KHR_surface"]
    pub mod surface {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_SURFACE_NAME as NAME,
            crate::vk::KHR_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn load(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle;
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub destroy_surface_khr: PFN_vkDestroySurfaceKHR,
            pub get_physical_device_surface_support_khr: PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
            pub get_physical_device_surface_capabilities_khr:
                PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
            pub get_physical_device_surface_formats_khr: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
            pub get_physical_device_surface_present_modes_khr:
                PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    destroy_surface_khr: unsafe {
                        unsafe extern "system" fn destroy_surface_khr(
                            _instance: crate::vk::Instance,
                            _surface: SurfaceKHR,
                            _p_allocator: *const AllocationCallbacks,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(destroy_surface_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroySurfaceKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_surface_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_surface_support_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_surface_support_khr(
                            _physical_device: PhysicalDevice,
                            _queue_family_index: u32,
                            _surface: SurfaceKHR,
                            _p_supported: *mut Bool32,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_surface_support_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSurfaceSupportKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_surface_support_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_surface_capabilities_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_surface_capabilities_khr(
                            _physical_device: PhysicalDevice,
                            _surface: SurfaceKHR,
                            _p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_surface_capabilities_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_surface_capabilities_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_surface_formats_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_surface_formats_khr(
                            _physical_device: PhysicalDevice,
                            _surface: SurfaceKHR,
                            _p_surface_format_count: *mut u32,
                            _p_surface_formats: *mut SurfaceFormatKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_surface_formats_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSurfaceFormatsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_surface_formats_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_surface_present_modes_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_surface_present_modes_khr(
                            _physical_device: PhysicalDevice,
                            _surface: SurfaceKHR,
                            _p_present_mode_count: *mut u32,
                            _p_present_modes: *mut PresentModeKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_surface_present_modes_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSurfacePresentModesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_surface_present_modes_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_swapchain"]
    pub mod swapchain {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_SWAPCHAIN_NAME as NAME,
            crate::vk::KHR_SWAPCHAIN_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_swapchain instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn load(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle;
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_swapchain instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_present_rectangles_khr:
                PFN_vkGetPhysicalDevicePresentRectanglesKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_present_rectangles_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_present_rectangles_khr(
                            _physical_device: PhysicalDevice,
                            _surface: SurfaceKHR,
                            _p_rect_count: *mut u32,
                            _p_rects: *mut Rect2D,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_present_rectangles_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDevicePresentRectanglesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_present_rectangles_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
        #[doc = "VK_KHR_swapchain device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_swapchain device-level function pointers"]
        pub struct DeviceFn {
            pub create_swapchain_khr: PFN_vkCreateSwapchainKHR,
            pub destroy_swapchain_khr: PFN_vkDestroySwapchainKHR,
            pub get_swapchain_images_khr: PFN_vkGetSwapchainImagesKHR,
            pub acquire_next_image_khr: PFN_vkAcquireNextImageKHR,
            pub queue_present_khr: PFN_vkQueuePresentKHR,
            pub get_device_group_present_capabilities_khr:
                PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
            pub get_device_group_surface_present_modes_khr:
                PFN_vkGetDeviceGroupSurfacePresentModesKHR,
            pub acquire_next_image2_khr: PFN_vkAcquireNextImage2KHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_swapchain_khr: unsafe {
                        unsafe extern "system" fn create_swapchain_khr(
                            _device: crate::vk::Device,
                            _p_create_info: *const SwapchainCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks,
                            _p_swapchain: *mut SwapchainKHR,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(create_swapchain_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateSwapchainKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_swapchain_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_swapchain_khr: unsafe {
                        unsafe extern "system" fn destroy_swapchain_khr(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                            _p_allocator: *const AllocationCallbacks,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_swapchain_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkDestroySwapchainKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_swapchain_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_swapchain_images_khr: unsafe {
                        unsafe extern "system" fn get_swapchain_images_khr(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                            _p_swapchain_image_count: *mut u32,
                            _p_swapchain_images: *mut Image,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_swapchain_images_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainImagesKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_swapchain_images_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    acquire_next_image_khr: unsafe {
                        unsafe extern "system" fn acquire_next_image_khr(
                            _device: crate::vk::Device,
                            _swapchain: SwapchainKHR,
                            _timeout: u64,
                            _semaphore: Semaphore,
                            _fence: Fence,
                            _p_image_index: *mut u32,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(acquire_next_image_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImageKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            acquire_next_image_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    queue_present_khr: unsafe {
                        unsafe extern "system" fn queue_present_khr(
                            _queue: Queue,
                            _p_present_info: *const PresentInfoKHR<'_>,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(queue_present_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkQueuePresentKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            queue_present_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_group_present_capabilities_khr: unsafe {
                        unsafe extern "system" fn get_device_group_present_capabilities_khr(
                            _device: crate::vk::Device,
                            _p_device_group_present_capabilities : * mut DeviceGroupPresentCapabilitiesKHR < '_ >,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_group_present_capabilities_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceGroupPresentCapabilitiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_group_present_capabilities_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_group_surface_present_modes_khr: unsafe {
                        unsafe extern "system" fn get_device_group_surface_present_modes_khr(
                            _device: crate::vk::Device,
                            _surface: SurfaceKHR,
                            _p_modes: *mut DeviceGroupPresentModeFlagsKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_group_surface_present_modes_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceGroupSurfacePresentModesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_group_surface_present_modes_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    acquire_next_image2_khr: unsafe {
                        unsafe extern "system" fn acquire_next_image2_khr(
                            _device: crate::vk::Device,
                            _p_acquire_info: *const AcquireNextImageInfoKHR<'_>,
                            _p_image_index: *mut u32,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(acquire_next_image2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImage2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            acquire_next_image2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_display"]
    pub mod display {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_DISPLAY_NAME as NAME,
            crate::vk::KHR_DISPLAY_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_display instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn load(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle;
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_display instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_display_properties_khr:
                PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,
            pub get_physical_device_display_plane_properties_khr:
                PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
            pub get_display_plane_supported_displays_khr: PFN_vkGetDisplayPlaneSupportedDisplaysKHR,
            pub get_display_mode_properties_khr: PFN_vkGetDisplayModePropertiesKHR,
            pub create_display_mode_khr: PFN_vkCreateDisplayModeKHR,
            pub get_display_plane_capabilities_khr: PFN_vkGetDisplayPlaneCapabilitiesKHR,
            pub create_display_plane_surface_khr: PFN_vkCreateDisplayPlaneSurfaceKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_display_properties_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_display_properties_khr(
                            _physical_device: PhysicalDevice,
                            _p_property_count: *mut u32,
                            _p_properties: *mut DisplayPropertiesKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_display_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceDisplayPropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_display_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_display_plane_properties_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_display_plane_properties_khr(
                            _physical_device: PhysicalDevice,
                            _p_property_count: *mut u32,
                            _p_properties: *mut DisplayPlanePropertiesKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_display_plane_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceDisplayPlanePropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_display_plane_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_display_plane_supported_displays_khr: unsafe {
                        unsafe extern "system" fn get_display_plane_supported_displays_khr(
                            _physical_device: PhysicalDevice,
                            _plane_index: u32,
                            _p_display_count: *mut u32,
                            _p_displays: *mut DisplayKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_display_plane_supported_displays_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDisplayPlaneSupportedDisplaysKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_display_plane_supported_displays_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_display_mode_properties_khr: unsafe {
                        unsafe extern "system" fn get_display_mode_properties_khr(
                            _physical_device: PhysicalDevice,
                            _display: DisplayKHR,
                            _p_property_count: *mut u32,
                            _p_properties: *mut DisplayModePropertiesKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_display_mode_properties_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetDisplayModePropertiesKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_display_mode_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    create_display_mode_khr: unsafe {
                        unsafe extern "system" fn create_display_mode_khr(
                            _physical_device: PhysicalDevice,
                            _display: DisplayKHR,
                            _p_create_info: *const DisplayModeCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks,
                            _p_mode: *mut DisplayModeKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_display_mode_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateDisplayModeKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_display_mode_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_display_plane_capabilities_khr: unsafe {
                        unsafe extern "system" fn get_display_plane_capabilities_khr(
                            _physical_device: PhysicalDevice,
                            _mode: DisplayModeKHR,
                            _plane_index: u32,
                            _p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_display_plane_capabilities_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDisplayPlaneCapabilitiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_display_plane_capabilities_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    create_display_plane_surface_khr: unsafe {
                        unsafe extern "system" fn create_display_plane_surface_khr(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const DisplaySurfaceCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_display_plane_surface_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateDisplayPlaneSurfaceKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_display_plane_surface_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_xlib_surface"]
    pub mod xlib_surface {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_XLIB_SURFACE_NAME as NAME,
            crate::vk::KHR_XLIB_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_xlib_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn load(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle;
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_xlib_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_xlib_surface_khr: PFN_vkCreateXlibSurfaceKHR,
            pub get_physical_device_xlib_presentation_support_khr:
                PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_xlib_surface_khr: unsafe {
                        unsafe extern "system" fn create_xlib_surface_khr(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const XlibSurfaceCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_xlib_surface_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateXlibSurfaceKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_xlib_surface_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_xlib_presentation_support_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_xlib_presentation_support_khr(
                            _physical_device: PhysicalDevice,
                            _queue_family_index: u32,
                            _dpy: *mut Display,
                            _visual_id: VisualID,
                        ) -> Bool32 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_xlib_presentation_support_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceXlibPresentationSupportKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_xlib_presentation_support_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_xcb_surface"]
    pub mod xcb_surface {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_XCB_SURFACE_NAME as NAME,
            crate::vk::KHR_XCB_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_xcb_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn load(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle;
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_xcb_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_xcb_surface_khr: PFN_vkCreateXcbSurfaceKHR,
            pub get_physical_device_xcb_presentation_support_khr:
                PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_xcb_surface_khr: unsafe {
                        unsafe extern "system" fn create_xcb_surface_khr(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const XcbSurfaceCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_xcb_surface_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCreateXcbSurfaceKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_xcb_surface_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_xcb_presentation_support_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_xcb_presentation_support_khr(
                            _physical_device: PhysicalDevice,
                            _queue_family_index: u32,
                            _connection: *mut xcb_connection_t,
                            _visual_id: xcb_visualid_t,
                        ) -> Bool32 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_xcb_presentation_support_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceXcbPresentationSupportKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_xcb_presentation_support_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_wayland_surface"]
    pub mod wayland_surface {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_WAYLAND_SURFACE_NAME as NAME,
            crate::vk::KHR_WAYLAND_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_wayland_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn load(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle;
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_wayland_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_wayland_surface_khr: PFN_vkCreateWaylandSurfaceKHR,
            pub get_physical_device_wayland_presentation_support_khr:
                PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_wayland_surface_khr: unsafe {
                        unsafe extern "system" fn create_wayland_surface_khr(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const WaylandSurfaceCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_wayland_surface_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateWaylandSurfaceKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_wayland_surface_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_wayland_presentation_support_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_wayland_presentation_support_khr(
                            _physical_device: PhysicalDevice,
                            _queue_family_index: u32,
                            _display: *mut wl_display,
                        ) -> Bool32 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_wayland_presentation_support_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceWaylandPresentationSupportKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_wayland_presentation_support_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_android_surface"]
    pub mod android_surface {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_ANDROID_SURFACE_NAME as NAME,
            crate::vk::KHR_ANDROID_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_android_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn load(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle;
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_android_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_android_surface_khr: PFN_vkCreateAndroidSurfaceKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_android_surface_khr: unsafe {
                        unsafe extern "system" fn create_android_surface_khr(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const AndroidSurfaceCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_android_surface_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateAndroidSurfaceKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_android_surface_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_win32_surface"]
    pub mod win32_surface {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_WIN32_SURFACE_NAME as NAME,
            crate::vk::KHR_WIN32_SURFACE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_win32_surface instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn load(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle;
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_win32_surface instance-level function pointers"]
        pub struct InstanceFn {
            pub create_win32_surface_khr: PFN_vkCreateWin32SurfaceKHR,
            pub get_physical_device_win32_presentation_support_khr:
                PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_win32_surface_khr: unsafe {
                        unsafe extern "system" fn create_win32_surface_khr(
                            _instance: crate::vk::Instance,
                            _p_create_info: *const Win32SurfaceCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks,
                            _p_surface: *mut SurfaceKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_win32_surface_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateWin32SurfaceKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_win32_surface_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_win32_presentation_support_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_win32_presentation_support_khr(
                            _physical_device: PhysicalDevice,
                            _queue_family_index: u32,
                        ) -> Bool32 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_win32_presentation_support_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceWin32PresentationSupportKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_win32_presentation_support_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_dynamic_rendering"]
    pub mod dynamic_rendering {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_DYNAMIC_RENDERING_NAME as NAME,
            crate::vk::KHR_DYNAMIC_RENDERING_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_dynamic_rendering device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_dynamic_rendering device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_begin_rendering_khr: PFN_vkCmdBeginRendering,
            pub cmd_end_rendering_khr: PFN_vkCmdEndRendering,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_begin_rendering_khr: unsafe {
                        unsafe extern "system" fn cmd_begin_rendering_khr(
                            _command_buffer: CommandBuffer,
                            _p_rendering_info: *const RenderingInfo<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_begin_rendering_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderingKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_begin_rendering_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_end_rendering_khr: unsafe {
                        unsafe extern "system" fn cmd_end_rendering_khr(
                            _command_buffer: CommandBuffer,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_end_rendering_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderingKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_end_rendering_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_get_physical_device_properties2"]
    pub mod get_physical_device_properties2 {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_NAME as NAME,
            crate::vk::KHR_GET_PHYSICAL_DEVICE_PROPERTIES2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_get_physical_device_properties2 instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn load(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle;
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_get_physical_device_properties2 instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_features2_khr: PFN_vkGetPhysicalDeviceFeatures2,
            pub get_physical_device_properties2_khr: PFN_vkGetPhysicalDeviceProperties2,
            pub get_physical_device_format_properties2_khr:
                PFN_vkGetPhysicalDeviceFormatProperties2,
            pub get_physical_device_image_format_properties2_khr:
                PFN_vkGetPhysicalDeviceImageFormatProperties2,
            pub get_physical_device_queue_family_properties2_khr:
                PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
            pub get_physical_device_memory_properties2_khr:
                PFN_vkGetPhysicalDeviceMemoryProperties2,
            pub get_physical_device_sparse_image_format_properties2_khr:
                PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_features2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_features2_khr(
                            _physical_device: PhysicalDevice,
                            _p_features: *mut PhysicalDeviceFeatures2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_features2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceFeatures2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_features2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_properties2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _p_properties: *mut PhysicalDeviceProperties2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_format_properties2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_format_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _format: Format,
                            _p_format_properties: *mut FormatProperties2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_format_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceFormatProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_format_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_image_format_properties2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_image_format_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _p_image_format_info: *const PhysicalDeviceImageFormatInfo2<'_>,
                            _p_image_format_properties: *mut ImageFormatProperties2<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_image_format_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceImageFormatProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_image_format_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_queue_family_properties2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_queue_family_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _p_queue_family_property_count: *mut u32,
                            _p_queue_family_properties: *mut QueueFamilyProperties2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_queue_family_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceQueueFamilyProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_queue_family_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_memory_properties2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_memory_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _p_memory_properties: *mut PhysicalDeviceMemoryProperties2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_memory_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceMemoryProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_memory_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_sparse_image_format_properties2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_sparse_image_format_properties2_khr(
                            _physical_device: PhysicalDevice,
                            _p_format_info: *const PhysicalDeviceSparseImageFormatInfo2<'_>,
                            _p_property_count: *mut u32,
                            _p_properties: *mut SparseImageFormatProperties2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_sparse_image_format_properties2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSparseImageFormatProperties2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_sparse_image_format_properties2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_maintenance1"]
    pub mod maintenance1 {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_MAINTENANCE1_NAME as NAME,
            crate::vk::KHR_MAINTENANCE1_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_maintenance1 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_maintenance1 device-level function pointers"]
        pub struct DeviceFn {
            pub trim_command_pool_khr: PFN_vkTrimCommandPool,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    trim_command_pool_khr: unsafe {
                        unsafe extern "system" fn trim_command_pool_khr(
                            _device: crate::vk::Device,
                            _command_pool: CommandPool,
                            _flags: CommandPoolTrimFlags,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(trim_command_pool_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkTrimCommandPoolKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            trim_command_pool_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_maintenance2"]
    pub mod maintenance2 {
        pub use {
            crate::vk::KHR_MAINTENANCE2_NAME as NAME,
            crate::vk::KHR_MAINTENANCE2_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_get_surface_capabilities2"]
    pub mod get_surface_capabilities2 {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_GET_SURFACE_CAPABILITIES2_NAME as NAME,
            crate::vk::KHR_GET_SURFACE_CAPABILITIES2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_get_surface_capabilities2 instance-level functions"]
        #[derive(Clone)]
        pub struct Instance {
            pub(crate) fp: InstanceFn,
            pub(crate) handle: crate::vk::Instance,
        }
        impl Instance {
            pub fn load(entry: &crate::Entry, instance: &crate::Instance) -> Self {
                let handle = instance.handle;
                let fp = InstanceFn::load(|name| unsafe {
                    core::mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &InstanceFn {
                &self.fp
            }
            #[inline]
            pub fn instance(&self) -> crate::vk::Instance {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_get_surface_capabilities2 instance-level function pointers"]
        pub struct InstanceFn {
            pub get_physical_device_surface_capabilities2_khr:
                PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR,
            pub get_physical_device_surface_formats2_khr: PFN_vkGetPhysicalDeviceSurfaceFormats2KHR,
        }
        unsafe impl Send for InstanceFn {}
        unsafe impl Sync for InstanceFn {}
        impl InstanceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_physical_device_surface_capabilities2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_surface_capabilities2_khr(
                            _physical_device: PhysicalDevice,
                            _p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
                            _p_surface_capabilities: *mut SurfaceCapabilities2KHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_surface_capabilities2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSurfaceCapabilities2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_surface_capabilities2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_physical_device_surface_formats2_khr: unsafe {
                        unsafe extern "system" fn get_physical_device_surface_formats2_khr(
                            _physical_device: PhysicalDevice,
                            _p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
                            _p_surface_format_count: *mut u32,
                            _p_surface_formats: *mut SurfaceFormat2KHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_physical_device_surface_formats2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetPhysicalDeviceSurfaceFormats2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_physical_device_surface_formats2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_acceleration_structure"]
    pub mod acceleration_structure {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_ACCELERATION_STRUCTURE_NAME as NAME,
            crate::vk::KHR_ACCELERATION_STRUCTURE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_acceleration_structure device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_acceleration_structure device-level function pointers"]
        pub struct DeviceFn {
            #[doc = "Deprecated: <https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-buffer-commands>"]
            pub create_acceleration_structure_khr: PFN_vkCreateAccelerationStructureKHR,
            pub destroy_acceleration_structure_khr: PFN_vkDestroyAccelerationStructureKHR,
            pub cmd_build_acceleration_structures_khr: PFN_vkCmdBuildAccelerationStructuresKHR,
            pub cmd_build_acceleration_structures_indirect_khr:
                PFN_vkCmdBuildAccelerationStructuresIndirectKHR,
            pub build_acceleration_structures_khr: PFN_vkBuildAccelerationStructuresKHR,
            pub copy_acceleration_structure_khr: PFN_vkCopyAccelerationStructureKHR,
            pub copy_acceleration_structure_to_memory_khr:
                PFN_vkCopyAccelerationStructureToMemoryKHR,
            pub copy_memory_to_acceleration_structure_khr:
                PFN_vkCopyMemoryToAccelerationStructureKHR,
            pub write_acceleration_structures_properties_khr:
                PFN_vkWriteAccelerationStructuresPropertiesKHR,
            pub cmd_copy_acceleration_structure_khr: PFN_vkCmdCopyAccelerationStructureKHR,
            pub cmd_copy_acceleration_structure_to_memory_khr:
                PFN_vkCmdCopyAccelerationStructureToMemoryKHR,
            pub cmd_copy_memory_to_acceleration_structure_khr:
                PFN_vkCmdCopyMemoryToAccelerationStructureKHR,
            pub get_acceleration_structure_device_address_khr:
                PFN_vkGetAccelerationStructureDeviceAddressKHR,
            pub cmd_write_acceleration_structures_properties_khr:
                PFN_vkCmdWriteAccelerationStructuresPropertiesKHR,
            pub get_device_acceleration_structure_compatibility_khr:
                PFN_vkGetDeviceAccelerationStructureCompatibilityKHR,
            pub get_acceleration_structure_build_sizes_khr:
                PFN_vkGetAccelerationStructureBuildSizesKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_acceleration_structure_khr: unsafe {
                        unsafe extern "system" fn create_acceleration_structure_khr(
                            _device: crate::vk::Device,
                            _p_create_info: *const AccelerationStructureCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks,
                            _p_acceleration_structure: *mut AccelerationStructureKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_acceleration_structure_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateAccelerationStructureKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_acceleration_structure_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_acceleration_structure_khr: unsafe {
                        unsafe extern "system" fn destroy_acceleration_structure_khr(
                            _device: crate::vk::Device,
                            _acceleration_structure: AccelerationStructureKHR,
                            _p_allocator: *const AllocationCallbacks,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_acceleration_structure_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkDestroyAccelerationStructureKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_acceleration_structure_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_build_acceleration_structures_khr: unsafe {
                        unsafe extern "system" fn cmd_build_acceleration_structures_khr(
                            _command_buffer: CommandBuffer,
                            _info_count: u32,
                            _p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
                            _pp_build_range_infos : * const * const AccelerationStructureBuildRangeInfoKHR,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_build_acceleration_structures_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdBuildAccelerationStructuresKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_build_acceleration_structures_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_build_acceleration_structures_indirect_khr: unsafe {
                        unsafe extern "system" fn cmd_build_acceleration_structures_indirect_khr(
                            _command_buffer: CommandBuffer,
                            _info_count: u32,
                            _p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
                            _p_indirect_device_addresses: *const DeviceAddress,
                            _p_indirect_strides: *const u32,
                            _pp_max_primitive_counts: *const *const u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_build_acceleration_structures_indirect_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdBuildAccelerationStructuresIndirectKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_build_acceleration_structures_indirect_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    build_acceleration_structures_khr: unsafe {
                        unsafe extern "system" fn build_acceleration_structures_khr(
                            _device: crate::vk::Device,
                            _deferred_operation: DeferredOperationKHR,
                            _info_count: u32,
                            _p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
                            _pp_build_range_infos : * const * const AccelerationStructureBuildRangeInfoKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(build_acceleration_structures_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkBuildAccelerationStructuresKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            build_acceleration_structures_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    copy_acceleration_structure_khr: unsafe {
                        unsafe extern "system" fn copy_acceleration_structure_khr(
                            _device: crate::vk::Device,
                            _deferred_operation: DeferredOperationKHR,
                            _p_info: *const CopyAccelerationStructureInfoKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(copy_acceleration_structure_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCopyAccelerationStructureKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            copy_acceleration_structure_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    copy_acceleration_structure_to_memory_khr: unsafe {
                        unsafe extern "system" fn copy_acceleration_structure_to_memory_khr(
                            _device: crate::vk::Device,
                            _deferred_operation: DeferredOperationKHR,
                            _p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(copy_acceleration_structure_to_memory_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCopyAccelerationStructureToMemoryKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            copy_acceleration_structure_to_memory_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    copy_memory_to_acceleration_structure_khr: unsafe {
                        unsafe extern "system" fn copy_memory_to_acceleration_structure_khr(
                            _device: crate::vk::Device,
                            _deferred_operation: DeferredOperationKHR,
                            _p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(copy_memory_to_acceleration_structure_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCopyMemoryToAccelerationStructureKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            copy_memory_to_acceleration_structure_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    write_acceleration_structures_properties_khr: unsafe {
                        unsafe extern "system" fn write_acceleration_structures_properties_khr(
                            _device: crate::vk::Device,
                            _acceleration_structure_count: u32,
                            _p_acceleration_structures: *const AccelerationStructureKHR,
                            _query_type: QueryType,
                            _data_size: usize,
                            _p_data: *mut c_void,
                            _stride: usize,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(write_acceleration_structures_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkWriteAccelerationStructuresPropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            write_acceleration_structures_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_acceleration_structure_khr: unsafe {
                        unsafe extern "system" fn cmd_copy_acceleration_structure_khr(
                            _command_buffer: CommandBuffer,
                            _p_info: *const CopyAccelerationStructureInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_acceleration_structure_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdCopyAccelerationStructureKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_acceleration_structure_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_acceleration_structure_to_memory_khr: unsafe {
                        unsafe extern "system" fn cmd_copy_acceleration_structure_to_memory_khr(
                            _command_buffer: CommandBuffer,
                            _p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_acceleration_structure_to_memory_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdCopyAccelerationStructureToMemoryKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_acceleration_structure_to_memory_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_memory_to_acceleration_structure_khr: unsafe {
                        unsafe extern "system" fn cmd_copy_memory_to_acceleration_structure_khr(
                            _command_buffer: CommandBuffer,
                            _p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_memory_to_acceleration_structure_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdCopyMemoryToAccelerationStructureKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_memory_to_acceleration_structure_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_acceleration_structure_device_address_khr: unsafe {
                        unsafe extern "system" fn get_acceleration_structure_device_address_khr(
                            _device: crate::vk::Device,
                            _p_info: *const AccelerationStructureDeviceAddressInfoKHR<'_>,
                        ) -> DeviceAddress {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_acceleration_structure_device_address_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetAccelerationStructureDeviceAddressKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_acceleration_structure_device_address_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_write_acceleration_structures_properties_khr: unsafe {
                        unsafe extern "system" fn cmd_write_acceleration_structures_properties_khr(
                            _command_buffer: CommandBuffer,
                            _acceleration_structure_count: u32,
                            _p_acceleration_structures: *const AccelerationStructureKHR,
                            _query_type: QueryType,
                            _query_pool: QueryPool,
                            _first_query: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_write_acceleration_structures_properties_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdWriteAccelerationStructuresPropertiesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_write_acceleration_structures_properties_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_acceleration_structure_compatibility_khr: unsafe {
                        unsafe extern "system" fn get_device_acceleration_structure_compatibility_khr(
                            _device: crate::vk::Device,
                            _p_version_info: *const AccelerationStructureVersionInfoKHR<'_>,
                            _p_compatibility: *mut AccelerationStructureCompatibilityKHR,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_acceleration_structure_compatibility_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceAccelerationStructureCompatibilityKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_acceleration_structure_compatibility_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_acceleration_structure_build_sizes_khr: unsafe {
                        unsafe extern "system" fn get_acceleration_structure_build_sizes_khr(
                            _device: crate::vk::Device,
                            _build_type: AccelerationStructureBuildTypeKHR,
                            _p_build_info: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
                            _p_max_primitive_counts: *const u32,
                            _p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_acceleration_structure_build_sizes_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetAccelerationStructureBuildSizesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_acceleration_structure_build_sizes_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_ray_tracing_pipeline"]
    pub mod ray_tracing_pipeline {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_RAY_TRACING_PIPELINE_NAME as NAME,
            crate::vk::KHR_RAY_TRACING_PIPELINE_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_ray_tracing_pipeline device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_ray_tracing_pipeline device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_trace_rays_khr: PFN_vkCmdTraceRaysKHR,
            pub create_ray_tracing_pipelines_khr: PFN_vkCreateRayTracingPipelinesKHR,
            pub get_ray_tracing_shader_group_handles_khr: PFN_vkGetRayTracingShaderGroupHandlesKHR,
            pub get_ray_tracing_capture_replay_shader_group_handles_khr:
                PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR,
            pub cmd_trace_rays_indirect_khr: PFN_vkCmdTraceRaysIndirectKHR,
            pub get_ray_tracing_shader_group_stack_size_khr:
                PFN_vkGetRayTracingShaderGroupStackSizeKHR,
            pub cmd_set_ray_tracing_pipeline_stack_size_khr:
                PFN_vkCmdSetRayTracingPipelineStackSizeKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_trace_rays_khr: unsafe {
                        unsafe extern "system" fn cmd_trace_rays_khr(
                            _command_buffer: CommandBuffer,
                            _p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _width: u32,
                            _height: u32,
                            _depth: u32,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_trace_rays_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_trace_rays_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    create_ray_tracing_pipelines_khr: unsafe {
                        unsafe extern "system" fn create_ray_tracing_pipelines_khr(
                            _device: crate::vk::Device,
                            _deferred_operation: DeferredOperationKHR,
                            _pipeline_cache: PipelineCache,
                            _create_info_count: u32,
                            _p_create_infos: *const RayTracingPipelineCreateInfoKHR<'_>,
                            _p_allocator: *const AllocationCallbacks,
                            _p_pipelines: *mut Pipeline,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_ray_tracing_pipelines_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCreateRayTracingPipelinesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            create_ray_tracing_pipelines_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_ray_tracing_shader_group_handles_khr: unsafe {
                        unsafe extern "system" fn get_ray_tracing_shader_group_handles_khr(
                            _device: crate::vk::Device,
                            _pipeline: Pipeline,
                            _first_group: u32,
                            _group_count: u32,
                            _data_size: usize,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_ray_tracing_shader_group_handles_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetRayTracingShaderGroupHandlesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_ray_tracing_shader_group_handles_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_ray_tracing_capture_replay_shader_group_handles_khr: unsafe {
                        unsafe extern "system" fn get_ray_tracing_capture_replay_shader_group_handles_khr(
                            _device: crate::vk::Device,
                            _pipeline: Pipeline,
                            _first_group: u32,
                            _group_count: u32,
                            _data_size: usize,
                            _p_data: *mut c_void,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_ray_tracing_capture_replay_shader_group_handles_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_ray_tracing_capture_replay_shader_group_handles_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_trace_rays_indirect_khr: unsafe {
                        unsafe extern "system" fn cmd_trace_rays_indirect_khr(
                            _command_buffer: CommandBuffer,
                            _p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                            _indirect_device_address: DeviceAddress,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_trace_rays_indirect_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysIndirectKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_trace_rays_indirect_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_ray_tracing_shader_group_stack_size_khr: unsafe {
                        unsafe extern "system" fn get_ray_tracing_shader_group_stack_size_khr(
                            _device: crate::vk::Device,
                            _pipeline: Pipeline,
                            _group: u32,
                            _group_shader: ShaderGroupShaderKHR,
                        ) -> DeviceSize {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_ray_tracing_shader_group_stack_size_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetRayTracingShaderGroupStackSizeKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_ray_tracing_shader_group_stack_size_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_ray_tracing_pipeline_stack_size_khr: unsafe {
                        unsafe extern "system" fn cmd_set_ray_tracing_pipeline_stack_size_khr(
                            _command_buffer: CommandBuffer,
                            _pipeline_stack_size: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_ray_tracing_pipeline_stack_size_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRayTracingPipelineStackSizeKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_ray_tracing_pipeline_stack_size_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_ray_query"]
    pub mod ray_query {
        pub use {
            crate::vk::KHR_RAY_QUERY_NAME as NAME,
            crate::vk::KHR_RAY_QUERY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_portability_subset"]
    #[cfg(feature = "provisional")]
    pub mod portability_subset {
        pub use {
            crate::vk::KHR_PORTABILITY_SUBSET_NAME as NAME,
            crate::vk::KHR_PORTABILITY_SUBSET_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_maintenance3"]
    pub mod maintenance3 {
        pub use {
            crate::vk::KHR_MAINTENANCE3_NAME as NAME,
            crate::vk::KHR_MAINTENANCE3_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_draw_indirect_count"]
    pub mod draw_indirect_count {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_DRAW_INDIRECT_COUNT_NAME as NAME,
            crate::vk::KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_draw_indirect_count device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_draw_indirect_count device-level function pointers"]
        pub struct DeviceFn {
            #[doc = "Deprecated: <https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-buffer-commands>"]
            pub cmd_draw_indirect_count_khr: PFN_vkCmdDrawIndirectCount,
            #[doc = "Deprecated: <https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-buffer-commands>"]
            pub cmd_draw_indexed_indirect_count_khr: PFN_vkCmdDrawIndexedIndirectCount,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_draw_indirect_count_khr: unsafe {
                        unsafe extern "system" fn cmd_draw_indirect_count_khr(
                            _command_buffer: CommandBuffer,
                            _buffer: Buffer,
                            _offset: DeviceSize,
                            _count_buffer: Buffer,
                            _count_buffer_offset: DeviceSize,
                            _max_draw_count: u32,
                            _stride: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_draw_indirect_count_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectCountKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_indirect_count_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_draw_indexed_indirect_count_khr: unsafe {
                        unsafe extern "system" fn cmd_draw_indexed_indirect_count_khr(
                            _command_buffer: CommandBuffer,
                            _buffer: Buffer,
                            _offset: DeviceSize,
                            _count_buffer: Buffer,
                            _count_buffer_offset: DeviceSize,
                            _max_draw_count: u32,
                            _stride: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_draw_indexed_indirect_count_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdDrawIndexedIndirectCountKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_draw_indexed_indirect_count_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_dynamic_rendering_local_read"]
    pub mod dynamic_rendering_local_read {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_DYNAMIC_RENDERING_LOCAL_READ_NAME as NAME,
            crate::vk::KHR_DYNAMIC_RENDERING_LOCAL_READ_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_dynamic_rendering_local_read device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_dynamic_rendering_local_read device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_rendering_attachment_locations_khr:
                PFN_vkCmdSetRenderingAttachmentLocations,
            pub cmd_set_rendering_input_attachment_indices_khr:
                PFN_vkCmdSetRenderingInputAttachmentIndices,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_rendering_attachment_locations_khr: unsafe {
                        unsafe extern "system" fn cmd_set_rendering_attachment_locations_khr(
                            _command_buffer: CommandBuffer,
                            _p_location_info: *const RenderingAttachmentLocationInfo<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rendering_attachment_locations_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRenderingAttachmentLocationsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rendering_attachment_locations_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_set_rendering_input_attachment_indices_khr: unsafe {
                        unsafe extern "system" fn cmd_set_rendering_input_attachment_indices_khr(
                            _command_buffer: CommandBuffer,
                            _p_input_attachment_index_info : * const RenderingInputAttachmentIndexInfo < '_ >,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_set_rendering_input_attachment_indices_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkCmdSetRenderingInputAttachmentIndicesKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_rendering_input_attachment_indices_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_spirv_1_4"]
    pub mod spirv_1_4 {
        pub use {
            crate::vk::KHR_SPIRV_1_4_NAME as NAME,
            crate::vk::KHR_SPIRV_1_4_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_buffer_device_address"]
    pub mod buffer_device_address {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_BUFFER_DEVICE_ADDRESS_NAME as NAME,
            crate::vk::KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_buffer_device_address device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_buffer_device_address device-level function pointers"]
        pub struct DeviceFn {
            pub get_buffer_device_address_khr: PFN_vkGetBufferDeviceAddress,
            pub get_buffer_opaque_capture_address_khr: PFN_vkGetBufferOpaqueCaptureAddress,
            pub get_device_memory_opaque_capture_address_khr:
                PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_buffer_device_address_khr: unsafe {
                        unsafe extern "system" fn get_buffer_device_address_khr(
                            _device: crate::vk::Device,
                            _p_info: *const BufferDeviceAddressInfo<'_>,
                        ) -> DeviceAddress {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_buffer_device_address_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkGetBufferDeviceAddressKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            get_buffer_device_address_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_buffer_opaque_capture_address_khr: unsafe {
                        unsafe extern "system" fn get_buffer_opaque_capture_address_khr(
                            _device: crate::vk::Device,
                            _p_info: *const BufferDeviceAddressInfo<'_>,
                        ) -> u64 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_buffer_opaque_capture_address_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetBufferOpaqueCaptureAddressKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_buffer_opaque_capture_address_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_memory_opaque_capture_address_khr: unsafe {
                        unsafe extern "system" fn get_device_memory_opaque_capture_address_khr(
                            _device: crate::vk::Device,
                            _p_info: *const DeviceMemoryOpaqueCaptureAddressInfo<'_>,
                        ) -> u64 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_memory_opaque_capture_address_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceMemoryOpaqueCaptureAddressKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_memory_opaque_capture_address_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_deferred_host_operations"]
    pub mod deferred_host_operations {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_DEFERRED_HOST_OPERATIONS_NAME as NAME,
            crate::vk::KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_deferred_host_operations device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_deferred_host_operations device-level function pointers"]
        pub struct DeviceFn {
            pub create_deferred_operation_khr: PFN_vkCreateDeferredOperationKHR,
            pub destroy_deferred_operation_khr: PFN_vkDestroyDeferredOperationKHR,
            pub get_deferred_operation_max_concurrency_khr:
                PFN_vkGetDeferredOperationMaxConcurrencyKHR,
            pub get_deferred_operation_result_khr: PFN_vkGetDeferredOperationResultKHR,
            pub deferred_operation_join_khr: PFN_vkDeferredOperationJoinKHR,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    create_deferred_operation_khr: unsafe {
                        unsafe extern "system" fn create_deferred_operation_khr(
                            _device: crate::vk::Device,
                            _p_allocator: *const AllocationCallbacks,
                            _p_deferred_operation: *mut DeferredOperationKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(create_deferred_operation_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCreateDeferredOperationKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            create_deferred_operation_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    destroy_deferred_operation_khr: unsafe {
                        unsafe extern "system" fn destroy_deferred_operation_khr(
                            _device: crate::vk::Device,
                            _operation: DeferredOperationKHR,
                            _p_allocator: *const AllocationCallbacks,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(destroy_deferred_operation_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkDestroyDeferredOperationKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            destroy_deferred_operation_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_deferred_operation_max_concurrency_khr: unsafe {
                        unsafe extern "system" fn get_deferred_operation_max_concurrency_khr(
                            _device: crate::vk::Device,
                            _operation: DeferredOperationKHR,
                        ) -> u32 {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_deferred_operation_max_concurrency_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeferredOperationMaxConcurrencyKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_deferred_operation_max_concurrency_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_deferred_operation_result_khr: unsafe {
                        unsafe extern "system" fn get_deferred_operation_result_khr(
                            _device: crate::vk::Device,
                            _operation: DeferredOperationKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_deferred_operation_result_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeferredOperationResultKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_deferred_operation_result_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    deferred_operation_join_khr: unsafe {
                        unsafe extern "system" fn deferred_operation_join_khr(
                            _device: crate::vk::Device,
                            _operation: DeferredOperationKHR,
                        ) -> Result {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(deferred_operation_join_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkDeferredOperationJoinKHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            deferred_operation_join_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_pipeline_library"]
    pub mod pipeline_library {
        pub use {
            crate::vk::KHR_PIPELINE_LIBRARY_NAME as NAME,
            crate::vk::KHR_PIPELINE_LIBRARY_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_synchronization2"]
    pub mod synchronization2 {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_SYNCHRONIZATION2_NAME as NAME,
            crate::vk::KHR_SYNCHRONIZATION2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_synchronization2 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_synchronization2 device-level function pointers"]
        pub struct DeviceFn {
            pub cmd_set_event2_khr: PFN_vkCmdSetEvent2,
            pub cmd_reset_event2_khr: PFN_vkCmdResetEvent2,
            pub cmd_wait_events2_khr: PFN_vkCmdWaitEvents2,
            pub cmd_pipeline_barrier2_khr: PFN_vkCmdPipelineBarrier2,
            pub cmd_write_timestamp2_khr: PFN_vkCmdWriteTimestamp2,
            pub queue_submit2_khr: PFN_vkQueueSubmit2,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_set_event2_khr: unsafe {
                        unsafe extern "system" fn cmd_set_event2_khr(
                            _command_buffer: CommandBuffer,
                            _event: Event,
                            _p_dependency_info: *const DependencyInfo<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_set_event2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdSetEvent2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_set_event2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_reset_event2_khr: unsafe {
                        unsafe extern "system" fn cmd_reset_event2_khr(
                            _command_buffer: CommandBuffer,
                            _event: Event,
                            _stage_mask: PipelineStageFlags2,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_reset_event2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdResetEvent2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_reset_event2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_wait_events2_khr: unsafe {
                        unsafe extern "system" fn cmd_wait_events2_khr(
                            _command_buffer: CommandBuffer,
                            _event_count: u32,
                            _p_events: *const Event,
                            _p_dependency_infos: *const DependencyInfo<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_wait_events2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdWaitEvents2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_wait_events2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_pipeline_barrier2_khr: unsafe {
                        unsafe extern "system" fn cmd_pipeline_barrier2_khr(
                            _command_buffer: CommandBuffer,
                            _p_dependency_info: *const DependencyInfo<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_pipeline_barrier2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdPipelineBarrier2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_pipeline_barrier2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_write_timestamp2_khr: unsafe {
                        unsafe extern "system" fn cmd_write_timestamp2_khr(
                            _command_buffer: CommandBuffer,
                            _stage: PipelineStageFlags2,
                            _query_pool: QueryPool,
                            _query: u32,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_write_timestamp2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteTimestamp2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_write_timestamp2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    queue_submit2_khr: unsafe {
                        unsafe extern "system" fn queue_submit2_khr(
                            _queue: Queue,
                            _submit_count: u32,
                            _p_submits: *const SubmitInfo2<'_>,
                            _fence: Fence,
                        ) -> Result {
                            panic!(concat!("Unable to load ", stringify!(queue_submit2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkQueueSubmit2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            queue_submit2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_copy_commands2"]
    pub mod copy_commands2 {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_COPY_COMMANDS2_NAME as NAME,
            crate::vk::KHR_COPY_COMMANDS2_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_copy_commands2 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_copy_commands2 device-level function pointers"]
        pub struct DeviceFn {
            #[doc = "Deprecated: <https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-buffer-commands>"]
            pub cmd_copy_buffer2_khr: PFN_vkCmdCopyBuffer2,
            pub cmd_copy_image2_khr: PFN_vkCmdCopyImage2,
            #[doc = "Deprecated: <https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-buffer-commands>"]
            pub cmd_copy_buffer_to_image2_khr: PFN_vkCmdCopyBufferToImage2,
            #[doc = "Deprecated: <https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-buffer-commands>"]
            pub cmd_copy_image_to_buffer2_khr: PFN_vkCmdCopyImageToBuffer2,
            pub cmd_blit_image2_khr: PFN_vkCmdBlitImage2,
            pub cmd_resolve_image2_khr: PFN_vkCmdResolveImage2,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_copy_buffer2_khr: unsafe {
                        unsafe extern "system" fn cmd_copy_buffer2_khr(
                            _command_buffer: CommandBuffer,
                            _p_copy_buffer_info: *const CopyBufferInfo2<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_copy_buffer2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBuffer2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_buffer2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_image2_khr: unsafe {
                        unsafe extern "system" fn cmd_copy_image2_khr(
                            _command_buffer: CommandBuffer,
                            _p_copy_image_info: *const CopyImageInfo2<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_copy_image2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImage2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_image2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_buffer_to_image2_khr: unsafe {
                        unsafe extern "system" fn cmd_copy_buffer_to_image2_khr(
                            _command_buffer: CommandBuffer,
                            _p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_buffer_to_image2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBufferToImage2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_buffer_to_image2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_copy_image_to_buffer2_khr: unsafe {
                        unsafe extern "system" fn cmd_copy_image_to_buffer2_khr(
                            _command_buffer: CommandBuffer,
                            _p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_copy_image_to_buffer2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImageToBuffer2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_copy_image_to_buffer2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_blit_image2_khr: unsafe {
                        unsafe extern "system" fn cmd_blit_image2_khr(
                            _command_buffer: CommandBuffer,
                            _p_blit_image_info: *const BlitImageInfo2<'_>,
                        ) {
                            panic!(concat!("Unable to load ", stringify!(cmd_blit_image2_khr)))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdBlitImage2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_blit_image2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    cmd_resolve_image2_khr: unsafe {
                        unsafe extern "system" fn cmd_resolve_image2_khr(
                            _command_buffer: CommandBuffer,
                            _p_resolve_image_info: *const ResolveImageInfo2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_resolve_image2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(b"vkCmdResolveImage2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_resolve_image2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_portability_enumeration"]
    pub mod portability_enumeration {
        pub use {
            crate::vk::KHR_PORTABILITY_ENUMERATION_NAME as NAME,
            crate::vk::KHR_PORTABILITY_ENUMERATION_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_maintenance4"]
    pub mod maintenance4 {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_MAINTENANCE4_NAME as NAME,
            crate::vk::KHR_MAINTENANCE4_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_maintenance4 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_maintenance4 device-level function pointers"]
        pub struct DeviceFn {
            pub get_device_buffer_memory_requirements_khr: PFN_vkGetDeviceBufferMemoryRequirements,
            pub get_device_image_memory_requirements_khr: PFN_vkGetDeviceImageMemoryRequirements,
            pub get_device_image_sparse_memory_requirements_khr:
                PFN_vkGetDeviceImageSparseMemoryRequirements,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    get_device_buffer_memory_requirements_khr: unsafe {
                        unsafe extern "system" fn get_device_buffer_memory_requirements_khr(
                            _device: crate::vk::Device,
                            _p_info: *const DeviceBufferMemoryRequirements<'_>,
                            _p_memory_requirements: *mut MemoryRequirements2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_buffer_memory_requirements_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceBufferMemoryRequirementsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_buffer_memory_requirements_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_image_memory_requirements_khr: unsafe {
                        unsafe extern "system" fn get_device_image_memory_requirements_khr(
                            _device: crate::vk::Device,
                            _p_info: *const DeviceImageMemoryRequirements<'_>,
                            _p_memory_requirements: *mut MemoryRequirements2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_image_memory_requirements_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceImageMemoryRequirementsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_image_memory_requirements_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_image_sparse_memory_requirements_khr: unsafe {
                        unsafe extern "system" fn get_device_image_sparse_memory_requirements_khr(
                            _device: crate::vk::Device,
                            _p_info: *const DeviceImageMemoryRequirements<'_>,
                            _p_sparse_memory_requirement_count: *mut u32,
                            _p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_image_sparse_memory_requirements_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceImageSparseMemoryRequirementsKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_image_sparse_memory_requirements_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_maintenance5"]
    pub mod maintenance5 {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_MAINTENANCE5_NAME as NAME,
            crate::vk::KHR_MAINTENANCE5_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_maintenance5 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_maintenance5 device-level function pointers"]
        pub struct DeviceFn {
            #[doc = "Deprecated: <https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-buffer-commands>"]
            pub cmd_bind_index_buffer2_khr: PFN_vkCmdBindIndexBuffer2,
            pub get_rendering_area_granularity_khr: PFN_vkGetRenderingAreaGranularity,
            pub get_device_image_subresource_layout_khr: PFN_vkGetDeviceImageSubresourceLayout,
            pub get_image_subresource_layout2_khr: PFN_vkGetImageSubresourceLayout2,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_bind_index_buffer2_khr: unsafe {
                        unsafe extern "system" fn cmd_bind_index_buffer2_khr(
                            _command_buffer: CommandBuffer,
                            _buffer: Buffer,
                            _offset: DeviceSize,
                            _size: DeviceSize,
                            _index_type: IndexType,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_bind_index_buffer2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdBindIndexBuffer2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_bind_index_buffer2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_rendering_area_granularity_khr: unsafe {
                        unsafe extern "system" fn get_rendering_area_granularity_khr(
                            _device: crate::vk::Device,
                            _p_rendering_area_info: *const RenderingAreaInfo<'_>,
                            _p_granularity: *mut Extent2D,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_rendering_area_granularity_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetRenderingAreaGranularityKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_rendering_area_granularity_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_device_image_subresource_layout_khr: unsafe {
                        unsafe extern "system" fn get_device_image_subresource_layout_khr(
                            _device: crate::vk::Device,
                            _p_info: *const DeviceImageSubresourceInfo<'_>,
                            _p_layout: *mut SubresourceLayout2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_device_image_subresource_layout_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetDeviceImageSubresourceLayoutKHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_device_image_subresource_layout_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                    get_image_subresource_layout2_khr: unsafe {
                        unsafe extern "system" fn get_image_subresource_layout2_khr(
                            _device: crate::vk::Device,
                            _image: Image,
                            _p_subresource: *const ImageSubresource2<'_>,
                            _p_layout: *mut SubresourceLayout2<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(get_image_subresource_layout2_khr)
                            ))
                        }
                        let cname = CStr::from_bytes_with_nul_unchecked(
                            b"vkGetImageSubresourceLayout2KHR\0",
                        );
                        let val = _f(cname);
                        if val.is_null() {
                            get_image_subresource_layout2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_ray_tracing_position_fetch"]
    pub mod ray_tracing_position_fetch {
        pub use {
            crate::vk::KHR_RAY_TRACING_POSITION_FETCH_NAME as NAME,
            crate::vk::KHR_RAY_TRACING_POSITION_FETCH_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_maintenance6"]
    pub mod maintenance6 {
        use crate::vk::*;
        use core::ffi::*;
        pub use {
            crate::vk::KHR_MAINTENANCE6_NAME as NAME,
            crate::vk::KHR_MAINTENANCE6_SPEC_VERSION as SPEC_VERSION,
        };
        #[doc = "VK_KHR_maintenance6 device-level functions"]
        #[derive(Clone)]
        pub struct Device {
            pub(crate) fp: DeviceFn,
            pub(crate) handle: crate::vk::Device,
        }
        impl Device {
            pub fn load(instance: &crate::Instance, device: &crate::Device) -> Self {
                let handle = device.handle;
                let fp = DeviceFn::load(|name| unsafe {
                    core::mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
                });
                Self { handle, fp }
            }
            #[inline]
            pub fn fp(&self) -> &DeviceFn {
                &self.fp
            }
            #[inline]
            pub fn device(&self) -> crate::vk::Device {
                self.handle
            }
        }
        #[derive(Clone)]
        #[doc = "Raw VK_KHR_maintenance6 device-level function pointers"]
        pub struct DeviceFn {
            #[doc = "Deprecated: <https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-descriptor-sets>"]
            pub cmd_push_constants2_khr: PFN_vkCmdPushConstants2,
        }
        unsafe impl Send for DeviceFn {}
        unsafe impl Sync for DeviceFn {}
        impl DeviceFn {
            pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
                Self::load_erased(&mut f)
            }
            fn load_erased(_f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
                Self {
                    cmd_push_constants2_khr: unsafe {
                        unsafe extern "system" fn cmd_push_constants2_khr(
                            _command_buffer: CommandBuffer,
                            _p_push_constants_info: *const PushConstantsInfo<'_>,
                        ) {
                            panic!(concat!(
                                "Unable to load ",
                                stringify!(cmd_push_constants2_khr)
                            ))
                        }
                        let cname =
                            CStr::from_bytes_with_nul_unchecked(b"vkCmdPushConstants2KHR\0");
                        let val = _f(cname);
                        if val.is_null() {
                            cmd_push_constants2_khr
                        } else {
                            ::core::mem::transmute(val)
                        }
                    },
                }
            }
        }
    }
    #[doc = "VK_KHR_maintenance7"]
    pub mod maintenance7 {
        pub use {
            crate::vk::KHR_MAINTENANCE7_NAME as NAME,
            crate::vk::KHR_MAINTENANCE7_SPEC_VERSION as SPEC_VERSION,
        };
    }
    #[doc = "VK_KHR_maintenance8"]
    pub mod maintenance8 {
        pub use {
            crate::vk::KHR_MAINTENANCE8_NAME as NAME,
            crate::vk::KHR_MAINTENANCE8_SPEC_VERSION as SPEC_VERSION,
        };
    }
}
