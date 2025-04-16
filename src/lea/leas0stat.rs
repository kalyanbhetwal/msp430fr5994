#[doc = "Register `LEAS0STAT` reader"]
pub type R = crate::R<Leas0statSpec>;
#[doc = "Register `LEAS0STAT` writer"]
pub type W = crate::W<Leas0statSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LEA Source 0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leas0stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leas0stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Leas0statSpec;
impl crate::RegisterSpec for Leas0statSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leas0stat::R`](R) reader structure"]
impl crate::Readable for Leas0statSpec {}
#[doc = "`write(|w| ..)` method takes [`leas0stat::W`](W) writer structure"]
impl crate::Writable for Leas0statSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEAS0STAT to value 0"]
impl crate::Resettable for Leas0statSpec {}
