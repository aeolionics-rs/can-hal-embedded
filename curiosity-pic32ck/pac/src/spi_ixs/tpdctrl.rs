#[doc = "Register `TPDCTRL` reader"]
pub type R = crate::R<TpdctrlSpec>;
#[doc = "Register `TPDCTRL` writer"]
pub type W = crate::W<TpdctrlSpec>;
#[doc = "Master Slave TPD mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pkfmtselect {
    #[doc = "0: 32 bit data in 4x32 packed format"]
    Pkt4x32 = 0,
    #[doc = "1: 32 bit data in 3x32 packed format"]
    Pkt3x32 = 1,
    #[doc = "2: 24 bit data in 4x24 packed format"]
    Pkt4x24 = 2,
    #[doc = "3: 24 bit data in 3x24 packed format"]
    Pkt3x24 = 3,
    #[doc = "4: 16 bit data in 6x16 packed format"]
    Pkt6x16 = 4,
    #[doc = "5: 16 bit data in 4x16 packed format"]
    Pkt4x16 = 5,
    #[doc = "6: 16 bit data in 2x16 packed format"]
    Pkt2x16 = 6,
}
impl From<Pkfmtselect> for u8 {
    #[inline(always)]
    fn from(variant: Pkfmtselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pkfmtselect {
    type Ux = u8;
}
impl crate::IsEnum for Pkfmtselect {}
#[doc = "Field `PKFMT` reader - Master Slave TPD mode."]
pub type PkfmtR = crate::FieldReader<Pkfmtselect>;
impl PkfmtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pkfmtselect> {
        match self.bits {
            0 => Some(Pkfmtselect::Pkt4x32),
            1 => Some(Pkfmtselect::Pkt3x32),
            2 => Some(Pkfmtselect::Pkt4x24),
            3 => Some(Pkfmtselect::Pkt3x24),
            4 => Some(Pkfmtselect::Pkt6x16),
            5 => Some(Pkfmtselect::Pkt4x16),
            6 => Some(Pkfmtselect::Pkt2x16),
            _ => None,
        }
    }
    #[doc = "32 bit data in 4x32 packed format"]
    #[inline(always)]
    pub fn is_pkt4x32(&self) -> bool {
        *self == Pkfmtselect::Pkt4x32
    }
    #[doc = "32 bit data in 3x32 packed format"]
    #[inline(always)]
    pub fn is_pkt3x32(&self) -> bool {
        *self == Pkfmtselect::Pkt3x32
    }
    #[doc = "24 bit data in 4x24 packed format"]
    #[inline(always)]
    pub fn is_pkt4x24(&self) -> bool {
        *self == Pkfmtselect::Pkt4x24
    }
    #[doc = "24 bit data in 3x24 packed format"]
    #[inline(always)]
    pub fn is_pkt3x24(&self) -> bool {
        *self == Pkfmtselect::Pkt3x24
    }
    #[doc = "16 bit data in 6x16 packed format"]
    #[inline(always)]
    pub fn is_pkt6x16(&self) -> bool {
        *self == Pkfmtselect::Pkt6x16
    }
    #[doc = "16 bit data in 4x16 packed format"]
    #[inline(always)]
    pub fn is_pkt4x16(&self) -> bool {
        *self == Pkfmtselect::Pkt4x16
    }
    #[doc = "16 bit data in 2x16 packed format"]
    #[inline(always)]
    pub fn is_pkt2x16(&self) -> bool {
        *self == Pkfmtselect::Pkt2x16
    }
}
#[doc = "Field `PKFMT` writer - Master Slave TPD mode."]
pub type PkfmtW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pkfmtselect>;
impl<'a, REG> PkfmtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32 bit data in 4x32 packed format"]
    #[inline(always)]
    pub fn pkt4x32(self) -> &'a mut crate::W<REG> {
        self.variant(Pkfmtselect::Pkt4x32)
    }
    #[doc = "32 bit data in 3x32 packed format"]
    #[inline(always)]
    pub fn pkt3x32(self) -> &'a mut crate::W<REG> {
        self.variant(Pkfmtselect::Pkt3x32)
    }
    #[doc = "24 bit data in 4x24 packed format"]
    #[inline(always)]
    pub fn pkt4x24(self) -> &'a mut crate::W<REG> {
        self.variant(Pkfmtselect::Pkt4x24)
    }
    #[doc = "24 bit data in 3x24 packed format"]
    #[inline(always)]
    pub fn pkt3x24(self) -> &'a mut crate::W<REG> {
        self.variant(Pkfmtselect::Pkt3x24)
    }
    #[doc = "16 bit data in 6x16 packed format"]
    #[inline(always)]
    pub fn pkt6x16(self) -> &'a mut crate::W<REG> {
        self.variant(Pkfmtselect::Pkt6x16)
    }
    #[doc = "16 bit data in 4x16 packed format"]
    #[inline(always)]
    pub fn pkt4x16(self) -> &'a mut crate::W<REG> {
        self.variant(Pkfmtselect::Pkt4x16)
    }
    #[doc = "16 bit data in 2x16 packed format"]
    #[inline(always)]
    pub fn pkt2x16(self) -> &'a mut crate::W<REG> {
        self.variant(Pkfmtselect::Pkt2x16)
    }
}
#[doc = "Number of the Slave designated: to be used with the PKFMT to determine the slave.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slvnumselect {
    #[doc = "0: SLAVE 0"]
    Slave0 = 0,
    #[doc = "1: SLAVE 1"]
    Slave1 = 1,
    #[doc = "2: SLAVE 2"]
    Slave2 = 2,
    #[doc = "3: SLAVE 3"]
    Slave3 = 3,
    #[doc = "4: SLAVE 4"]
    Slave4 = 4,
}
impl From<Slvnumselect> for u8 {
    #[inline(always)]
    fn from(variant: Slvnumselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Slvnumselect {
    type Ux = u8;
}
impl crate::IsEnum for Slvnumselect {}
#[doc = "Field `SLVNUM` reader - Number of the Slave designated: to be used with the PKFMT to determine the slave."]
pub type SlvnumR = crate::FieldReader<Slvnumselect>;
impl SlvnumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Slvnumselect> {
        match self.bits {
            0 => Some(Slvnumselect::Slave0),
            1 => Some(Slvnumselect::Slave1),
            2 => Some(Slvnumselect::Slave2),
            3 => Some(Slvnumselect::Slave3),
            4 => Some(Slvnumselect::Slave4),
            _ => None,
        }
    }
    #[doc = "SLAVE 0"]
    #[inline(always)]
    pub fn is_slave0(&self) -> bool {
        *self == Slvnumselect::Slave0
    }
    #[doc = "SLAVE 1"]
    #[inline(always)]
    pub fn is_slave1(&self) -> bool {
        *self == Slvnumselect::Slave1
    }
    #[doc = "SLAVE 2"]
    #[inline(always)]
    pub fn is_slave2(&self) -> bool {
        *self == Slvnumselect::Slave2
    }
    #[doc = "SLAVE 3"]
    #[inline(always)]
    pub fn is_slave3(&self) -> bool {
        *self == Slvnumselect::Slave3
    }
    #[doc = "SLAVE 4"]
    #[inline(always)]
    pub fn is_slave4(&self) -> bool {
        *self == Slvnumselect::Slave4
    }
}
#[doc = "Field `SLVNUM` writer - Number of the Slave designated: to be used with the PKFMT to determine the slave."]
pub type SlvnumW<'a, REG> = crate::FieldWriter<'a, REG, 3, Slvnumselect>;
impl<'a, REG> SlvnumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SLAVE 0"]
    #[inline(always)]
    pub fn slave0(self) -> &'a mut crate::W<REG> {
        self.variant(Slvnumselect::Slave0)
    }
    #[doc = "SLAVE 1"]
    #[inline(always)]
    pub fn slave1(self) -> &'a mut crate::W<REG> {
        self.variant(Slvnumselect::Slave1)
    }
    #[doc = "SLAVE 2"]
    #[inline(always)]
    pub fn slave2(self) -> &'a mut crate::W<REG> {
        self.variant(Slvnumselect::Slave2)
    }
    #[doc = "SLAVE 3"]
    #[inline(always)]
    pub fn slave3(self) -> &'a mut crate::W<REG> {
        self.variant(Slvnumselect::Slave3)
    }
    #[doc = "SLAVE 4"]
    #[inline(always)]
    pub fn slave4(self) -> &'a mut crate::W<REG> {
        self.variant(Slvnumselect::Slave4)
    }
}
impl R {
    #[doc = "Bits 0:2 - Master Slave TPD mode."]
    #[inline(always)]
    pub fn pkfmt(&self) -> PkfmtR {
        PkfmtR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Number of the Slave designated: to be used with the PKFMT to determine the slave."]
    #[inline(always)]
    pub fn slvnum(&self) -> SlvnumR {
        SlvnumR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Master Slave TPD mode."]
    #[inline(always)]
    pub fn pkfmt(&mut self) -> PkfmtW<'_, TpdctrlSpec> {
        PkfmtW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Number of the Slave designated: to be used with the PKFMT to determine the slave."]
    #[inline(always)]
    pub fn slvnum(&mut self) -> SlvnumW<'_, TpdctrlSpec> {
        SlvnumW::new(self, 8)
    }
}
#[doc = "SPI Control Tpd Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tpdctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpdctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TpdctrlSpec;
impl crate::RegisterSpec for TpdctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpdctrl::R`](R) reader structure"]
impl crate::Readable for TpdctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tpdctrl::W`](W) writer structure"]
impl crate::Writable for TpdctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPDCTRL to value 0"]
impl crate::Resettable for TpdctrlSpec {}
