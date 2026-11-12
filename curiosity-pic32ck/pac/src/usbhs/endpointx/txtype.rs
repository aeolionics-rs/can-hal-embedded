#[doc = "Register `TXTYPE` reader"]
pub type R = crate::R<TxtypeSpec>;
#[doc = "Register `TXTYPE` writer"]
pub type W = crate::W<TxtypeSpec>;
#[doc = "Field `ENDPOINTNUMBER` reader - the cpu should set this value to the endpoint number contained in the tx endpoint descriptor"]
pub type EndpointnumberR = crate::FieldReader;
#[doc = "Field `ENDPOINTNUMBER` writer - the cpu should set this value to the endpoint number contained in the tx endpoint descriptor"]
pub type EndpointnumberW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "the cpu should set this to select the required protocol for the tx endpoint 00: control 01: isochronous 10: bulk 11: interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Protocolselect {
    #[doc = "0: `0`"]
    Control = 0,
    #[doc = "1: `1`"]
    Isochronous = 1,
    #[doc = "2: `10`"]
    Bulk = 2,
    #[doc = "3: `11`"]
    Interrupt = 3,
}
impl From<Protocolselect> for u8 {
    #[inline(always)]
    fn from(variant: Protocolselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Protocolselect {
    type Ux = u8;
}
impl crate::IsEnum for Protocolselect {}
#[doc = "Field `PROTOCOL` reader - the cpu should set this to select the required protocol for the tx endpoint 00: control 01: isochronous 10: bulk 11: interrupt"]
pub type ProtocolR = crate::FieldReader<Protocolselect>;
impl ProtocolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Protocolselect {
        match self.bits {
            0 => Protocolselect::Control,
            1 => Protocolselect::Isochronous,
            2 => Protocolselect::Bulk,
            3 => Protocolselect::Interrupt,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == Protocolselect::Control
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_isochronous(&self) -> bool {
        *self == Protocolselect::Isochronous
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == Protocolselect::Bulk
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Protocolselect::Interrupt
    }
}
#[doc = "Field `PROTOCOL` writer - the cpu should set this to select the required protocol for the tx endpoint 00: control 01: isochronous 10: bulk 11: interrupt"]
pub type ProtocolW<'a, REG> = crate::FieldWriter<'a, REG, 2, Protocolselect, crate::Safe>;
impl<'a, REG> ProtocolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn control(self) -> &'a mut crate::W<REG> {
        self.variant(Protocolselect::Control)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn isochronous(self) -> &'a mut crate::W<REG> {
        self.variant(Protocolselect::Isochronous)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut crate::W<REG> {
        self.variant(Protocolselect::Bulk)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Protocolselect::Interrupt)
    }
}
#[doc = "operating speed of the target device when the core is configured with the multipoint option: 00: unused (note: if selected, the target will be assumed to be using the same connection speed as the core.) 01: high 10: full 11: low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Speedselect {
    #[doc = "1: Device In High Speed"]
    High = 1,
    #[doc = "2: Device In Full Speed"]
    Full = 2,
    #[doc = "3: Device In Low Speed"]
    Low = 3,
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
#[doc = "Field `SPEED` reader - operating speed of the target device when the core is configured with the multipoint option: 00: unused (note: if selected, the target will be assumed to be using the same connection speed as the core.) 01: high 10: full 11: low"]
pub type SpeedR = crate::FieldReader<Speedselect>;
impl SpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Speedselect> {
        match self.bits {
            1 => Some(Speedselect::High),
            2 => Some(Speedselect::Full),
            3 => Some(Speedselect::Low),
            _ => None,
        }
    }
    #[doc = "Device In High Speed"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Speedselect::High
    }
    #[doc = "Device In Full Speed"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Speedselect::Full
    }
    #[doc = "Device In Low Speed"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Speedselect::Low
    }
}
#[doc = "Field `SPEED` writer - operating speed of the target device when the core is configured with the multipoint option: 00: unused (note: if selected, the target will be assumed to be using the same connection speed as the core.) 01: high 10: full 11: low"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2, Speedselect>;
impl<'a, REG> SpeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Device In High Speed"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Speedselect::High)
    }
    #[doc = "Device In Full Speed"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Speedselect::Full)
    }
    #[doc = "Device In Low Speed"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Speedselect::Low)
    }
}
impl R {
    #[doc = "Bits 0:3 - the cpu should set this value to the endpoint number contained in the tx endpoint descriptor"]
    #[inline(always)]
    pub fn endpointnumber(&self) -> EndpointnumberR {
        EndpointnumberR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - the cpu should set this to select the required protocol for the tx endpoint 00: control 01: isochronous 10: bulk 11: interrupt"]
    #[inline(always)]
    pub fn protocol(&self) -> ProtocolR {
        ProtocolR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - operating speed of the target device when the core is configured with the multipoint option: 00: unused (note: if selected, the target will be assumed to be using the same connection speed as the core.) 01: high 10: full 11: low"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - the cpu should set this value to the endpoint number contained in the tx endpoint descriptor"]
    #[inline(always)]
    pub fn endpointnumber(&mut self) -> EndpointnumberW<'_, TxtypeSpec> {
        EndpointnumberW::new(self, 0)
    }
    #[doc = "Bits 4:5 - the cpu should set this to select the required protocol for the tx endpoint 00: control 01: isochronous 10: bulk 11: interrupt"]
    #[inline(always)]
    pub fn protocol(&mut self) -> ProtocolW<'_, TxtypeSpec> {
        ProtocolW::new(self, 4)
    }
    #[doc = "Bits 6:7 - operating speed of the target device when the core is configured with the multipoint option: 00: unused (note: if selected, the target will be assumed to be using the same connection speed as the core.) 01: high 10: full 11: low"]
    #[inline(always)]
    pub fn speed(&mut self) -> SpeedW<'_, TxtypeSpec> {
        SpeedW::new(self, 6)
    }
}
#[doc = "ENDPOINTX Host Mode Only: Should Be Written With The Endpoint Number To Be Targeted By The Endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`txtype::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txtype::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxtypeSpec;
impl crate::RegisterSpec for TxtypeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txtype::R`](R) reader structure"]
impl crate::Readable for TxtypeSpec {}
#[doc = "`write(|w| ..)` method takes [`txtype::W`](W) writer structure"]
impl crate::Writable for TxtypeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXTYPE to value 0"]
impl crate::Resettable for TxtypeSpec {}
