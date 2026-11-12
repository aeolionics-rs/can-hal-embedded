#[doc = "Register `CHEVCTRL` reader"]
pub type R = crate::R<ChevctrlSpec>;
#[doc = "Register `CHEVCTRL` writer"]
pub type W = crate::W<ChevctrlSpec>;
#[doc = "Channel Auxiliary Event Input Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evauxactselect {
    #[doc = "0: Event Aborts Block Transfer"]
    AbortBlkXfer = 0,
    #[doc = "1: Event Increments Channel Priority"]
    IncrChnPri = 1,
    #[doc = "2: Event acts as a Conditional Trigger"]
    CondTrig = 2,
    #[doc = "3: Event aborts Linked List operation and Block Transfer"]
    AbortLinkList = 3,
}
impl From<Evauxactselect> for u8 {
    #[inline(always)]
    fn from(variant: Evauxactselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evauxactselect {
    type Ux = u8;
}
impl crate::IsEnum for Evauxactselect {}
#[doc = "Field `EVAUXACT` reader - Channel Auxiliary Event Input Action"]
pub type EvauxactR = crate::FieldReader<Evauxactselect>;
impl EvauxactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evauxactselect {
        match self.bits {
            0 => Evauxactselect::AbortBlkXfer,
            1 => Evauxactselect::IncrChnPri,
            2 => Evauxactselect::CondTrig,
            3 => Evauxactselect::AbortLinkList,
            _ => unreachable!(),
        }
    }
    #[doc = "Event Aborts Block Transfer"]
    #[inline(always)]
    pub fn is_abort_blk_xfer(&self) -> bool {
        *self == Evauxactselect::AbortBlkXfer
    }
    #[doc = "Event Increments Channel Priority"]
    #[inline(always)]
    pub fn is_incr_chn_pri(&self) -> bool {
        *self == Evauxactselect::IncrChnPri
    }
    #[doc = "Event acts as a Conditional Trigger"]
    #[inline(always)]
    pub fn is_cond_trig(&self) -> bool {
        *self == Evauxactselect::CondTrig
    }
    #[doc = "Event aborts Linked List operation and Block Transfer"]
    #[inline(always)]
    pub fn is_abort_link_list(&self) -> bool {
        *self == Evauxactselect::AbortLinkList
    }
}
#[doc = "Field `EVAUXACT` writer - Channel Auxiliary Event Input Action"]
pub type EvauxactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Evauxactselect, crate::Safe>;
impl<'a, REG> EvauxactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Event Aborts Block Transfer"]
    #[inline(always)]
    pub fn abort_blk_xfer(self) -> &'a mut crate::W<REG> {
        self.variant(Evauxactselect::AbortBlkXfer)
    }
    #[doc = "Event Increments Channel Priority"]
    #[inline(always)]
    pub fn incr_chn_pri(self) -> &'a mut crate::W<REG> {
        self.variant(Evauxactselect::IncrChnPri)
    }
    #[doc = "Event acts as a Conditional Trigger"]
    #[inline(always)]
    pub fn cond_trig(self) -> &'a mut crate::W<REG> {
        self.variant(Evauxactselect::CondTrig)
    }
    #[doc = "Event aborts Linked List operation and Block Transfer"]
    #[inline(always)]
    pub fn abort_link_list(self) -> &'a mut crate::W<REG> {
        self.variant(Evauxactselect::AbortLinkList)
    }
}
#[doc = "Channel Event Output Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Evomodeselect {
    #[doc = "0: Generate a channel event strobe for 1 clock cycle at the end of block transfer"]
    StrbOnBlckXfer = 0,
    #[doc = "1: Generate a channel event strobe for 1 clock cycle at the end of cell transfer"]
    StrbOnCellXfer = 1,
    #[doc = "2: Generate a channel event strobe from start event trigger to the completion of cell transfer read"]
    StrbStartEvtTrgToCellXferRead = 2,
    #[doc = "3: Generate a channel event strobe from start event trigger to the completion of cell transfer write"]
    StrbStartEvtTrgToCellXferWrite = 3,
}
impl From<Evomodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Evomodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Evomodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Evomodeselect {}
#[doc = "Field `EVOMODE` reader - Channel Event Output Mode"]
pub type EvomodeR = crate::FieldReader<Evomodeselect>;
impl EvomodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evomodeselect {
        match self.bits {
            0 => Evomodeselect::StrbOnBlckXfer,
            1 => Evomodeselect::StrbOnCellXfer,
            2 => Evomodeselect::StrbStartEvtTrgToCellXferRead,
            3 => Evomodeselect::StrbStartEvtTrgToCellXferWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Generate a channel event strobe for 1 clock cycle at the end of block transfer"]
    #[inline(always)]
    pub fn is_strb_on_blck_xfer(&self) -> bool {
        *self == Evomodeselect::StrbOnBlckXfer
    }
    #[doc = "Generate a channel event strobe for 1 clock cycle at the end of cell transfer"]
    #[inline(always)]
    pub fn is_strb_on_cell_xfer(&self) -> bool {
        *self == Evomodeselect::StrbOnCellXfer
    }
    #[doc = "Generate a channel event strobe from start event trigger to the completion of cell transfer read"]
    #[inline(always)]
    pub fn is_strb_start_evt_trg_to_cell_xfer_read(&self) -> bool {
        *self == Evomodeselect::StrbStartEvtTrgToCellXferRead
    }
    #[doc = "Generate a channel event strobe from start event trigger to the completion of cell transfer write"]
    #[inline(always)]
    pub fn is_strb_start_evt_trg_to_cell_xfer_write(&self) -> bool {
        *self == Evomodeselect::StrbStartEvtTrgToCellXferWrite
    }
}
#[doc = "Field `EVOMODE` writer - Channel Event Output Mode"]
pub type EvomodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Evomodeselect, crate::Safe>;
impl<'a, REG> EvomodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generate a channel event strobe for 1 clock cycle at the end of block transfer"]
    #[inline(always)]
    pub fn strb_on_blck_xfer(self) -> &'a mut crate::W<REG> {
        self.variant(Evomodeselect::StrbOnBlckXfer)
    }
    #[doc = "Generate a channel event strobe for 1 clock cycle at the end of cell transfer"]
    #[inline(always)]
    pub fn strb_on_cell_xfer(self) -> &'a mut crate::W<REG> {
        self.variant(Evomodeselect::StrbOnCellXfer)
    }
    #[doc = "Generate a channel event strobe from start event trigger to the completion of cell transfer read"]
    #[inline(always)]
    pub fn strb_start_evt_trg_to_cell_xfer_read(self) -> &'a mut crate::W<REG> {
        self.variant(Evomodeselect::StrbStartEvtTrgToCellXferRead)
    }
    #[doc = "Generate a channel event strobe from start event trigger to the completion of cell transfer write"]
    #[inline(always)]
    pub fn strb_start_evt_trg_to_cell_xfer_write(self) -> &'a mut crate::W<REG> {
        self.variant(Evomodeselect::StrbStartEvtTrgToCellXferWrite)
    }
}
#[doc = "Field `EVAUXIE` reader - Channel Auxiliary Event Enable (0: disable; 1: enable)"]
pub type EvauxieR = crate::BitReader;
#[doc = "Field `EVAUXIE` writer - Channel Auxiliary Event Enable (0: disable; 1: enable)"]
pub type EvauxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVSTRIE` reader - Channel Start Event Input Enable (0: disable; 1:enable)"]
pub type EvstrieR = crate::BitReader;
#[doc = "Field `EVSTRIE` writer - Channel Start Event Input Enable (0: disable; 1:enable)"]
pub type EvstrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVOE` reader - Channel Event Output Enable (0: disable; 1: enable)"]
pub type EvoeR = crate::BitReader;
#[doc = "Field `EVOE` writer - Channel Event Output Enable (0: disable; 1: enable)"]
pub type EvoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Channel Auxiliary Event Input Action"]
    #[inline(always)]
    pub fn evauxact(&self) -> EvauxactR {
        EvauxactR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Channel Event Output Mode"]
    #[inline(always)]
    pub fn evomode(&self) -> EvomodeR {
        EvomodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - Channel Auxiliary Event Enable (0: disable; 1: enable)"]
    #[inline(always)]
    pub fn evauxie(&self) -> EvauxieR {
        EvauxieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Start Event Input Enable (0: disable; 1:enable)"]
    #[inline(always)]
    pub fn evstrie(&self) -> EvstrieR {
        EvstrieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Event Output Enable (0: disable; 1: enable)"]
    #[inline(always)]
    pub fn evoe(&self) -> EvoeR {
        EvoeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel Auxiliary Event Input Action"]
    #[inline(always)]
    pub fn evauxact(&mut self) -> EvauxactW<'_, ChevctrlSpec> {
        EvauxactW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Channel Event Output Mode"]
    #[inline(always)]
    pub fn evomode(&mut self) -> EvomodeW<'_, ChevctrlSpec> {
        EvomodeW::new(self, 2)
    }
    #[doc = "Bit 5 - Channel Auxiliary Event Enable (0: disable; 1: enable)"]
    #[inline(always)]
    pub fn evauxie(&mut self) -> EvauxieW<'_, ChevctrlSpec> {
        EvauxieW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel Start Event Input Enable (0: disable; 1:enable)"]
    #[inline(always)]
    pub fn evstrie(&mut self) -> EvstrieW<'_, ChevctrlSpec> {
        EvstrieW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel Event Output Enable (0: disable; 1: enable)"]
    #[inline(always)]
    pub fn evoe(&mut self) -> EvoeW<'_, ChevctrlSpec> {
        EvoeW::new(self, 7)
    }
}
#[doc = "CHANNEL EVENT CONTROL REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chevctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chevctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChevctrlSpec;
impl crate::RegisterSpec for ChevctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chevctrl::R`](R) reader structure"]
impl crate::Readable for ChevctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`chevctrl::W`](W) writer structure"]
impl crate::Writable for ChevctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHEVCTRL to value 0"]
impl crate::Resettable for ChevctrlSpec {}
