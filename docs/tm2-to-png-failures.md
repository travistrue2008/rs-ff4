# TM2 -> PNG Failures

Below is a list of files that have failed to be converted to PNGs. This could be due to the following:

- LZS/LZSS compression was not correct
- Buggy/lack of support in the TM2 loader

## Bug by Type

### Index Out-of-Range (Panics)

/data/battle/TheAfterCommon/btl_system_06.tm2
/data/CN_baron_castle1_char/fChara_279.tm2
/data/SampleMenu/formationchkline.tm2
/data/menu/menu_gallery_pic/image_panel003.tm2
/data/menu/menu_gallery_pic/image_panel017.tm2
/data/menu/menu_gallery_pic/image_panel016.tm2
/data/menu/menu_gallery_pic/image_panel002.tm2
/data/menu/menu_gallery_pic/image_panel028.tm2
/data/menu/menu_gallery_pic/image_panel014.tm2
/data/menu/menu_gallery_pic/image_panel000.tm2
/data/menu/menu_gallery_pic/image_panel001.tm2
/data/menu/menu_gallery_pic/image_panel015.tm2
/data/menu/menu_gallery_pic/image_panel029.tm2
/data/menu/menu_gallery_pic/image_panel011.tm2
/data/menu/menu_gallery_pic/image_panel005.tm2
/data/menu/menu_gallery_pic/image_panel039.tm2
/data/menu/menu_gallery_pic/image_panel038.tm2
/data/menu/menu_gallery_pic/image_panel004.tm2
/data/menu/menu_gallery_pic/image_panel010.tm2
/data/menu/menu_gallery_pic/image_panel006.tm2
/data/menu/menu_gallery_pic/image_panel012.tm2
/data/menu/menu_gallery_pic/image_panel013.tm2
/data/menu/menu_gallery_pic/image_panel007.tm2
/data/menu/menu_gallery_pic/image_panel048.tm2
/data/menu/menu_gallery_pic/image_panel060.tm2
/data/menu/menu_gallery_pic/image_panel074.tm2
/data/menu/menu_gallery_pic/image_panel075.tm2
/data/menu/menu_gallery_pic/image_panel061.tm2
/data/menu/menu_gallery_pic/image_panel049.tm2
/data/menu/menu_gallery_pic/image_panel088.tm2
/data/menu/menu_gallery_pic/image_panel077.tm2
/data/menu/menu_gallery_pic/image_panel063.tm2
/data/menu/menu_gallery_pic/image_panel062.tm2
/data/menu/menu_gallery_pic/image_panel076.tm2
/data/menu/menu_gallery_pic/image_panel089.tm2
/data/menu/menu_gallery_pic/image_panel099.tm2
/data/menu/menu_gallery_pic/image_panel072.tm2
/data/menu/menu_gallery_pic/image_panel066.tm2
/data/menu/menu_gallery_pic/image_panel067.tm2
/data/menu/menu_gallery_pic/image_panel073.tm2
/data/menu/menu_gallery_pic/image_panel098.tm2
/data/menu/menu_gallery_pic/image_panel065.tm2
/data/menu/menu_gallery_pic/image_panel071.tm2
/data/menu/menu_gallery_pic/image_panel059.tm2
/data/menu/menu_gallery_pic/image_panel058.tm2
/data/menu/menu_gallery_pic/image_panel070.tm2
/data/menu/menu_gallery_pic/image_panel064.tm2
/data/menu/menu_gallery_pic/image_panel082.tm2
/data/menu/menu_gallery_pic/image_panel096.tm2
/data/menu/menu_gallery_pic/image_panel069.tm2
/data/menu/menu_gallery_pic/image_panel041.tm2
/data/menu/menu_gallery_pic/image_panel055.tm2
/data/menu/menu_gallery_pic/image_panel054.tm2
/data/menu/menu_gallery_pic/image_panel040.tm2
/data/menu/menu_gallery_pic/image_panel068.tm2
/data/menu/menu_gallery_pic/image_panel097.tm2
/data/menu/menu_gallery_pic/image_panel083.tm2
/data/menu/menu_gallery_pic/image_panel095.tm2
/data/menu/menu_gallery_pic/image_panel081.tm2
/data/menu/menu_gallery_pic/image_panel056.tm2
/data/menu/menu_gallery_pic/image_panel042.tm2
/data/menu/menu_gallery_pic/image_panel043.tm2
/data/menu/menu_gallery_pic/image_panel057.tm2
/data/menu/menu_gallery_pic/image_panel080.tm2
/data/menu/menu_gallery_pic/image_panel094.tm2
/data/menu/menu_gallery_pic/image_panel090.tm2
/data/menu/menu_gallery_pic/image_panel084.tm2
/data/menu/menu_gallery_pic/image_panel053.tm2
/data/menu/menu_gallery_pic/image_panel047.tm2
/data/menu/menu_gallery_pic/image_panel046.tm2
/data/menu/menu_gallery_pic/image_panel052.tm2
/data/menu/menu_gallery_pic/image_panel085.tm2
/data/menu/menu_gallery_pic/image_panel091.tm2
/data/menu/menu_gallery_pic/image_panel087.tm2
/data/menu/menu_gallery_pic/image_panel093.tm2
/data/menu/menu_gallery_pic/image_panel044.tm2
/data/menu/menu_gallery_pic/image_panel050.tm2
/data/menu/menu_gallery_pic/image_panel078.tm2
/data/menu/menu_gallery_pic/image_panel079.tm2
/data/menu/menu_gallery_pic/image_panel051.tm2
/data/menu/menu_gallery_pic/image_panel045.tm2
/data/menu/menu_gallery_pic/image_panel092.tm2
/data/menu/menu_gallery_pic/image_panel086.tm2
/data/menu/menu_gallery_pic/image_panel022.tm2
/data/menu/menu_gallery_pic/image_panel036.tm2
/data/menu/menu_gallery_pic/image_panel037.tm2
/data/menu/menu_gallery_pic/image_panel023.tm2
/data/menu/menu_gallery_pic/image_panel009.tm2
/data/menu/menu_gallery_pic/image_panel035.tm2
/data/menu/menu_gallery_pic/image_panel021.tm2
/data/menu/menu_gallery_pic/image_panel020.tm2
/data/menu/menu_gallery_pic/image_panel034.tm2
/data/menu/menu_gallery_pic/image_panel008.tm2
/data/menu/menu_gallery_pic/image_panel030.tm2
/data/menu/menu_gallery_pic/image_panel024.tm2
/data/menu/menu_gallery_pic/image_panel018.tm2
/data/menu/menu_gallery_pic/image_panel019.tm2
/data/menu/menu_gallery_pic/image_panel025.tm2
/data/menu/menu_gallery_pic/image_panel031.tm2
/data/menu/menu_gallery_pic/image_panel027.tm2
/data/menu/menu_gallery_pic/image_panel033.tm2
/data/menu/menu_gallery_pic/image_panel032.tm2
/data/menu/menu_gallery_pic/image_panel026.tm2
/data/game_common/em_black_hole00.tm2
/data/game_common/em_black_hole00_Glow.tm2
/data/event/evtta_yan/TA_fChara_033_0.tm2
/data/event/evtta_endD/ta_effect_11.tm2
/data/event/evtta_endD/TA_fChara_051_0.tm2
/data/event/evtta_endC/ta_effect_01.tm2
/data/event/evtta_gol/ta_effect_03.tm2
/data/event/evtta_ryd/ta_effect_06.tm2
/data/event/evtta_ryd/TA_fChara_028_0.tm2
/data/event/evtta_pro/ta_effect_05.tm2
/data/event/evtta_pol/ta_effect_02.tm2
/data/event/evtta_edg/TA_fChara_035_0.tm2
/data/event/evtta_gil/TA_fChara_029_0.tm2
/data/event/evtta_endA/ta_effect_01.tm2
/data/event/evtta_endA/TA_fChara_052_0.tm2
/data/CN_baron_castle2_char/fChara_279.tm2

### Divide By Zero (Panics)

/data/menu/zukan/msd_089.tm2.tm2
/data/menu/zukan/msd_089b.tm2.tm2

### Interlacing Distortion

/data/battle/00ta_mon/ms_283.tm2
/data/battle/00ta_mon/ms_284.tm2
/data/battle/00ta_mon/ms_285.tm2
/data/battle/00ta_mon/ms_286.tm2
/data/battle/00ta_mon/ms_314.tm2
/data/battle/00ta_mon/ms_398.tm2
/data/battle/08ta_mon/ms_335.tm2
/data/battle/08ta_mon/ms_388.tm2
/data/battle/08ta_mon/ms_389.tm2
/data/battle/08ta_mon/ms_390.tm2
/data/battle/08ta_mon/ms_391.tm2
/data/battle/08ta_mon/ms_392.tm2
/data/battle/08ta_mon/ms_426.tm2
/data/battle/08ta_mon/ms_427.tm2
/data/battle/08ta_mon/ms_430.tm2
/data/battle/08ta_mon/ms_431.tm2
