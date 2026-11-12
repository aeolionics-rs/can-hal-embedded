#[doc = "Register `PFFCTRL` reader"]
pub type R = crate::R<PffctrlSpec>;
#[doc = "Register `PFFCTRL` writer"]
pub type W = crate::W<PffctrlSpec>;
#[doc = "Field `PFFEN` reader - APB FIFO Enable"]
pub type PffenR = crate::BitReader;
#[doc = "Field `PFFEN` writer - APB FIFO Enable"]
pub type PffenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFFCR` reader - APB CORE FIFO Enable"]
pub type PffcrR = crate::FieldReader;
#[doc = "Field `PFFCR` writer - APB CORE FIFO Enable"]
pub type PffcrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "DMA APB FIFO Data Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pffrdydmselect {
    #[doc = "0: Selects CTLINTFLAG.PFFHFUL for the ADC DMA PFFRDY trigger signal to the DMAC"]
    CtlintflagPffhful = 0,
    #[doc = "1: Selects CTLINTFLAG.PFFRDY for the ADC DMA PFFRDY trigger signal to the DMAC"]
    CtlintflagPffrdy = 1,
}
impl From<Pffrdydmselect> for bool {
    #[inline(always)]
    fn from(variant: Pffrdydmselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFFRDYDM` reader - DMA APB FIFO Data Ready"]
pub type PffrdydmR = crate::BitReader<Pffrdydmselect>;
impl PffrdydmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pffrdydmselect {
        match self.bits {
            false => Pffrdydmselect::CtlintflagPffhful,
            true => Pffrdydmselect::CtlintflagPffrdy,
        }
    }
    #[doc = "Selects CTLINTFLAG.PFFHFUL for the ADC DMA PFFRDY trigger signal to the DMAC"]
    #[inline(always)]
    pub fn is_ctlintflag_pffhful(&self) -> bool {
        *self == Pffrdydmselect::CtlintflagPffhful
    }
    #[doc = "Selects CTLINTFLAG.PFFRDY for the ADC DMA PFFRDY trigger signal to the DMAC"]
    #[inline(always)]
    pub fn is_ctlintflag_pffrdy(&self) -> bool {
        *self == Pffrdydmselect::CtlintflagPffrdy
    }
}
#[doc = "Field `PFFRDYDM` writer - DMA APB FIFO Data Ready"]
pub type PffrdydmW<'a, REG> = crate::BitWriter<'a, REG, Pffrdydmselect>;
impl<'a, REG> PffrdydmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects CTLINTFLAG.PFFHFUL for the ADC DMA PFFRDY trigger signal to the DMAC"]
    #[inline(always)]
    pub fn ctlintflag_pffhful(self) -> &'a mut crate::W<REG> {
        self.variant(Pffrdydmselect::CtlintflagPffhful)
    }
    #[doc = "Selects CTLINTFLAG.PFFRDY for the ADC DMA PFFRDY trigger signal to the DMAC"]
    #[inline(always)]
    pub fn ctlintflag_pffrdy(self) -> &'a mut crate::W<REG> {
        self.variant(Pffrdydmselect::CtlintflagPffrdy)
    }
}
impl R {
    #[doc = "Bit 1 - APB FIFO Enable"]
    #[inline(always)]
    pub fn pffen(&self) -> PffenR {
        PffenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - APB CORE FIFO Enable"]
    #[inline(always)]
    pub fn pffcr(&self) -> PffcrR {
        PffcrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - DMA APB FIFO Data Ready"]
    #[inline(always)]
    pub fn pffrdydm(&self) -> PffrdydmR {
        PffrdydmR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - APB FIFO Enable"]
    #[inline(always)]
    pub fn pffen(&mut self) -> PffenW<'_, PffctrlSpec> {
        PffenW::new(self, 1)
    }
    #[doc = "Bits 4:7 - APB CORE FIFO Enable"]
    #[inline(always)]
    pub fn pffcr(&mut self) -> PffcrW<'_, PffctrlSpec> {
        PffcrW::new(self, 4)
    }
    #[doc = "Bit 16 - DMA APB FIFO Data Ready"]
    #[inline(always)]
    pub fn pffrdydm(&mut self) -> PffrdydmW<'_, PffctrlSpec> {
        PffrdydmW::new(self, 16)
    }
}
#[doc = "APB FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pffctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pffctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PffctrlSpec;
impl crate::RegisterSpec for PffctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pffctrl::R`](R) reader structure"]
impl crate::Readable for PffctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pffctrl::W`](W) writer structure"]
impl crate::Writable for PffctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PFFCTRL to value 0"]
impl crate::Resettable for PffctrlSpec {}
