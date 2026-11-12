#[doc = "Register `LPMATTR` reader"]
pub type R = crate::R<LpmattrSpec>;
#[doc = "Register `LPMATTR` writer"]
pub type W = crate::W<LpmattrSpec>;
#[doc = "this value is provided by the host to the peripheral to indicate what state the peripheral must transition to after the receipt and acceptance of a lpm transaction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Linkstateselect {
    #[doc = "1: Sleep State (L1)"]
    Sleepstate = 1,
}
impl From<Linkstateselect> for u8 {
    #[inline(always)]
    fn from(variant: Linkstateselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Linkstateselect {
    type Ux = u8;
}
impl crate::IsEnum for Linkstateselect {}
#[doc = "Field `LINKSTATE` reader - this value is provided by the host to the peripheral to indicate what state the peripheral must transition to after the receipt and acceptance of a lpm transaction."]
pub type LinkstateR = crate::FieldReader<Linkstateselect>;
impl LinkstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Linkstateselect> {
        match self.bits {
            1 => Some(Linkstateselect::Sleepstate),
            _ => None,
        }
    }
    #[doc = "Sleep State (L1)"]
    #[inline(always)]
    pub fn is_sleepstate(&self) -> bool {
        *self == Linkstateselect::Sleepstate
    }
}
#[doc = "Field `LINKSTATE` writer - this value is provided by the host to the peripheral to indicate what state the peripheral must transition to after the receipt and acceptance of a lpm transaction."]
pub type LinkstateW<'a, REG> = crate::FieldWriter<'a, REG, 4, Linkstateselect>;
impl<'a, REG> LinkstateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sleep State (L1)"]
    #[inline(always)]
    pub fn sleepstate(self) -> &'a mut crate::W<REG> {
        self.variant(Linkstateselect::Sleepstate)
    }
}
#[doc = "Field `HIRD` reader - this is the host initiated resume duration."]
pub type HirdR = crate::FieldReader;
#[doc = "Field `HIRD` writer - this is the host initiated resume duration."]
pub type HirdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RMTWAK` reader - this bit is the remote wakeup enable bit."]
pub type RmtwakR = crate::BitReader;
#[doc = "Field `RMTWAK` writer - this bit is the remote wakeup enable bit."]
pub type RmtwakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDPOINT` reader - this is the endpnt that in the token packet of the lpm transaction."]
pub type EndpointR = crate::FieldReader;
#[doc = "Field `ENDPOINT` writer - this is the endpnt that in the token packet of the lpm transaction."]
pub type EndpointW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - this value is provided by the host to the peripheral to indicate what state the peripheral must transition to after the receipt and acceptance of a lpm transaction."]
    #[inline(always)]
    pub fn linkstate(&self) -> LinkstateR {
        LinkstateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - this is the host initiated resume duration."]
    #[inline(always)]
    pub fn hird(&self) -> HirdR {
        HirdR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - this bit is the remote wakeup enable bit."]
    #[inline(always)]
    pub fn rmtwak(&self) -> RmtwakR {
        RmtwakR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - this is the endpnt that in the token packet of the lpm transaction."]
    #[inline(always)]
    pub fn endpoint(&self) -> EndpointR {
        EndpointR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - this value is provided by the host to the peripheral to indicate what state the peripheral must transition to after the receipt and acceptance of a lpm transaction."]
    #[inline(always)]
    pub fn linkstate(&mut self) -> LinkstateW<'_, LpmattrSpec> {
        LinkstateW::new(self, 0)
    }
    #[doc = "Bits 4:7 - this is the host initiated resume duration."]
    #[inline(always)]
    pub fn hird(&mut self) -> HirdW<'_, LpmattrSpec> {
        HirdW::new(self, 4)
    }
    #[doc = "Bit 8 - this bit is the remote wakeup enable bit."]
    #[inline(always)]
    pub fn rmtwak(&mut self) -> RmtwakW<'_, LpmattrSpec> {
        RmtwakW::new(self, 8)
    }
    #[doc = "Bits 12:15 - this is the endpnt that in the token packet of the lpm transaction."]
    #[inline(always)]
    pub fn endpoint(&mut self) -> EndpointW<'_, LpmattrSpec> {
        EndpointW::new(self, 12)
    }
}
#[doc = "Defines The Attributes Of An LPM Transaction And Sleep Cycle.\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmattr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmattr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmattrSpec;
impl crate::RegisterSpec for LpmattrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`lpmattr::R`](R) reader structure"]
impl crate::Readable for LpmattrSpec {}
#[doc = "`write(|w| ..)` method takes [`lpmattr::W`](W) writer structure"]
impl crate::Writable for LpmattrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPMATTR to value 0"]
impl crate::Resettable for LpmattrSpec {}
