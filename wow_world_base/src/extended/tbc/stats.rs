use crate::shared::player_race_tbc_wrath::PlayerRace;
use crate::tbc::exp::MAX_LEVEL;
use crate::tbc::{Class, RaceClass};

/// Calculate base melee attack power.
///
/// For druids this does not take into account the different attack power calculations when
/// the different forms. Use [`bear_form_base_attack_power`],
/// [`cat_form_base_attack_power`], or [`moonkin_form_base_attack_power`] instead.
pub fn base_melee_attack_power(class: Class, strength: u16, agility: u16, level: u8) -> u32 {
    let level = level as u32;
    let strength = strength as u32;
    let agility = agility as u32;

    match class {
        Class::Paladin | Class::Warrior => (level * 3 + strength * 2).saturating_sub(20),
        Class::Hunter | Class::Rogue => (level * 2 + strength + agility).saturating_sub(20),
        Class::Shaman => (level * 2 + strength * 2).saturating_sub(20),
        Class::Druid => (strength * 2).saturating_sub(20),
        Class::Priest | Class::Mage | Class::Warlock => strength.saturating_sub(10),
    }
}

fn predatory_strikes(predatory_strikes_rank: u8) -> f32 {
    if predatory_strikes_rank == 1 {
        0.5
    } else if predatory_strikes_rank == 2 {
        1.0
    } else if predatory_strikes_rank == 3 {
        1.5
    } else {
        0.0
    }
}

/// Calculate base attack power for druids in bear, dire bear, and moonkin forms.
///
/// Valid values for `predatory_strikes_rank` is 0, 1, 2, or 3.
/// All other values will be treated as 0.
///
/// Predatory Strikes is a feral druid talent that increases the attack power in
/// cat, bear, and dire bear forms by a percentage of their level.
///
/// Rank 1 (spell id 16972) increases by 50% of level.
///
/// Rank 2 (spell id 16974) increases by 100% of level.
///
/// Rank 3 (spell id 16975) increases by 150% of level.
pub fn bear_form_base_attack_power(strength: u16, level: u8, predatory_strikes_rank: u8) -> u32 {
    let strength = strength as f32;
    let level = level as f32;

    (level * (predatory_strikes(predatory_strikes_rank) + 3.0) * strength * 2.0 - 20.0) as u32
}

/// Calculate base attack power for druids in bear, dire bear, and moonkin forms.
///
/// Valid values for `predatory_strikes_rank` is 0, 1, 2, or 3.
/// All other values will be treated as 0.
///
/// Predatory Strikes is a feral druid talent that increases the attack power in
/// cat, bear, and dire bear forms by a percentage of their level.
///
/// Rank 1 (spell id 16972) increases by 50% of level.
///
/// Rank 2 (spell id 16974) increases by 100% of level.
///
/// Rank 3 (spell id 16975) increases by 150% of level.
pub fn moonkin_form_base_attack_power(strength: u16, level: u8, predatory_strikes_rank: u8) -> u32 {
    let strength = strength as f32;
    let level = level as f32;

    (level * (predatory_strikes(predatory_strikes_rank) + 1.5) * strength * 2.0 - 20.0) as u32
}

/// Calculate base attack power for druids in cat form.
///
/// Valid values for `predatory_strikes_rank` is 0, 1, 2, or 3.
/// All other values will be treated as 0.
///
/// Predatory Strikes is a feral druid talent that increases the attack power in
/// cat, bear, and dire bear forms by a percentage of their level.
///
/// Rank 1 (spell id 16972) increases by 50% of level.
///
/// Rank 2 (spell id 16974) increases by 100% of level.
///
/// Rank 3 (spell id 16975) increases by 150% of level.
pub fn cat_form_base_attack_power(
    strength: u16,
    agility: u16,
    level: u8,
    predatory_strikes_rank: u8,
) -> u32 {
    let strength = strength as f32;
    let agility = agility as f32;
    let level = level as f32;

    (level * (predatory_strikes(predatory_strikes_rank) + 2.0) * strength * 2.0 + agility - 20.0)
        as u32
}

/// Calculate base ranged attack power.
///
/// Since only warrior, rogue, and hunter can use weapons that use ranged attack power
/// all other classes return 0.
pub fn base_ranged_attack_power(class: Class, agility: u16, level: u8) -> u32 {
    let level = level as u32;
    let agility = agility as u32;

    match class {
        Class::Warrior | Class::Rogue => (level + agility).saturating_sub(10),
        Class::Hunter => (level * 2 + agility).saturating_sub(10),

        Class::Paladin
        | Class::Priest
        | Class::Shaman
        | Class::Mage
        | Class::Warlock
        | Class::Druid => 0,
    }
}

