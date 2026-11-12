#[doc = "Register `VREFCTRL` reader"]
pub type R = crate::R<VrefctrlSpec>;
#[doc = "Register `VREFCTRL` writer"]
pub type W = crate::W<VrefctrlSpec>;
#[doc = "Bandgap and Regulators Low Power Standby Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpstdbyselect {
    #[doc = "0: In standby mode, bandgap and enabled regulator(s) are set to nominal power mode."]
    FullPower = 0,
    #[doc = "1: In standby mode, bandgap and enabled regulator(s) are set to low power mode."]
    LowPower = 1,
}
impl From<Lpstdbyselect> for bool {
    #[inline(always)]
    fn from(variant: Lpstdbyselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPSTDBY` reader - Bandgap and Regulators Low Power Standby Enable"]
pub type LpstdbyR = crate::BitReader<Lpstdbyselect>;
impl LpstdbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpstdbyselect {
        match self.bits {
            false => Lpstdbyselect::FullPower,
            true => Lpstdbyselect::LowPower,
        }
    }
    #[doc = "In standby mode, bandgap and enabled regulator(s) are set to nominal power mode."]
    #[inline(always)]
    pub fn is_full_power(&self) -> bool {
        *self == Lpstdbyselect::FullPower
    }
    #[doc = "In standby mode, bandgap and enabled regulator(s) are set to low power mode."]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == Lpstdbyselect::LowPower
    }
}
#[doc = "Field `LPSTDBY` writer - Bandgap and Regulators Low Power Standby Enable"]
pub type LpstdbyW<'a, REG> = crate::BitWriter<'a, REG, Lpstdbyselect>;
impl<'a, REG> LpstdbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In standby mode, bandgap and enabled regulator(s) are set to nominal power mode."]
    #[inline(always)]
    pub fn full_power(self) -> &'a mut crate::W<REG> {
        self.variant(Lpstdbyselect::FullPower)
    }
    #[doc = "In standby mode, bandgap and enabled regulator(s) are set to low power mode."]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Lpstdbyselect::LowPower)
    }
}
#[doc = "Bandgap and Regulators Low Power Hibernate Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lphibselect {
    #[doc = "0: In hibernate mode, bandgap is set to nominal power mode. As a consequence, enabled regulator(s) are set to nominal power mode."]
    FullPower = 0,
    #[doc = "1: In hibernate mode, bandgap is set to low power mode. As a consequence, enabled regulator(s) are set to low power mode."]
    LowPower = 1,
}
impl From<Lphibselect> for bool {
    #[inline(always)]
    fn from(variant: Lphibselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPHIB` reader - Bandgap and Regulators Low Power Hibernate Enable"]
pub type LphibR = crate::BitReader<Lphibselect>;
impl LphibR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lphibselect {
        match self.bits {
            false => Lphibselect::FullPower,
            true => Lphibselect::LowPower,
        }
    }
    #[doc = "In hibernate mode, bandgap is set to nominal power mode. As a consequence, enabled regulator(s) are set to nominal power mode."]
    #[inline(always)]
    pub fn is_full_power(&self) -> bool {
        *self == Lphibselect::FullPower
    }
    #[doc = "In hibernate mode, bandgap is set to low power mode. As a consequence, enabled regulator(s) are set to low power mode."]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == Lphibselect::LowPower
    }
}
#[doc = "Field `LPHIB` writer - Bandgap and Regulators Low Power Hibernate Enable"]
pub type LphibW<'a, REG> = crate::BitWriter<'a, REG, Lphibselect>;
impl<'a, REG> LphibW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "In hibernate mode, bandgap is set to nominal power mode. As a consequence, enabled regulator(s) are set to nominal power mode."]
    #[inline(always)]
    pub fn full_power(self) -> &'a mut crate::W<REG> {
        self.variant(Lphibselect::FullPower)
    }
    #[doc = "In hibernate mode, bandgap is set to low power mode. As a consequence, enabled regulator(s) are set to low power mode."]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(Lphibselect::LowPower)
    }
}
#[doc = "Field `TSEN` reader - Temperature Sensor Output Enable"]
pub type TsenR = crate::BitReader;
#[doc = "Field `TSEN` writer - Temperature Sensor Output Enable"]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bandgap and Regulators Low Power Standby Enable"]
    #[inline(always)]
    pub fn lpstdby(&self) -> LpstdbyR {
        LpstdbyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bandgap and Regulators Low Power Hibernate Enable"]
    #[inline(always)]
    pub fn lphib(&self) -> LphibR {
        LphibR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Temperature Sensor Output Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bandgap and Regulators Low Power Standby Enable"]
    #[inline(always)]
    pub fn lpstdby(&mut self) -> LpstdbyW<'_, VrefctrlSpec> {
        LpstdbyW::new(self, 0)
    }
    #[doc = "Bit 1 - Bandgap and Regulators Low Power Hibernate Enable"]
    #[inline(always)]
    pub fn lphib(&mut self) -> LphibW<'_, VrefctrlSpec> {
        LphibW::new(self, 1)
    }
    #[doc = "Bit 4 - Temperature Sensor Output Enable"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TsenW<'_, VrefctrlSpec> {
        TsenW::new(self, 4)
    }
}
#[doc = "VREF Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vrefctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vrefctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefctrlSpec;
impl crate::RegisterSpec for VrefctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vrefctrl::R`](R) reader structure"]
impl crate::Readable for VrefctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vrefctrl::W`](W) writer structure"]
impl crate::Writable for VrefctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VREFCTRL to value 0x03"]
impl crate::Resettable for VrefctrlSpec {
    const RESET_VALUE: u32 = 0x03;
}
