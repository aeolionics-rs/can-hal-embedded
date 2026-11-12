#[doc = "Register `CHINTF` reader"]
pub type R = crate::R<ChintfSpec>;
#[doc = "Register `CHINTF` writer"]
pub type W = crate::W<ChintfSpec>;
#[doc = "Field `SD` reader - Start Detected Interrupt Flag (0: none ; 1: Start Detected)"]
pub type SdR = crate::BitReader;
#[doc = "Field `SD` writer - Start Detected Interrupt Flag (0: none ; 1: Start Detected)"]
pub type SdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TA` reader - Transfer Abort Interrupt Flag (0: none; 1: Transfer Aborted)"]
pub type TaR = crate::BitReader;
#[doc = "Field `TA` writer - Transfer Abort Interrupt Flag (0: none; 1: Transfer Aborted)"]
pub type TaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CC` reader - Cell Transfer Complete Interrupt Flag (0: none ; 1: complete)"]
pub type CcR = crate::BitReader;
#[doc = "Field `CC` writer - Cell Transfer Complete Interrupt Flag (0: none ; 1: complete)"]
pub type CcW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BC` reader - Block Transfer Complete Interrupt Flag (0: none; 1:complete;)"]
pub type BcR = crate::BitReader;
#[doc = "Field `BC` writer - Block Transfer Complete Interrupt Flag (0: none; 1:complete;)"]
pub type BcW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BH` reader - Block Transfer Half Complete Interrupt Flag (0: none; 1: half complete)"]
pub type BhR = crate::BitReader;
#[doc = "Field `BH` writer - Block Transfer Half Complete Interrupt Flag (0: none; 1: half complete)"]
pub type BhW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LL` reader - Linked List Done Interrupt Flag (0: none; 1: done)"]
pub type LlR = crate::BitReader;
#[doc = "Field `LL` writer - Linked List Done Interrupt Flag (0: none; 1: done)"]
pub type LlW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WRE` reader - Write Error Flag (0: none; 1: Write Error)"]
pub type WreR = crate::BitReader;
#[doc = "Field `WRE` writer - Write Error Flag (0: none; 1: Write Error)"]
pub type WreW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RDE` reader - Read Error Flag (0: none; 1: Read Error)"]
pub type RdeR = crate::BitReader;
#[doc = "Field `RDE` writer - Read Error Flag (0: none; 1: Read Error)"]
pub type RdeW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start Detected Interrupt Flag (0: none ; 1: Start Detected)"]
    #[inline(always)]
    pub fn sd(&self) -> SdR {
        SdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Abort Interrupt Flag (0: none; 1: Transfer Aborted)"]
    #[inline(always)]
    pub fn ta(&self) -> TaR {
        TaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cell Transfer Complete Interrupt Flag (0: none ; 1: complete)"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Block Transfer Complete Interrupt Flag (0: none; 1:complete;)"]
    #[inline(always)]
    pub fn bc(&self) -> BcR {
        BcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Block Transfer Half Complete Interrupt Flag (0: none; 1: half complete)"]
    #[inline(always)]
    pub fn bh(&self) -> BhR {
        BhR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Linked List Done Interrupt Flag (0: none; 1: done)"]
    #[inline(always)]
    pub fn ll(&self) -> LlR {
        LlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 17 - Write Error Flag (0: none; 1: Write Error)"]
    #[inline(always)]
    pub fn wre(&self) -> WreR {
        WreR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Read Error Flag (0: none; 1: Read Error)"]
    #[inline(always)]
    pub fn rde(&self) -> RdeR {
        RdeR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Detected Interrupt Flag (0: none ; 1: Start Detected)"]
    #[inline(always)]
    pub fn sd(&mut self) -> SdW<'_, ChintfSpec> {
        SdW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Abort Interrupt Flag (0: none; 1: Transfer Aborted)"]
    #[inline(always)]
    pub fn ta(&mut self) -> TaW<'_, ChintfSpec> {
        TaW::new(self, 1)
    }
    #[doc = "Bit 2 - Cell Transfer Complete Interrupt Flag (0: none ; 1: complete)"]
    #[inline(always)]
    pub fn cc(&mut self) -> CcW<'_, ChintfSpec> {
        CcW::new(self, 2)
    }
    #[doc = "Bit 3 - Block Transfer Complete Interrupt Flag (0: none; 1:complete;)"]
    #[inline(always)]
    pub fn bc(&mut self) -> BcW<'_, ChintfSpec> {
        BcW::new(self, 3)
    }
    #[doc = "Bit 4 - Block Transfer Half Complete Interrupt Flag (0: none; 1: half complete)"]
    #[inline(always)]
    pub fn bh(&mut self) -> BhW<'_, ChintfSpec> {
        BhW::new(self, 4)
    }
    #[doc = "Bit 5 - Linked List Done Interrupt Flag (0: none; 1: done)"]
    #[inline(always)]
    pub fn ll(&mut self) -> LlW<'_, ChintfSpec> {
        LlW::new(self, 5)
    }
    #[doc = "Bit 17 - Write Error Flag (0: none; 1: Write Error)"]
    #[inline(always)]
    pub fn wre(&mut self) -> WreW<'_, ChintfSpec> {
        WreW::new(self, 17)
    }
    #[doc = "Bit 18 - Read Error Flag (0: none; 1: Read Error)"]
    #[inline(always)]
    pub fn rde(&mut self) -> RdeW<'_, ChintfSpec> {
        RdeW::new(self, 18)
    }
}
#[doc = "CHANNEL INTERRUPT FLAG REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chintf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChintfSpec;
impl crate::RegisterSpec for ChintfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chintf::R`](R) reader structure"]
impl crate::Readable for ChintfSpec {}
#[doc = "`write(|w| ..)` method takes [`chintf::W`](W) writer structure"]
impl crate::Writable for ChintfSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0006_003f;
}
#[doc = "`reset()` method sets CHINTF to value 0"]
impl crate::Resettable for ChintfSpec {}
