#[doc = "Register `P7IES` reader"]
pub type R = crate::R<P7iesSpec>;
#[doc = "Register `P7IES` writer"]
pub type W = crate::W<P7iesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 7 Interrupt Edge Select\n\nYou can [`read`](crate::Reg::read) this register and get [`p7ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P7iesSpec;
impl crate::RegisterSpec for P7iesSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p7ies::R`](R) reader structure"]
impl crate::Readable for P7iesSpec {}
#[doc = "`write(|w| ..)` method takes [`p7ies::W`](W) writer structure"]
impl crate::Writable for P7iesSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P7IES to value 0"]
impl crate::Resettable for P7iesSpec {}