// Also found in the `gtChanceToMeleeCrit` .dbc.
// In there it's an array that can be indexed by `(class.as_int() - 1) * 100 + (level - 1)`.
// Careful that level 0 does not wrap around.
#[allow(clippy::excessive_precision)]
const fn crit_ratio(class: Class, level: u8) -> f32 {
    let level_index = if level == 0 {
        0
    } else if level > MAX_LEVEL {
        MAX_LEVEL - 1
    } else {
        level - 1
    } as usize;

    let ratio = match class {
        Class::Warrior => &[
            0.00249999994412065,
            0.00238099996931851,
            0.00238099996931851,
            0.0022730000782758,
            0.00217400002293289,
            0.00208300002850592,
            0.00208300002850592,
            0.0020000000949949,
            0.0019229999743402,
            0.0019229999743402,
            0.00185200001578778,
            0.00178599997889251,
            0.00166700000409037,
            0.00161299994215369,
            0.00156300002709031,
            0.00151500001084059,
            0.00147100002504885,
            0.001388999982737,
            0.00135100004263222,
            0.00128199998289347,
            0.00128199998289347,
            0.00124999997206032,
            0.00118999998085201,
            0.00116300000809133,
            0.00111099996138364,
            0.00108700001146644,
            0.00106399995274842,
            0.00101999996695668,
            0.00100000004749745,
            0.000961999990977347,
            0.000943000020924956,
            0.00092600000789389,
            0.000892999989446253,
            0.00087699998402968,
            0.000846999988425523,
            0.000832999998237938,
            0.000820000015664846,
            0.000793999992311001,
            0.000781000009737909,
            0.000758000009227544,
            0.000735000008717179,
            0.000724999990779907,
            0.000704000005498528,
            0.000693999987561256,
            0.000676000025123358,
            0.000667000014800578,
            0.00064899999415502,
            0.000632999988738447,
            0.000624999986030161,
            0.000609999988228083,
            0.000594999990426004,
            0.000587999995332211,
            0.000575000012759119,
            0.000561999971978366,
            0.000548999989405274,
            0.000543000001925975,
            0.000531999976374209,
            0.000521000009030104,
            0.000509999983478338,
            0.000500000023748726,
            0.000468999991426244,
            0.000441999989561737,
            0.000418000010540709,
            0.0003969999961555,
            0.000376999989384785,
            0.00036000000545755,
            0.000344000000040978,
            0.000329000002238899,
            0.000315000012051314,
            0.000303000007988885,
        ],

        Class::Paladin => &[
            0.00217400002293289,
            0.00206999992951751,
            0.00206999992951751,
            0.00197599991224706,
            0.00197599991224706,
            0.00188999995589256,
            0.00188999995589256,
            0.00181199994403869,
            0.00181199994403869,
            0.00173899997025728,
            0.00173899997025728,
            0.00167200004216284,
            0.00155299995094538,
            0.00155299995094538,
            0.00144899997394532,
            0.00144899997394532,
            0.00140299997292459,
            0.00131800002418458,
            0.00131800002418458,
            0.0012420000275597,
            0.00120800000149757,
            0.00120800000149757,
            0.00114399997983128,
            0.00111499999184161,
            0.00108700001146644,
            0.00106000003870577,
            0.00103499996475875,
            0.00101100001484156,
            0.000987999956123531,
            0.000944999977946281,
            0.000925000000279397,
            0.000925000000279397,
            0.000887000001966953,
            0.000869999988935888,
            0.000836000021081418,
            0.000820000015664846,
            0.000820000015664846,
            0.000791000027675182,
            0.000775999971665442,
            0.000750000006519258,
            0.000737000023946166,
            0.000737000023946166,
            0.000713000015821308,
            0.000700999982655048,
            0.000678999989759177,
            0.000668999971821904,
            0.000659000012092292,
            0.000638999976217747,
            0.000630000024102628,
            0.000612000003457069,
            0.000604000000748783,
            0.000595999998040497,
            0.000579999992623925,
            0.000571999989915639,
            0.00055699999211356,
            0.000549999997019768,
            0.000544000009540468,
            0.000530000019352883,
            0.000523999973665923,
            0.000511999998707324,
            0.000491000013425946,
            0.00048300001071766,
            0.000472000014269724,
            0.000456000008853152,
            0.00044599999091588,
            0.000437000009696931,
            0.000425000005634502,
            0.000415999995311722,
            0.000407999992603436,
            0.00039999998989515,
        ],

        Class::Hunter => &[
            0.00283999997191131,
            0.00283399992622435,
            0.00271099992096424,
            0.0025299999397248,
            0.00243000010959804,
            0.00233700009994209,
            0.00225099991075695,
            0.00217100000008941,
            0.00205100001767278,
            0.00198399997316301,
            0.00184799998532981,
            0.00167000002693385,
            0.00154700002167374,
            0.00144100002944469,
            0.00132999999914318,
            0.00126699998509139,
            0.00119400001130998,
            0.0011170000070706,
            0.00106000003870577,
            0.000998000032268465,
            0.000961999990977347,
            0.000910000002477318,
            0.000872000004164875,
            0.000829000025987625,
            0.000797000015154481,
            0.000767000019550323,
            0.000734000001102686,
            0.000708999985363334,
            0.00067999999737367,
            0.000653999974019825,
            0.000637000019196421,
            0.000614000018686056,
            0.000592000025790185,
            0.000575000012759119,
            0.000555999984499067,
            0.000540999986696988,
            0.000523999973665923,
            0.000508000026457012,
            0.000493000028654933,
            0.000480999995488673,
            0.000469999999040738,
            0.000456999987363815,
            0.000444000004790723,
            0.000433000008342788,
            0.000421000004280359,
            0.000413000001572073,
            0.000402000005124137,
            0.000391000008676201,
            0.000381999998353422,
            0.000372999988030642,
            0.00036599999293685,
            0.000357999990228564,
            0.000349999987520278,
            0.000341000006301329,
            0.000334000011207536,
            0.000327999994624406,
            0.000320999999530613,
            0.000314000004436821,
            0.000307000009343028,
            0.000300999992759898,
            0.000296999991405755,
            0.000289999996311963,
            0.000284000008832663,
            0.000278999999864027,
            0.000273000012384728,
            0.000269999989541247,
            0.000264000002061948,
            0.000258999993093312,
            0.000254000013228506,
            0.000250000011874363,
        ],

        Class::Rogue => &[
            0.00447600008919835,
            0.00429000006988645,
            0.00411800015717745,
            0.00381299993023276,
            0.00367700005881488,
            0.0035500000230968,
            0.00332099990919232,
            0.00321700004860759,
            0.00312000000849366,
            0.00294100004248321,
            0.00264000007882714,
            0.00239400006830692,
            0.00214500003494322,
            0.00197999994270504,
            0.0017750000115484,
            0.00165999995078892,
            0.00156000000424683,
            0.00144999998155981,
            0.00135499995667487,
            0.00127100001554936,
            0.00119700003415346,
            0.00114399997983128,
            0.00108399998862296,
            0.00104000000283122,
            0.000980000011622906,
            0.000936000025831163,
            0.000903000007383525,
            0.000865000009071082,
            0.000829999975394458,
            0.000791999977082014,
            0.000768000027164817,
            0.000740999996196479,
            0.000714999972842634,
            0.000691000022925436,
            0.000663999991957098,
            0.00064300000667572,
            0.000628000008873641,
            0.000608999980613589,
            0.000592000025790185,
            0.000571999989915639,
            0.000555999984499067,
            0.000541999994311482,
            0.000528000004123896,
            0.000511999998707324,
            0.000497000000905246,
            0.00048600000445731,
            0.000474000000394881,
            0.000464000011561438,
            0.000453999993624166,
            0.00044000000343658,
            0.000430999993113801,
            0.000422000011894852,
            0.000411999993957579,
            0.000403999991249293,
            0.000394000002415851,
            0.000385999999707565,
            0.000377999996999279,
            0.000369999994290993,
            0.000364000006811693,
            0.000354999996488914,
            0.000334000011207536,
            0.000322000007145107,
            0.000307000009343028,
            0.000296000012895092,
            0.00028599999495782,
            0.000276000006124377,
            0.000268000003416091,
            0.000261999986832961,
            0.000255999999353662,
            0.000250000011874363,
        ],

        Class::Priest => &[
            0.000908999994862825,
            0.000908999994862825,
            0.000908999994862825,
            0.000865000009071082,
            0.000865000009071082,
            0.000865000009071082,
            0.000865000009071082,
            0.000826000003144145,
            0.000826000003144145,
            0.000826000003144145,
            0.000826000003144145,
            0.000790000020060688,
            0.000790000020060688,
            0.000790000020060688,
            0.000790000020060688,
            0.000757000001613051,
            0.000757000001613051,
            0.000757000001613051,
            0.000727000006008893,
            0.000727000006008893,
            0.000727000006008893,
            0.000727000006008893,
            0.000699000025633723,
            0.000699000025633723,
            0.000699000025633723,
            0.000673000002279878,
            0.000673000002279878,
            0.000673000002279878,
            0.00064899999415502,
            0.00064899999415502,
            0.00064899999415502,
            0.000627000001259148,
            0.000627000001259148,
            0.000627000001259148,
            0.00060600001597777,
            0.00060600001597777,
            0.00060600001597777,
            0.000585999980103225,
            0.000585999980103225,
            0.000585999980103225,
            0.000568000017665327,
            0.000568000017665327,
            0.000551000004634261,
            0.000551000004634261,
            0.000551000004634261,
            0.000533999991603196,
            0.000533999991603196,
            0.000518999993801117,
            0.000518999993801117,
            0.000518999993801117,
            0.000505000003613532,
            0.000505000003613532,
            0.000491000013425946,
            0.000491000013425946,
            0.000478000001749024,
            0.000478000001749024,
            0.000465999997686595,
            0.000465999997686595,
            0.000453999993624166,
            0.000453999993624166,
            0.00044299999717623,
            0.000444000004790723,
            0.000441000011051074,
            0.000433000008342788,
            0.000426000013248995,
            0.000418999989051372,
            0.000414000009186566,
            0.000411999993957579,
            0.000410000007832423,
            0.00039999998989515,
        ],

        Class::Shaman => &[
            0.0016629999736324,
            0.0016629999736324,
            0.00158299994654953,
            0.00158299994654953,
            0.00151099998038262,
            0.00151099998038262,
            0.00151099998038262,
            0.00144599995110184,
            0.00144599995110184,
            0.00138499995227903,
            0.00138499995227903,
            0.00132999999914318,
            0.00132999999914318,
            0.00127899996004999,
            0.00123099994380027,
            0.00118799996562302,
            0.00118799996562302,
            0.00114700000267476,
            0.00114700000267476,
            0.00107300002127886,
            0.00107300002127886,
            0.00103899999521673,
            0.00103899999521673,
            0.00100799999199808,
            0.000977999996393919,
            0.000950000016018748,
            0.000950000016018748,
            0.000923999992664903,
            0.000923999992664903,
            0.000875000027008355,
            0.000875000027008355,
            0.000852999975904822,
            0.000830999983008951,
            0.000830999983008951,
            0.000791999977082014,
            0.000773000007029623,
            0.000773000007029623,
            0.000755999993998557,
            0.000755999993998557,
            0.00072299997555092,
            0.000707000028342009,
            0.000707000028342009,
            0.000692999979946762,
            0.000678999989759177,
            0.000664999999571592,
            0.0006520000169985,
            0.000638999976217747,
            0.000627000001259148,
            0.000627000001259148,
            0.000605000008363277,
            0.000593999982811511,
            0.000583000015467405,
            0.000583000015467405,
            0.000572999997530133,
            0.000554000027477741,
            0.000545000017154962,
            0.000536000006832182,
            0.000536000006832182,
            0.000528000004123896,
            0.000511999998707324,
            0.000495999993290752,
            0.00048600000445731,
            0.000469999999040738,
            0.000456000008853152,
            0.00044900001375936,
            0.000437000009696931,
            0.000426999991759658,
            0.000417000002926216,
            0.000407999992603436,
            0.00039999998989515,
        ],

        Class::Mage => &[
            0.000770999991800636,
            0.000770999991800636,
            0.000770999991800636,
            0.000735000008717179,
            0.000735000008717179,
            0.000735000008717179,
            0.000735000008717179,
            0.000735000008717179,
            0.000735000008717179,
            0.000700999982655048,
            0.000700999982655048,
            0.000700999982655048,
            0.000700999982655048,
            0.000700999982655048,
            0.000670999987050891,
            0.000670999987050891,
            0.000670999987050891,
            0.000670999987050891,
            0.000670999987050891,
            0.00064300000667572,
            0.00064300000667572,
            0.00064300000667572,
            0.00064300000667572,
            0.000616999983321875,
            0.000616999983321875,
            0.000616999983321875,
            0.000616999983321875,
            0.000616999983321875,
            0.000592999975197017,
            0.000592999975197017,
            0.000592999975197017,
            0.000592999975197017,
            0.000570999982301146,
            0.000570999982301146,
            0.000570999982301146,
            0.000551000004634261,
            0.000551000004634261,
            0.000551000004634261,
            0.000551000004634261,
            0.000531999976374209,
            0.000531999976374209,
            0.000531999976374209,
            0.000531999976374209,
            0.000514000013936311,
            0.000514000013936311,
            0.000514000013936311,
            0.000498000008519739,
            0.000498000008519739,
            0.000498000008519739,
            0.000482000003103167,
            0.000482000003103167,
            0.000482000003103167,
            0.000467000005301088,
            0.000467000005301088,
            0.000467000005301088,
            0.000453999993624166,
            0.000453999993624166,
            0.000453999993624166,
            0.000441000011051074,
            0.000441000011051074,
            0.000434999994467944,
            0.000432000000728294,
            0.000423999998020008,
            0.000422999990405515,
            0.000422000011894852,
            0.000410999986343086,
            0.000411999993957579,
            0.000407999992603436,
            0.000403999991249293,
            0.00039999998989515,
        ],

        Class::Warlock => &[
            0.00150000001303852,
            0.00150000001303852,
            0.0014290000544861,
            0.0014290000544861,
            0.0014290000544861,
            0.00136400002520531,
            0.00136400002520531,
            0.00136400002520531,
            0.001304000033997,
            0.001304000033997,
            0.00124999997206032,
            0.00124999997206032,
            0.00124999997206032,
            0.00120000005699694,
            0.00115400005597621,
            0.00111099996138364,
            0.00111099996138364,
            0.00111099996138364,
            0.00107100000604987,
            0.00103399995714426,
            0.00100000004749745,
            0.00100000004749745,
            0.000967999978456646,
            0.000967999978456646,
            0.000908999994862825,
            0.000908999994862825,
            0.000908999994862825,
            0.000882000022102147,
            0.000882000022102147,
            0.000832999998237938,
            0.000832999998237938,
            0.000811000005342066,
            0.000811000005342066,
            0.000789000012446195,
            0.000768999976571649,
            0.000750000006519258,
            0.000731999985873699,
            0.000731999985873699,
            0.000714000023435801,
            0.000698000018019229,
            0.000682000012602657,
            0.000682000012602657,
            0.000667000014800578,
            0.000667000014800578,
            0.000638000026810914,
            0.000624999986030161,
            0.000624999986030161,
            0.000612000003457069,
            0.000600000028498471,
            0.000587999995332211,
            0.000577000027988106,
            0.000577000027988106,
            0.00056600000243634,
            0.000555999984499067,
            0.000545000017154962,
            0.000536000006832182,
            0.00052599998889491,
            0.00051699997857213,
            0.00051699997857213,
            0.000500000023748726,
            0.000483999989228323,
            0.000480999995488673,
            0.000469999999040738,
            0.000455000001238659,
            0.000448000006144866,
            0.000434999994467944,
            0.000436000002082437,
            0.000423999998020008,
            0.000414000009186566,
            0.000404999998863786,
        ],

        Class::Druid => &[
            0.00202000001445413,
            0.00202000001445413,
            0.0019229999743402,
            0.0019229999743402,
            0.00183600001037121,
            0.00183600001037121,
            0.00175599998328835,
            0.00175599998328835,
            0.00168300000950694,
            0.00155299995094538,
            0.00149599998258054,
            0.00149599998258054,
            0.00144300004467368,
            0.00144300004467368,
            0.00134600000455976,
            0.00134600000455976,
            0.00130300002638251,
            0.00126199994701892,
            0.00126199994701892,
            0.00112200004514307,
            0.00112200004514307,
            0.00109200004953891,
            0.00106299994513392,
            0.00106299994513392,
            0.00101000000722706,
            0.00101000000722706,
            0.000985000049695373,
            0.000961999990977347,
            0.000961999990977347,
            0.000877999991644174,
            0.000859000021591783,
            0.000859000021591783,
            0.000841000000946224,
            0.000823999987915158,
            0.000807999982498586,
            0.000791999977082014,
            0.000776999979279935,
            0.000776999979279935,
            0.000761999981477857,
            0.000708999985363334,
            0.000696000002790242,
            0.000696000002790242,
            0.000684999977238476,
            0.000673000002279878,
            0.000651000009384006,
            0.000640999991446733,
            0.000640999991446733,
            0.000630999973509461,
            0.000621000013779849,
            0.000584999972488731,
            0.000577000027988106,
            0.00056900002527982,
            0.000561000022571534,
            0.000561000022571534,
            0.000546000024769455,
            0.000538999971468002,
            0.000531000026967376,
            0.000524999981280416,
            0.000517999986186624,
            0.000493000028654933,
            0.000478000001749024,
            0.000472000014269724,
            0.000456000008853152,
            0.000446999998530373,
            0.000437999988207594,
            0.000429999985499308,
            0.000423999998020008,
            0.000411999993957579,
            0.00040600000647828,
            0.00039999998989515,
        ],
    };

    ratio[level_index]
}

