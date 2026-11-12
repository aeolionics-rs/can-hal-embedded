#[doc = "Register `CTLINTENCLR` reader"]
pub type R = crate::R<CtlintenclrSpec>;
#[doc = "Register `CTLINTENCLR` writer"]
pub type W = crate::W<CtlintenclrSpec>;
#[doc = "Field `CRRDY` reader - Core Ready Disable"]
pub type CrrdyR = crate::FieldReader;
#[doc = "Field `CRRDY` writer - Core Ready Disable"]
pub type CrrdyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VREFUPD` reader - VREF Update"]
pub type VrefupdR = crate::BitReader;
#[doc = "Field `VREFUPD` writer - VREF Update"]
pub type VrefupdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `VREFRDY` reader - VREF Ready"]
pub type VrefrdyR = crate::BitReader;
#[doc = "Field `VREFRDY` writer - VREF Ready"]
pub type VrefrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PFFUNF` reader - APB FIFO underflow"]
pub type PffunfR = crate::BitReader;
#[doc = "Field `PFFUNF` writer - APB FIFO underflow"]
pub type PffunfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PFFOVF` reader - APB FIFO overflow"]
pub type PffovfR = crate::BitReader;
#[doc = "Field `PFFOVF` writer - APB FIFO overflow"]
pub type PffovfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PFFRDY` reader - APB FIFO Ready"]
pub type PffrdyR = crate::BitReader;
#[doc = "Field `PFFRDY` writer - APB FIFO Ready"]
pub type PffrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PFFHFUL` reader - APB FIFO Half Full"]
pub type PffhfulR = crate::BitReader;
#[doc = "Field `PFFHFUL` writer - APB FIFO Half Full"]
pub type PffhfulW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Core Ready Disable"]
    #[inline(always)]
    pub fn crrdy(&self) -> CrrdyR {
        CrrdyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - VREF Update"]
    #[inline(always)]
    pub fn vrefupd(&self) -> VrefupdR {
        VrefupdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VREF Ready"]
    #[inline(always)]
    pub fn vrefrdy(&self) -> VrefrdyR {
        VrefrdyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - APB FIFO underflow"]
    #[inline(always)]
    pub fn pffunf(&self) -> PffunfR {
        PffunfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - APB FIFO overflow"]
    #[inline(always)]
    pub fn pffovf(&self) -> PffovfR {
        PffovfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - APB FIFO Ready"]
    #[inline(always)]
    pub fn pffrdy(&self) -> PffrdyR {
        PffrdyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - APB FIFO Half Full"]
    #[inline(always)]
    pub fn pffhful(&self) -> PffhfulR {
        PffhfulR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Core Ready Disable"]
    #[inline(always)]
    pub fn crrdy(&mut self) -> CrrdyW<'_, CtlintenclrSpec> {
        CrrdyW::new(self, 0)
    }
    #[doc = "Bit 6 - VREF Update"]
    #[inline(always)]
    pub fn vrefupd(&mut self) -> VrefupdW<'_, CtlintenclrSpec> {
        VrefupdW::new(self, 6)
    }
    #[doc = "Bit 7 - VREF Ready"]
    #[inline(always)]
    pub fn vrefrdy(&mut self) -> VrefrdyW<'_, CtlintenclrSpec> {
        VrefrdyW::new(self, 7)
    }
    #[doc = "Bit 8 - APB FIFO underflow"]
    #[inline(always)]
    pub fn pffunf(&mut self) -> PffunfW<'_, CtlintenclrSpec> {
        PffunfW::new(self, 8)
    }
    #[doc = "Bit 9 - APB FIFO overflow"]
    #[inline(always)]
    pub fn pffovf(&mut self) -> PffovfW<'_, CtlintenclrSpec> {
        PffovfW::new(self, 9)
    }
    #[doc = "Bit 10 - APB FIFO Ready"]
    #[inline(always)]
    pub fn pffrdy(&mut self) -> PffrdyW<'_, CtlintenclrSpec> {
        PffrdyW::new(self, 10)
    }
    #[doc = "Bit 11 - APB FIFO Half Full"]
    #[inline(always)]
    pub fn pffhful(&mut self) -> PffhfulW<'_, CtlintenclrSpec> {
        PffhfulW::new(self, 11)
    }
}
#[doc = "CORE Controller Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`ctlintenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctlintenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlintenclrSpec;
impl crate::RegisterSpec for CtlintenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlintenclr::R`](R) reader structure"]
impl crate::Readable for CtlintenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctlintenclr::W`](W) writer structure"]
impl crate::Writable for CtlintenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0fcf;
}
#[doc = "`reset()` method sets CTLINTENCLR to value 0"]
impl crate::Resettable for CtlintenclrSpec {}
