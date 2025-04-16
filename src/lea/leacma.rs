#[doc = "Register `LEACMA` reader"]
pub type R = crate::R<LeacmaSpec>;
#[doc = "Register `LEACMA` writer"]
pub type W = crate::W<LeacmaSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Code Memory Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leacma::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leacma::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeacmaSpec;
impl crate::RegisterSpec for LeacmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leacma::R`](R) reader structure"]
impl crate::Readable for LeacmaSpec {}
#[doc = "`write(|w| ..)` method takes [`leacma::W`](W) writer structure"]
impl crate::Writable for LeacmaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEACMA to value 0"]
impl crate::Resettable for LeacmaSpec {}
