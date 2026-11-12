#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `SPIRXBFEN` reader - Enablie Interrupt Events via SPIRXBF"]
pub type SpirxbfenR = crate::BitReader;
#[doc = "Field `SPIRXBFEN` writer - Enablie Interrupt Events via SPIRXBF"]
pub type SpirxbfenW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SPITXBEEN` reader - Enablie Interrupt Events via SPITXBE"]
pub type SpitxbeenR = crate::BitReader;
#[doc = "Field `SPITXBEEN` writer - Enablie Interrupt Events via SPITXBE"]
pub type SpitxbeenW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FRMERREN` reader - Enable Interrupt Events via FRMERR"]
pub type FrmerrenR = crate::BitReader;
#[doc = "Field `FRMERREN` writer - Enable Interrupt Events via FRMERR"]
pub type FrmerrenW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SPITUREN` reader - Enable Interrupt Events via SPITUR"]
pub type SpiturenR = crate::BitReader;
#[doc = "Field `SPITUREN` writer - Enable Interrupt Events via SPITUR"]
pub type SpiturenW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SPIROVEN` reader - Enable Interrupt Events via SPIROV"]
pub type SpirovenR = crate::BitReader;
#[doc = "Field `SPIROVEN` writer - Enable Interrupt Events via SPIROV"]
pub type SpirovenW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enablie Interrupt Events via SPIRXBF"]
    #[inline(always)]
    pub fn spirxbfen(&self) -> SpirxbfenR {
        SpirxbfenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Enablie Interrupt Events via SPITXBE"]
    #[inline(always)]
    pub fn spitxbeen(&self) -> SpitxbeenR {
        SpitxbeenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Interrupt Events via FRMERR"]
    #[inline(always)]
    pub fn frmerren(&self) -> FrmerrenR {
        FrmerrenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Interrupt Events via SPITUR"]
    #[inline(always)]
    pub fn spituren(&self) -> SpiturenR {
        SpiturenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Interrupt Events via SPIROV"]
    #[inline(always)]
    pub fn spiroven(&self) -> SpirovenR {
        SpirovenR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enablie Interrupt Events via SPIRXBF"]
    #[inline(always)]
    pub fn spirxbfen(&mut self) -> SpirxbfenW<'_, IntenclrSpec> {
        SpirxbfenW::new(self, 0)
    }
    #[doc = "Bit 4 - Enablie Interrupt Events via SPITXBE"]
    #[inline(always)]
    pub fn spitxbeen(&mut self) -> SpitxbeenW<'_, IntenclrSpec> {
        SpitxbeenW::new(self, 4)
    }
    #[doc = "Bit 15 - Enable Interrupt Events via FRMERR"]
    #[inline(always)]
    pub fn frmerren(&mut self) -> FrmerrenW<'_, IntenclrSpec> {
        FrmerrenW::new(self, 15)
    }
    #[doc = "Bit 27 - Enable Interrupt Events via SPITUR"]
    #[inline(always)]
    pub fn spituren(&mut self) -> SpiturenW<'_, IntenclrSpec> {
        SpiturenW::new(self, 27)
    }
    #[doc = "Bit 30 - Enable Interrupt Events via SPIROV"]
    #[inline(always)]
    pub fn spiroven(&mut self) -> SpirovenW<'_, IntenclrSpec> {
        SpirovenW::new(self, 30)
    }
}
#[doc = "SPI Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x4800_8011;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
