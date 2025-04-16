#[doc = "Register `LEACNF0` reader"]
pub type R = crate::R<Leacnf0Spec>;
#[doc = "Register `LEACNF0` writer"]
pub type W = crate::W<Leacnf0Spec>;
#[doc = "Field `LEASWRST` reader - LEA module software restart. Setting this bit to one restarts the LEA module. As long this bit remains set to one the LEA is held in Restart. (The LEA accessible memory behaves as system RAM)"]
pub type LeaswrstR = crate::BitReader;
#[doc = "Field `LEASWRST` writer - LEA module software restart. Setting this bit to one restarts the LEA module. As long this bit remains set to one the LEA is held in Restart. (The LEA accessible memory behaves as system RAM)"]
pub type LeaswrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Hold on faults and NMIs for all pending LEA operations transfers.This is for all system wide fault/NMI cases (for our first implementation we may just consider local LEA triggered fault cases)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leafthold {
    #[doc = "0: LEA transfers continue on faults/NMIs"]
    Leafthold0 = 0,
    #[doc = "1: LEA transfers enter HOLD on faults/NMIs"]
    Leafthold1 = 1,
}
impl From<Leafthold> for bool {
    #[inline(always)]
    fn from(variant: Leafthold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEAFTHOLD` reader - Hold on faults and NMIs for all pending LEA operations transfers.This is for all system wide fault/NMI cases (for our first implementation we may just consider local LEA triggered fault cases)"]
pub type LeaftholdR = crate::BitReader<Leafthold>;
impl LeaftholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leafthold {
        match self.bits {
            false => Leafthold::Leafthold0,
            true => Leafthold::Leafthold1,
        }
    }
    #[doc = "LEA transfers continue on faults/NMIs"]
    #[inline(always)]
    pub fn is_leafthold_0(&self) -> bool {
        *self == Leafthold::Leafthold0
    }
    #[doc = "LEA transfers enter HOLD on faults/NMIs"]
    #[inline(always)]
    pub fn is_leafthold_1(&self) -> bool {
        *self == Leafthold::Leafthold1
    }
}
#[doc = "Field `LEAFTHOLD` writer - Hold on faults and NMIs for all pending LEA operations transfers.This is for all system wide fault/NMI cases (for our first implementation we may just consider local LEA triggered fault cases)"]
pub type LeaftholdW<'a, REG> = crate::BitWriter<'a, REG, Leafthold>;
impl<'a, REG> LeaftholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LEA transfers continue on faults/NMIs"]
    #[inline(always)]
    pub fn leafthold_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leafthold::Leafthold0)
    }
    #[doc = "LEA transfers enter HOLD on faults/NMIs"]
    #[inline(always)]
    pub fn leafthold_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leafthold::Leafthold1)
    }
}
#[doc = "This bit defined if command execution shall be continued in LPM modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lealpr {
    #[doc = "0: LEA command execution stops in deep low power modes"]
    Lealpr0 = 0,
    #[doc = "1: LEA command execution continues in deep low power modes"]
    Lealpr1 = 1,
}
impl From<Lealpr> for bool {
    #[inline(always)]
    fn from(variant: Lealpr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEALPR` reader - This bit defined if command execution shall be continued in LPM modes"]
pub type LealprR = crate::BitReader<Lealpr>;
impl LealprR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lealpr {
        match self.bits {
            false => Lealpr::Lealpr0,
            true => Lealpr::Lealpr1,
        }
    }
    #[doc = "LEA command execution stops in deep low power modes"]
    #[inline(always)]
    pub fn is_lealpr_0(&self) -> bool {
        *self == Lealpr::Lealpr0
    }
    #[doc = "LEA command execution continues in deep low power modes"]
    #[inline(always)]
    pub fn is_lealpr_1(&self) -> bool {
        *self == Lealpr::Lealpr1
    }
}
#[doc = "Field `LEALPR` writer - This bit defined if command execution shall be continued in LPM modes"]
pub type LealprW<'a, REG> = crate::BitWriter<'a, REG, Lealpr>;
impl<'a, REG> LealprW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LEA command execution stops in deep low power modes"]
    #[inline(always)]
    pub fn lealpr_0(self) -> &'a mut crate::W<REG> {
        self.variant(Lealpr::Lealpr0)
    }
    #[doc = "LEA command execution continues in deep low power modes"]
    #[inline(always)]
    pub fn lealpr_1(self) -> &'a mut crate::W<REG> {
        self.variant(Lealpr::Lealpr1)
    }
}
#[doc = "This bit defines if a \"Command done interrupt\" shall be triggered in LPM mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leailpm {
    #[doc = "0: Interrupt of LEA is suppressed in LPM mode until AM is entered then the LEA interrupt is triggered as well"]
    Leailpm0 = 0,
    #[doc = "1: Interrupt of LEA is always triggered on completion of an LEA command"]
    Leailpm1 = 1,
}
impl From<Leailpm> for bool {
    #[inline(always)]
    fn from(variant: Leailpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEAILPM` reader - This bit defines if a \"Command done interrupt\" shall be triggered in LPM mode"]
pub type LeailpmR = crate::BitReader<Leailpm>;
impl LeailpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leailpm {
        match self.bits {
            false => Leailpm::Leailpm0,
            true => Leailpm::Leailpm1,
        }
    }
    #[doc = "Interrupt of LEA is suppressed in LPM mode until AM is entered then the LEA interrupt is triggered as well"]
    #[inline(always)]
    pub fn is_leailpm_0(&self) -> bool {
        *self == Leailpm::Leailpm0
    }
    #[doc = "Interrupt of LEA is always triggered on completion of an LEA command"]
    #[inline(always)]
    pub fn is_leailpm_1(&self) -> bool {
        *self == Leailpm::Leailpm1
    }
}
#[doc = "Field `LEAILPM` writer - This bit defines if a \"Command done interrupt\" shall be triggered in LPM mode"]
pub type LeailpmW<'a, REG> = crate::BitWriter<'a, REG, Leailpm>;
impl<'a, REG> LeailpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt of LEA is suppressed in LPM mode until AM is entered then the LEA interrupt is triggered as well"]
    #[inline(always)]
    pub fn leailpm_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leailpm::Leailpm0)
    }
    #[doc = "Interrupt of LEA is always triggered on completion of an LEA command"]
    #[inline(always)]
    pub fn leailpm_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leailpm::Leailpm1)
    }
}
#[doc = "Field `LEAILB` reader - LEA instruction loop buffer disable. Debugging function for LEA (leave it zero)."]
pub type LeailbR = crate::BitReader;
#[doc = "LEA module timer fault enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leatimflte {
    #[doc = "0: LEA module timer timeout will not cause a fault indication"]
    Leatimflt0 = 0,
    #[doc = "1: LEA module timer timeout will cause a fault indication. LEA stops operation and enters \"Ready-state\"."]
    Leatimflte1 = 1,
}
impl From<Leatimflte> for bool {
    #[inline(always)]
    fn from(variant: Leatimflte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEATIMFLTE` reader - LEA module timer fault enable."]
pub type LeatimflteR = crate::BitReader<Leatimflte>;
impl LeatimflteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leatimflte {
        match self.bits {
            false => Leatimflte::Leatimflt0,
            true => Leatimflte::Leatimflte1,
        }
    }
    #[doc = "LEA module timer timeout will not cause a fault indication"]
    #[inline(always)]
    pub fn is_leatimflt_0(&self) -> bool {
        *self == Leatimflte::Leatimflt0
    }
    #[doc = "LEA module timer timeout will cause a fault indication. LEA stops operation and enters \"Ready-state\"."]
    #[inline(always)]
    pub fn is_leatimflte_1(&self) -> bool {
        *self == Leatimflte::Leatimflte1
    }
}
#[doc = "Field `LEATIMFLTE` writer - LEA module timer fault enable."]
pub type LeatimflteW<'a, REG> = crate::BitWriter<'a, REG, Leatimflte>;
impl<'a, REG> LeatimflteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LEA module timer timeout will not cause a fault indication"]
    #[inline(always)]
    pub fn leatimflt_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leatimflte::Leatimflt0)
    }
    #[doc = "LEA module timer timeout will cause a fault indication. LEA stops operation and enters \"Ready-state\"."]
    #[inline(always)]
    pub fn leatimflte_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leatimflte::Leatimflte1)
    }
}
#[doc = "LEAHPCFLTE when hardware trigger available later Enable bit on peripheral mapped command faults and hardware triggered command faults.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leacflt {
    #[doc = "0: LEAHPCFLT is disabled"]
    Leacflt0 = 0,
    #[doc = "1: LEAHPCFLT is enabled"]
    Leacflt1 = 1,
}
impl From<Leacflt> for bool {
    #[inline(always)]
    fn from(variant: Leacflt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEACFLT` reader - LEAHPCFLTE when hardware trigger available later Enable bit on peripheral mapped command faults and hardware triggered command faults."]
pub type LeacfltR = crate::BitReader<Leacflt>;
impl LeacfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leacflt {
        match self.bits {
            false => Leacflt::Leacflt0,
            true => Leacflt::Leacflt1,
        }
    }
    #[doc = "LEAHPCFLT is disabled"]
    #[inline(always)]
    pub fn is_leacflt_0(&self) -> bool {
        *self == Leacflt::Leacflt0
    }
    #[doc = "LEAHPCFLT is enabled"]
    #[inline(always)]
    pub fn is_leacflt_1(&self) -> bool {
        *self == Leacflt::Leacflt1
    }
}
#[doc = "Field `LEACFLT` writer - LEAHPCFLTE when hardware trigger available later Enable bit on peripheral mapped command faults and hardware triggered command faults."]
pub type LeacfltW<'a, REG> = crate::BitWriter<'a, REG, Leacflt>;
impl<'a, REG> LeacfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LEAHPCFLT is disabled"]
    #[inline(always)]
    pub fn leacflt_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leacflt::Leacflt0)
    }
    #[doc = "LEAHPCFLT is enabled"]
    #[inline(always)]
    pub fn leacflt_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leacflt::Leacflt1)
    }
}
#[doc = "Enable bit on memory faults.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leamemflte {
    #[doc = "0: LEA memory faults are disabled"]
    Leamemflte0 = 0,
    #[doc = "1: LEA memory faults are enabled"]
    Leamemflte1 = 1,
}
impl From<Leamemflte> for bool {
    #[inline(always)]
    fn from(variant: Leamemflte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEAMEMFLTE` reader - Enable bit on memory faults."]
pub type LeamemflteR = crate::BitReader<Leamemflte>;
impl LeamemflteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leamemflte {
        match self.bits {
            false => Leamemflte::Leamemflte0,
            true => Leamemflte::Leamemflte1,
        }
    }
    #[doc = "LEA memory faults are disabled"]
    #[inline(always)]
    pub fn is_leamemflte_0(&self) -> bool {
        *self == Leamemflte::Leamemflte0
    }
    #[doc = "LEA memory faults are enabled"]
    #[inline(always)]
    pub fn is_leamemflte_1(&self) -> bool {
        *self == Leamemflte::Leamemflte1
    }
}
#[doc = "Field `LEAMEMFLTE` writer - Enable bit on memory faults."]
pub type LeamemflteW<'a, REG> = crate::BitWriter<'a, REG, Leamemflte>;
impl<'a, REG> LeamemflteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LEA memory faults are disabled"]
    #[inline(always)]
    pub fn leamemflte_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leamemflte::Leamemflte0)
    }
    #[doc = "LEA memory faults are enabled"]
    #[inline(always)]
    pub fn leamemflte_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leamemflte::Leamemflte1)
    }
}
#[doc = "Field `LEADONES` reader - LEA done event indication and set flag. This bit indicated the done event for LEA. This bit can be set by writing a one to it. Writing a zero has no effect."]
pub type LeadonesR = crate::BitReader;
#[doc = "Field `LEADONES` writer - LEA done event indication and set flag. This bit indicated the done event for LEA. This bit can be set by writing a one to it. Writing a zero has no effect."]
pub type LeadonesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEAFREES` reader - LEA free event indication and set flag. This bit indicated the free event for LEA. This bit can be set by writing a one to it. Writing a zero has no effect."]
pub type LeafreesR = crate::BitReader;
#[doc = "Field `LEAFREES` writer - LEA free event indication and set flag. This bit indicated the free event for LEA. This bit can be set by writing a one to it. Writing a zero has no effect."]
pub type LeafreesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEATIMFLTS` reader - LEA timeout fault indication and set flag; This bits indicates that timer timeout occurred. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module (A package option)"]
pub type LeatimfltsR = crate::BitReader;
#[doc = "Field `LEATIMFLTS` writer - LEA timeout fault indication and set flag; This bits indicates that timer timeout occurred. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module (A package option)"]
pub type LeatimfltsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "LEAHPCFLTS when hardware trigger enabled later LEA command fault on peripheral interface or hardware triggered indication and set flag; This bits indicates that a command was invoked that is not implemented. This fault is also signaled to the SYS module as a \"User-NMI\" when enabled. Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leacflts {
    #[doc = "0: No command fault occurred since this bit was cleared"]
    Leacflts0 = 0,
    #[doc = "1: At least one command fault occurred since this bit was cleared"]
    Leacflts1 = 1,
}
impl From<Leacflts> for bool {
    #[inline(always)]
    fn from(variant: Leacflts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEACFLTS` reader - LEAHPCFLTS when hardware trigger enabled later LEA command fault on peripheral interface or hardware triggered indication and set flag; This bits indicates that a command was invoked that is not implemented. This fault is also signaled to the SYS module as a \"User-NMI\" when enabled. Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module."]
pub type LeacfltsR = crate::BitReader<Leacflts>;
impl LeacfltsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leacflts {
        match self.bits {
            false => Leacflts::Leacflts0,
            true => Leacflts::Leacflts1,
        }
    }
    #[doc = "No command fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn is_leacflts_0(&self) -> bool {
        *self == Leacflts::Leacflts0
    }
    #[doc = "At least one command fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn is_leacflts_1(&self) -> bool {
        *self == Leacflts::Leacflts1
    }
}
#[doc = "Field `LEACFLTS` writer - LEAHPCFLTS when hardware trigger enabled later LEA command fault on peripheral interface or hardware triggered indication and set flag; This bits indicates that a command was invoked that is not implemented. This fault is also signaled to the SYS module as a \"User-NMI\" when enabled. Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module."]
pub type LeacfltsW<'a, REG> = crate::BitWriter<'a, REG, Leacflts>;
impl<'a, REG> LeacfltsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No command fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn leacflts_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leacflts::Leacflts0)
    }
    #[doc = "At least one command fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn leacflts_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leacflts::Leacflts1)
    }
}
#[doc = "LEA memory fault indication and set flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEACNF1. LEAWRSTAT and LEACNF1.LEARDSTAT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leamemflts {
    #[doc = "0: No memory fault occurred since this bit was cleared"]
    Leamemflts0 = 0,
    #[doc = "1: At least one memory fault since this bit was cleared"]
    Leamemflts1 = 1,
}
impl From<Leamemflts> for bool {
    #[inline(always)]
    fn from(variant: Leamemflts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEAMEMFLTS` reader - LEA memory fault indication and set flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEACNF1. LEAWRSTAT and LEACNF1.LEARDSTAT."]
pub type LeamemfltsR = crate::BitReader<Leamemflts>;
impl LeamemfltsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leamemflts {
        match self.bits {
            false => Leamemflts::Leamemflts0,
            true => Leamemflts::Leamemflts1,
        }
    }
    #[doc = "No memory fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn is_leamemflts_0(&self) -> bool {
        *self == Leamemflts::Leamemflts0
    }
    #[doc = "At least one memory fault since this bit was cleared"]
    #[inline(always)]
    pub fn is_leamemflts_1(&self) -> bool {
        *self == Leamemflts::Leamemflts1
    }
}
#[doc = "Field `LEAMEMFLTS` writer - LEA memory fault indication and set flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEACNF1. LEAWRSTAT and LEACNF1.LEARDSTAT."]
pub type LeamemfltsW<'a, REG> = crate::BitWriter<'a, REG, Leamemflts>;
impl<'a, REG> LeamemfltsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No memory fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn leamemflts_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leamemflts::Leamemflts0)
    }
    #[doc = "At least one memory fault since this bit was cleared"]
    #[inline(always)]
    pub fn leamemflts_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leamemflts::Leamemflts1)
    }
}
#[doc = "Field `LEATRST` reader - LEA module timer reset. Setting this bit to one clears LEA module timer. This bit is self clearing and will always be read as zero."]
pub type LeatrstR = crate::BitReader;
#[doc = "Field `LEATRST` writer - LEA module timer reset. Setting this bit to one clears LEA module timer. This bit is self clearing and will always be read as zero."]
pub type LeatrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEATEN` reader - LEA timer enable; writing a one to this bit enables LEA timer operations."]
pub type LeatenR = crate::BitReader;
#[doc = "Field `LEATEN` writer - LEA timer enable; writing a one to this bit enables LEA timer operations."]
pub type LeatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "LEA timer interval select. These bits select LEA timer interval. t#sub#CLK#/sub# = 1 / f#sub#CLK#/sub# f#sub#CLK#/sub# = MCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Leatisel {
    #[doc = "0: Timeout period: 128 x t#sub#CLK#/sub# (16 us at 8 MHz); Interval period: 256 x t#sub#CLK#/sub# (32 us at 8 MHz)"]
    Leatisel0 = 0,
    #[doc = "1: Timeout period: 256 x t#sub#CLK#/sub# (32 us at 8 MHz); Interval period: 512 x t#sub#CLK#/sub# (64 us at 8 MHz)"]
    Leatisel1 = 1,
    #[doc = "2: Timeout period: 512 x t#sub#CLK#/sub# (64 us at 8 MHz); Interval period: 1024 x t#sub#CLK#/sub# (128 us at 8 MHz)"]
    Leatisel2 = 2,
    #[doc = "3: Timeout period: 1024 x t#sub#CLK#/sub# (128 us at 8 MHz); Interval period: 2048 x t#sub#CLK#/sub# (256 us at 8 MHz)"]
    Leatisel3 = 3,
    #[doc = "4: Timeout period: 2048 x t#sub#CLK#/sub# (256 us at 8 MHz); Interval period: 4096 x t#sub#CLK#/sub# (512 us at 8 MHz)"]
    Leatisel4 = 4,
    #[doc = "5: Timeout period: 4096 x t#sub#CLK#/sub# (512 us at 8 MHz); Interval period: 8192 x t#sub#CLK#/sub# (1ms at 8 MHz)"]
    Leatisel5 = 5,
    #[doc = "6: Timeout period: 8192 x t#sub#CLK#/sub# (1 ms at 8 MHz); Interval period: 16384 x t#sub#CLK#/sub# (2 ms at 8 MHz)"]
    Leatisel6 = 6,
    #[doc = "7: Timeout period: 16384 x t#sub#CLK#/sub# (2 ms at 8 MHz); Interval period: 32768 x t#sub#CLK#/sub# (4 ms at 8 MHz)"]
    Leatisel7 = 7,
    #[doc = "8: Timeout period: 32768 x t#sub#CLK#/sub# (4ms at 8 MHz); Interval period: 65536 x t#sub#CLK#/sub# (8 ms at 8 MHz)"]
    Leatisel8 = 8,
    #[doc = "9: Timeout period: 65536 x t#sub#CLK#/sub# (8 ms at 8 MHz); Interval period: 131072 x t#sub#CLK#/sub# (16 ms at 8 MHz)"]
    Leatisel9 = 9,
    #[doc = "10: Timeout period: 131072 x t#sub#CLK#/sub# (16 ms at 8 MHz); Interval period: 262144 x t#sub#CLK#/sub# (32 ms at 8 MHz)"]
    Leatisel10 = 10,
    #[doc = "11: Timeout period: 524288 x t#sub#CLK#/sub# (65 ms at 8 MHz); Interval period: 1048576 x t#sub#CLK#/sub# (131 ms at 8 MHz)"]
    Leatisel11 = 11,
    #[doc = "12: Timeout period: 1048576 x t#sub#CLK#/sub# (131 ms at 8 MHz); Interval period: 2097152 x t#sub#CLK#/sub# (262 ms at 8 MHz)"]
    Leatisel12 = 12,
    #[doc = "13: Timeout period: 2097152 x t#sub#CLK#/sub# (262 ms at 8 MHz); Interval period: 4194304 x t#sub#CLK#/sub# (524 ms at 8 MHz)"]
    Leatisel13 = 13,
    #[doc = "14: Timeout period: 4194304 x t#sub#CLK#/sub# (524 ms at 8 MHz); Interval period: 8388608 x t#sub#CLK#/sub# (1.05 s at 8 MHz)"]
    Leatisel14 = 14,
    #[doc = "15: Timeout period: 8388608 x t#sub#CLK#/sub# (1.05 s at 8 MHz); Interval period: 16777216 x t#sub#CLK#/sub# (2.1 s at 8 MHz)"]
    Leatisel15 = 15,
}
impl From<Leatisel> for u8 {
    #[inline(always)]
    fn from(variant: Leatisel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Leatisel {
    type Ux = u8;
}
impl crate::IsEnum for Leatisel {}
#[doc = "Field `LEATISEL` reader - LEA timer interval select. These bits select LEA timer interval. t#sub#CLK#/sub# = 1 / f#sub#CLK#/sub# f#sub#CLK#/sub# = MCLK"]
pub type LeatiselR = crate::FieldReader<Leatisel>;
impl LeatiselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leatisel {
        match self.bits {
            0 => Leatisel::Leatisel0,
            1 => Leatisel::Leatisel1,
            2 => Leatisel::Leatisel2,
            3 => Leatisel::Leatisel3,
            4 => Leatisel::Leatisel4,
            5 => Leatisel::Leatisel5,
            6 => Leatisel::Leatisel6,
            7 => Leatisel::Leatisel7,
            8 => Leatisel::Leatisel8,
            9 => Leatisel::Leatisel9,
            10 => Leatisel::Leatisel10,
            11 => Leatisel::Leatisel11,
            12 => Leatisel::Leatisel12,
            13 => Leatisel::Leatisel13,
            14 => Leatisel::Leatisel14,
            15 => Leatisel::Leatisel15,
            _ => unreachable!(),
        }
    }
    #[doc = "Timeout period: 128 x t#sub#CLK#/sub# (16 us at 8 MHz); Interval period: 256 x t#sub#CLK#/sub# (32 us at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_0(&self) -> bool {
        *self == Leatisel::Leatisel0
    }
    #[doc = "Timeout period: 256 x t#sub#CLK#/sub# (32 us at 8 MHz); Interval period: 512 x t#sub#CLK#/sub# (64 us at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_1(&self) -> bool {
        *self == Leatisel::Leatisel1
    }
    #[doc = "Timeout period: 512 x t#sub#CLK#/sub# (64 us at 8 MHz); Interval period: 1024 x t#sub#CLK#/sub# (128 us at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_2(&self) -> bool {
        *self == Leatisel::Leatisel2
    }
    #[doc = "Timeout period: 1024 x t#sub#CLK#/sub# (128 us at 8 MHz); Interval period: 2048 x t#sub#CLK#/sub# (256 us at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_3(&self) -> bool {
        *self == Leatisel::Leatisel3
    }
    #[doc = "Timeout period: 2048 x t#sub#CLK#/sub# (256 us at 8 MHz); Interval period: 4096 x t#sub#CLK#/sub# (512 us at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_4(&self) -> bool {
        *self == Leatisel::Leatisel4
    }
    #[doc = "Timeout period: 4096 x t#sub#CLK#/sub# (512 us at 8 MHz); Interval period: 8192 x t#sub#CLK#/sub# (1ms at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_5(&self) -> bool {
        *self == Leatisel::Leatisel5
    }
    #[doc = "Timeout period: 8192 x t#sub#CLK#/sub# (1 ms at 8 MHz); Interval period: 16384 x t#sub#CLK#/sub# (2 ms at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_6(&self) -> bool {
        *self == Leatisel::Leatisel6
    }
    #[doc = "Timeout period: 16384 x t#sub#CLK#/sub# (2 ms at 8 MHz); Interval period: 32768 x t#sub#CLK#/sub# (4 ms at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_7(&self) -> bool {
        *self == Leatisel::Leatisel7
    }
    #[doc = "Timeout period: 32768 x t#sub#CLK#/sub# (4ms at 8 MHz); Interval period: 65536 x t#sub#CLK#/sub# (8 ms at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_8(&self) -> bool {
        *self == Leatisel::Leatisel8
    }
    #[doc = "Timeout period: 65536 x t#sub#CLK#/sub# (8 ms at 8 MHz); Interval period: 131072 x t#sub#CLK#/sub# (16 ms at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_9(&self) -> bool {
        *self == Leatisel::Leatisel9
    }
    #[doc = "Timeout period: 131072 x t#sub#CLK#/sub# (16 ms at 8 MHz); Interval period: 262144 x t#sub#CLK#/sub# (32 ms at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_10(&self) -> bool {
        *self == Leatisel::Leatisel10
    }
    #[doc = "Timeout period: 524288 x t#sub#CLK#/sub# (65 ms at 8 MHz); Interval period: 1048576 x t#sub#CLK#/sub# (131 ms at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_11(&self) -> bool {
        *self == Leatisel::Leatisel11
    }
    #[doc = "Timeout period: 1048576 x t#sub#CLK#/sub# (131 ms at 8 MHz); Interval period: 2097152 x t#sub#CLK#/sub# (262 ms at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_12(&self) -> bool {
        *self == Leatisel::Leatisel12
    }
    #[doc = "Timeout period: 2097152 x t#sub#CLK#/sub# (262 ms at 8 MHz); Interval period: 4194304 x t#sub#CLK#/sub# (524 ms at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_13(&self) -> bool {
        *self == Leatisel::Leatisel13
    }
    #[doc = "Timeout period: 4194304 x t#sub#CLK#/sub# (524 ms at 8 MHz); Interval period: 8388608 x t#sub#CLK#/sub# (1.05 s at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_14(&self) -> bool {
        *self == Leatisel::Leatisel14
    }
    #[doc = "Timeout period: 8388608 x t#sub#CLK#/sub# (1.05 s at 8 MHz); Interval period: 16777216 x t#sub#CLK#/sub# (2.1 s at 8 MHz)"]
    #[inline(always)]
    pub fn is_leatisel_15(&self) -> bool {
        *self == Leatisel::Leatisel15
    }
}
#[doc = "Field `LEATISEL` writer - LEA timer interval select. These bits select LEA timer interval. t#sub#CLK#/sub# = 1 / f#sub#CLK#/sub# f#sub#CLK#/sub# = MCLK"]
pub type LeatiselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Leatisel, crate::Safe>;
impl<'a, REG> LeatiselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timeout period: 128 x t#sub#CLK#/sub# (16 us at 8 MHz); Interval period: 256 x t#sub#CLK#/sub# (32 us at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel0)
    }
    #[doc = "Timeout period: 256 x t#sub#CLK#/sub# (32 us at 8 MHz); Interval period: 512 x t#sub#CLK#/sub# (64 us at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel1)
    }
    #[doc = "Timeout period: 512 x t#sub#CLK#/sub# (64 us at 8 MHz); Interval period: 1024 x t#sub#CLK#/sub# (128 us at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel2)
    }
    #[doc = "Timeout period: 1024 x t#sub#CLK#/sub# (128 us at 8 MHz); Interval period: 2048 x t#sub#CLK#/sub# (256 us at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel3)
    }
    #[doc = "Timeout period: 2048 x t#sub#CLK#/sub# (256 us at 8 MHz); Interval period: 4096 x t#sub#CLK#/sub# (512 us at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel4)
    }
    #[doc = "Timeout period: 4096 x t#sub#CLK#/sub# (512 us at 8 MHz); Interval period: 8192 x t#sub#CLK#/sub# (1ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel5)
    }
    #[doc = "Timeout period: 8192 x t#sub#CLK#/sub# (1 ms at 8 MHz); Interval period: 16384 x t#sub#CLK#/sub# (2 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel6)
    }
    #[doc = "Timeout period: 16384 x t#sub#CLK#/sub# (2 ms at 8 MHz); Interval period: 32768 x t#sub#CLK#/sub# (4 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel7)
    }
    #[doc = "Timeout period: 32768 x t#sub#CLK#/sub# (4ms at 8 MHz); Interval period: 65536 x t#sub#CLK#/sub# (8 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_8(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel8)
    }
    #[doc = "Timeout period: 65536 x t#sub#CLK#/sub# (8 ms at 8 MHz); Interval period: 131072 x t#sub#CLK#/sub# (16 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_9(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel9)
    }
    #[doc = "Timeout period: 131072 x t#sub#CLK#/sub# (16 ms at 8 MHz); Interval period: 262144 x t#sub#CLK#/sub# (32 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_10(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel10)
    }
    #[doc = "Timeout period: 524288 x t#sub#CLK#/sub# (65 ms at 8 MHz); Interval period: 1048576 x t#sub#CLK#/sub# (131 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_11(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel11)
    }
    #[doc = "Timeout period: 1048576 x t#sub#CLK#/sub# (131 ms at 8 MHz); Interval period: 2097152 x t#sub#CLK#/sub# (262 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_12(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel12)
    }
    #[doc = "Timeout period: 2097152 x t#sub#CLK#/sub# (262 ms at 8 MHz); Interval period: 4194304 x t#sub#CLK#/sub# (524 ms at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_13(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel13)
    }
    #[doc = "Timeout period: 4194304 x t#sub#CLK#/sub# (524 ms at 8 MHz); Interval period: 8388608 x t#sub#CLK#/sub# (1.05 s at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_14(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel14)
    }
    #[doc = "Timeout period: 8388608 x t#sub#CLK#/sub# (1.05 s at 8 MHz); Interval period: 16777216 x t#sub#CLK#/sub# (2.1 s at 8 MHz)"]
    #[inline(always)]
    pub fn leatisel_15(self) -> &'a mut crate::W<REG> {
        self.variant(Leatisel::Leatisel15)
    }
}
impl R {
    #[doc = "Bit 0 - LEA module software restart. Setting this bit to one restarts the LEA module. As long this bit remains set to one the LEA is held in Restart. (The LEA accessible memory behaves as system RAM)"]
    #[inline(always)]
    pub fn leaswrst(&self) -> LeaswrstR {
        LeaswrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hold on faults and NMIs for all pending LEA operations transfers.This is for all system wide fault/NMI cases (for our first implementation we may just consider local LEA triggered fault cases)"]
    #[inline(always)]
    pub fn leafthold(&self) -> LeaftholdR {
        LeaftholdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit defined if command execution shall be continued in LPM modes"]
    #[inline(always)]
    pub fn lealpr(&self) -> LealprR {
        LealprR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit defines if a \"Command done interrupt\" shall be triggered in LPM mode"]
    #[inline(always)]
    pub fn leailpm(&self) -> LeailpmR {
        LeailpmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LEA instruction loop buffer disable. Debugging function for LEA (leave it zero)."]
    #[inline(always)]
    pub fn leailb(&self) -> LeailbR {
        LeailbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - LEA module timer fault enable."]
    #[inline(always)]
    pub fn leatimflte(&self) -> LeatimflteR {
        LeatimflteR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LEAHPCFLTE when hardware trigger available later Enable bit on peripheral mapped command faults and hardware triggered command faults."]
    #[inline(always)]
    pub fn leacflt(&self) -> LeacfltR {
        LeacfltR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable bit on memory faults."]
    #[inline(always)]
    pub fn leamemflte(&self) -> LeamemflteR {
        LeamemflteR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LEA done event indication and set flag. This bit indicated the done event for LEA. This bit can be set by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leadones(&self) -> LeadonesR {
        LeadonesR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LEA free event indication and set flag. This bit indicated the free event for LEA. This bit can be set by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leafrees(&self) -> LeafreesR {
        LeafreesR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - LEA timeout fault indication and set flag; This bits indicates that timer timeout occurred. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module (A package option)"]
    #[inline(always)]
    pub fn leatimflts(&self) -> LeatimfltsR {
        LeatimfltsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LEAHPCFLTS when hardware trigger enabled later LEA command fault on peripheral interface or hardware triggered indication and set flag; This bits indicates that a command was invoked that is not implemented. This fault is also signaled to the SYS module as a \"User-NMI\" when enabled. Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module."]
    #[inline(always)]
    pub fn leacflts(&self) -> LeacfltsR {
        LeacfltsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LEA memory fault indication and set flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEACNF1. LEAWRSTAT and LEACNF1.LEARDSTAT."]
    #[inline(always)]
    pub fn leamemflts(&self) -> LeamemfltsR {
        LeamemfltsR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - LEA module timer reset. Setting this bit to one clears LEA module timer. This bit is self clearing and will always be read as zero."]
    #[inline(always)]
    pub fn leatrst(&self) -> LeatrstR {
        LeatrstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LEA timer enable; writing a one to this bit enables LEA timer operations."]
    #[inline(always)]
    pub fn leaten(&self) -> LeatenR {
        LeatenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:31 - LEA timer interval select. These bits select LEA timer interval. t#sub#CLK#/sub# = 1 / f#sub#CLK#/sub# f#sub#CLK#/sub# = MCLK"]
    #[inline(always)]
    pub fn leatisel(&self) -> LeatiselR {
        LeatiselR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LEA module software restart. Setting this bit to one restarts the LEA module. As long this bit remains set to one the LEA is held in Restart. (The LEA accessible memory behaves as system RAM)"]
    #[inline(always)]
    pub fn leaswrst(&mut self) -> LeaswrstW<Leacnf0Spec> {
        LeaswrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Hold on faults and NMIs for all pending LEA operations transfers.This is for all system wide fault/NMI cases (for our first implementation we may just consider local LEA triggered fault cases)"]
    #[inline(always)]
    pub fn leafthold(&mut self) -> LeaftholdW<Leacnf0Spec> {
        LeaftholdW::new(self, 1)
    }
    #[doc = "Bit 8 - This bit defined if command execution shall be continued in LPM modes"]
    #[inline(always)]
    pub fn lealpr(&mut self) -> LealprW<Leacnf0Spec> {
        LealprW::new(self, 8)
    }
    #[doc = "Bit 10 - This bit defines if a \"Command done interrupt\" shall be triggered in LPM mode"]
    #[inline(always)]
    pub fn leailpm(&mut self) -> LeailpmW<Leacnf0Spec> {
        LeailpmW::new(self, 10)
    }
    #[doc = "Bit 13 - LEA module timer fault enable."]
    #[inline(always)]
    pub fn leatimflte(&mut self) -> LeatimflteW<Leacnf0Spec> {
        LeatimflteW::new(self, 13)
    }
    #[doc = "Bit 14 - LEAHPCFLTE when hardware trigger available later Enable bit on peripheral mapped command faults and hardware triggered command faults."]
    #[inline(always)]
    pub fn leacflt(&mut self) -> LeacfltW<Leacnf0Spec> {
        LeacfltW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable bit on memory faults."]
    #[inline(always)]
    pub fn leamemflte(&mut self) -> LeamemflteW<Leacnf0Spec> {
        LeamemflteW::new(self, 15)
    }
    #[doc = "Bit 16 - LEA done event indication and set flag. This bit indicated the done event for LEA. This bit can be set by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leadones(&mut self) -> LeadonesW<Leacnf0Spec> {
        LeadonesW::new(self, 16)
    }
    #[doc = "Bit 17 - LEA free event indication and set flag. This bit indicated the free event for LEA. This bit can be set by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leafrees(&mut self) -> LeafreesW<Leacnf0Spec> {
        LeafreesW::new(self, 17)
    }
    #[doc = "Bit 21 - LEA timeout fault indication and set flag; This bits indicates that timer timeout occurred. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module (A package option)"]
    #[inline(always)]
    pub fn leatimflts(&mut self) -> LeatimfltsW<Leacnf0Spec> {
        LeatimfltsW::new(self, 21)
    }
    #[doc = "Bit 22 - LEAHPCFLTS when hardware trigger enabled later LEA command fault on peripheral interface or hardware triggered indication and set flag; This bits indicates that a command was invoked that is not implemented. This fault is also signaled to the SYS module as a \"User-NMI\" when enabled. Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault may also be set by writing a one to it. Writing a zero has no effect.The corresponding terminal is connected to one of the UNMI inputs of the SYS module."]
    #[inline(always)]
    pub fn leacflts(&mut self) -> LeacfltsW<Leacnf0Spec> {
        LeacfltsW::new(self, 22)
    }
    #[doc = "Bit 23 - LEA memory fault indication and set flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEACNF1. LEAWRSTAT and LEACNF1.LEARDSTAT."]
    #[inline(always)]
    pub fn leamemflts(&mut self) -> LeamemfltsW<Leacnf0Spec> {
        LeamemfltsW::new(self, 23)
    }
    #[doc = "Bit 24 - LEA module timer reset. Setting this bit to one clears LEA module timer. This bit is self clearing and will always be read as zero."]
    #[inline(always)]
    pub fn leatrst(&mut self) -> LeatrstW<Leacnf0Spec> {
        LeatrstW::new(self, 24)
    }
    #[doc = "Bit 25 - LEA timer enable; writing a one to this bit enables LEA timer operations."]
    #[inline(always)]
    pub fn leaten(&mut self) -> LeatenW<Leacnf0Spec> {
        LeatenW::new(self, 25)
    }
    #[doc = "Bits 28:31 - LEA timer interval select. These bits select LEA timer interval. t#sub#CLK#/sub# = 1 / f#sub#CLK#/sub# f#sub#CLK#/sub# = MCLK"]
    #[inline(always)]
    pub fn leatisel(&mut self) -> LeatiselW<Leacnf0Spec> {
        LeatiselW::new(self, 28)
    }
}
#[doc = "Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`leacnf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leacnf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Leacnf0Spec;
impl crate::RegisterSpec for Leacnf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leacnf0::R`](R) reader structure"]
impl crate::Readable for Leacnf0Spec {}
#[doc = "`write(|w| ..)` method takes [`leacnf0::W`](W) writer structure"]
impl crate::Writable for Leacnf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEACNF0 to value 0"]
impl crate::Resettable for Leacnf0Spec {}
