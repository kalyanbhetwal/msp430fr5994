#[doc = "Register `LEAS1STAT` reader"]
pub type R = crate::R<Leas1statSpec>;
#[doc = "Register `LEAS1STAT` writer"]
pub type W = crate::W<Leas1statSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "LEA Source 1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leas1stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leas1stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Leas1statSpec;
impl crate::RegisterSpec for Leas1statSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leas1stat::R`](R) reader structure"]
impl crate::Readable for Leas1statSpec {}
#[doc = "`write(|w| ..)` method takes [`leas1stat::W`](W) writer structure"]
impl crate::Writable for Leas1statSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEAS1STAT to value 0"]
impl crate::Resettable for Leas1statSpec {}