/// Calculate base melee crit from agility.
///
/// Does not return the chance as a percentage, but as a whole number.
/// So a 4% chance to crit would return 4.0.
#[allow(clippy::excessive_precision)]
pub fn base_melee_crit(class: Class, agility: u16, level: u8) -> f32 {
    let base: f32 = match class {
        Class::Warrior => 0.01140000019222498,
        Class::Paladin => 0.006519999820739031,
        Class::Hunter => -0.015320000238716602,
        Class::Rogue => -0.002950000111013651,
        Class::Priest => 0.03183000162243843,
        Class::Shaman => 0.016750000417232513,
        Class::Mage => 0.034575000405311584,
        Class::Warlock => 0.019999999552965164,
        Class::Druid => 0.009610000066459179,
    };

    let ratio = crit_ratio(class, level);

    let crit = base + agility as f32 * ratio;
    crit * 100.0
}

/// Calculate base dodge chance from agility.
///
/// *Does* include the 1% from the night elf Quickness racial (skill id 20582)
/// and the base class dodge chances.
pub fn base_dodge_chance(class: Class, race: PlayerRace, agility: u16, level: u8) -> f32 {
    let class_base: f32 = match class {
        Class::Warrior => 0.0075,
        Class::Paladin => 0.00652,
        Class::Hunter => -0.0545,
        Class::Rogue => -0.0059,
        Class::Priest => 0.03183,
        Class::Shaman => 0.0167,
        Class::Mage => 0.034575,
        Class::Warlock => 0.02011,
        Class::Druid => -0.0187,
    };

    let crit_per_agility_to_dodge_per_agility_coefficient: f32 = match class {
        Class::Warrior => 1.1,
        Class::Paladin => 1.0,
        Class::Hunter => 1.6,
        Class::Rogue => 2.0,
        Class::Priest => 1.0,
        Class::Shaman => 1.0,
        Class::Mage => 1.0,
        Class::Warlock => 1.0,
        Class::Druid => 1.7,
    };

    let dodge_ratio = crit_ratio(class, level) * crit_per_agility_to_dodge_per_agility_coefficient;

    let dodge = class_base + agility as f32 * dodge_ratio;

    /// 1% from Quickness racial (skill id 20582)
    let racial_bonus: f32 = if matches!(race, PlayerRace::NightElf) {
        1.0
    } else {
        0.0
    };

    (dodge * 100.0) + racial_bonus
}

