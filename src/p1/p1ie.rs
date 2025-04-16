#[doc = "Register `P1IE` reader"]
pub type R = crate::R<P1ieSpec>;
#[doc = "Register `P1IE` writer"]
pub type W = crate::W<P1ieSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 1 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1ieSpec;
impl crate::RegisterSpec for P1ieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1ie::R`](R) reader structure"]
impl crate::Readable for P1ieSpec {}
#[doc = "`write(|w| ..)` method takes [`p1ie::W`](W) writer structure"]
impl crate::Writable for P1ieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P1IE to value 0"]
impl crate::Resettable for P1ieSpec {}
