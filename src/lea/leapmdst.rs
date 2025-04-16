#[doc = "Register `LEAPMDST` reader"]
pub type R = crate::R<LeapmdstSpec>;
#[doc = "Register `LEAPMDST` writer"]
pub type W = crate::W<LeapmdstSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PM Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leapmdst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leapmdst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeapmdstSpec;
impl crate::RegisterSpec for LeapmdstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leapmdst::R`](R) reader structure"]
impl crate::Readable for LeapmdstSpec {}
#[doc = "`write(|w| ..)` method takes [`leapmdst::W`](W) writer structure"]
impl crate::Writable for LeapmdstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEAPMDST to value 0"]
impl crate::Resettable for LeapmdstSpec {}
