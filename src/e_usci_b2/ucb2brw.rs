#[doc = "Register `UCB2BRW` reader"]
pub type R = crate::R<Ucb2brwSpec>;
#[doc = "Register `UCB2BRW` writer"]
pub type W = crate::W<Ucb2brwSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "eUSCI_Bx Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2brw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2brw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb2brwSpec;
impl crate::RegisterSpec for Ucb2brwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb2brw::R`](R) reader structure"]
impl crate::Readable for Ucb2brwSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb2brw::W`](W) writer structure"]
impl crate::Writable for Ucb2brwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB2BRW to value 0"]
impl crate::Resettable for Ucb2brwSpec {}
