#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_ucb2ctlw: [u8; 0x02],
    ucb2ctlw1: Ucb2ctlw1,
    _reserved2: [u8; 0x02],
    _reserved_2_ucb2: [u8; 0x02],
    _reserved_3_ucb2: [u8; 0x02],
    ucb2tbcnt: Ucb2tbcnt,
    _reserved_5_ucb2: [u8; 0x02],
    _reserved_6_ucb2: [u8; 0x02],
    _reserved7: [u8; 0x04],
    ucb2i2coa0: Ucb2i2coa0,
    ucb2i2coa1: Ucb2i2coa1,
    ucb2i2coa2: Ucb2i2coa2,
    ucb2i2coa3: Ucb2i2coa3,
    ucb2addrx: Ucb2addrx,
    ucb2addmask: Ucb2addmask,
    ucb2i2csa: Ucb2i2csa,
    _reserved14: [u8; 0x08],
    _reserved_14_ucb2: [u8; 0x02],
    _reserved_15_ucb2: [u8; 0x02],
    _reserved_16_ucb2: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub const fn ucb2ctlw0_spi(&self) -> &Ucb2ctlw0Spi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    #[inline(always)]
    pub const fn ucb2ctlw0(&self) -> &Ucb2ctlw0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x02 - eUSCI_Bx Control Word Register 1"]
    #[inline(always)]
    pub const fn ucb2ctlw1(&self) -> &Ucb2ctlw1 {
        &self.ucb2ctlw1
    }
    #[doc = "0x06 - eUSCI_Bx Bit Rate Control Register 1"]
    #[inline(always)]
    pub const fn ucb2brw_spi(&self) -> &Ucb2brwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    #[inline(always)]
    pub const fn ucb2brw(&self) -> &Ucb2brw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    #[doc = "0x08 - UCB2STATW_SPI"]
    #[inline(always)]
    pub const fn ucb2statw_spi(&self) -> &Ucb2statwSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    #[inline(always)]
    pub const fn ucb2statw(&self) -> &Ucb2statw {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0a - eUSCI_Bx Byte Counter Threshold Register"]
    #[inline(always)]
    pub const fn ucb2tbcnt(&self) -> &Ucb2tbcnt {
        &self.ucb2tbcnt
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub const fn ucb2rxbuf_spi(&self) -> &Ucb2rxbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    #[inline(always)]
    pub const fn ucb2rxbuf(&self) -> &Ucb2rxbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub const fn ucb2txbuf_spi(&self) -> &Ucb2txbufSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    #[inline(always)]
    pub const fn ucb2txbuf(&self) -> &Ucb2txbuf {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    #[doc = "0x14 - eUSCI_Bx I2C Own Address 0 Register"]
    #[inline(always)]
    pub const fn ucb2i2coa0(&self) -> &Ucb2i2coa0 {
        &self.ucb2i2coa0
    }
    #[doc = "0x16 - eUSCI_Bx I2C Own Address 1 Register"]
    #[inline(always)]
    pub const fn ucb2i2coa1(&self) -> &Ucb2i2coa1 {
        &self.ucb2i2coa1
    }
    #[doc = "0x18 - eUSCI_Bx I2C Own Address 2 Register"]
    #[inline(always)]
    pub const fn ucb2i2coa2(&self) -> &Ucb2i2coa2 {
        &self.ucb2i2coa2
    }
    #[doc = "0x1a - eUSCI_Bx I2C Own Address 3 Register"]
    #[inline(always)]
    pub const fn ucb2i2coa3(&self) -> &Ucb2i2coa3 {
        &self.ucb2i2coa3
    }
    #[doc = "0x1c - eUSCI_Bx I2C Received Address Register"]
    #[inline(always)]
    pub const fn ucb2addrx(&self) -> &Ucb2addrx {
        &self.ucb2addrx
    }
    #[doc = "0x1e - eUSCI_Bx I2C Address Mask Register"]
    #[inline(always)]
    pub const fn ucb2addmask(&self) -> &Ucb2addmask {
        &self.ucb2addmask
    }
    #[doc = "0x20 - eUSCI_Bx I2C Slave Address Register"]
    #[inline(always)]
    pub const fn ucb2i2csa(&self) -> &Ucb2i2csa {
        &self.ucb2i2csa
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb2ie_spi(&self) -> &Ucb2ieSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ucb2ie(&self) -> &Ucb2ie {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub const fn ucb2ifg_spi(&self) -> &Ucb2ifgSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    #[inline(always)]
    pub const fn ucb2ifg(&self) -> &Ucb2ifg {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb2iv_spi(&self) -> &Ucb2ivSpi {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(46).cast() }
    }
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    #[inline(always)]
    pub const fn ucb2iv(&self) -> &Ucb2iv {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(46).cast() }
    }
}
#[doc = "UCB2CTLW0 (rw) register accessor: eUSCI_Bx Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2ctlw0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2ctlw0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2ctlw0`] module"]
#[doc(alias = "UCB2CTLW0")]
pub type Ucb2ctlw0 = crate::Reg<ucb2ctlw0::Ucb2ctlw0Spec>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb2ctlw0;
#[doc = "UCB2CTLW0_SPI (rw) register accessor: eUSCI_Bx Control Word Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2ctlw0_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2ctlw0_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2ctlw0_spi`] module"]
#[doc(alias = "UCB2CTLW0_SPI")]
pub type Ucb2ctlw0Spi = crate::Reg<ucb2ctlw0_spi::Ucb2ctlw0SpiSpec>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucb2ctlw0_spi;
#[doc = "UCB2CTLW1 (rw) register accessor: eUSCI_Bx Control Word Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2ctlw1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2ctlw1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2ctlw1`] module"]
#[doc(alias = "UCB2CTLW1")]
pub type Ucb2ctlw1 = crate::Reg<ucb2ctlw1::Ucb2ctlw1Spec>;
#[doc = "eUSCI_Bx Control Word Register 1"]
pub mod ucb2ctlw1;
#[doc = "UCB2BRW (rw) register accessor: eUSCI_Bx Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2brw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2brw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2brw`] module"]
#[doc(alias = "UCB2BRW")]
pub type Ucb2brw = crate::Reg<ucb2brw::Ucb2brwSpec>;
#[doc = "eUSCI_Bx Baud Rate Control Word Register"]
pub mod ucb2brw;
#[doc = "UCB2BRW_SPI (rw) register accessor: eUSCI_Bx Bit Rate Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2brw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2brw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2brw_spi`] module"]
#[doc(alias = "UCB2BRW_SPI")]
pub type Ucb2brwSpi = crate::Reg<ucb2brw_spi::Ucb2brwSpiSpec>;
#[doc = "eUSCI_Bx Bit Rate Control Register 1"]
pub mod ucb2brw_spi;
#[doc = "UCB2STATW (rw) register accessor: eUSCI_Bx Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2statw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2statw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2statw`] module"]
#[doc(alias = "UCB2STATW")]
pub type Ucb2statw = crate::Reg<ucb2statw::Ucb2statwSpec>;
#[doc = "eUSCI_Bx Status Register"]
pub mod ucb2statw;
#[doc = "UCB2STATW_SPI (rw) register accessor: UCB2STATW_SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2statw_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2statw_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2statw_spi`] module"]
#[doc(alias = "UCB2STATW_SPI")]
pub type Ucb2statwSpi = crate::Reg<ucb2statw_spi::Ucb2statwSpiSpec>;
#[doc = "UCB2STATW_SPI"]
pub mod ucb2statw_spi;
#[doc = "UCB2TBCNT (rw) register accessor: eUSCI_Bx Byte Counter Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2tbcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2tbcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2tbcnt`] module"]
#[doc(alias = "UCB2TBCNT")]
pub type Ucb2tbcnt = crate::Reg<ucb2tbcnt::Ucb2tbcntSpec>;
#[doc = "eUSCI_Bx Byte Counter Threshold Register"]
pub mod ucb2tbcnt;
#[doc = "UCB2RXBUF (rw) register accessor: eUSCI_Bx Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2rxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2rxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2rxbuf`] module"]
#[doc(alias = "UCB2RXBUF")]
pub type Ucb2rxbuf = crate::Reg<ucb2rxbuf::Ucb2rxbufSpec>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb2rxbuf;
#[doc = "UCB2RXBUF_SPI (rw) register accessor: eUSCI_Bx Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2rxbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2rxbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2rxbuf_spi`] module"]
#[doc(alias = "UCB2RXBUF_SPI")]
pub type Ucb2rxbufSpi = crate::Reg<ucb2rxbuf_spi::Ucb2rxbufSpiSpec>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucb2rxbuf_spi;
#[doc = "UCB2TXBUF (rw) register accessor: eUSCI_Bx Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2txbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2txbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2txbuf`] module"]
#[doc(alias = "UCB2TXBUF")]
pub type Ucb2txbuf = crate::Reg<ucb2txbuf::Ucb2txbufSpec>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb2txbuf;
#[doc = "UCB2TXBUF_SPI (rw) register accessor: eUSCI_Bx Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2txbuf_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2txbuf_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2txbuf_spi`] module"]
#[doc(alias = "UCB2TXBUF_SPI")]
pub type Ucb2txbufSpi = crate::Reg<ucb2txbuf_spi::Ucb2txbufSpiSpec>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucb2txbuf_spi;
#[doc = "UCB2I2COA0 (rw) register accessor: eUSCI_Bx I2C Own Address 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2i2coa0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2i2coa0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2i2coa0`] module"]
#[doc(alias = "UCB2I2COA0")]
pub type Ucb2i2coa0 = crate::Reg<ucb2i2coa0::Ucb2i2coa0Spec>;
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucb2i2coa0;
#[doc = "UCB2I2COA1 (rw) register accessor: eUSCI_Bx I2C Own Address 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2i2coa1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2i2coa1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2i2coa1`] module"]
#[doc(alias = "UCB2I2COA1")]
pub type Ucb2i2coa1 = crate::Reg<ucb2i2coa1::Ucb2i2coa1Spec>;
#[doc = "eUSCI_Bx I2C Own Address 1 Register"]
pub mod ucb2i2coa1;
#[doc = "UCB2I2COA2 (rw) register accessor: eUSCI_Bx I2C Own Address 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2i2coa2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2i2coa2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2i2coa2`] module"]
#[doc(alias = "UCB2I2COA2")]
pub type Ucb2i2coa2 = crate::Reg<ucb2i2coa2::Ucb2i2coa2Spec>;
#[doc = "eUSCI_Bx I2C Own Address 2 Register"]
pub mod ucb2i2coa2;
#[doc = "UCB2I2COA3 (rw) register accessor: eUSCI_Bx I2C Own Address 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2i2coa3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2i2coa3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2i2coa3`] module"]
#[doc(alias = "UCB2I2COA3")]
pub type Ucb2i2coa3 = crate::Reg<ucb2i2coa3::Ucb2i2coa3Spec>;
#[doc = "eUSCI_Bx I2C Own Address 3 Register"]
pub mod ucb2i2coa3;
#[doc = "UCB2ADDRX (rw) register accessor: eUSCI_Bx I2C Received Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2addrx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2addrx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2addrx`] module"]
#[doc(alias = "UCB2ADDRX")]
pub type Ucb2addrx = crate::Reg<ucb2addrx::Ucb2addrxSpec>;
#[doc = "eUSCI_Bx I2C Received Address Register"]
pub mod ucb2addrx;
#[doc = "UCB2ADDMASK (rw) register accessor: eUSCI_Bx I2C Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2addmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2addmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2addmask`] module"]
#[doc(alias = "UCB2ADDMASK")]
pub type Ucb2addmask = crate::Reg<ucb2addmask::Ucb2addmaskSpec>;
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucb2addmask;
#[doc = "UCB2I2CSA (rw) register accessor: eUSCI_Bx I2C Slave Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2i2csa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2i2csa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2i2csa`] module"]
#[doc(alias = "UCB2I2CSA")]
pub type Ucb2i2csa = crate::Reg<ucb2i2csa::Ucb2i2csaSpec>;
#[doc = "eUSCI_Bx I2C Slave Address Register"]
pub mod ucb2i2csa;
#[doc = "UCB2IE (rw) register accessor: eUSCI_Bx Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2ie`] module"]
#[doc(alias = "UCB2IE")]
pub type Ucb2ie = crate::Reg<ucb2ie::Ucb2ieSpec>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb2ie;
#[doc = "UCB2IE_SPI (rw) register accessor: eUSCI_Bx Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2ie_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2ie_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2ie_spi`] module"]
#[doc(alias = "UCB2IE_SPI")]
pub type Ucb2ieSpi = crate::Reg<ucb2ie_spi::Ucb2ieSpiSpec>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucb2ie_spi;
#[doc = "UCB2IFG (rw) register accessor: eUSCI_Bx Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2ifg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2ifg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2ifg`] module"]
#[doc(alias = "UCB2IFG")]
pub type Ucb2ifg = crate::Reg<ucb2ifg::Ucb2ifgSpec>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb2ifg;
#[doc = "UCB2IFG_SPI (rw) register accessor: eUSCI_Bx Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2ifg_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2ifg_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2ifg_spi`] module"]
#[doc(alias = "UCB2IFG_SPI")]
pub type Ucb2ifgSpi = crate::Reg<ucb2ifg_spi::Ucb2ifgSpiSpec>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucb2ifg_spi;
#[doc = "UCB2IV (rw) register accessor: eUSCI_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2iv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2iv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2iv`] module"]
#[doc(alias = "UCB2IV")]
pub type Ucb2iv = crate::Reg<ucb2iv::Ucb2ivSpec>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb2iv;
#[doc = "UCB2IV_SPI (rw) register accessor: eUSCI_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2iv_spi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2iv_spi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucb2iv_spi`] module"]
#[doc(alias = "UCB2IV_SPI")]
pub type Ucb2ivSpi = crate::Reg<ucb2iv_spi::Ucb2ivSpiSpec>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucb2iv_spi;