impl RaceClass {
    /// Calculate base melee attack power.
    ///
    /// For druids this does not take into account the different attack power calculations when
    /// the different forms. Use [`bear_form_base_attack_power`],
    /// [`cat_form_base_attack_power`], or [`moonkin_form_base_attack_power`] instead.
    pub fn base_melee_attack_power(&self, strength: u16, agility: u16, level: u8) -> u32 {
        base_melee_attack_power(self.class(), strength, agility, level)
    }

    /// Calculate base ranged attack power.
    ///
    /// Since only warrior, rogue, and hunter can use weapons that use ranged attack power
    /// all other classes return 0.
    pub fn base_ranged_attack_power(&self, agility: u16, level: u8) -> u32 {
        base_ranged_attack_power(self.class(), agility, level)
    }

    /// Calculate base melee crit from agility.
    ///
    /// Does not return the chance as a percentage, but as a whole number.
    /// So a 4% chance to crit would return 4.0.
    pub fn base_melee_crit(&self, agility: u16, level: u8) -> f32 {
        base_melee_crit(self.class(), agility, level)
    }

    /// Calculate base dodge chance from agility.
    ///
    /// *Does* include the 1% from the night elf Quickness racial (skill id 20582)
    /// and the base class dodge chances.
    pub fn base_dodge_chance(&self, agility: u16, level: u8) -> f32 {
        base_dodge_chance(self.class(), self.race(), agility, level)
    }
}