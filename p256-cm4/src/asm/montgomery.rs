#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Montgomery([u32; 8]);

impl Montgomery {
    pub const fn zero() -> Self {
        Self([0u32; _])
    }
}

impl core::ops::Index<usize> for Montgomery {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Default for Montgomery {
    fn default() -> Self {
        Self::zero()
    }
}
