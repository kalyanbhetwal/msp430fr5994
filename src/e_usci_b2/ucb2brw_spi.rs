#[doc = "Register `UCB2BRW_SPI` reader"]
pub type R = crate::R<Ucb2brwSpiSpec>;
#[doc = "Register `UCB2BRW_SPI` writer"]
pub type W = crate::W<Ucb2brwSpiSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "eUSCI_Bx Bit Rate Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2brw_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2brw_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb2brwSpiSpec;
impl crate::RegisterSpec for Ucb2brwSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb2brw_spi::R`](R) reader structure"]
impl crate::Readable for Ucb2brwSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb2brw_spi::W`](W) writer structure"]
impl crate::Writable for Ucb2brwSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB2BRW_SPI to value 0"]
impl crate::Resettable for Ucb2brwSpiSpec {}
