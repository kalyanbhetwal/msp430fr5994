#[doc = "Register `LEAMT` reader"]
pub type R = crate::R<LeamtSpec>;
#[doc = "Register `LEAMT` writer"]
pub type W = crate::W<LeamtSpec>;
#[doc = "Field `LEAMT` reader - LEA memory top address boundary in byte address units"]
pub type LeamtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - LEA memory top address boundary in byte address units"]
    #[inline(always)]
    pub fn leamt(&self) -> LeamtR {
        LeamtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Memory Top Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leamt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leamt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeamtSpec;
impl crate::RegisterSpec for LeamtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leamt::R`](R) reader structure"]
impl crate::Readable for LeamtSpec {}
#[doc = "`write(|w| ..)` method takes [`leamt::W`](W) writer structure"]
impl crate::Writable for LeamtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEAMT to value 0"]
impl crate::Resettable for LeamtSpec {}
