#[doc = "Register `VREGCTRL` reader"]
pub type R = crate::R<VregctrlSpec>;
#[doc = "Register `VREGCTRL` writer"]
pub type W = crate::W<VregctrlSpec>;
#[doc = "Off in Standby Control for VREGSW\\[N-1\\]. Useful for Riverside only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Offstdbyselect {
    #[doc = "0: In standby mode, VREGSW1,2,3 are OFF"]
    Off = 0,
    #[doc = "1: In standby mode, VREGSW1,2,3 are ON"]
    On = 1,
}
impl From<Offstdbyselect> for bool {
    #[inline(always)]
    fn from(variant: Offstdbyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OFFSTDBY` reader - Off in Standby Control for VREGSW\\[N-1\\]. Useful for Riverside only."]
pub type OffstdbyR = crate::BitReader<Offstdbyselect>;
impl OffstdbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Offstdbyselect {
        match self.bits {
            false => Offstdbyselect::Off,
            true => Offstdbyselect::On,
        }
    }
    #[doc = "In standby mode, VREGSW1,2,3 are OFF"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Offstdbyselect::Off
    }
    #[doc = "In standby mode, VREGSW1,2,3 are ON"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Offstdbyselect::On
    }
}
#[doc = "Field `OFFSTDBY` writer - Off in Standby Control for VREGSW\\[N-1\\]. Useful for Riverside only."]
pub type OffstdbyW<'a, REG> = crate::BitWriter<'a, REG, Offstdbyselect>;
impl<'a, REG> OffstdbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In standby mode, VREGSW1,2,3 are OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Offstdbyselect::Off)
    }
    #[doc = "In standby mode, VREGSW1,2,3 are ON"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Offstdbyselect::On)
    }
}
#[doc = "Low Voltage Standby Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvstdbyselect {
    #[doc = "0: In standby mode, VDDCORE_BU, VDDCORE_RAM, VDDCORE_SW and optionnaly VDDCOREUSB/PLL are set to 1.2v."]
    _1p2v = 0,
    #[doc = "1: In standby mode, VDDCORE_BU, VDDCORE_RAM, VDDCORE_SW and optionnaly VDDCOREUSB/PLL are set to 0.8v."]
    _0p8v = 1,
}
impl From<Lvstdbyselect> for bool {
    #[inline(always)]
    fn from(variant: Lvstdbyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVSTDBY` reader - Low Voltage Standby Enable"]
pub type LvstdbyR = crate::BitReader<Lvstdbyselect>;
impl LvstdbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvstdbyselect {
        match self.bits {
            false => Lvstdbyselect::_1p2v,
            true => Lvstdbyselect::_0p8v,
        }
    }
    #[doc = "In standby mode, VDDCORE_BU, VDDCORE_RAM, VDDCORE_SW and optionnaly VDDCOREUSB/PLL are set to 1.2v."]
    #[inline(always)]
    pub fn is_1p2v(&self) -> bool {
        *self == Lvstdbyselect::_1p2v
    }
    #[doc = "In standby mode, VDDCORE_BU, VDDCORE_RAM, VDDCORE_SW and optionnaly VDDCOREUSB/PLL are set to 0.8v."]
    #[inline(always)]
    pub fn is_0p8v(&self) -> bool {
        *self == Lvstdbyselect::_0p8v
    }
}
#[doc = "Field `LVSTDBY` writer - Low Voltage Standby Enable"]
pub type LvstdbyW<'a, REG> = crate::BitWriter<'a, REG, Lvstdbyselect>;
impl<'a, REG> LvstdbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In standby mode, VDDCORE_BU, VDDCORE_RAM, VDDCORE_SW and optionnaly VDDCOREUSB/PLL are set to 1.2v."]
    #[inline(always)]
    pub fn _1p2v(self) -> &'a mut crate::W<REG> {
        self.variant(Lvstdbyselect::_1p2v)
    }
    #[doc = "In standby mode, VDDCORE_BU, VDDCORE_RAM, VDDCORE_SW and optionnaly VDDCOREUSB/PLL are set to 0.8v."]
    #[inline(always)]
    pub fn _0p8v(self) -> &'a mut crate::W<REG> {
        self.variant(Lvstdbyselect::_0p8v)
    }
}
#[doc = "Low Voltage Hibernate Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvhibselect {
    #[doc = "0: In Hibernate mode, VDDCORE_BU and VDDCORE_RAM are set to 1.2v."]
    _1p2v = 0,
    #[doc = "1: In Hibernate mode, VDDCORE_BU and VDDCORE_RAM are set to 0.8v"]
    _0p8v = 1,
}
impl From<Lvhibselect> for bool {
    #[inline(always)]
    fn from(variant: Lvhibselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVHIB` reader - Low Voltage Hibernate Enable"]
pub type LvhibR = crate::BitReader<Lvhibselect>;
impl LvhibR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvhibselect {
        match self.bits {
            false => Lvhibselect::_1p2v,
            true => Lvhibselect::_0p8v,
        }
    }
    #[doc = "In Hibernate mode, VDDCORE_BU and VDDCORE_RAM are set to 1.2v."]
    #[inline(always)]
    pub fn is_1p2v(&self) -> bool {
        *self == Lvhibselect::_1p2v
    }
    #[doc = "In Hibernate mode, VDDCORE_BU and VDDCORE_RAM are set to 0.8v"]
    #[inline(always)]
    pub fn is_0p8v(&self) -> bool {
        *self == Lvhibselect::_0p8v
    }
}
#[doc = "Field `LVHIB` writer - Low Voltage Hibernate Enable"]
pub type LvhibW<'a, REG> = crate::BitWriter<'a, REG, Lvhibselect>;
impl<'a, REG> LvhibW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In Hibernate mode, VDDCORE_BU and VDDCORE_RAM are set to 1.2v."]
    #[inline(always)]
    pub fn _1p2v(self) -> &'a mut crate::W<REG> {
        self.variant(Lvhibselect::_1p2v)
    }
    #[doc = "In Hibernate mode, VDDCORE_BU and VDDCORE_RAM are set to 0.8v"]
    #[inline(always)]
    pub fn _0p8v(self) -> &'a mut crate::W<REG> {
        self.variant(Lvhibselect::_0p8v)
    }
}
#[doc = "Field `CPEN` reader - Charge Pump Enable and Auto-enable."]
pub type CpenR = crate::FieldReader;
#[doc = "Field `CPEN` writer - Charge Pump Enable and Auto-enable."]
pub type CpenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AVREGEN` reader - Additional Voltage Regulator Enable"]
pub type AvregenR = crate::BitReader;
#[doc = "Field `AVREGEN` writer - Additional Voltage Regulator Enable"]
pub type AvregenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Additional Voltage Regulator Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Avregstdbyselect {
    #[doc = "0: Regulator is OFF while in sleep mode equal or deeper than standby mode. It is OFF in hibernate and backup mode as well."]
    Offinstdby = 0,
    #[doc = "1: Regulator is ON in Standby mode if AVREGEN bit is set. is OFF from Hibernate mode. It is OFF in backup mode as well."]
    Oninstdby = 1,
}
impl From<Avregstdbyselect> for bool {
    #[inline(always)]
    fn from(variant: Avregstdbyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVREGSTDBY` reader - Additional Voltage Regulator Configuration"]
pub type AvregstdbyR = crate::BitReader<Avregstdbyselect>;
impl AvregstdbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avregstdbyselect {
        match self.bits {
            false => Avregstdbyselect::Offinstdby,
            true => Avregstdbyselect::Oninstdby,
        }
    }
    #[doc = "Regulator is OFF while in sleep mode equal or deeper than standby mode. It is OFF in hibernate and backup mode as well."]
    #[inline(always)]
    pub fn is_offinstdby(&self) -> bool {
        *self == Avregstdbyselect::Offinstdby
    }
    #[doc = "Regulator is ON in Standby mode if AVREGEN bit is set. is OFF from Hibernate mode. It is OFF in backup mode as well."]
    #[inline(always)]
    pub fn is_oninstdby(&self) -> bool {
        *self == Avregstdbyselect::Oninstdby
    }
}
#[doc = "Field `AVREGSTDBY` writer - Additional Voltage Regulator Configuration"]
pub type AvregstdbyW<'a, REG> = crate::BitWriter<'a, REG, Avregstdbyselect>;
impl<'a, REG> AvregstdbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regulator is OFF while in sleep mode equal or deeper than standby mode. It is OFF in hibernate and backup mode as well."]
    #[inline(always)]
    pub fn offinstdby(self) -> &'a mut crate::W<REG> {
        self.variant(Avregstdbyselect::Offinstdby)
    }
    #[doc = "Regulator is ON in Standby mode if AVREGEN bit is set. is OFF from Hibernate mode. It is OFF in backup mode as well."]
    #[inline(always)]
    pub fn oninstdby(self) -> &'a mut crate::W<REG> {
        self.variant(Avregstdbyselect::Oninstdby)
    }
}
impl R {
    #[doc = "Bit 2 - Off in Standby Control for VREGSW\\[N-1\\]. Useful for Riverside only."]
    #[inline(always)]
    pub fn offstdby(&self) -> OffstdbyR {
        OffstdbyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Low Voltage Standby Enable"]
    #[inline(always)]
    pub fn lvstdby(&self) -> LvstdbyR {
        LvstdbyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Low Voltage Hibernate Enable"]
    #[inline(always)]
    pub fn lvhib(&self) -> LvhibR {
        LvhibR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Charge Pump Enable and Auto-enable."]
    #[inline(always)]
    pub fn cpen(&self) -> CpenR {
        CpenR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Additional Voltage Regulator Enable"]
    #[inline(always)]
    pub fn avregen(&self) -> AvregenR {
        AvregenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Additional Voltage Regulator Configuration"]
    #[inline(always)]
    pub fn avregstdby(&self) -> AvregstdbyR {
        AvregstdbyR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Off in Standby Control for VREGSW\\[N-1\\]. Useful for Riverside only."]
    #[inline(always)]
    pub fn offstdby(&mut self) -> OffstdbyW<'_, VregctrlSpec> {
        OffstdbyW::new(self, 2)
    }
    #[doc = "Bit 4 - Low Voltage Standby Enable"]
    #[inline(always)]
    pub fn lvstdby(&mut self) -> LvstdbyW<'_, VregctrlSpec> {
        LvstdbyW::new(self, 4)
    }
    #[doc = "Bit 5 - Low Voltage Hibernate Enable"]
    #[inline(always)]
    pub fn lvhib(&mut self) -> LvhibW<'_, VregctrlSpec> {
        LvhibW::new(self, 5)
    }
    #[doc = "Bits 8:9 - Charge Pump Enable and Auto-enable."]
    #[inline(always)]
    pub fn cpen(&mut self) -> CpenW<'_, VregctrlSpec> {
        CpenW::new(self, 8)
    }
    #[doc = "Bit 16 - Additional Voltage Regulator Enable"]
    #[inline(always)]
    pub fn avregen(&mut self) -> AvregenW<'_, VregctrlSpec> {
        AvregenW::new(self, 16)
    }
    #[doc = "Bit 24 - Additional Voltage Regulator Configuration"]
    #[inline(always)]
    pub fn avregstdby(&mut self) -> AvregstdbyW<'_, VregctrlSpec> {
        AvregstdbyW::new(self, 24)
    }
}
#[doc = "VREG Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vregctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vregctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VregctrlSpec;
impl crate::RegisterSpec for VregctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vregctrl::R`](R) reader structure"]
impl crate::Readable for VregctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vregctrl::W`](W) writer structure"]
impl crate::Writable for VregctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VREGCTRL to value 0"]
impl crate::Resettable for VregctrlSpec {}
