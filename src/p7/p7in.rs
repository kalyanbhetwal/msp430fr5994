#[doc = "Register `P7IN` reader"]
pub type R = crate::R<P7inSpec>;
#[doc = "Register `P7IN` writer"]
pub type W = crate::W<P7inSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 7 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p7in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7inSpec;
impl crate::RegisterSpec for P7inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7in::R`](R) reader structure"]
impl crate::Readable for P7inSpec {}
#[doc = "`write(|w| ..)` method takes [`p7in::W`](W) writer structure"]
impl crate::Writable for P7inSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P7IN to value 0"]
impl crate::Resettable for P7inSpec {}
