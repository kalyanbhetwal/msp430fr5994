#[doc = "Register `UCB3BRW` reader"]
pub type R = crate::R<Ucb3brwSpec>;
#[doc = "Register `UCB3BRW` writer"]
pub type W = crate::W<Ucb3brwSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "eUSCI_Bx Baud Rate Control Word Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3brw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3brw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb3brwSpec;
impl crate::RegisterSpec for Ucb3brwSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb3brw::R`](R) reader structure"]
impl crate::Readable for Ucb3brwSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb3brw::W`](W) writer structure"]
impl crate::Writable for Ucb3brwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB3BRW to value 0"]
impl crate::Resettable for Ucb3brwSpec {}
