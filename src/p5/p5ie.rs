#[doc = "Register `P5IE` reader"]
pub type R = crate::R<P5ieSpec>;
#[doc = "Register `P5IE` writer"]
pub type W = crate::W<P5ieSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 5 Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`p5ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5ieSpec;
impl crate::RegisterSpec for P5ieSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p5ie::R`](R) reader structure"]
impl crate::Readable for P5ieSpec {}
#[doc = "`write(|w| ..)` method takes [`p5ie::W`](W) writer structure"]
impl crate::Writable for P5ieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P5IE to value 0"]
impl crate::Resettable for P5ieSpec {}
