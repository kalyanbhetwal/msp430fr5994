#[doc = "Register `LEAMB` reader"]
pub type R = crate::R<LeambSpec>;
#[doc = "Register `LEAMB` writer"]
pub type W = crate::W<LeambSpec>;
#[doc = "Field `LEAMB` reader - LEA memory bottom address boundary in byte address units"]
pub type LeambR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - LEA memory bottom address boundary in byte address units"]
    #[inline(always)]
    pub fn leamb(&self) -> LeambR {
        LeambR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Memory Bottom Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leamb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leamb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeambSpec;
impl crate::RegisterSpec for LeambSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leamb::R`](R) reader structure"]
impl crate::Readable for LeambSpec {}
#[doc = "`write(|w| ..)` method takes [`leamb::W`](W) writer structure"]
impl crate::Writable for LeambSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEAMB to value 0"]
impl crate::Resettable for LeambSpec {}
