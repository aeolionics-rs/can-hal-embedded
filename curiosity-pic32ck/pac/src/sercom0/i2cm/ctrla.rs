#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modeselect {
    #[doc = "0: USART with external clock"]
    UsartExtClk = 0,
    #[doc = "1: USART with internal clock"]
    UsartIntClk = 1,
    #[doc = "2: SPI in slave operation"]
    SpiSlave = 2,
    #[doc = "3: SPI in master operation"]
    SpiMaster = 3,
    #[doc = "4: I2C slave operation"]
    I2cSlave = 4,
    #[doc = "5: I2C master operation"]
    I2cMaster = 5,
}
impl From<Modeselect> for u8 {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modeselect {
    type Ux = u8;
}
impl crate::IsEnum for Modeselect {}
#[doc = "Field `MODE` reader - Operating Mode"]
pub type ModeR = crate::FieldReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modeselect> {
        match self.bits {
            0 => Some(Modeselect::UsartExtClk),
            1 => Some(Modeselect::UsartIntClk),
            2 => Some(Modeselect::SpiSlave),
            3 => Some(Modeselect::SpiMaster),
            4 => Some(Modeselect::I2cSlave),
            5 => Some(Modeselect::I2cMaster),
            _ => None,
        }
    }
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == Modeselect::UsartExtClk
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == Modeselect::UsartIntClk
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == Modeselect::SpiSlave
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == Modeselect::SpiMaster
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        *self == Modeselect::I2cSlave
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == Modeselect::I2cMaster
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::UsartExtClk)
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::UsartIntClk)
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::SpiSlave)
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::SpiMaster)
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::I2cSlave)
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::I2cMaster)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filtselselect {
    #[doc = "0: Disable"]
    Dis = 0,
    #[doc = "2: Minimum 50ns filter"]
    _50ns = 2,
    #[doc = "3: Minimum 10ns filter"]
    _10ns = 3,
}
impl From<Filtselselect> for u8 {
    #[inline(always)]
    fn from(variant: Filtselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filtselselect {
    type Ux = u8;
}
impl crate::IsEnum for Filtselselect {}
#[doc = "Field `FILTSEL` reader - Input Filter Selection"]
pub type FiltselR = crate::FieldReader<Filtselselect>;
impl FiltselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Filtselselect> {
        match self.bits {
            0 => Some(Filtselselect::Dis),
            2 => Some(Filtselselect::_50ns),
            3 => Some(Filtselselect::_10ns),
            _ => None,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Filtselselect::Dis
    }
    #[doc = "Minimum 50ns filter"]
    #[inline(always)]
    pub fn is_50ns(&self) -> bool {
        *self == Filtselselect::_50ns
    }
    #[doc = "Minimum 10ns filter"]
    #[inline(always)]
    pub fn is_10ns(&self) -> bool {
        *self == Filtselselect::_10ns
    }
}
#[doc = "Field `FILTSEL` writer - Input Filter Selection"]
pub type FiltselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Filtselselect>;
impl<'a, REG> FiltselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Filtselselect::Dis)
    }
    #[doc = "Minimum 50ns filter"]
    #[inline(always)]
    pub fn _50ns(self) -> &'a mut crate::W<REG> {
        self.variant(Filtselselect::_50ns)
    }
    #[doc = "Minimum 10ns filter"]
    #[inline(always)]
    pub fn _10ns(self) -> &'a mut crate::W<REG> {
        self.variant(Filtselselect::_10ns)
    }
}
#[doc = "Slew Rate Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slewrateselect {
    #[doc = "0: Standard mode"]
    Smm = 0,
    #[doc = "1: Fast mode"]
    Fm = 1,
    #[doc = "2: Fast mode plus"]
    Fmp = 2,
    #[doc = "3: High-speed mode"]
    Hs = 3,
}
impl From<Slewrateselect> for u8 {
    #[inline(always)]
    fn from(variant: Slewrateselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Slewrateselect {
    type Ux = u8;
}
impl crate::IsEnum for Slewrateselect {}
#[doc = "Field `SLEWRATE` reader - Slew Rate Selection"]
pub type SlewrateR = crate::FieldReader<Slewrateselect>;
impl SlewrateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slewrateselect {
        match self.bits {
            0 => Slewrateselect::Smm,
            1 => Slewrateselect::Fm,
            2 => Slewrateselect::Fmp,
            3 => Slewrateselect::Hs,
            _ => unreachable!(),
        }
    }
    #[doc = "Standard mode"]
    #[inline(always)]
    pub fn is_smm(&self) -> bool {
        *self == Slewrateselect::Smm
    }
    #[doc = "Fast mode"]
    #[inline(always)]
    pub fn is_fm(&self) -> bool {
        *self == Slewrateselect::Fm
    }
    #[doc = "Fast mode plus"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == Slewrateselect::Fmp
    }
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == Slewrateselect::Hs
    }
}
#[doc = "Field `SLEWRATE` writer - Slew Rate Selection"]
pub type SlewrateW<'a, REG> = crate::FieldWriter<'a, REG, 2, Slewrateselect, crate::Safe>;
impl<'a, REG> SlewrateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard mode"]
    #[inline(always)]
    pub fn smm(self) -> &'a mut crate::W<REG> {
        self.variant(Slewrateselect::Smm)
    }
    #[doc = "Fast mode"]
    #[inline(always)]
    pub fn fm(self) -> &'a mut crate::W<REG> {
        self.variant(Slewrateselect::Fm)
    }
    #[doc = "Fast mode plus"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(Slewrateselect::Fmp)
    }
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn hs(self) -> &'a mut crate::W<REG> {
        self.variant(Slewrateselect::Hs)
    }
}
#[doc = "Field `PINOUT` reader - Pin Usage"]
pub type PinoutR = crate::BitReader;
#[doc = "Field `PINOUT` writer - Pin Usage"]
pub type PinoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBUSEN` reader - SMBUS Input Buffer Enable"]
pub type SmbusenR = crate::BitReader;
#[doc = "Field `SMBUSEN` writer - SMBUS Input Buffer Enable"]
pub type SmbusenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SDA Hold Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sdaholdselect {
    #[doc = "0: Disable"]
    Dis = 0,
    #[doc = "1: 50ns - 100ns hold time"]
    _75ns = 1,
    #[doc = "2: 300ns - 600ns hold time"]
    _450ns = 2,
    #[doc = "3: 400ns - 800ns hold time"]
    _600ns = 3,
}
impl From<Sdaholdselect> for u8 {
    #[inline(always)]
    fn from(variant: Sdaholdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sdaholdselect {
    type Ux = u8;
}
impl crate::IsEnum for Sdaholdselect {}
#[doc = "Field `SDAHOLD` reader - SDA Hold Time"]
pub type SdaholdR = crate::FieldReader<Sdaholdselect>;
impl SdaholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdaholdselect {
        match self.bits {
            0 => Sdaholdselect::Dis,
            1 => Sdaholdselect::_75ns,
            2 => Sdaholdselect::_450ns,
            3 => Sdaholdselect::_600ns,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Sdaholdselect::Dis
    }
    #[doc = "50ns - 100ns hold time"]
    #[inline(always)]
    pub fn is_75ns(&self) -> bool {
        *self == Sdaholdselect::_75ns
    }
    #[doc = "300ns - 600ns hold time"]
    #[inline(always)]
    pub fn is_450ns(&self) -> bool {
        *self == Sdaholdselect::_450ns
    }
    #[doc = "400ns - 800ns hold time"]
    #[inline(always)]
    pub fn is_600ns(&self) -> bool {
        *self == Sdaholdselect::_600ns
    }
}
#[doc = "Field `SDAHOLD` writer - SDA Hold Time"]
pub type SdaholdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sdaholdselect, crate::Safe>;
impl<'a, REG> SdaholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Sdaholdselect::Dis)
    }
    #[doc = "50ns - 100ns hold time"]
    #[inline(always)]
    pub fn _75ns(self) -> &'a mut crate::W<REG> {
        self.variant(Sdaholdselect::_75ns)
    }
    #[doc = "300ns - 600ns hold time"]
    #[inline(always)]
    pub fn _450ns(self) -> &'a mut crate::W<REG> {
        self.variant(Sdaholdselect::_450ns)
    }
    #[doc = "400ns - 800ns hold time"]
    #[inline(always)]
    pub fn _600ns(self) -> &'a mut crate::W<REG> {
        self.variant(Sdaholdselect::_600ns)
    }
}
#[doc = "Field `MEXTTOEN` reader - Master SCL Low Extend Timeout"]
pub type MexttoenR = crate::BitReader;
#[doc = "Field `MEXTTOEN` writer - Master SCL Low Extend Timeout"]
pub type MexttoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEXTTOEN` reader - Slave SCL Low Extend Timeout"]
pub type SexttoenR = crate::BitReader;
#[doc = "Field `SEXTTOEN` writer - Slave SCL Low Extend Timeout"]
pub type SexttoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Transfer Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Speedselect {
    #[doc = "0: Standard-Mode (SM) and Fast-Mode (FM)"]
    Sm = 0,
    #[doc = "1: Fast-Mode Plus (FM+)"]
    Fmp = 1,
    #[doc = "2: High-Speed Mode"]
    Hs = 2,
}
impl From<Speedselect> for u8 {
    #[inline(always)]
    fn from(variant: Speedselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Speedselect {
    type Ux = u8;
}
impl crate::IsEnum for Speedselect {}
#[doc = "Field `SPEED` reader - Transfer Speed"]
pub type SpeedR = crate::FieldReader<Speedselect>;
impl SpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Speedselect> {
        match self.bits {
            0 => Some(Speedselect::Sm),
            1 => Some(Speedselect::Fmp),
            2 => Some(Speedselect::Hs),
            _ => None,
        }
    }
    #[doc = "Standard-Mode (SM) and Fast-Mode (FM)"]
    #[inline(always)]
    pub fn is_sm(&self) -> bool {
        *self == Speedselect::Sm
    }
    #[doc = "Fast-Mode Plus (FM+)"]
    #[inline(always)]
    pub fn is_fmp(&self) -> bool {
        *self == Speedselect::Fmp
    }
    #[doc = "High-Speed Mode"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == Speedselect::Hs
    }
}
#[doc = "Field `SPEED` writer - Transfer Speed"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2, Speedselect>;
impl<'a, REG> SpeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard-Mode (SM) and Fast-Mode (FM)"]
    #[inline(always)]
    pub fn sm(self) -> &'a mut crate::W<REG> {
        self.variant(Speedselect::Sm)
    }
    #[doc = "Fast-Mode Plus (FM+)"]
    #[inline(always)]
    pub fn fmp(self) -> &'a mut crate::W<REG> {
        self.variant(Speedselect::Fmp)
    }
    #[doc = "High-Speed Mode"]
    #[inline(always)]
    pub fn hs(self) -> &'a mut crate::W<REG> {
        self.variant(Speedselect::Hs)
    }
}
#[doc = "Field `SCLSM` reader - SCL Clock Stretch Mode"]
pub type SclsmR = crate::BitReader;
#[doc = "Field `SCLSM` writer - SCL Clock Stretch Mode"]
pub type SclsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Inactive Time-Out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Inactoutselect {
    #[doc = "0: Disable"]
    Dis = 0,
    #[doc = "1: 5-6 SCL cycle time-out"]
    _55us = 1,
    #[doc = "2: 10-11 SCL cycle time-out"]
    _105us = 2,
    #[doc = "3: 20-21-6 SCL cycle time-out"]
    _205us = 3,
}
impl From<Inactoutselect> for u8 {
    #[inline(always)]
    fn from(variant: Inactoutselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inactoutselect {
    type Ux = u8;
}
impl crate::IsEnum for Inactoutselect {}
#[doc = "Field `INACTOUT` reader - Inactive Time-Out"]
pub type InactoutR = crate::FieldReader<Inactoutselect>;
impl InactoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inactoutselect {
        match self.bits {
            0 => Inactoutselect::Dis,
            1 => Inactoutselect::_55us,
            2 => Inactoutselect::_105us,
            3 => Inactoutselect::_205us,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Inactoutselect::Dis
    }
    #[doc = "5-6 SCL cycle time-out"]
    #[inline(always)]
    pub fn is_55us(&self) -> bool {
        *self == Inactoutselect::_55us
    }
    #[doc = "10-11 SCL cycle time-out"]
    #[inline(always)]
    pub fn is_105us(&self) -> bool {
        *self == Inactoutselect::_105us
    }
    #[doc = "20-21-6 SCL cycle time-out"]
    #[inline(always)]
    pub fn is_205us(&self) -> bool {
        *self == Inactoutselect::_205us
    }
}
#[doc = "Field `INACTOUT` writer - Inactive Time-Out"]
pub type InactoutW<'a, REG> = crate::FieldWriter<'a, REG, 2, Inactoutselect, crate::Safe>;
impl<'a, REG> InactoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Inactoutselect::Dis)
    }
    #[doc = "5-6 SCL cycle time-out"]
    #[inline(always)]
    pub fn _55us(self) -> &'a mut crate::W<REG> {
        self.variant(Inactoutselect::_55us)
    }
    #[doc = "10-11 SCL cycle time-out"]
    #[inline(always)]
    pub fn _105us(self) -> &'a mut crate::W<REG> {
        self.variant(Inactoutselect::_105us)
    }
    #[doc = "20-21-6 SCL cycle time-out"]
    #[inline(always)]
    pub fn _205us(self) -> &'a mut crate::W<REG> {
        self.variant(Inactoutselect::_205us)
    }
}
#[doc = "Field `LOWTOUTEN` reader - SCL Low Timeout Enable"]
pub type LowtoutenR = crate::BitReader;
#[doc = "Field `LOWTOUTEN` writer - SCL Low Timeout Enable"]
pub type LowtoutenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 7 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Input Filter Selection"]
    #[inline(always)]
    pub fn filtsel(&self) -> FiltselR {
        FiltselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Slew Rate Selection"]
    #[inline(always)]
    pub fn slewrate(&self) -> SlewrateR {
        SlewrateR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    pub fn pinout(&self) -> PinoutR {
        PinoutR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SMBUS Input Buffer Enable"]
    #[inline(always)]
    pub fn smbusen(&self) -> SmbusenR {
        SmbusenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&self) -> SdaholdR {
        SdaholdR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Master SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn mexttoen(&self) -> MexttoenR {
        MexttoenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn sexttoen(&self) -> SexttoenR {
        SexttoenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline(always)]
    pub fn sclsm(&self) -> SclsmR {
        SclsmR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Inactive Time-Out"]
    #[inline(always)]
    pub fn inactout(&self) -> InactoutR {
        InactoutR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline(always)]
    pub fn lowtouten(&self) -> LowtoutenR {
        LowtoutenR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CtrlaSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bit 7 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, CtrlaSpec> {
        RunstdbyW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Input Filter Selection"]
    #[inline(always)]
    pub fn filtsel(&mut self) -> FiltselW<'_, CtrlaSpec> {
        FiltselW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Slew Rate Selection"]
    #[inline(always)]
    pub fn slewrate(&mut self) -> SlewrateW<'_, CtrlaSpec> {
        SlewrateW::new(self, 10)
    }
    #[doc = "Bit 16 - Pin Usage"]
    #[inline(always)]
    pub fn pinout(&mut self) -> PinoutW<'_, CtrlaSpec> {
        PinoutW::new(self, 16)
    }
    #[doc = "Bit 17 - SMBUS Input Buffer Enable"]
    #[inline(always)]
    pub fn smbusen(&mut self) -> SmbusenW<'_, CtrlaSpec> {
        SmbusenW::new(self, 17)
    }
    #[doc = "Bits 20:21 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&mut self) -> SdaholdW<'_, CtrlaSpec> {
        SdaholdW::new(self, 20)
    }
    #[doc = "Bit 22 - Master SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn mexttoen(&mut self) -> MexttoenW<'_, CtrlaSpec> {
        MexttoenW::new(self, 22)
    }
    #[doc = "Bit 23 - Slave SCL Low Extend Timeout"]
    #[inline(always)]
    pub fn sexttoen(&mut self) -> SexttoenW<'_, CtrlaSpec> {
        SexttoenW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Transfer Speed"]
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<'_, CtrlaSpec> {
        SpeedW::new(self, 24)
    }
    #[doc = "Bit 27 - SCL Clock Stretch Mode"]
    #[inline(always)]
    pub fn sclsm(&mut self) -> SclsmW<'_, CtrlaSpec> {
        SclsmW::new(self, 27)
    }
    #[doc = "Bits 28:29 - Inactive Time-Out"]
    #[inline(always)]
    pub fn inactout(&mut self) -> InactoutW<'_, CtrlaSpec> {
        InactoutW::new(self, 28)
    }
    #[doc = "Bit 30 - SCL Low Timeout Enable"]
    #[inline(always)]
    pub fn lowtouten(&mut self) -> LowtoutenW<'_, CtrlaSpec> {
        LowtoutenW::new(self, 30)
    }
}
#[doc = "I2CM Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {}
