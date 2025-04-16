#[doc = "Register `UCB3I2CSA` reader"]
pub type R = crate::R<Ucb3i2csaSpec>;
#[doc = "Register `UCB3I2CSA` writer"]
pub type W = crate::W<Ucb3i2csaSpec>;
#[doc = "Field `I2CSA` reader - I2C slave address"]
pub type I2csaR = crate::FieldReader<u16>;
#[doc = "Field `I2CSA` writer - I2C slave address"]
pub type I2csaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - I2C slave address"]
    #[inline(always)]
    pub fn i2csa(&self) -> I2csaR {
        I2csaR::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C slave address"]
    #[inline(always)]
    pub fn i2csa(&mut self) -> I2csaW<Ucb3i2csaSpec> {
        I2csaW::new(self, 0)
    }
}
#[doc = "eUSCI_Bx I2C Slave Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3i2csa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3i2csa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb3i2csaSpec;
impl crate::RegisterSpec for Ucb3i2csaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb3i2csa::R`](R) reader structure"]
impl crate::Readable for Ucb3i2csaSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb3i2csa::W`](W) writer structure"]
impl crate::Writable for Ucb3i2csaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB3I2CSA to value 0"]
impl crate::Resettable for Ucb3i2csaSpec {}
