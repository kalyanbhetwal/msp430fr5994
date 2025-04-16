#[doc = "Register `LEAPMS0` reader"]
pub type R = crate::R<Leapms0Spec>;
#[doc = "Register `LEAPMS0` writer"]
pub type W = crate::W<Leapms0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PM Source 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leapms0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leapms0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Leapms0Spec;
impl crate::RegisterSpec for Leapms0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leapms0::R`](R) reader structure"]
impl crate::Readable for Leapms0Spec {}
#[doc = "`write(|w| ..)` method takes [`leapms0::W`](W) writer structure"]
impl crate::Writable for Leapms0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEAPMS0 to value 0"]
impl crate::Resettable for Leapms0Spec {}
