use crate::ops::DOCUMENTS;

pub fn free_image(id: &u32) -> anyhow::Result<()> {
    DOCUMENTS.remove(id);
    Ok(())
}
