#[doc = "Register `UCB3TXBUF` reader"]
pub type R = crate::R<Ucb3txbufSpec>;
#[doc = "Register `UCB3TXBUF` writer"]
pub type W = crate::W<Ucb3txbufSpec>;
#[doc = "Field `UCTXBUF` reader - Transmit data buffer"]
pub type UctxbufR = crate::FieldReader;
#[doc = "Field `UCTXBUF` writer - Transmit data buffer"]
pub type UctxbufW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&self) -> UctxbufR {
        UctxbufR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&mut self) -> UctxbufW<Ucb3txbufSpec> {
        UctxbufW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx Transmit Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3txbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3txbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb3txbufSpec;
impl crate::RegisterSpec for Ucb3txbufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb3txbuf::R`](R) reader structure"]
impl crate::Readable for Ucb3txbufSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb3txbuf::W`](W) writer structure"]
impl crate::Writable for Ucb3txbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB3TXBUF to value 0"]
impl crate::Resettable for Ucb3txbufSpec {}
