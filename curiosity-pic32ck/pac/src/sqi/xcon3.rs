#[doc = "Register `XCON3` reader"]
pub type R = crate::R<Xcon3Spec>;
#[doc = "Register `XCON3` writer"]
pub type W = crate::W<Xcon3Spec>;
#[doc = "Field `INIT1CMD1` reader - 8bit SPI Init1 Code1 value."]
pub type Init1cmd1R = crate::FieldReader;
#[doc = "Field `INIT1CMD1` writer - 8bit SPI Init1 Code1 value."]
pub type Init1cmd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INIT1CMD2` reader - 8bit SPI Init1 Code2 value."]
pub type Init1cmd2R = crate::FieldReader;
#[doc = "Field `INIT1CMD2` writer - 8bit SPI Init1 Code2 value."]
pub type Init1cmd2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INIT1CMD3` reader - 8bit SPI Init1 Code3 value."]
pub type Init1cmd3R = crate::FieldReader;
#[doc = "Field `INIT1CMD3` writer - 8bit SPI Init1 Code3 value."]
pub type Init1cmd3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INIT1TYPE` reader - SPI TYPE INIT1: 00 - Single lane mode , 01 - Dual lane mode , 10 - Quad lane mode , 11 - Reserved for Future use. Based on this field, the boot controller will send the INIT1 Code in 1/2/4 lane."]
pub type Init1typeR = crate::FieldReader;
#[doc = "Field `INIT1TYPE` writer - SPI TYPE INIT1: 00 - Single lane mode , 01 - Dual lane mode , 10 - Quad lane mode , 11 - Reserved for Future use. Based on this field, the boot controller will send the INIT1 Code in 1/2/4 lane."]
pub type Init1typeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INIT1COUNT` reader - 2bit SPI INIT1 Count. Indicates whether 1, 2 or 3 bytes of INIT1 Code value are being sent."]
pub type Init1countR = crate::FieldReader;
#[doc = "Field `INIT1COUNT` writer - 2bit SPI INIT1 Count. Indicates whether 1, 2 or 3 bytes of INIT1 Code value are being sent."]
pub type Init1countW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INIT1SCHECK` reader - SPI Init1 Status Check: 0 - do not check status, 1 - check status. If this bit is set to 1, the hardware will issue the status command automatically after the current operation and wait for busy bit to clear. The hardware will issue the next command only after this Status check phase. Note: The Status Command will be formed according to the settings in the SPI Status Control Reg (@0x60)"]
pub type Init1scheckR = crate::BitReader;
#[doc = "Field `INIT1SCHECK` writer - SPI Init1 Status Check: 0 - do not check status, 1 - check status. If this bit is set to 1, the hardware will issue the status command automatically after the current operation and wait for busy bit to clear. The hardware will issue the next command only after this Status check phase. Note: The Status Command will be formed according to the settings in the SPI Status Control Reg (@0x60)"]
pub type Init1scheckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 8bit SPI Init1 Code1 value."]
    #[inline(always)]
    pub fn init1cmd1(&self) -> Init1cmd1R {
        Init1cmd1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8bit SPI Init1 Code2 value."]
    #[inline(always)]
    pub fn init1cmd2(&self) -> Init1cmd2R {
        Init1cmd2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8bit SPI Init1 Code3 value."]
    #[inline(always)]
    pub fn init1cmd3(&self) -> Init1cmd3R {
        Init1cmd3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - SPI TYPE INIT1: 00 - Single lane mode , 01 - Dual lane mode , 10 - Quad lane mode , 11 - Reserved for Future use. Based on this field, the boot controller will send the INIT1 Code in 1/2/4 lane."]
    #[inline(always)]
    pub fn init1type(&self) -> Init1typeR {
        Init1typeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - 2bit SPI INIT1 Count. Indicates whether 1, 2 or 3 bytes of INIT1 Code value are being sent."]
    #[inline(always)]
    pub fn init1count(&self) -> Init1countR {
        Init1countR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - SPI Init1 Status Check: 0 - do not check status, 1 - check status. If this bit is set to 1, the hardware will issue the status command automatically after the current operation and wait for busy bit to clear. The hardware will issue the next command only after this Status check phase. Note: The Status Command will be formed according to the settings in the SPI Status Control Reg (@0x60)"]
    #[inline(always)]
    pub fn init1scheck(&self) -> Init1scheckR {
        Init1scheckR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8bit SPI Init1 Code1 value."]
    #[inline(always)]
    pub fn init1cmd1(&mut self) -> Init1cmd1W<'_, Xcon3Spec> {
        Init1cmd1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8bit SPI Init1 Code2 value."]
    #[inline(always)]
    pub fn init1cmd2(&mut self) -> Init1cmd2W<'_, Xcon3Spec> {
        Init1cmd2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8bit SPI Init1 Code3 value."]
    #[inline(always)]
    pub fn init1cmd3(&mut self) -> Init1cmd3W<'_, Xcon3Spec> {
        Init1cmd3W::new(self, 16)
    }
    #[doc = "Bits 24:25 - SPI TYPE INIT1: 00 - Single lane mode , 01 - Dual lane mode , 10 - Quad lane mode , 11 - Reserved for Future use. Based on this field, the boot controller will send the INIT1 Code in 1/2/4 lane."]
    #[inline(always)]
    pub fn init1type(&mut self) -> Init1typeW<'_, Xcon3Spec> {
        Init1typeW::new(self, 24)
    }
    #[doc = "Bits 26:27 - 2bit SPI INIT1 Count. Indicates whether 1, 2 or 3 bytes of INIT1 Code value are being sent."]
    #[inline(always)]
    pub fn init1count(&mut self) -> Init1countW<'_, Xcon3Spec> {
        Init1countW::new(self, 26)
    }
    #[doc = "Bit 28 - SPI Init1 Status Check: 0 - do not check status, 1 - check status. If this bit is set to 1, the hardware will issue the status command automatically after the current operation and wait for busy bit to clear. The hardware will issue the next command only after this Status check phase. Note: The Status Command will be formed according to the settings in the SPI Status Control Reg (@0x60)"]
    #[inline(always)]
    pub fn init1scheck(&mut self) -> Init1scheckW<'_, Xcon3Spec> {
        Init1scheckW::new(self, 28)
    }
}
#[doc = "SPI XIP Control 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xcon3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcon3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xcon3Spec;
impl crate::RegisterSpec for Xcon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xcon3::R`](R) reader structure"]
impl crate::Readable for Xcon3Spec {}
#[doc = "`write(|w| ..)` method takes [`xcon3::W`](W) writer structure"]
impl crate::Writable for Xcon3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XCON3 to value 0"]
impl crate::Resettable for Xcon3Spec {}
