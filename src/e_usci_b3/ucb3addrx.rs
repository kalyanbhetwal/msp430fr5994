#[doc = "Register `UCB3ADDRX` reader"]
pub type R = crate::R<Ucb3addrxSpec>;
#[doc = "Register `UCB3ADDRX` writer"]
pub type W = crate::W<Ucb3addrxSpec>;
#[doc = "Field `ADDRX` reader - Received Address Register"]
pub type AddrxR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Received Address Register"]
    #[inline(always)]
    pub fn addrx(&self) -> AddrxR {
        AddrxR::new(self.bits & 0x03ff)
    }
}
impl W {}
#[doc = "eUSCI_Bx I2C Received Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3addrx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3addrx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb3addrxSpec;
impl crate::RegisterSpec for Ucb3addrxSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb3addrx::R`](R) reader structure"]
impl crate::Readable for Ucb3addrxSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb3addrx::W`](W) writer structure"]
impl crate::Writable for Ucb3addrxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB3ADDRX to value 0"]
impl crate::Resettable for Ucb3addrxSpec {}
