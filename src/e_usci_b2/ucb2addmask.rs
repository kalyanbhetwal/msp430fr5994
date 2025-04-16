#[doc = "Register `UCB2ADDMASK` reader"]
pub type R = crate::R<Ucb2addmaskSpec>;
#[doc = "Register `UCB2ADDMASK` writer"]
pub type W = crate::W<Ucb2addmaskSpec>;
#[doc = "Field `ADDMASK` reader - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
pub type AddmaskR = crate::FieldReader<u16>;
#[doc = "Field `ADDMASK` writer - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
pub type AddmaskW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
    #[inline(always)]
    pub fn addmask(&self) -> AddmaskR {
        AddmaskR::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
    #[inline(always)]
    pub fn addmask(&mut self) -> AddmaskW<Ucb2addmaskSpec> {
        AddmaskW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx I2C Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb2addmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb2addmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb2addmaskSpec;
impl crate::RegisterSpec for Ucb2addmaskSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb2addmask::R`](R) reader structure"]
impl crate::Readable for Ucb2addmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb2addmask::W`](W) writer structure"]
impl crate::Writable for Ucb2addmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB2ADDMASK to value 0"]
impl crate::Resettable for Ucb2addmaskSpec {}
