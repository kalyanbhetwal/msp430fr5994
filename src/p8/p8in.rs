#[doc = "Register `P8IN` reader"]
pub type R = crate::R<P8inSpec>;
#[doc = "Register `P8IN` writer"]
pub type W = crate::W<P8inSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 8 Input\n\nYou can [`read`](crate::Reg::read) this register and get [`p8in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p8in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P8inSpec;
impl crate::RegisterSpec for P8inSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p8in::R`](R) reader structure"]
impl crate::Readable for P8inSpec {}
#[doc = "`write(|w| ..)` method takes [`p8in::W`](W) writer structure"]
impl crate::Writable for P8inSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P8IN to value 0"]
impl crate::Resettable for P8inSpec {}
