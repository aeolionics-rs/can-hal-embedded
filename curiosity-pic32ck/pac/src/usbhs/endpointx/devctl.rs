#[doc = "Register `DEVCTL` reader"]
pub type R = crate::R<DevctlSpec>;
#[doc = "Register `DEVCTL` writer"]
pub type W = crate::W<DevctlSpec>;
#[doc = "Field `SESSION` reader - when operating as an 'A' device, this bit is set or cleared by CPU to state or end a session. when operating as a 'B' device, this bit is set/cleared by the USBCORE when a session starts/ends"]
pub type SessionR = crate::BitReader;
#[doc = "Field `SESSION` writer - when operating as an 'A' device, this bit is set or cleared by CPU to state or end a session. when operating as a 'B' device, this bit is set/cleared by the USBCORE when a session starts/ends"]
pub type SessionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTREQ` reader - when set, the USBCORE will initiate the host negotiation when suspend mode is entered."]
pub type HostreqR = crate::BitReader;
#[doc = "Field `HOSTREQ` writer - when set, the USBCORE will initiate the host negotiation when suspend mode is entered."]
pub type HostreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTMODE` reader - this read-only bit is set when the USBCORE is acting as a host"]
pub type HostmodeR = crate::BitReader;
#[doc = "Field `HOSTMODE` writer - this read-only bit is set when the USBCORE is acting as a host"]
pub type HostmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "encoded value for the VBus level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vbusselect {
    #[doc = "0: Below SessionEnd"]
    Bsessend = 0,
    #[doc = "1: Above SessionEnd, below AValid"]
    Asessend = 1,
    #[doc = "2: Above AValid, below VBus Valid"]
    Aavalid = 2,
    #[doc = "3: Above VBusValid"]
    Avbusvalid = 3,
}
impl From<Vbusselect> for u8 {
    #[inline(always)]
    fn from(variant: Vbusselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vbusselect {
    type Ux = u8;
}
impl crate::IsEnum for Vbusselect {}
#[doc = "Field `VBUS` reader - encoded value for the VBus level"]
pub type VbusR = crate::FieldReader<Vbusselect>;
impl VbusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbusselect {
        match self.bits {
            0 => Vbusselect::Bsessend,
            1 => Vbusselect::Asessend,
            2 => Vbusselect::Aavalid,
            3 => Vbusselect::Avbusvalid,
            _ => unreachable!(),
        }
    }
    #[doc = "Below SessionEnd"]
    #[inline(always)]
    pub fn is_bsessend(&self) -> bool {
        *self == Vbusselect::Bsessend
    }
    #[doc = "Above SessionEnd, below AValid"]
    #[inline(always)]
    pub fn is_asessend(&self) -> bool {
        *self == Vbusselect::Asessend
    }
    #[doc = "Above AValid, below VBus Valid"]
    #[inline(always)]
    pub fn is_aavalid(&self) -> bool {
        *self == Vbusselect::Aavalid
    }
    #[doc = "Above VBusValid"]
    #[inline(always)]
    pub fn is_avbusvalid(&self) -> bool {
        *self == Vbusselect::Avbusvalid
    }
}
#[doc = "Field `VBUS` writer - encoded value for the VBus level"]
pub type VbusW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vbusselect, crate::Safe>;
impl<'a, REG> VbusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Below SessionEnd"]
    #[inline(always)]
    pub fn bsessend(self) -> &'a mut crate::W<REG> {
        self.variant(Vbusselect::Bsessend)
    }
    #[doc = "Above SessionEnd, below AValid"]
    #[inline(always)]
    pub fn asessend(self) -> &'a mut crate::W<REG> {
        self.variant(Vbusselect::Asessend)
    }
    #[doc = "Above AValid, below VBus Valid"]
    #[inline(always)]
    pub fn aavalid(self) -> &'a mut crate::W<REG> {
        self.variant(Vbusselect::Aavalid)
    }
    #[doc = "Above VBusValid"]
    #[inline(always)]
    pub fn avbusvalid(self) -> &'a mut crate::W<REG> {
        self.variant(Vbusselect::Avbusvalid)
    }
}
#[doc = "Field `LSDEV` reader - host mode only: this bit is set when a low speed device has been detected being connected to the port"]
pub type LsdevR = crate::BitReader;
#[doc = "Field `LSDEV` writer - host mode only: this bit is set when a low speed device has been detected being connected to the port"]
pub type LsdevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSDEV` reader - host mode only: this bit is set when a full or high speed device has been detected being connected to the port"]
pub type FsdevR = crate::BitReader;
#[doc = "Field `FSDEV` writer - host mode only: this bit is set when a full or high speed device has been detected being connected to the port"]
pub type FsdevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDEVICE` reader - indicates whether the core is operating as a 'A' device or 'B' device"]
pub type BdeviceR = crate::BitReader;
#[doc = "Field `BDEVICE` writer - indicates whether the core is operating as a 'A' device or 'B' device"]
pub type BdeviceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - when operating as an 'A' device, this bit is set or cleared by CPU to state or end a session. when operating as a 'B' device, this bit is set/cleared by the USBCORE when a session starts/ends"]
    #[inline(always)]
    pub fn session(&self) -> SessionR {
        SessionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when set, the USBCORE will initiate the host negotiation when suspend mode is entered."]
    #[inline(always)]
    pub fn hostreq(&self) -> HostreqR {
        HostreqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - this read-only bit is set when the USBCORE is acting as a host"]
    #[inline(always)]
    pub fn hostmode(&self) -> HostmodeR {
        HostmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - encoded value for the VBus level"]
    #[inline(always)]
    pub fn vbus(&self) -> VbusR {
        VbusR::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - host mode only: this bit is set when a low speed device has been detected being connected to the port"]
    #[inline(always)]
    pub fn lsdev(&self) -> LsdevR {
        LsdevR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - host mode only: this bit is set when a full or high speed device has been detected being connected to the port"]
    #[inline(always)]
    pub fn fsdev(&self) -> FsdevR {
        FsdevR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - indicates whether the core is operating as a 'A' device or 'B' device"]
    #[inline(always)]
    pub fn bdevice(&self) -> BdeviceR {
        BdeviceR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - when operating as an 'A' device, this bit is set or cleared by CPU to state or end a session. when operating as a 'B' device, this bit is set/cleared by the USBCORE when a session starts/ends"]
    #[inline(always)]
    pub fn session(&mut self) -> SessionW<'_, DevctlSpec> {
        SessionW::new(self, 0)
    }
    #[doc = "Bit 1 - when set, the USBCORE will initiate the host negotiation when suspend mode is entered."]
    #[inline(always)]
    pub fn hostreq(&mut self) -> HostreqW<'_, DevctlSpec> {
        HostreqW::new(self, 1)
    }
    #[doc = "Bit 2 - this read-only bit is set when the USBCORE is acting as a host"]
    #[inline(always)]
    pub fn hostmode(&mut self) -> HostmodeW<'_, DevctlSpec> {
        HostmodeW::new(self, 2)
    }
    #[doc = "Bits 3:4 - encoded value for the VBus level"]
    #[inline(always)]
    pub fn vbus(&mut self) -> VbusW<'_, DevctlSpec> {
        VbusW::new(self, 3)
    }
    #[doc = "Bit 5 - host mode only: this bit is set when a low speed device has been detected being connected to the port"]
    #[inline(always)]
    pub fn lsdev(&mut self) -> LsdevW<'_, DevctlSpec> {
        LsdevW::new(self, 5)
    }
    #[doc = "Bit 6 - host mode only: this bit is set when a full or high speed device has been detected being connected to the port"]
    #[inline(always)]
    pub fn fsdev(&mut self) -> FsdevW<'_, DevctlSpec> {
        FsdevW::new(self, 6)
    }
    #[doc = "Bit 7 - indicates whether the core is operating as a 'A' device or 'B' device"]
    #[inline(always)]
    pub fn bdevice(&mut self) -> BdeviceW<'_, DevctlSpec> {
        BdeviceW::new(self, 7)
    }
}
#[doc = "Selects Whether The Core Is Operating In Peripheral Or Host Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`devctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevctlSpec;
impl crate::RegisterSpec for DevctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devctl::R`](R) reader structure"]
impl crate::Readable for DevctlSpec {}
#[doc = "`write(|w| ..)` method takes [`devctl::W`](W) writer structure"]
impl crate::Writable for DevctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVCTL to value 0x80"]
impl crate::Resettable for DevctlSpec {
    const RESET_VALUE: u8 = 0x80;
}
