use std::{ptr, ffi::CString};
//Container object for everything within the current VulkanApp
pub struct VulkanApp{
    pub app_version: u32,
    pub vulkan_api_version: u32,
    pub vk_instance : ash::Instance
}

impl VulkanApp{
    pub fn new(app_version: u32, vulkan_api_version: u32, 
        extensions : &Vec<&str>) -> VulkanApp{
        let entry = unsafe{
            ash::Entry::load().unwrap()
        };
        let app_info = ash::vk::ApplicationInfo{
            s_type: ash::vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: ptr::null(),
            application_version: app_version,
            p_engine_name: ptr::null(),
            engine_version: app_version,
            api_version: vulkan_api_version,
        };

        let mut ptr_vec: Vec<*const i8> = Vec::new();
        extensions.iter().for_each(|extension| {
            let as_c_str = CString::new(*extension).expect("Unable to create extension string");
            let as_c_ptr = as_c_str.into_raw();
            ptr_vec.push(as_c_ptr);
        });
        
        let create_info = ash::vk::InstanceCreateInfo {
            s_type: ash::vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: ash::vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR,
            p_application_info: &app_info,
            enabled_layer_count: 0,
            pp_enabled_layer_names: ptr::null(),
            enabled_extension_count: (extensions.len() as u32),
            pp_enabled_extension_names: ptr_vec.as_ptr()
        };
        let instance = unsafe{
            entry.create_instance(
                &create_info, 
                None
            ).expect("Failed to create instance")
        };
        unsafe{
            //Explicitly deallocate cstrings that were used for config
            //Prevents memory leakage
            for ptr in ptr_vec {
                let _ = CString::from_raw(ptr as *mut i8);
            }
        }

        VulkanApp {
            app_version: app_version,
            vulkan_api_version: vulkan_api_version,
            vk_instance: instance
        }
    }
    pub fn destroy(self){
        unsafe {
            self.vk_instance.destroy_instance(None)
        }
    }
    
}
