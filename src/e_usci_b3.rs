#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_ucb3ctlw: [u8; 0x02],
    ucb3ctlw1: Ucb3ctlw1,
    _reserved2: [u8; 0x02],
    _reserved_2_ucb3: [u8; 0x02],
    _reserved_3_ucb3: [u8; 0x02],
    ucb3tbcnt: Ucb3tbcnt,
    _reserved_5_ucb3: [u8; 0x02],
    _reserved_6_ucb3: [u8; 0x02],
    _reserved7: [u8; 0x04],
    ucb3i2coa0: Ucb3i2coa0,
    ucb3i2coa1: Ucb3i2coa1,
    ucb3i2coa2: Ucb3i2coa2,
    ucb3i2coa3: Ucb3i2coa3,
    ucb3addrx: Ucb3addrx,
    ucb3addmask: Ucb3addmask,
    ucb3i2csa: Ucb3i2csa,
    _reserved14: [u8; 0x08],
    _reserved_14_ucb3: [u8; 0x02],
    _reserved_15_ucb3: [u8; 0x02],
    _reserved_16_ucb3: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub const fn ucb3ctlw0_spi(&self) -> &Ucb3ctlw0Spi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub const fn ucb3ctlw0(&self) -> &Ucb3ctlw0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x02 - eUSCI_Bx Control Word Register 1"]
    #[inline(always)]
    pub const fn ucb3ctlw1(&self) -> &Ucb3ctlw1 {
        &self.ucb3ctlw1
    }
    #[doc = "0x06 - eUSCI_Bx Bit Rate Control Register 1"]
    #[inline(always)]
    pub const fn ucb3brw_spi(&self) -> &Ucb3brwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub const fn ucb3brw(&self) -> &Ucb3brw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x08 - UCB3STATW_SPI"]
    #[inline(always)]
    pub const fn ucb3statw_spi(&self) -> &Ucb3statwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub const fn ucb3statw(&self) -> &Ucb3statw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0a - eUSCI_Bx Byte Counter Threshold Register"]
    #[inline(always)]
    pub const fn ucb3tbcnt(&self) -> &Ucb3tbcnt {
        &self.ucb3tbcnt
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub const fn ucb3rxbuf_spi(&self) -> &Ucb3rxbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub const fn ucb3rxbuf(&self) -> &Ucb3rxbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub const fn ucb3txbuf_spi(&self) -> &Ucb3txbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub const fn ucb3txbuf(&self) -> &Ucb3txbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x14 - eUSCI_Bx I2C Own Address 0 Register"]
    #[inline(always)]
    pub const fn ucb3i2coa0(&self) -> &Ucb3i2coa0 {
        &self.ucb3i2coa0
    }
    #[doc = "0x16 - eUSCI_Bx I2C Own Address 1 Register"]
    #[inline(always)]
    pub const fn ucb3i2coa1(&self) -> &Ucb3i2coa1 {
        &self.ucb3i2coa1
    }
    #[doc = "0x18 - eUSCI_Bx I2C Own Address 2 Register"]
    #[inline(always)]
    pub const fn ucb3i2coa2(&self) -> &Ucb3i2coa2 {
        &self.ucb3i2coa2
    }
    #[doc = "0x1a - eUSCI_Bx I2C Own Address 3 Register"]
    #[inline(always)]
    pub const fn ucb3i2coa3(&self) -> &Ucb3i2coa3 {
        &self.ucb3i2coa3
    }
    #[doc = "0x1c - eUSCI_Bx I2C Received Address Register"]
    #[inline(always)]
    pub const fn ucb3addrx(&self) -> &Ucb3addrx {
        &self.ucb3addrx
    }
    #[doc = "0x1e - eUSCI_Bx I2C Address Mask Register"]
    #[inline(always)]
    pub const fn ucb3addmask(&self) -> &Ucb3addmask {
        &self.ucb3addmask
    }
    #[doc = "0x20 - eUSCI_Bx I2C Slave Address Register"]
    #[inline(always)]
    pub const fn ucb3i2csa(&self) -> &Ucb3i2csa {
        &self.ucb3i2csa
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb3ie_spi(&self) -> &Ucb3ieSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb3ie(&self) -> &Ucb3ie {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub const fn ucb3ifg_spi(&self) -> &Ucb3ifgSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub const fn ucb3ifg(&self) -> &Ucb3ifg {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb3iv_spi(&self) -> &Ucb3ivSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(46).cast() }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb3iv(&self) -> &Ucb3iv {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(46).cast() }
    }
}
#[doc = "UCB3CTLW0 (rw) register accessor: eUSCI_Bx Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3ctlw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3ctlw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3ctlw0`] module"]
#[doc(alias = "UCB3CTLW0")]
pub type Ucb3ctlw0 = crate::Reg<ucb3ctlw0::Ucb3ctlw0Spec>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb3ctlw0;
#[doc = "UCB3CTLW0_SPI (rw) register accessor: eUSCI_Bx Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3ctlw0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3ctlw0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3ctlw0_spi`] module"]
#[doc(alias = "UCB3CTLW0_SPI")]
pub type Ucb3ctlw0Spi = crate::Reg<ucb3ctlw0_spi::Ucb3ctlw0SpiSpec>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb3ctlw0_spi;
#[doc = "UCB3CTLW1 (rw) register accessor: eUSCI_Bx Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3ctlw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3ctlw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3ctlw1`] module"]
#[doc(alias = "UCB3CTLW1")]
pub type Ucb3ctlw1 = crate::Reg<ucb3ctlw1::Ucb3ctlw1Spec>;
#[doc = "eUSCI_Bx Control Word Register 1"]
pub mod ucb3ctlw1;
#[doc = "UCB3BRW (rw) register accessor: eUSCI_Bx Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3brw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3brw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3brw`] module"]
#[doc(alias = "UCB3BRW")]
pub type Ucb3brw = crate::Reg<ucb3brw::Ucb3brwSpec>;
#[doc = "eUSCI_Bx Baud Rate Control Word Register"]
pub mod ucb3brw;
#[doc = "UCB3BRW_SPI (rw) register accessor: eUSCI_Bx Bit Rate Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3brw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3brw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3brw_spi`] module"]
#[doc(alias = "UCB3BRW_SPI")]
pub type Ucb3brwSpi = crate::Reg<ucb3brw_spi::Ucb3brwSpiSpec>;
#[doc = "eUSCI_Bx Bit Rate Control Register 1"]
pub mod ucb3brw_spi;
#[doc = "UCB3STATW (rw) register accessor: eUSCI_Bx Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3statw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3statw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3statw`] module"]
#[doc(alias = "UCB3STATW")]
pub type Ucb3statw = crate::Reg<ucb3statw::Ucb3statwSpec>;
#[doc = "eUSCI_Bx Status Register"]
pub mod ucb3statw;
#[doc = "UCB3STATW_SPI (rw) register accessor: UCB3STATW_SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3statw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3statw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3statw_spi`] module"]
#[doc(alias = "UCB3STATW_SPI")]
pub type Ucb3statwSpi = crate::Reg<ucb3statw_spi::Ucb3statwSpiSpec>;
#[doc = "UCB3STATW_SPI"]
pub mod ucb3statw_spi;
#[doc = "UCB3TBCNT (rw) register accessor: eUSCI_Bx Byte Counter Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3tbcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3tbcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3tbcnt`] module"]
#[doc(alias = "UCB3TBCNT")]
pub type Ucb3tbcnt = crate::Reg<ucb3tbcnt::Ucb3tbcntSpec>;
#[doc = "eUSCI_Bx Byte Counter Threshold Register"]
pub mod ucb3tbcnt;
#[doc = "UCB3RXBUF (rw) register accessor: eUSCI_Bx Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3rxbuf`] module"]
#[doc(alias = "UCB3RXBUF")]
pub type Ucb3rxbuf = crate::Reg<ucb3rxbuf::Ucb3rxbufSpec>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb3rxbuf;
#[doc = "UCB3RXBUF_SPI (rw) register accessor: eUSCI_Bx Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3rxbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3rxbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3rxbuf_spi`] module"]
#[doc(alias = "UCB3RXBUF_SPI")]
pub type Ucb3rxbufSpi = crate::Reg<ucb3rxbuf_spi::Ucb3rxbufSpiSpec>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb3rxbuf_spi;
#[doc = "UCB3TXBUF (rw) register accessor: eUSCI_Bx Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3txbuf`] module"]
#[doc(alias = "UCB3TXBUF")]
pub type Ucb3txbuf = crate::Reg<ucb3txbuf::Ucb3txbufSpec>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb3txbuf;
#[doc = "UCB3TXBUF_SPI (rw) register accessor: eUSCI_Bx Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3txbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3txbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3txbuf_spi`] module"]
#[doc(alias = "UCB3TXBUF_SPI")]
pub type Ucb3txbufSpi = crate::Reg<ucb3txbuf_spi::Ucb3txbufSpiSpec>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb3txbuf_spi;
#[doc = "UCB3I2COA0 (rw) register accessor: eUSCI_Bx I2C Own Address 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3i2coa0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3i2coa0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3i2coa0`] module"]
#[doc(alias = "UCB3I2COA0")]
pub type Ucb3i2coa0 = crate::Reg<ucb3i2coa0::Ucb3i2coa0Spec>;
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucb3i2coa0;
#[doc = "UCB3I2COA1 (rw) register accessor: eUSCI_Bx I2C Own Address 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3i2coa1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3i2coa1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3i2coa1`] module"]
#[doc(alias = "UCB3I2COA1")]
pub type Ucb3i2coa1 = crate::Reg<ucb3i2coa1::Ucb3i2coa1Spec>;
#[doc = "eUSCI_Bx I2C Own Address 1 Register"]
pub mod ucb3i2coa1;
#[doc = "UCB3I2COA2 (rw) register accessor: eUSCI_Bx I2C Own Address 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3i2coa2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3i2coa2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3i2coa2`] module"]
#[doc(alias = "UCB3I2COA2")]
pub type Ucb3i2coa2 = crate::Reg<ucb3i2coa2::Ucb3i2coa2Spec>;
#[doc = "eUSCI_Bx I2C Own Address 2 Register"]
pub mod ucb3i2coa2;
#[doc = "UCB3I2COA3 (rw) register accessor: eUSCI_Bx I2C Own Address 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3i2coa3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3i2coa3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3i2coa3`] module"]
#[doc(alias = "UCB3I2COA3")]
pub type Ucb3i2coa3 = crate::Reg<ucb3i2coa3::Ucb3i2coa3Spec>;
#[doc = "eUSCI_Bx I2C Own Address 3 Register"]
pub mod ucb3i2coa3;
#[doc = "UCB3ADDRX (rw) register accessor: eUSCI_Bx I2C Received Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3addrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3addrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3addrx`] module"]
#[doc(alias = "UCB3ADDRX")]
pub type Ucb3addrx = crate::Reg<ucb3addrx::Ucb3addrxSpec>;
#[doc = "eUSCI_Bx I2C Received Address Register"]
pub mod ucb3addrx;
#[doc = "UCB3ADDMASK (rw) register accessor: eUSCI_Bx I2C Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3addmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3addmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3addmask`] module"]
#[doc(alias = "UCB3ADDMASK")]
pub type Ucb3addmask = crate::Reg<ucb3addmask::Ucb3addmaskSpec>;
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucb3addmask;
#[doc = "UCB3I2CSA (rw) register accessor: eUSCI_Bx I2C Slave Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3i2csa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3i2csa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3i2csa`] module"]
#[doc(alias = "UCB3I2CSA")]
pub type Ucb3i2csa = crate::Reg<ucb3i2csa::Ucb3i2csaSpec>;
#[doc = "eUSCI_Bx I2C Slave Address Register"]
pub mod ucb3i2csa;
#[doc = "UCB3IE (rw) register accessor: eUSCI_Bx Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3ie`] module"]
#[doc(alias = "UCB3IE")]
pub type Ucb3ie = crate::Reg<ucb3ie::Ucb3ieSpec>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb3ie;
#[doc = "UCB3IE_SPI (rw) register accessor: eUSCI_Bx Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3ie_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3ie_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3ie_spi`] module"]
#[doc(alias = "UCB3IE_SPI")]
pub type Ucb3ieSpi = crate::Reg<ucb3ie_spi::Ucb3ieSpiSpec>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb3ie_spi;
#[doc = "UCB3IFG (rw) register accessor: eUSCI_Bx Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3ifg`] module"]
#[doc(alias = "UCB3IFG")]
pub type Ucb3ifg = crate::Reg<ucb3ifg::Ucb3ifgSpec>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb3ifg;
#[doc = "UCB3IFG_SPI (rw) register accessor: eUSCI_Bx Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3ifg_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3ifg_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3ifg_spi`] module"]
#[doc(alias = "UCB3IFG_SPI")]
pub type Ucb3ifgSpi = crate::Reg<ucb3ifg_spi::Ucb3ifgSpiSpec>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb3ifg_spi;
#[doc = "UCB3IV (rw) register accessor: eUSCI_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3iv`] module"]
#[doc(alias = "UCB3IV")]
pub type Ucb3iv = crate::Reg<ucb3iv::Ucb3ivSpec>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb3iv;
#[doc = "UCB3IV_SPI (rw) register accessor: eUSCI_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3iv_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3iv_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb3iv_spi`] module"]
#[doc(alias = "UCB3IV_SPI")]
pub type Ucb3ivSpi = crate::Reg<ucb3iv_spi::Ucb3ivSpiSpec>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb3iv_spi;
