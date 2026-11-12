#[doc = "Register `CALCTRL_FUSES_ADC_SARCORE_12BIT_V7A0_MODE` reader"]
pub type R = crate::R<CalctrlFusesAdcSarcore12bitV7a0ModeSpec>;
#[doc = "Register `CALCTRL_FUSES_ADC_SARCORE_12BIT_V7A0_MODE` writer"]
pub type W = crate::W<CalctrlFusesAdcSarcore12bitV7a0ModeSpec>;
#[doc = "Field `en_cmbf` reader - Enable Common Mode Buffer"]
pub type EnCmbfR = crate::BitReader;
#[doc = "Field `en_cmbf` writer - Enable Common Mode Buffer"]
pub type EnCmbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_dither` reader - Enable Dither"]
pub type EnDitherR = crate::BitReader;
#[doc = "Field `en_dither` writer - Enable Dither"]
pub type EnDitherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dis_faz` reader - Disable auto-zeroing"]
pub type DisFazR = crate::BitReader;
#[doc = "Field `dis_faz` writer - Disable auto-zeroing"]
pub type DisFazW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dis_saz` reader - Disable auto-zeroing"]
pub type DisSazR = crate::BitReader;
#[doc = "Field `dis_saz` writer - Disable auto-zeroing"]
pub type DisSazW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dis_laz` reader - Disable auto-zeroing"]
pub type DisLazR = crate::BitReader;
#[doc = "Field `dis_laz` writer - Disable auto-zeroing"]
pub type DisLazW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_rdac` reader - Disable Power Cycling"]
pub type EnRdacR = crate::BitReader;
#[doc = "Field `en_rdac` writer - Disable Power Cycling"]
pub type EnRdacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dbg_sel` reader - Debug Bus Select"]
pub type DbgSelR = crate::BitReader;
#[doc = "Field `dbg_sel` writer - Debug Bus Select"]
pub type DbgSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sel_del` reader - Scan Mode comp_out"]
pub type SelDelR = crate::BitReader;
#[doc = "Field `sel_del` writer - Scan Mode comp_out"]
pub type SelDelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `t1_dly` reader - Regen Latch Delay"]
pub type T1DlyR = crate::FieldReader;
#[doc = "Field `t1_dly` writer - Regen Latch Delay"]
pub type T1DlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tclk_div` reader - Test Clock Divider"]
pub type TclkDivR = crate::FieldReader;
#[doc = "Field `tclk_div` writer - Test Clock Divider"]
pub type TclkDivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `iadc_1` reader - Current Consumption 1"]
pub type Iadc1R = crate::FieldReader;
#[doc = "Field `iadc_1` writer - Current Consumption 1"]
pub type Iadc1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `iadc_2` reader - Current Consumption 2"]
pub type Iadc2R = crate::FieldReader;
#[doc = "Field `iadc_2` writer - Current Consumption 2"]
pub type Iadc2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `icmp_1` reader - Bias Current Stage 1"]
pub type Icmp1R = crate::FieldReader;
#[doc = "Field `icmp_1` writer - Bias Current Stage 1"]
pub type Icmp1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `icmp_2` reader - Bias Current Stage 2"]
pub type Icmp2R = crate::FieldReader;
#[doc = "Field `icmp_2` writer - Bias Current Stage 2"]
pub type Icmp2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `icmbf` reader - Bias Current Common Mode Buffer"]
pub type IcmbfR = crate::FieldReader;
#[doc = "Field `icmbf` writer - Bias Current Common Mode Buffer"]
pub type IcmbfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `en_ext_bias` reader - Dsiable Internal Bias Circuit"]
pub type EnExtBiasR = crate::BitReader;
#[doc = "Field `en_ext_bias` writer - Dsiable Internal Bias Circuit"]
pub type EnExtBiasW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Common Mode Buffer"]
    #[inline(always)]
    pub fn en_cmbf(&self) -> EnCmbfR {
        EnCmbfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Dither"]
    #[inline(always)]
    pub fn en_dither(&self) -> EnDitherR {
        EnDitherR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable auto-zeroing"]
    #[inline(always)]
    pub fn dis_faz(&self) -> DisFazR {
        DisFazR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable auto-zeroing"]
    #[inline(always)]
    pub fn dis_saz(&self) -> DisSazR {
        DisSazR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable auto-zeroing"]
    #[inline(always)]
    pub fn dis_laz(&self) -> DisLazR {
        DisLazR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Power Cycling"]
    #[inline(always)]
    pub fn en_rdac(&self) -> EnRdacR {
        EnRdacR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debug Bus Select"]
    #[inline(always)]
    pub fn dbg_sel(&self) -> DbgSelR {
        DbgSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan Mode comp_out"]
    #[inline(always)]
    pub fn sel_del(&self) -> SelDelR {
        SelDelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Regen Latch Delay"]
    #[inline(always)]
    pub fn t1_dly(&self) -> T1DlyR {
        T1DlyR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:15 - Test Clock Divider"]
    #[inline(always)]
    pub fn tclk_div(&self) -> TclkDivR {
        TclkDivR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - Current Consumption 1"]
    #[inline(always)]
    pub fn iadc_1(&self) -> Iadc1R {
        Iadc1R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Current Consumption 2"]
    #[inline(always)]
    pub fn iadc_2(&self) -> Iadc2R {
        Iadc2R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Bias Current Stage 1"]
    #[inline(always)]
    pub fn icmp_1(&self) -> Icmp1R {
        Icmp1R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Bias Current Stage 2"]
    #[inline(always)]
    pub fn icmp_2(&self) -> Icmp2R {
        Icmp2R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Bias Current Common Mode Buffer"]
    #[inline(always)]
    pub fn icmbf(&self) -> IcmbfR {
        IcmbfR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Dsiable Internal Bias Circuit"]
    #[inline(always)]
    pub fn en_ext_bias(&self) -> EnExtBiasR {
        EnExtBiasR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Common Mode Buffer"]
    #[inline(always)]
    pub fn en_cmbf(&mut self) -> EnCmbfW<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        EnCmbfW::new(self, 0)
    }
    #[doc = "Bit 2 - Enable Dither"]
    #[inline(always)]
    pub fn en_dither(&mut self) -> EnDitherW<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        EnDitherW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable auto-zeroing"]
    #[inline(always)]
    pub fn dis_faz(&mut self) -> DisFazW<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        DisFazW::new(self, 3)
    }
    #[doc = "Bit 4 - Disable auto-zeroing"]
    #[inline(always)]
    pub fn dis_saz(&mut self) -> DisSazW<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        DisSazW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable auto-zeroing"]
    #[inline(always)]
    pub fn dis_laz(&mut self) -> DisLazW<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        DisLazW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Power Cycling"]
    #[inline(always)]
    pub fn en_rdac(&mut self) -> EnRdacW<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        EnRdacW::new(self, 6)
    }
    #[doc = "Bit 7 - Debug Bus Select"]
    #[inline(always)]
    pub fn dbg_sel(&mut self) -> DbgSelW<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        DbgSelW::new(self, 7)
    }
    #[doc = "Bit 8 - Scan Mode comp_out"]
    #[inline(always)]
    pub fn sel_del(&mut self) -> SelDelW<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        SelDelW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Regen Latch Delay"]
    #[inline(always)]
    pub fn t1_dly(&mut self) -> T1DlyW<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        T1DlyW::new(self, 9)
    }
    #[doc = "Bits 11:15 - Test Clock Divider"]
    #[inline(always)]
    pub fn tclk_div(&mut self) -> TclkDivW<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        TclkDivW::new(self, 11)
    }
    #[doc = "Bits 20:21 - Current Consumption 1"]
    #[inline(always)]
    pub fn iadc_1(&mut self) -> Iadc1W<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        Iadc1W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Current Consumption 2"]
    #[inline(always)]
    pub fn iadc_2(&mut self) -> Iadc2W<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        Iadc2W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Bias Current Stage 1"]
    #[inline(always)]
    pub fn icmp_1(&mut self) -> Icmp1W<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        Icmp1W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Bias Current Stage 2"]
    #[inline(always)]
    pub fn icmp_2(&mut self) -> Icmp2W<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        Icmp2W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Bias Current Common Mode Buffer"]
    #[inline(always)]
    pub fn icmbf(&mut self) -> IcmbfW<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        IcmbfW::new(self, 28)
    }
    #[doc = "Bit 31 - Dsiable Internal Bias Circuit"]
    #[inline(always)]
    pub fn en_ext_bias(&mut self) -> EnExtBiasW<'_, CalctrlFusesAdcSarcore12bitV7a0ModeSpec> {
        EnExtBiasW::new(self, 31)
    }
}
#[doc = "SARCORE Calibration Value\n\nYou can [`read`](crate::Reg::read) this register and get [`calctrl_fuses_adc_sarcore_12bit_v7a0_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctrl_fuses_adc_sarcore_12bit_v7a0_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalctrlFusesAdcSarcore12bitV7a0ModeSpec;
impl crate::RegisterSpec for CalctrlFusesAdcSarcore12bitV7a0ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calctrl_fuses_adc_sarcore_12bit_v7a0_mode::R`](R) reader structure"]
impl crate::Readable for CalctrlFusesAdcSarcore12bitV7a0ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`calctrl_fuses_adc_sarcore_12bit_v7a0_mode::W`](W) writer structure"]
impl crate::Writable for CalctrlFusesAdcSarcore12bitV7a0ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALCTRL_FUSES_ADC_SARCORE_12BIT_V7A0_MODE to value 0"]
impl crate::Resettable for CalctrlFusesAdcSarcore12bitV7a0ModeSpec {}
