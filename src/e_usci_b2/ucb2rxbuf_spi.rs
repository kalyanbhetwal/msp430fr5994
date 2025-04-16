#[doc = "Register `UCB2RXBUF_SPI` reader"]
pub type R = crate::R<Ucb2rxbufSpiSpec>;
#[doc = "Register `UCB2RXBUF_SPI` writer"]
pub type W = crate::W<Ucb2rxbufSpiSpec>;
#[doc = "Field `UCRXBUF` reader - Receive data buffer"]
pub type UcrxbufR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive data buffer"]
    #[inline(always)]
    pub fn ucrxbuf(&self) -> UcrxbufR {
        UcrxbufR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "eUSCI_Bx Receive Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2rxbuf_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2rxbuf_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb2rxbufSpiSpec;
impl crate::RegisterSpec for Ucb2rxbufSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb2rxbuf_spi::R`](R) reader structure"]
impl crate::Readable for Ucb2rxbufSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb2rxbuf_spi::W`](W) writer structure"]
impl crate::Writable for Ucb2rxbufSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB2RXBUF_SPI to value 0"]
impl crate::Resettable for Ucb2rxbufSpiSpec {}
