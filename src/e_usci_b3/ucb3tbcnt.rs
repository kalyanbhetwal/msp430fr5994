#[doc = "Register `UCB3TBCNT` reader"]
pub type R = crate::R<Ucb3tbcntSpec>;
#[doc = "Register `UCB3TBCNT` writer"]
pub type W = crate::W<Ucb3tbcntSpec>;
#[doc = "Field `UCTBCNT` reader - Byte counter threshold value"]
pub type UctbcntR = crate::FieldReader;
#[doc = "Field `UCTBCNT` writer - Byte counter threshold value"]
pub type UctbcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Byte counter threshold value"]
    #[inline(always)]
    pub fn uctbcnt(&self) -> UctbcntR {
        UctbcntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Byte counter threshold value"]
    #[inline(always)]
    pub fn uctbcnt(&mut self) -> UctbcntW<Ucb3tbcntSpec> {
        UctbcntW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx Byte Counter Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3tbcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3tbcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb3tbcntSpec;
impl crate::RegisterSpec for Ucb3tbcntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb3tbcnt::R`](R) reader structure"]
impl crate::Readable for Ucb3tbcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb3tbcnt::W`](W) writer structure"]
impl crate::Writable for Ucb3tbcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB3TBCNT to value 0"]
impl crate::Resettable for Ucb3tbcntSpec {}
