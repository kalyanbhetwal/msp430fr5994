#[doc = "Register `P1IFG` reader"]
pub type R = crate::R<P1ifgSpec>;
#[doc = "Register `P1IFG` writer"]
pub type W = crate::W<P1ifgSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port 1 Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`p1ifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1ifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1ifgSpec;
impl crate::RegisterSpec for P1ifgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p1ifg::R`](R) reader structure"]
impl crate::Readable for P1ifgSpec {}
#[doc = "`write(|w| ..)` method takes [`p1ifg::W`](W) writer structure"]
impl crate::Writable for P1ifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P1IFG to value 0"]
impl crate::Resettable for P1ifgSpec {}
