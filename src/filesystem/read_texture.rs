use std::ffi::CString;

pub fn register_spiffs_partition(mount_point: &str, partition_name: &str) -> anyhow::Result<()> {
    let base_path = CString::new(mount_point)?;
    let partition = CString::new(partition_name)?;

    let conf = esp_idf_svc::sys::esp_vfs_spiffs_conf_t {
        base_path: base_path.as_ptr(),
        partition_label: partition.as_ptr(),
        max_files: 5,
        format_if_mount_failed: true,
    };

    unsafe {
        esp_idf_svc::sys::esp_nofail!(esp_idf_svc::sys::esp_vfs_spiffs_register(&conf));
    }

    Ok(())
}

pub fn read_texture_to_buffer(file_path: &str) -> anyhow::Result<Vec<Vec<u8>>> {
    let mut texture = std::fs::read(file_path).map_err(anyhow::Error::from)?;
    let w = ((texture[0] as u16) << 8) | texture[1] as u16;
    let _dims = texture.split_off(4);
    Ok(texture
        .chunks(w as usize)
        .map(|el| el.to_vec())
        .collect::<Vec<Vec<u8>>>())
}
