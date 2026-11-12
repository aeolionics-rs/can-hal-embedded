#[doc = "Register `TAPCON` reader"]
pub type R = crate::R<TapconSpec>;
#[doc = "Register `TAPCON` writer"]
pub type W = crate::W<TapconSpec>;
#[doc = "Field `CLKOUTDLY` reader - Used to add delay on spi clock output.16 taps available."]
pub type ClkoutdlyR = crate::FieldReader;
#[doc = "Field `CLKOUTDLY` writer - Used to add delay on spi clock output.16 taps available."]
pub type ClkoutdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAOUTDLY` reader - Used to add delay on spi data outputs (1/2/4lanes). 16 taps available."]
pub type DataoutdlyR = crate::FieldReader;
#[doc = "Field `DATAOUTDLY` writer - Used to add delay on spi data outputs (1/2/4lanes). 16 taps available."]
pub type DataoutdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDRCLKINDLY` reader - Used only in SDR mode. This field is used to add tap delay cells on the spi clock input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (CLK_IN_DLY_CNT Vs CLK_IN_ALT_DLY_CNT)."]
pub type SdrclkindlyR = crate::FieldReader;
#[doc = "Field `SDRCLKINDLY` writer - Used only in SDR mode. This field is used to add tap delay cells on the spi clock input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (CLK_IN_DLY_CNT Vs CLK_IN_ALT_DLY_CNT)."]
pub type SdrclkindlyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DDRDATINDLY` reader - Used only in DDR mode. This field is used to add tap delay cells on the data input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (DATA_IN_ALT_DLY_CNT Vs DATA_IN_DLY_CNT)."]
pub type DdrdatindlyR = crate::FieldReader;
#[doc = "Field `DDRDATINDLY` writer - Used only in DDR mode. This field is used to add tap delay cells on the data input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (DATA_IN_ALT_DLY_CNT Vs DATA_IN_DLY_CNT)."]
pub type DdrdatindlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDRDATINDLY` reader - Used only in SDR mode. This field is used to add tap delay cells on the Data input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (DATA_IN_ALT_DLY_CNT Vs DATA_IN_DLY_CNT)."]
pub type SdrdatindlyR = crate::FieldReader;
#[doc = "Field `SDRDATINDLY` writer - Used only in SDR mode. This field is used to add tap delay cells on the Data input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (DATA_IN_ALT_DLY_CNT Vs DATA_IN_DLY_CNT)."]
pub type SdrdatindlyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DDRCLKINDLY` reader - Used only in DDR mode. This field is used to add tap delay cells on the spi clock input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (CLK_IN_DLY_CNT Vs CLK_IN_ALT_DLY_CNT)."]
pub type DdrclkindlyR = crate::FieldReader;
#[doc = "Field `DDRCLKINDLY` writer - Used only in DDR mode. This field is used to add tap delay cells on the spi clock input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (CLK_IN_DLY_CNT Vs CLK_IN_ALT_DLY_CNT)."]
pub type DdrclkindlyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - Used to add delay on spi clock output.16 taps available."]
    #[inline(always)]
    pub fn clkoutdly(&self) -> ClkoutdlyR {
        ClkoutdlyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Used to add delay on spi data outputs (1/2/4lanes). 16 taps available."]
    #[inline(always)]
    pub fn dataoutdly(&self) -> DataoutdlyR {
        DataoutdlyR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Used only in SDR mode. This field is used to add tap delay cells on the spi clock input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (CLK_IN_DLY_CNT Vs CLK_IN_ALT_DLY_CNT)."]
    #[inline(always)]
    pub fn sdrclkindly(&self) -> SdrclkindlyR {
        SdrclkindlyR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - Used only in DDR mode. This field is used to add tap delay cells on the data input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (DATA_IN_ALT_DLY_CNT Vs DATA_IN_DLY_CNT)."]
    #[inline(always)]
    pub fn ddrdatindly(&self) -> DdrdatindlyR {
        DdrdatindlyR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Used only in SDR mode. This field is used to add tap delay cells on the Data input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (DATA_IN_ALT_DLY_CNT Vs DATA_IN_DLY_CNT)."]
    #[inline(always)]
    pub fn sdrdatindly(&self) -> SdrdatindlyR {
        SdrdatindlyR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - Used only in DDR mode. This field is used to add tap delay cells on the spi clock input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (CLK_IN_DLY_CNT Vs CLK_IN_ALT_DLY_CNT)."]
    #[inline(always)]
    pub fn ddrclkindly(&self) -> DdrclkindlyR {
        DdrclkindlyR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Used to add delay on spi clock output.16 taps available."]
    #[inline(always)]
    pub fn clkoutdly(&mut self) -> ClkoutdlyW<'_, TapconSpec> {
        ClkoutdlyW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Used to add delay on spi data outputs (1/2/4lanes). 16 taps available."]
    #[inline(always)]
    pub fn dataoutdly(&mut self) -> DataoutdlyW<'_, TapconSpec> {
        DataoutdlyW::new(self, 4)
    }
    #[doc = "Bits 8:13 - Used only in SDR mode. This field is used to add tap delay cells on the spi clock input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (CLK_IN_DLY_CNT Vs CLK_IN_ALT_DLY_CNT)."]
    #[inline(always)]
    pub fn sdrclkindly(&mut self) -> SdrclkindlyW<'_, TapconSpec> {
        SdrclkindlyW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Used only in DDR mode. This field is used to add tap delay cells on the data input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (DATA_IN_ALT_DLY_CNT Vs DATA_IN_DLY_CNT)."]
    #[inline(always)]
    pub fn ddrdatindly(&mut self) -> DdrdatindlyW<'_, TapconSpec> {
        DdrdatindlyW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Used only in SDR mode. This field is used to add tap delay cells on the Data input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (DATA_IN_ALT_DLY_CNT Vs DATA_IN_DLY_CNT)."]
    #[inline(always)]
    pub fn sdrdatindly(&mut self) -> SdrdatindlyW<'_, TapconSpec> {
        SdrdatindlyW::new(self, 20)
    }
    #[doc = "Bits 24:29 - Used only in DDR mode. This field is used to add tap delay cells on the spi clock input. Based on the mode of operation (SDR Vs DDR), the controller dynamically picks the appropriate delay values (CLK_IN_DLY_CNT Vs CLK_IN_ALT_DLY_CNT)."]
    #[inline(always)]
    pub fn ddrclkindly(&mut self) -> DdrclkindlyW<'_, TapconSpec> {
        DdrclkindlyW::new(self, 24)
    }
}
#[doc = "Tap Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tapcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tapcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TapconSpec;
impl crate::RegisterSpec for TapconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tapcon::R`](R) reader structure"]
impl crate::Readable for TapconSpec {}
#[doc = "`write(|w| ..)` method takes [`tapcon::W`](W) writer structure"]
impl crate::Writable for TapconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TAPCON to value 0x0011_0021"]
impl crate::Resettable for TapconSpec {
    const RESET_VALUE: u32 = 0x0011_0021;
}
