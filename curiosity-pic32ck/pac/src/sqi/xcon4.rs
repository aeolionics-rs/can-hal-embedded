#[doc = "Register `XCON4` reader"]
pub type R = crate::R<Xcon4Spec>;
#[doc = "Register `XCON4` writer"]
pub type W = crate::W<Xcon4Spec>;
#[doc = "Field `INIT2CMD1` reader - 8bit SPI Init2 Code1 value."]
pub type Init2cmd1R = crate::FieldReader;
#[doc = "Field `INIT2CMD1` writer - 8bit SPI Init2 Code1 value."]
pub type Init2cmd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INIT2CMD2` reader - 8bit SPI Init2 Code2 value"]
pub type Init2cmd2R = crate::FieldReader;
#[doc = "Field `INIT2CMD2` writer - 8bit SPI Init2 Code2 value"]
pub type Init2cmd2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INIT2CMD3` reader - 8bit SPI Init2 Code3 value"]
pub type Init2cmd3R = crate::FieldReader;
#[doc = "Field `INIT2CMD3` writer - 8bit SPI Init2 Code3 value"]
pub type Init2cmd3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INIT2TYPE` reader - SPI TYPE INIT2: 00 - Single lane mode. 01 - Dual lane mode. 10 - Quad lane mode. 11 - Reserved for Future use. Based on this field, the boot controller will send the INIT2 Code in 1/2/4 lane."]
pub type Init2typeR = crate::FieldReader;
#[doc = "Field `INIT2TYPE` writer - SPI TYPE INIT2: 00 - Single lane mode. 01 - Dual lane mode. 10 - Quad lane mode. 11 - Reserved for Future use. Based on this field, the boot controller will send the INIT2 Code in 1/2/4 lane."]
pub type Init2typeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INIT2COUNT` reader - 2bit SPI INIT2 Count. Indicates whether 1, 2 or 3 bytes of INIT2 Code value are being sent."]
pub type Init2countR = crate::FieldReader;
#[doc = "Field `INIT2COUNT` writer - 2bit SPI INIT2 Count. Indicates whether 1, 2 or 3 bytes of INIT2 Code value are being sent."]
pub type Init2countW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INIT2SCHECK` reader - SPI Init2 Status Check: 0 - do not check status 1 - check status If this bit is set to 1, the hardware will issue the status command automatically after thecurrent operation and wait for busy bit to clear. The hardware will issue the next command only after this Status check phase. Note: The Status Command will be formed according to the settings in the SPI Status Control Reg (@0x60)"]
pub type Init2scheckR = crate::BitReader;
#[doc = "Field `INIT2SCHECK` writer - SPI Init2 Status Check: 0 - do not check status 1 - check status If this bit is set to 1, the hardware will issue the status command automatically after thecurrent operation and wait for busy bit to clear. The hardware will issue the next command only after this Status check phase. Note: The Status Command will be formed according to the settings in the SPI Status Control Reg (@0x60)"]
pub type Init2scheckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 8bit SPI Init2 Code1 value."]
    #[inline(always)]
    pub fn init2cmd1(&self) -> Init2cmd1R {
        Init2cmd1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 8bit SPI Init2 Code2 value"]
    #[inline(always)]
    pub fn init2cmd2(&self) -> Init2cmd2R {
        Init2cmd2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 8bit SPI Init2 Code3 value"]
    #[inline(always)]
    pub fn init2cmd3(&self) -> Init2cmd3R {
        Init2cmd3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - SPI TYPE INIT2: 00 - Single lane mode. 01 - Dual lane mode. 10 - Quad lane mode. 11 - Reserved for Future use. Based on this field, the boot controller will send the INIT2 Code in 1/2/4 lane."]
    #[inline(always)]
    pub fn init2type(&self) -> Init2typeR {
        Init2typeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - 2bit SPI INIT2 Count. Indicates whether 1, 2 or 3 bytes of INIT2 Code value are being sent."]
    #[inline(always)]
    pub fn init2count(&self) -> Init2countR {
        Init2countR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - SPI Init2 Status Check: 0 - do not check status 1 - check status If this bit is set to 1, the hardware will issue the status command automatically after thecurrent operation and wait for busy bit to clear. The hardware will issue the next command only after this Status check phase. Note: The Status Command will be formed according to the settings in the SPI Status Control Reg (@0x60)"]
    #[inline(always)]
    pub fn init2scheck(&self) -> Init2scheckR {
        Init2scheckR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8bit SPI Init2 Code1 value."]
    #[inline(always)]
    pub fn init2cmd1(&mut self) -> Init2cmd1W<'_, Xcon4Spec> {
        Init2cmd1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 8bit SPI Init2 Code2 value"]
    #[inline(always)]
    pub fn init2cmd2(&mut self) -> Init2cmd2W<'_, Xcon4Spec> {
        Init2cmd2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 8bit SPI Init2 Code3 value"]
    #[inline(always)]
    pub fn init2cmd3(&mut self) -> Init2cmd3W<'_, Xcon4Spec> {
        Init2cmd3W::new(self, 16)
    }
    #[doc = "Bits 24:25 - SPI TYPE INIT2: 00 - Single lane mode. 01 - Dual lane mode. 10 - Quad lane mode. 11 - Reserved for Future use. Based on this field, the boot controller will send the INIT2 Code in 1/2/4 lane."]
    #[inline(always)]
    pub fn init2type(&mut self) -> Init2typeW<'_, Xcon4Spec> {
        Init2typeW::new(self, 24)
    }
    #[doc = "Bits 26:27 - 2bit SPI INIT2 Count. Indicates whether 1, 2 or 3 bytes of INIT2 Code value are being sent."]
    #[inline(always)]
    pub fn init2count(&mut self) -> Init2countW<'_, Xcon4Spec> {
        Init2countW::new(self, 26)
    }
    #[doc = "Bit 28 - SPI Init2 Status Check: 0 - do not check status 1 - check status If this bit is set to 1, the hardware will issue the status command automatically after thecurrent operation and wait for busy bit to clear. The hardware will issue the next command only after this Status check phase. Note: The Status Command will be formed according to the settings in the SPI Status Control Reg (@0x60)"]
    #[inline(always)]
    pub fn init2scheck(&mut self) -> Init2scheckW<'_, Xcon4Spec> {
        Init2scheckW::new(self, 28)
    }
}
#[doc = "SPI XIP Control4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`xcon4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcon4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xcon4Spec;
impl crate::RegisterSpec for Xcon4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xcon4::R`](R) reader structure"]
impl crate::Readable for Xcon4Spec {}
#[doc = "`write(|w| ..)` method takes [`xcon4::W`](W) writer structure"]
impl crate::Writable for Xcon4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XCON4 to value 0"]
impl crate::Resettable for Xcon4Spec {}
