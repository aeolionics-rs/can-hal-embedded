#[doc = "Register `SOFTRST` reader"]
pub type R = crate::R<SoftrstSpec>;
#[doc = "Register `SOFTRST` writer"]
pub type W = crate::W<SoftrstSpec>;
#[doc = "Field `NRST` reader - when a 1 is written to this bit, the output nrsto will be asserted (low) within a minimum delay of 7 cycles of the clk input. the output nrsto will be asynchronously asserted and synchronously de-asserted with respect to clk."]
pub type NrstR = crate::BitReader;
#[doc = "Field `NRST` writer - when a 1 is written to this bit, the output nrsto will be asserted (low) within a minimum delay of 7 cycles of the clk input. the output nrsto will be asynchronously asserted and synchronously de-asserted with respect to clk."]
pub type NrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRSTX` reader - when a 1 is written to this bit, the output nrstxo will be asserted (low) within a minimum delay of 7 cycles of the clk input. the output nrstxo will be asynchronously asserted and synchronously de-asserted with respect to xclk."]
pub type NrstxR = crate::BitReader;
#[doc = "Field `NRSTX` writer - when a 1 is written to this bit, the output nrstxo will be asserted (low) within a minimum delay of 7 cycles of the clk input. the output nrstxo will be asynchronously asserted and synchronously de-asserted with respect to xclk."]
pub type NrstxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - when a 1 is written to this bit, the output nrsto will be asserted (low) within a minimum delay of 7 cycles of the clk input. the output nrsto will be asynchronously asserted and synchronously de-asserted with respect to clk."]
    #[inline(always)]
    pub fn nrst(&self) -> NrstR {
        NrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when a 1 is written to this bit, the output nrstxo will be asserted (low) within a minimum delay of 7 cycles of the clk input. the output nrstxo will be asynchronously asserted and synchronously de-asserted with respect to xclk."]
    #[inline(always)]
    pub fn nrstx(&self) -> NrstxR {
        NrstxR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - when a 1 is written to this bit, the output nrsto will be asserted (low) within a minimum delay of 7 cycles of the clk input. the output nrsto will be asynchronously asserted and synchronously de-asserted with respect to clk."]
    #[inline(always)]
    pub fn nrst(&mut self) -> NrstW<'_, SoftrstSpec> {
        NrstW::new(self, 0)
    }
    #[doc = "Bit 1 - when a 1 is written to this bit, the output nrstxo will be asserted (low) within a minimum delay of 7 cycles of the clk input. the output nrstxo will be asynchronously asserted and synchronously de-asserted with respect to xclk."]
    #[inline(always)]
    pub fn nrstx(&mut self) -> NrstxW<'_, SoftrstSpec> {
        NrstxW::new(self, 1)
    }
}
#[doc = "Assert Low The Output Reset Signals NRSTO and NRSTXO\n\nYou can [`read`](crate::Reg::read) this register and get [`softrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftrstSpec;
impl crate::RegisterSpec for SoftrstSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`softrst::R`](R) reader structure"]
impl crate::Readable for SoftrstSpec {}
#[doc = "`write(|w| ..)` method takes [`softrst::W`](W) writer structure"]
impl crate::Writable for SoftrstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOFTRST to value 0"]
impl crate::Resettable for SoftrstSpec {}
