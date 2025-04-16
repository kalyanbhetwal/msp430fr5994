#[doc = "Register `LEAPMS1` reader"]
pub type R = crate::R<Leapms1Spec>;
#[doc = "Register `LEAPMS1` writer"]
pub type W = crate::W<Leapms1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PM Source 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leapms1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leapms1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Leapms1Spec;
impl crate::RegisterSpec for Leapms1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leapms1::R`](R) reader structure"]
impl crate::Readable for Leapms1Spec {}
#[doc = "`write(|w| ..)` method takes [`leapms1::W`](W) writer structure"]
impl crate::Writable for Leapms1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEAPMS1 to value 0"]
impl crate::Resettable for Leapms1Spec {}
