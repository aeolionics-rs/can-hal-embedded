#[doc = "Register `POWR` reader"]
pub type R = crate::R<PowrSpec>;
#[doc = "Register `POWR` writer"]
pub type W = crate::W<PowrSpec>;
#[doc = "Field `SUSPEN` reader - set by the cpu to enable suspendm output"]
pub type SuspenR = crate::BitReader;
#[doc = "Field `SUSPEN` writer - set by the cpu to enable suspendm output"]
pub type SuspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPMODE` reader - set by cpu to enter suspend mode"]
pub type SuspmodeR = crate::BitReader;
#[doc = "Field `SUSPMODE` writer - set by cpu to enter suspend mode"]
pub type SuspmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME` reader - set by cpu to generate resume signaling when te device is in suspend"]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESUME` writer - set by cpu to generate resume signaling when te device is in suspend"]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - this bit is set when reset signaling is present on the bus"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - this bit is set when reset signaling is present on the bus"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSMODE` reader - when set this read only bit indicates high speed mode successfully negotiated during usb reset"]
pub type HsmodeR = crate::BitReader;
#[doc = "Field `HSMODE` writer - when set this read only bit indicates high speed mode successfully negotiated during usb reset"]
pub type HsmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEN` reader - when set by cpu the usbcore will negotiate the high speed mode"]
pub type HsenR = crate::BitReader;
#[doc = "Field `HSEN` writer - when set by cpu the usbcore will negotiate the high speed mode"]
pub type HsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTCONN` reader - if enabled the D+ D- lines are enabled when this bit is set by the cpu"]
pub type SoftconnR = crate::BitReader;
#[doc = "Field `SOFTCONN` writer - if enabled the D+ D- lines are enabled when this bit is set by the cpu"]
pub type SoftconnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOUPD` reader - when set by cpu the usbcore will wait for an sof token"]
pub type IsoupdR = crate::BitReader;
#[doc = "Field `ISOUPD` writer - when set by cpu the usbcore will wait for an sof token"]
pub type IsoupdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set by the cpu to enable suspendm output"]
    #[inline(always)]
    pub fn suspen(&self) -> SuspenR {
        SuspenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set by cpu to enter suspend mode"]
    #[inline(always)]
    pub fn suspmode(&self) -> SuspmodeR {
        SuspmodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - set by cpu to generate resume signaling when te device is in suspend"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - this bit is set when reset signaling is present on the bus"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - when set this read only bit indicates high speed mode successfully negotiated during usb reset"]
    #[inline(always)]
    pub fn hsmode(&self) -> HsmodeR {
        HsmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - when set by cpu the usbcore will negotiate the high speed mode"]
    #[inline(always)]
    pub fn hsen(&self) -> HsenR {
        HsenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - if enabled the D+ D- lines are enabled when this bit is set by the cpu"]
    #[inline(always)]
    pub fn softconn(&self) -> SoftconnR {
        SoftconnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - when set by cpu the usbcore will wait for an sof token"]
    #[inline(always)]
    pub fn isoupd(&self) -> IsoupdR {
        IsoupdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set by the cpu to enable suspendm output"]
    #[inline(always)]
    pub fn suspen(&mut self) -> SuspenW<'_, PowrSpec> {
        SuspenW::new(self, 0)
    }
    #[doc = "Bit 1 - set by cpu to enter suspend mode"]
    #[inline(always)]
    pub fn suspmode(&mut self) -> SuspmodeW<'_, PowrSpec> {
        SuspmodeW::new(self, 1)
    }
    #[doc = "Bit 2 - set by cpu to generate resume signaling when te device is in suspend"]
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<'_, PowrSpec> {
        ResumeW::new(self, 2)
    }
    #[doc = "Bit 3 - this bit is set when reset signaling is present on the bus"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, PowrSpec> {
        ResetW::new(self, 3)
    }
    #[doc = "Bit 4 - when set this read only bit indicates high speed mode successfully negotiated during usb reset"]
    #[inline(always)]
    pub fn hsmode(&mut self) -> HsmodeW<'_, PowrSpec> {
        HsmodeW::new(self, 4)
    }
    #[doc = "Bit 5 - when set by cpu the usbcore will negotiate the high speed mode"]
    #[inline(always)]
    pub fn hsen(&mut self) -> HsenW<'_, PowrSpec> {
        HsenW::new(self, 5)
    }
    #[doc = "Bit 6 - if enabled the D+ D- lines are enabled when this bit is set by the cpu"]
    #[inline(always)]
    pub fn softconn(&mut self) -> SoftconnW<'_, PowrSpec> {
        SoftconnW::new(self, 6)
    }
    #[doc = "Bit 7 - when set by cpu the usbcore will wait for an sof token"]
    #[inline(always)]
    pub fn isoupd(&mut self) -> IsoupdW<'_, PowrSpec> {
        IsoupdW::new(self, 7)
    }
}
#[doc = "Used For Controlling Suspend And Resume Signaling And Some Basic Operation Of USBCORE\n\nYou can [`read`](crate::Reg::read) this register and get [`powr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`powr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowrSpec;
impl crate::RegisterSpec for PowrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`powr::R`](R) reader structure"]
impl crate::Readable for PowrSpec {}
#[doc = "`write(|w| ..)` method takes [`powr::W`](W) writer structure"]
impl crate::Writable for PowrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWR to value 0x20"]
impl crate::Resettable for PowrSpec {
    const RESET_VALUE: u8 = 0x20;
}
