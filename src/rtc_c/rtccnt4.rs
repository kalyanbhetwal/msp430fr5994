#[doc = "Register `RTCCNT4` reader"]
pub type R = crate::R<Rtccnt4Spec>;
#[doc = "Register `RTCCNT4` writer"]
pub type W = crate::W<Rtccnt4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The RTCCNT4 register is the count of RTCCNT4\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccnt4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccnt4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtccnt4Spec;
impl crate::RegisterSpec for Rtccnt4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rtccnt4::R`](R) reader structure"]
impl crate::Readable for Rtccnt4Spec {}
#[doc = "`write(|w| ..)` method takes [`rtccnt4::W`](W) writer structure"]
impl crate::Writable for Rtccnt4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCNT4 to value 0"]
impl crate::Resettable for Rtccnt4Spec {}
