#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `SPIRXBFEN` reader - Enablie Interrupt Events via SPIRXBF"]
pub type SpirxbfenR = crate::BitReader;
#[doc = "Field `SPIRXBFEN` writer - Enablie Interrupt Events via SPIRXBF"]
pub type SpirxbfenW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SPITXBEEN` reader - Enablie Interrupt Events via SPITXBE"]
pub type SpitxbeenR = crate::BitReader;
#[doc = "Field `SPITXBEEN` writer - Enablie Interrupt Events via SPITXBE"]
pub type SpitxbeenW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `FRMERREN` reader - Enable Interrupt Events via FRMERR"]
pub type FrmerrenR = crate::BitReader;
#[doc = "Field `FRMERREN` writer - Enable Interrupt Events via FRMERR"]
pub type FrmerrenW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SPITUREN` reader - Enable Interrupt Events via SPITUR"]
pub type SpiturenR = crate::BitReader;
#[doc = "Field `SPITUREN` writer - Enable Interrupt Events via SPITUR"]
pub type SpiturenW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SPIROVEN` reader - Enable Interrupt Events via SPIROV"]
pub type SpirovenR = crate::BitReader;
#[doc = "Field `SPIROVEN` writer - Enable Interrupt Events via SPIROV"]
pub type SpirovenW<'a, REG> = crate::BitWriter1S<'a, REG>;
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
    pub fn spirxbfen(&mut self) -> SpirxbfenW<'_, IntensetSpec> {
        SpirxbfenW::new(self, 0)
    }
    #[doc = "Bit 4 - Enablie Interrupt Events via SPITXBE"]
    #[inline(always)]
    pub fn spitxbeen(&mut self) -> SpitxbeenW<'_, IntensetSpec> {
        SpitxbeenW::new(self, 4)
    }
    #[doc = "Bit 15 - Enable Interrupt Events via FRMERR"]
    #[inline(always)]
    pub fn frmerren(&mut self) -> FrmerrenW<'_, IntensetSpec> {
        FrmerrenW::new(self, 15)
    }
    #[doc = "Bit 27 - Enable Interrupt Events via SPITUR"]
    #[inline(always)]
    pub fn spituren(&mut self) -> SpiturenW<'_, IntensetSpec> {
        SpiturenW::new(self, 27)
    }
    #[doc = "Bit 30 - Enable Interrupt Events via SPIROV"]
    #[inline(always)]
    pub fn spiroven(&mut self) -> SpirovenW<'_, IntensetSpec> {
        SpirovenW::new(self, 30)
    }
}
#[doc = "SPI Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x4800_8011;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
