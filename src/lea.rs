#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    leacap: Leacap,
    leacnf0: Leacnf0,
    leacnf1: Leacnf1,
    leacnf2: Leacnf2,
    leamb: Leamb,
    leamt: Leamt,
    leacma: Leacma,
    leacmctl: Leacmctl,
    _reserved8: [u8; 0x08],
    leacmdstat: Leacmdstat,
    leas1stat: Leas1stat,
    leas0stat: Leas0stat,
    leadststat: Leadststat,
    _reserved12: [u8; 0x08],
    leapmctl: Leapmctl,
    leapmdst: Leapmdst,
    leapms1: Leapms1,
    leapms0: Leapms0,
    leapmcb: Leapmcb,
    _reserved17: [u8; 0x1c],
    leaifgset: Leaifgset,
    leaie: Leaie,
    leaifg: Leaifg,
    leaiv: Leaiv,
}
impl RegisterBlock {
    #[doc = "0x00 - LEA Capability Register"]
    #[inline(always)]
    pub const fn leacap(&self) -> &Leacap {
        &self.leacap
    }
    #[doc = "0x04 - Configuration Register 0"]
    #[inline(always)]
    pub const fn leacnf0(&self) -> &Leacnf0 {
        &self.leacnf0
    }
    #[doc = "0x08 - Configuration Register 1"]
    #[inline(always)]
    pub const fn leacnf1(&self) -> &Leacnf1 {
        &self.leacnf1
    }
    #[doc = "0x0c - Configuration Register 2"]
    #[inline(always)]
    pub const fn leacnf2(&self) -> &Leacnf2 {
        &self.leacnf2
    }
    #[doc = "0x10 - Memory Bottom Register"]
    #[inline(always)]
    pub const fn leamb(&self) -> &Leamb {
        &self.leamb
    }
    #[doc = "0x14 - Memory Top Register"]
    #[inline(always)]
    pub const fn leamt(&self) -> &Leamt {
        &self.leamt
    }
    #[doc = "0x18 - Code Memory Access Register"]
    #[inline(always)]
    pub const fn leacma(&self) -> &Leacma {
        &self.leacma
    }
    #[doc = "0x1c - Code Memory Control Register"]
    #[inline(always)]
    pub const fn leacmctl(&self) -> &Leacmctl {
        &self.leacmctl
    }
    #[doc = "0x28 - LEA Command Status Register"]
    #[inline(always)]
    pub const fn leacmdstat(&self) -> &Leacmdstat {
        &self.leacmdstat
    }
    #[doc = "0x2c - LEA Source 1 Status Register"]
    #[inline(always)]
    pub const fn leas1stat(&self) -> &Leas1stat {
        &self.leas1stat
    }
    #[doc = "0x30 - LEA Source 0 Status Register"]
    #[inline(always)]
    pub const fn leas0stat(&self) -> &Leas0stat {
        &self.leas0stat
    }
    #[doc = "0x34 - LEA Result Status Register"]
    #[inline(always)]
    pub const fn leadststat(&self) -> &Leadststat {
        &self.leadststat
    }
    #[doc = "0x40 - PM Control Register"]
    #[inline(always)]
    pub const fn leapmctl(&self) -> &Leapmctl {
        &self.leapmctl
    }
    #[doc = "0x44 - PM Result Register"]
    #[inline(always)]
    pub const fn leapmdst(&self) -> &Leapmdst {
        &self.leapmdst
    }
    #[doc = "0x48 - PM Source 1 Register"]
    #[inline(always)]
    pub const fn leapms1(&self) -> &Leapms1 {
        &self.leapms1
    }
    #[doc = "0x4c - PM Source 0 Register"]
    #[inline(always)]
    pub const fn leapms0(&self) -> &Leapms0 {
        &self.leapms0
    }
    #[doc = "0x50 - PM Command Buffer Register"]
    #[inline(always)]
    pub const fn leapmcb(&self) -> &Leapmcb {
        &self.leapmcb
    }
    #[doc = "0x70 - Interrupt Flag and Set Register"]
    #[inline(always)]
    pub const fn leaifgset(&self) -> &Leaifgset {
        &self.leaifgset
    }
    #[doc = "0x74 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn leaie(&self) -> &Leaie {
        &self.leaie
    }
    #[doc = "0x78 - Interrupt Flag and Clear Register"]
    #[inline(always)]
    pub const fn leaifg(&self) -> &Leaifg {
        &self.leaifg
    }
    #[doc = "0x7c - Interrupt Vector Register"]
    #[inline(always)]
    pub const fn leaiv(&self) -> &Leaiv {
        &self.leaiv
    }
}
#[doc = "LEACAP (rw) register accessor: LEA Capability Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leacap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leacap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leacap`] module"]
#[doc(alias = "LEACAP")]
pub type Leacap = crate::Reg<leacap::LeacapSpec>;
#[doc = "LEA Capability Register"]
pub mod leacap;
#[doc = "LEACNF0 (rw) register accessor: Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`leacnf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leacnf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leacnf0`] module"]
#[doc(alias = "LEACNF0")]
pub type Leacnf0 = crate::Reg<leacnf0::Leacnf0Spec>;
#[doc = "Configuration Register 0"]
pub mod leacnf0;
#[doc = "LEACNF1 (rw) register accessor: Configuration Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`leacnf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leacnf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leacnf1`] module"]
#[doc(alias = "LEACNF1")]
pub type Leacnf1 = crate::Reg<leacnf1::Leacnf1Spec>;
#[doc = "Configuration Register 1"]
pub mod leacnf1;
#[doc = "LEACNF2 (rw) register accessor: Configuration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`leacnf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leacnf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leacnf2`] module"]
#[doc(alias = "LEACNF2")]
pub type Leacnf2 = crate::Reg<leacnf2::Leacnf2Spec>;
#[doc = "Configuration Register 2"]
pub mod leacnf2;
#[doc = "LEAMB (rw) register accessor: Memory Bottom Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leamb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leamb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leamb`] module"]
#[doc(alias = "LEAMB")]
pub type Leamb = crate::Reg<leamb::LeambSpec>;
#[doc = "Memory Bottom Register"]
pub mod leamb;
#[doc = "LEAMT (rw) register accessor: Memory Top Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leamt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leamt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leamt`] module"]
#[doc(alias = "LEAMT")]
pub type Leamt = crate::Reg<leamt::LeamtSpec>;
#[doc = "Memory Top Register"]
pub mod leamt;
#[doc = "LEACMA (rw) register accessor: Code Memory Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leacma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leacma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leacma`] module"]
#[doc(alias = "LEACMA")]
pub type Leacma = crate::Reg<leacma::LeacmaSpec>;
#[doc = "Code Memory Access Register"]
pub mod leacma;
#[doc = "LEACMCTL (rw) register accessor: Code Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leacmctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leacmctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leacmctl`] module"]
#[doc(alias = "LEACMCTL")]
pub type Leacmctl = crate::Reg<leacmctl::LeacmctlSpec>;
#[doc = "Code Memory Control Register"]
pub mod leacmctl;
#[doc = "LEACMDSTAT (rw) register accessor: LEA Command Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leacmdstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leacmdstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leacmdstat`] module"]
#[doc(alias = "LEACMDSTAT")]
pub type Leacmdstat = crate::Reg<leacmdstat::LeacmdstatSpec>;
#[doc = "LEA Command Status Register"]
pub mod leacmdstat;
#[doc = "LEAS1STAT (rw) register accessor: LEA Source 1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leas1stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leas1stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leas1stat`] module"]
#[doc(alias = "LEAS1STAT")]
pub type Leas1stat = crate::Reg<leas1stat::Leas1statSpec>;
#[doc = "LEA Source 1 Status Register"]
pub mod leas1stat;
#[doc = "LEAS0STAT (rw) register accessor: LEA Source 0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leas0stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leas0stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leas0stat`] module"]
#[doc(alias = "LEAS0STAT")]
pub type Leas0stat = crate::Reg<leas0stat::Leas0statSpec>;
#[doc = "LEA Source 0 Status Register"]
pub mod leas0stat;
#[doc = "LEADSTSTAT (rw) register accessor: LEA Result Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leadststat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leadststat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leadststat`] module"]
#[doc(alias = "LEADSTSTAT")]
pub type Leadststat = crate::Reg<leadststat::LeadststatSpec>;
#[doc = "LEA Result Status Register"]
pub mod leadststat;
#[doc = "LEAPMCTL (rw) register accessor: PM Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leapmctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leapmctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leapmctl`] module"]
#[doc(alias = "LEAPMCTL")]
pub type Leapmctl = crate::Reg<leapmctl::LeapmctlSpec>;
#[doc = "PM Control Register"]
pub mod leapmctl;
#[doc = "LEAPMDST (rw) register accessor: PM Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leapmdst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leapmdst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leapmdst`] module"]
#[doc(alias = "LEAPMDST")]
pub type Leapmdst = crate::Reg<leapmdst::LeapmdstSpec>;
#[doc = "PM Result Register"]
pub mod leapmdst;
#[doc = "LEAPMS1 (rw) register accessor: PM Source 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leapms1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leapms1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leapms1`] module"]
#[doc(alias = "LEAPMS1")]
pub type Leapms1 = crate::Reg<leapms1::Leapms1Spec>;
#[doc = "PM Source 1 Register"]
pub mod leapms1;
#[doc = "LEAPMS0 (rw) register accessor: PM Source 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leapms0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leapms0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leapms0`] module"]
#[doc(alias = "LEAPMS0")]
pub type Leapms0 = crate::Reg<leapms0::Leapms0Spec>;
#[doc = "PM Source 0 Register"]
pub mod leapms0;
#[doc = "LEAPMCB (rw) register accessor: PM Command Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leapmcb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leapmcb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leapmcb`] module"]
#[doc(alias = "LEAPMCB")]
pub type Leapmcb = crate::Reg<leapmcb::LeapmcbSpec>;
#[doc = "PM Command Buffer Register"]
pub mod leapmcb;
#[doc = "LEAIFGSET (rw) register accessor: Interrupt Flag and Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leaifgset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leaifgset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leaifgset`] module"]
#[doc(alias = "LEAIFGSET")]
pub type Leaifgset = crate::Reg<leaifgset::LeaifgsetSpec>;
#[doc = "Interrupt Flag and Set Register"]
pub mod leaifgset;
#[doc = "LEAIE (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leaie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leaie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leaie`] module"]
#[doc(alias = "LEAIE")]
pub type Leaie = crate::Reg<leaie::LeaieSpec>;
#[doc = "Interrupt Enable Register"]
pub mod leaie;
#[doc = "LEAIFG (rw) register accessor: Interrupt Flag and Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leaifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leaifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leaifg`] module"]
#[doc(alias = "LEAIFG")]
pub type Leaifg = crate::Reg<leaifg::LeaifgSpec>;
#[doc = "Interrupt Flag and Clear Register"]
pub mod leaifg;
#[doc = "LEAIV (rw) register accessor: Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leaiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leaiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@leaiv`] module"]
#[doc(alias = "LEAIV")]
pub type Leaiv = crate::Reg<leaiv::LeaivSpec>;
#[doc = "Interrupt Vector Register"]
pub mod leaiv;
