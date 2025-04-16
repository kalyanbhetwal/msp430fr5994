#[doc = "Register `LEADSTSTAT` reader"]
pub type R = crate::R<LeadststatSpec>;
#[doc = "Register `LEADSTSTAT` writer"]
pub type W = crate::W<LeadststatSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LEA Result Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leadststat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leadststat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeadststatSpec;
impl crate::RegisterSpec for LeadststatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leadststat::R`](R) reader structure"]
impl crate::Readable for LeadststatSpec {}
#[doc = "`write(|w| ..)` method takes [`leadststat::W`](W) writer structure"]
impl crate::Writable for LeadststatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEADSTSTAT to value 0"]
impl crate::Resettable for LeadststatSpec {}
