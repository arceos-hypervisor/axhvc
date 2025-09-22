//! Arguments for hypercalls related to VM management.

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AxHVCCreateVMArg {
    pub cfg_file_gpa: u64,
    pub cfg_file_size: u64,

    pub kernel_image_size: u64,
    pub bios_image_size: u64,
    pub ramdisk_image_size: u64,

    pub vm_id: u64,
    pub kernel_load_gpa: u64,
    pub bios_load_gpa: u64,
    pub ramdisk_load_gpa: u64,
}
