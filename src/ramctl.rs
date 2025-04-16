#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rcctl0: Rcctl0,
}
impl RegisterBlock {
    #[doc = "0x00 - RAM Controller Control 0"]
    #[inline(always)]
    pub const fn rcctl0(&self) -> &Rcctl0 {
        &self.rcctl0
    }
}
#[doc = "RCCTL0 (rw) register accessor: RAM Controller Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rcctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcctl0`] module"]
#[doc(alias = "RCCTL0")]
pub type Rcctl0 = crate::Reg<rcctl0::Rcctl0Spec>;
#[doc = "RAM Controller Control 0"]
pub mod rcctl0;
