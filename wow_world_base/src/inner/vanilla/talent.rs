/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm#L8):
/// ```text
/// enum Talent : u32 {
///     BURNING_SOUL = 23;
///     IMPROVED_FIRE_WARD = 24;
///     IMPROVED_SCORCH = 25;
///     IMPROVED_FIREBALL = 26;
///     IMPROVED_FIRE_BLAST = 27;
///     FLAME_THROWING = 28;
///     PYROBLAST = 29;
///     IMPACT = 30;
///     IMPROVED_FLAMESTRIKE = 31;
///     BLAST_WAVE = 32;
///     CRITICAL_MASS = 33;
///     IGNITE = 34;
///     FIRE_POWER = 35;
///     COMBUSTION = 36;
///     IMPROVED_FROSTBOLT = 37;
///     FROSTBITE = 38;
///     PIERCING_ICE = 61;
///     IMPROVED_FROST_NOVA = 62;
///     IMPROVED_BLIZZARD = 63;
///     IMPROVED_CONE_OF_COLD = 64;
///     PERMAFROST = 65;
///     FROST_CHANNELING = 66;
///     SHATTER = 67;
///     WINTERS_CHILL = 68;
///     COLD_SNAP = 69;
///     FROST_WARDING = 70;
///     ICE_BARRIER = 71;
///     ICE_BLOCK = 72;
///     ICE_SHARDS = 73;
///     ARCANE_SUBTLETY = 74;
///     ARCANE_CONCENTRATION = 75;
///     ARCANE_FOCUS = 76;
///     ARCANE_MIND = 77;
///     MAGE_WAND_SPECIALIZATION = 78;
///     IMPROVED_ARCANE_MISSILES = 80;
///     IMPROVED_ARCANE_EXPLOSION = 81;
///     MAGIC_ATTUNEMENT = 82;
///     IMPROVED_MANA_SHIELD = 83;
///     ARCANE_RESILIENCE = 85;
///     PRESENCE_OF_MIND = 86;
///     ARCANE_POWER = 87;
///     IMPROVED_COUNTERSPELL = 88;
///     DEEP_WOUNDS = 121;
///     WARRIOR_SWORD_SPECIALIZATION = 123;
///     IMPROVED_HEROIC_STRIKE = 124;
///     WARRIOR_MACE_SPECIALIZATION = 125;
///     IMPROVED_CHARGE = 126;
///     IMPROVED_REND = 127;
///     IMPROVED_THUNDER_CLAP = 128;
///     IMPROVED_HAMSTRING = 129;
///     WARRIOR_DEFLECTION = 130;
///     IMPROVED_OVERPOWER = 131;
///     AXE_SPECIALIZATION = 132;
///     SWEEPING_STRIKES = 133;
///     POLEARM_SPECIALIZATION = 134;
///     MORTAL_STRIKE = 135;
///     WARRIOR_TWO_HANDED_WEAPON_SPECIALIZATION = 136;
///     ANGER_MANAGEMENT = 137;
///     WARRIOR_ANTICIPATION = 138;
///     WARRIOR_TOUGHNESS = 140;
///     IRON_WILL = 141;
///     IMPROVED_BLOODRAGE = 142;
///     IMPROVED_TAUNT = 143;
///     DEFIANCE = 144;
///     IMPROVED_SHIELD_BLOCK = 145;
///     IMPROVED_SUNDER_ARMOR = 146;
///     IMPROVED_REVENGE = 147;
///     SHIELD_SLAM = 148;
///     IMPROVED_SHIELD_BASH = 149;
///     IMPROVED_SHIELD_WALL = 150;
///     IMPROVED_DISARM = 151;
///     CONCUSSION_BLOW = 152;
///     LAST_STAND = 153;
///     IMPROVED_BATTLE_SHOUT = 154;
///     ENRAGE = 155;
///     WARRIOR_FLURRY = 156;
///     CRUELTY = 157;
///     BOOMING_VOICE = 158;
///     UNBRIDLED_WRATH = 159;
///     PIERCING_HOWL = 160;
///     IMPROVED_DEMORALIZING_SHOUT = 161;
///     DEATH_WISH = 165;
///     IMPROVED_CLEAVE = 166;
///     BLOODTHIRST = 167;
///     IMPROVED_SLAM = 168;
///     ROGUE_PRECISION = 181;
///     DAGGER_SPECIALIZATION = 182;
///     FIST_WEAPON_SPECIALIZATION = 183;
///     ROGUE_MACE_SPECIALIZATION = 184;
///     ROGUE_LIGHTNING_REFLEXES = 186;
///     ROGUE_DEFLECTION = 187;
///     IMPROVED_SINISTER_STRIKE = 201;
///     IMPROVED_BACKSTAB = 202;
///     IMPROVED_GOUGE = 203;
///     ENDURANCE = 204;
///     ADRENALINE_RUSH = 205;
///     IMPROVED_KICK = 206;
///     ROGUE_DUAL_WIELD_SPECIALIZATION = 221;
///     IMPROVED_SPRINT = 222;
///     BLADE_FLURRY = 223;
///     MASTER_OF_DECEPTION = 241;
///     ROGUE_SWORD_SPECIALIZATION = 242;
///     CAMOUFLAGE = 244;
///     INITIATIVE = 245;
///     SETUP = 246;
///     ELUSIVENESS = 247;
///     OPPORTUNITY = 261;
///     IMPROVED_SAP = 262;
///     IMPROVED_AMBUSH = 263;
///     DIRTY_DEEDS = 265;
///     IMPROVED_POISONS = 268;
///     LETHALITY = 269;
///     MALICE = 270;
///     REMORSELESS_ATTACKS = 272;
///     RUTHLESSNESS = 273;
///     MURDER = 274;
///     IMPROVED_EVISCERATE = 276;
///     IMPROVED_SLICE_AND_DICE = 277;
///     IMPROVED_EXPOSE_ARMOR = 278;
///     IMPROVED_KIDNEY_SHOT = 279;
///     COLD_BLOOD = 280;
///     RELENTLESS_STRIKES = 281;
///     SEAL_FATE = 283;
///     PREPARATION = 284;
///     RIPOSTE = 301;
///     GHOSTLY_STRIKE = 303;
///     MARTYRDOM = 321;
///     POWER_INFUSION = 322;
///     MENTAL_AGILITY = 341;
///     UNBREAKABLE_WILL = 342;
///     IMPROVED_POWER_WORD_SHIELD = 343;
///     IMPROVED_POWER_WORD_FORTITUDE = 344;
///     PRIEST_WAND_SPECIALIZATION = 345;
///     IMPROVED_INNER_FIRE = 346;
///     MEDITATION = 347;
///     INNER_FOCUS = 348;
///     IMPROVED_MANA_BURN = 350;
///     DIVINE_SPIRIT = 351;
///     SILENT_RESOLVE = 352;
///     INSPIRATION = 361;
///     PREMEDITATION = 381;
///     VIGOR = 382;
///     HOLY_SPECIALIZATION = 401;
///     SPIRITUAL_GUIDANCE = 402;
///     SEARING_LIGHT = 403;
///     SPIRITUAL_HEALING = 404;
///     IMPROVED_RENEW = 406;
///     IMPROVED_HEALING = 408;
///     PRIEST_HEALING_FOCUS = 410;
///     SPELL_WARDING = 411;
///     IMPROVED_PRAYER_OF_HEALING = 413;
///     ARCANE_INSTABILITY = 421;
///     HOLY_NOVA = 442;
///     SHADOW_WEAVING = 461;
///     DARKNESS = 462;
///     SHADOW_FOCUS = 463;
///     BLACKOUT = 464;
///     SPIRIT_TAP = 465;
///     SHADOW_AFFINITY = 466;
///     IMPROVED_MIND_BLAST = 481;
///     IMPROVED_SHADOW_WORD_PAIN = 482;
///     IMPROVED_FADE = 483;
///     VAMPIRIC_EMBRACE = 484;
///     MIND_FLAY = 501;
///     SHADOWFORM = 521;
///     SILENCE = 541;
///     IMPROVED_PSYCHIC_SCREAM = 542;
///     CALL_OF_FLAME = 561;
///     CALL_OF_THUNDER = 562;
///     CONCUSSION = 563;
///     CONVECTION = 564;
///     ELEMENTAL_FURY = 565;
///     IMPROVED_FIRE_TOTEMS = 567;
///     EARTHS_GRASP = 572;
///     ELEMENTAL_MASTERY = 573;
///     ELEMENTAL_FOCUS = 574;
///     REVERBERATION = 575;
///     ANCESTRAL_HEALING = 581;
///     TOTEMIC_MASTERY = 582;
///     NATURES_GUIDANCE = 583;
///     IMPROVED_HEALING_WAVE = 586;
///     SHAMAN_HEALING_FOCUS = 587;
///     RESTORATIVE_TOTEMS = 588;
///     IMPROVED_REINCARNATION = 589;
///     MANA_TIDE_TOTEM = 590;
///     SHAMAN_NATURES_SWIFTNESS = 591;
///     PURIFICATION = 592;
///     TIDAL_FOCUS = 593;
///     TIDAL_MASTERY = 594;
///     TOTEMIC_FOCUS = 595;
///     SHAMAN_ANTICIPATION = 601;
///     SHAMAN_FLURRY = 602;
///     IMPROVED_GHOST_WOLF = 605;
///     IMPROVED_LIGHTNING_SHIELD = 607;
///     GUARDIAN_TOTEMS = 609;
///     ENHANCING_TOTEMS = 610;
///     ELEMENTAL_WEAPONS = 611;
///     SHAMAN_SHIELD_SPECIALIZATION = 612;
///     THUNDERING_STRIKES = 613;
///     ANCESTRAL_KNOWLEDGE = 614;
///     SHAMAN_TOUGHNESS = 615;
///     PARRY = 616;
///     TWO_HANDED_AXES_AND_MACES = 617;
///     TACTICAL_MASTERY = 641;
///     BLOOD_CRAZE = 661;
///     IMPALE = 662;
///     HEMORRHAGE = 681;
///     VILE_POISONS = 682;
///     WARRIOR_ONE_HANDED_WEAPON_SPECIALIZATION = 702;
///     LIGHTNING_MASTERY = 721;
///     ARCTIC_REACH = 741;
///     NATURES_GRASP = 761;
///     IMPROVED_WRATH = 762;
///     IMPROVED_MOONFIRE = 763;
///     NATURES_REACH = 764;
///     NATURAL_SHAPESHIFTER = 781;
///     IMPROVED_THORNS = 782;
///     MOONGLOW = 783;
///     IMPROVED_STARFIRE = 784;
///     IMPROVED_ENTANGLING_ROOTS = 787;
///     OMEN_OF_CLARITY = 788;
///     NATURES_GRACE = 789;
///     MOONFURY = 790;
///     NATURAL_WEAPONS = 791;
///     DRUID_VENGEANCE = 792;
///     MOONKIN_FORM = 793;
///     DRUID_THICK_HIDE = 794;
///     FERAL_AGGRESSION = 795;
///     DRUID_FEROCITY = 796;
///     BRUTAL_IMPACT = 797;
///     SHARPENED_CLAWS = 798;
///     FERAL_INSTINCT = 799;
///     BLOOD_FRENZY = 800;
///     PRIMAL_FURY = 801;
///     IMPROVED_SHRED = 802;
///     PREDATORY_STRIKES = 803;
///     FERAL_CHARGE = 804;
///     SAVAGE_FURY = 805;
///     FELINE_SWIFTNESS = 807;
///     HEART_OF_THE_WILD = 808;
///     LEADER_OF_THE_PACK = 809;
///     IMPROVED_MARK_OF_THE_WILD = 821;
///     FUROR = 822;
///     NATURES_FOCUS = 823;
///     IMPROVED_HEALING_TOUCH = 824;
///     IMPROVED_REGROWTH = 825;
///     IMPROVED_ENRAGE = 826;
///     INSECT_SWARM = 827;
///     GIFT_OF_NATURE = 828;
///     REFLECTION = 829;
///     IMPROVED_REJUVENATION = 830;
///     DRUID_NATURES_SWIFTNESS = 831;
///     SUBTLETY = 841;
///     IMPROVED_TRANQUILITY = 842;
///     TRANQUIL_SPIRIT = 843;
///     SWIFTMEND = 844;
///     SHADOW_REACH = 881;
///     STORMSTRIKE = 901;
///     IMPROVED_NATURES_GRASP = 921;
///     CATACLYSM = 941;
///     BANE = 943;
///     IMPROVED_SHADOW_BOLT = 944;
///     IMPROVED_IMMOLATE = 961;
///     SHADOWBURN = 963;
///     DESTRUCTIVE_REACH = 964;
///     IMPROVED_SEARING_PAIN = 965;
///     EMBERSTORM = 966;
///     RUIN = 967;
///     CONFLAGRATE = 968;
///     DEVASTATION = 981;
///     AFTERMATH = 982;
///     IMPROVED_FIREBOLT = 983;
///     IMPROVED_LASH_OF_PAIN = 984;
///     INTENSITY = 985;
///     PYROCLASM = 986;
///     FEL_CONCENTRATION = 1001;
///     NIGHTFALL = 1002;
///     IMPROVED_CORRUPTION = 1003;
///     IMPROVED_DRAIN_LIFE = 1004;
///     SUPPRESSION = 1005;
///     IMPROVED_CURSE_OF_WEAKNESS = 1006;
///     IMPROVED_LIFE_TAP = 1007;
///     GRIM_REACH = 1021;
///     DARK_PACT = 1022;
///     SIPHON_LIFE = 1041;
///     SHADOW_MASTERY = 1042;
///     AMPLIFY_CURSE = 1061;
///     CURSE_OF_EXHAUSTION = 1081;
///     IMPROVED_CURSE_OF_EXHAUSTION = 1082;
///     IMPROVED_DRAIN_SOUL = 1101;
///     IMPROVED_DRAIN_MANA = 1121;
///     AGGRESSION = 1122;
///     SERRATED_BLADES = 1123;
///     INCINERATE = 1141;
///     ARCANE_MEDITATION = 1142;
///     FAERIE_FIRE_FERAL = 1162;
///     DIVINE_FURY = 1181;
///     MENTAL_STRENGTH = 1201;
///     FORCE_OF_WILL = 1202;
///     IMPROVED_HEALTHSTONE = 1221;
///     IMPROVED_IMP = 1222;
///     DEMONIC_EMBRACE = 1223;
///     IMPROVED_HEALTH_FUNNEL = 1224;
///     IMPROVED_VOIDWALKER = 1225;
///     FEL_DOMINATION = 1226;
///     MASTER_SUMMONER = 1227;
///     FEL_STAMINA = 1241;
///     FEL_INTELLECT = 1242;
///     IMPROVED_SUCCUBUS = 1243;
///     MASTER_DEMONOLOGIST = 1244;
///     IMPROVED_FIRESTONE = 1261;
///     UNHOLY_POWER = 1262;
///     IMPROVED_SPELLSTONE = 1263;
///     DEMONIC_SACRIFICE = 1281;
///     SOUL_LINK = 1282;
///     IMPROVED_ENSLAVE_DEMON = 1283;
///     IMPROVED_CURSE_OF_AGONY = 1284;
///     HUMANOID_SLAYING = 1301;
///     HUNTER_LIGHTNING_REFLEXES = 1303;
///     ENTRAPMENT = 1304;
///     IMPROVED_WING_CLIP = 1305;
///     CLEVER_TRAPS = 1306;
///     DETERRENCE = 1308;
///     IMPROVED_FEIGN_DEATH = 1309;
///     SUREFOOTED = 1310;
///     HUNTER_DEFLECTION = 1311;
///     COUNTERATTACK = 1312;
///     KILLER_INSTINCT = 1321;
///     TRAP_MASTERY = 1322;
///     WYVERN_STING = 1325;
///     IMPROVED_CONCUSSIVE_SHOT = 1341;
///     EFFICIENCY = 1342;
///     IMPROVED_HUNTERS_MARK = 1343;
///     LETHAL_SHOTS = 1344;
///     AIMED_SHOT = 1345;
///     IMPROVED_ARCANE_SHOT = 1346;
///     BARRAGE = 1347;
///     IMPROVED_SERPENT_STING = 1348;
///     MORTAL_SHOTS = 1349;
///     IMPROVED_SCORPID_STING = 1351;
///     HAWK_EYE = 1352;
///     SCATTER_SHOT = 1353;
///     TRUESHOT_AURA = 1361;
///     RANGED_WEAPON_SPECIALIZATION = 1362;
///     IMPROVED_ASPECT_OF_THE_MONKEY = 1381;
///     IMPROVED_ASPECT_OF_THE_HAWK = 1382;
///     PATHFINDING = 1384;
///     IMPROVED_MEND_PET = 1385;
///     BESTIAL_WRATH = 1386;
///     INTIMIDATION = 1387;
///     SPIRIT_BOND = 1388;
///     ENDURANCE_TRAINING = 1389;
///     BESTIAL_DISCIPLINE = 1390;
///     BESTIAL_SWIFTNESS = 1391;
///     HUNTER_FEROCITY = 1393;
///     HUNTER_THICK_HIDE = 1395;
///     UNLEASHED_FURY = 1396;
///     FRENZY = 1397;
///     IMPROVED_BLESSING_OF_MIGHT = 1401;
///     PALADIN_VENGEANCE = 1402;
///     PALADIN_DEFLECTION = 1403;
///     IMPROVED_RETRIBUTION_AURA = 1405;
///     BENEDICTION = 1407;
///     SANCTITY_AURA = 1409;
///     PALADIN_TWO_HANDED_WEAPON_SPECIALIZATION = 1410;
///     CONVICTION = 1411;
///     REDOUBT = 1421;
///     IMPROVED_DEVOTION_AURA = 1422;
///     PALADIN_TOUGHNESS = 1423;
///     PALADIN_SHIELD_SPECIALIZATION = 1424;
///     GUARDIANS_FAVOR = 1425;
///     RECKONING = 1426;
///     PALADIN_ONE_HANDED_WEAPON_SPECIALIZATION = 1429;
///     HOLY_SHIELD = 1430;
///     BLESSING_OF_SANCTUARY = 1431;
///     SPIRITUAL_FOCUS = 1432;
///     DIVINE_FAVOR = 1433;
///     CONSECRATION = 1435;
///     REPENTANCE = 1441;
///     BLESSING_OF_KINGS = 1442;
///     IMPROVED_LAY_ON_HANDS = 1443;
///     HEALING_LIGHT = 1444;
///     IMPROVED_BLESSING_OF_WISDOM = 1446;
///     DIVINE_INTELLECT = 1449;
///     DIVINE_STRENGTH = 1450;
///     ILLUMINATION = 1461;
///     IMPROVED_SEAL_OF_RIGHTEOUSNESS = 1463;
///     IMPROVED_SEAL_OF_THE_CRUSADER = 1464;
///     LASTING_JUDGEMENT = 1465;
///     SEAL_OF_COMMAND = 1481;
///     IMPROVED_RIGHTEOUS_FURY = 1501;
///     HOLY_SHOCK = 1502;
///     IMPROVED_HAMMER_OF_JUSTICE = 1521;
///     IMPROVED_BERSERKER_RAGE = 1541;
///     IMPROVED_EXECUTE = 1542;
///     IMPROVED_INTERCEPT = 1543;
///     SPIRIT_OF_REDEMPTION = 1561;
///     WARRIOR_DUAL_WIELD_SPECIALIZATION = 1581;
///     WARRIOR_SHIELD_SPECIALIZATION = 1601;
///     SAVAGE_STRIKES = 1621;
///     SURVIVALIST = 1622;
///     MONSTER_SLAYING = 1623;
///     IMPROVED_EYES_OF_THE_BEAST = 1624;
///     IMPROVED_REVIVE_PET = 1625;
///     IMPROVED_CONCENTRATION_AURA = 1626;
///     HOLY_POWER = 1627;
///     UNYIELDING_FAITH = 1628;
///     PALADIN_ANTICIPATION = 1629;
///     PALADIN_PRECISION = 1630;
///     IMPROVED_JUDGEMENT = 1631;
///     EYE_FOR_AN_EYE = 1632;
///     VINDICATION = 1633;
///     PURSUIT_OF_JUSTICE = 1634;
///     HOLY_REACH = 1635;
///     BLESSED_RECOVERY = 1636;
///     LIGHTWELL = 1637;
///     IMPROVED_VAMPIRIC_EMBRACE = 1638;
///     MASTER_OF_ELEMENTS = 1639;
///     ELEMENTAL_WARDING = 1640;
///     STORM_REACH = 1641;
///     EYE_OF_THE_STORM = 1642;
///     WEAPON_MASTERY = 1643;
///     ELEMENTAL_DEVASTATION = 1645;
///     HEALING_GRACE = 1646;
///     IMPROVED_WEAPON_TOTEMS = 1647;
///     HEALING_WAY = 1648;
///     ELEMENTAL_PRECISION = 1649;
///     MAGIC_ABSORPTION = 1650;
///     SLEIGHT_OF_HAND = 1700;
///     HEIGHTENED_SENSES = 1701;
///     DEADLINESS = 1702;
///     WEAPON_EXPERTISE = 1703;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Talent {
    BurningSoul,
    ImprovedFireWard,
    ImprovedScorch,
    ImprovedFireball,
    ImprovedFireBlast,
    FlameThrowing,
    Pyroblast,
    Impact,
    ImprovedFlamestrike,
    BlastWave,
    CriticalMass,
    Ignite,
    FirePower,
    Combustion,
    ImprovedFrostbolt,
    Frostbite,
    PiercingIce,
    ImprovedFrostNova,
    ImprovedBlizzard,
    ImprovedConeOfCold,
    Permafrost,
    FrostChanneling,
    Shatter,
    WintersChill,
    ColdSnap,
    FrostWarding,
    IceBarrier,
    IceBlock,
    IceShards,
    ArcaneSubtlety,
    ArcaneConcentration,
    ArcaneFocus,
    ArcaneMind,
    MageWandSpecialization,
    ImprovedArcaneMissiles,
    ImprovedArcaneExplosion,
    MagicAttunement,
    ImprovedManaShield,
    ArcaneResilience,
    PresenceOfMind,
    ArcanePower,
    ImprovedCounterspell,
    DeepWounds,
    WarriorSwordSpecialization,
    ImprovedHeroicStrike,
    WarriorMaceSpecialization,
    ImprovedCharge,
    ImprovedRend,
    ImprovedThunderClap,
    ImprovedHamstring,
    WarriorDeflection,
    ImprovedOverpower,
    AxeSpecialization,
    SweepingStrikes,
    PolearmSpecialization,
    MortalStrike,
    WarriorTwoHandedWeaponSpecialization,
    AngerManagement,
    WarriorAnticipation,
    WarriorToughness,
    IronWill,
    ImprovedBloodrage,
    ImprovedTaunt,
    Defiance,
    ImprovedShieldBlock,
    ImprovedSunderArmor,
    ImprovedRevenge,
    ShieldSlam,
    ImprovedShieldBash,
    ImprovedShieldWall,
    ImprovedDisarm,
    ConcussionBlow,
    LastStand,
    ImprovedBattleShout,
    Enrage,
    WarriorFlurry,
    Cruelty,
    BoomingVoice,
    UnbridledWrath,
    PiercingHowl,
    ImprovedDemoralizingShout,
    DeathWish,
    ImprovedCleave,
    Bloodthirst,
    ImprovedSlam,
    RoguePrecision,
    DaggerSpecialization,
    FistWeaponSpecialization,
    RogueMaceSpecialization,
    RogueLightningReflexes,
    RogueDeflection,
    ImprovedSinisterStrike,
    ImprovedBackstab,
    ImprovedGouge,
    Endurance,
    AdrenalineRush,
    ImprovedKick,
    RogueDualWieldSpecialization,
    ImprovedSprint,
    BladeFlurry,
    MasterOfDeception,
    RogueSwordSpecialization,
    Camouflage,
    Initiative,
    Setup,
    Elusiveness,
    Opportunity,
    ImprovedSap,
    ImprovedAmbush,
    DirtyDeeds,
    ImprovedPoisons,
    Lethality,
    Malice,
    RemorselessAttacks,
    Ruthlessness,
    Murder,
    ImprovedEviscerate,
    ImprovedSliceAndDice,
    ImprovedExposeArmor,
    ImprovedKidneyShot,
    ColdBlood,
    RelentlessStrikes,
    SealFate,
    Preparation,
    Riposte,
    GhostlyStrike,
    Martyrdom,
    PowerInfusion,
    MentalAgility,
    UnbreakableWill,
    ImprovedPowerWordShield,
    ImprovedPowerWordFortitude,
    PriestWandSpecialization,
    ImprovedInnerFire,
    Meditation,
    InnerFocus,
    ImprovedManaBurn,
    DivineSpirit,
    SilentResolve,
    Inspiration,
    Premeditation,
    Vigor,
    HolySpecialization,
    SpiritualGuidance,
    SearingLight,
    SpiritualHealing,
    ImprovedRenew,
    ImprovedHealing,
    PriestHealingFocus,
    SpellWarding,
    ImprovedPrayerOfHealing,
    ArcaneInstability,
    HolyNova,
    ShadowWeaving,
    Darkness,
    ShadowFocus,
    Blackout,
    SpiritTap,
    ShadowAffinity,
    ImprovedMindBlast,
    ImprovedShadowWordPain,
    ImprovedFade,
    VampiricEmbrace,
    MindFlay,
    Shadowform,
    Silence,
    ImprovedPsychicScream,
    CallOfFlame,
    CallOfThunder,
    Concussion,
    Convection,
    ElementalFury,
    ImprovedFireTotems,
    EarthsGrasp,
    ElementalMastery,
    ElementalFocus,
    Reverberation,
    AncestralHealing,
    TotemicMastery,
    NaturesGuidance,
    ImprovedHealingWave,
    ShamanHealingFocus,
    RestorativeTotems,
    ImprovedReincarnation,
    ManaTideTotem,
    ShamanNaturesSwiftness,
    Purification,
    TidalFocus,
    TidalMastery,
    TotemicFocus,
    ShamanAnticipation,
    ShamanFlurry,
    ImprovedGhostWolf,
    ImprovedLightningShield,
    GuardianTotems,
    EnhancingTotems,
    ElementalWeapons,
    ShamanShieldSpecialization,
    ThunderingStrikes,
    AncestralKnowledge,
    ShamanToughness,
    Parry,
    TwoHandedAxesAndMaces,
    TacticalMastery,
    BloodCraze,
    Impale,
    Hemorrhage,
    VilePoisons,
    WarriorOneHandedWeaponSpecialization,
    LightningMastery,
    ArcticReach,
    NaturesGrasp,
    ImprovedWrath,
    ImprovedMoonfire,
    NaturesReach,
    NaturalShapeshifter,
    ImprovedThorns,
    Moonglow,
    ImprovedStarfire,
    ImprovedEntanglingRoots,
    OmenOfClarity,
    NaturesGrace,
    Moonfury,
    NaturalWeapons,
    DruidVengeance,
    MoonkinForm,
    DruidThickHide,
    FeralAggression,
    DruidFerocity,
    BrutalImpact,
    SharpenedClaws,
    FeralInstinct,
    BloodFrenzy,
    PrimalFury,
    ImprovedShred,
    PredatoryStrikes,
    FeralCharge,
    SavageFury,
    FelineSwiftness,
    HeartOfTheWild,
    LeaderOfThePack,
    ImprovedMarkOfTheWild,
    Furor,
    NaturesFocus,
    ImprovedHealingTouch,
    ImprovedRegrowth,
    ImprovedEnrage,
    InsectSwarm,
    GiftOfNature,
    Reflection,
    ImprovedRejuvenation,
    DruidNaturesSwiftness,
    Subtlety,
    ImprovedTranquility,
    TranquilSpirit,
    Swiftmend,
    ShadowReach,
    Stormstrike,
    ImprovedNaturesGrasp,
    Cataclysm,
    Bane,
    ImprovedShadowBolt,
    ImprovedImmolate,
    Shadowburn,
    DestructiveReach,
    ImprovedSearingPain,
    Emberstorm,
    Ruin,
    Conflagrate,
    Devastation,
    Aftermath,
    ImprovedFirebolt,
    ImprovedLashOfPain,
    Intensity,
    Pyroclasm,
    FelConcentration,
    Nightfall,
    ImprovedCorruption,
    ImprovedDrainLife,
    Suppression,
    ImprovedCurseOfWeakness,
    ImprovedLifeTap,
    GrimReach,
    DarkPact,
    SiphonLife,
    ShadowMastery,
    AmplifyCurse,
    CurseOfExhaustion,
    ImprovedCurseOfExhaustion,
    ImprovedDrainSoul,
    ImprovedDrainMana,
    Aggression,
    SerratedBlades,
    Incinerate,
    ArcaneMeditation,
    FaerieFireFeral,
    DivineFury,
    MentalStrength,
    ForceOfWill,
    ImprovedHealthstone,
    ImprovedImp,
    DemonicEmbrace,
    ImprovedHealthFunnel,
    ImprovedVoidwalker,
    FelDomination,
    MasterSummoner,
    FelStamina,
    FelIntellect,
    ImprovedSuccubus,
    MasterDemonologist,
    ImprovedFirestone,
    UnholyPower,
    ImprovedSpellstone,
    DemonicSacrifice,
    SoulLink,
    ImprovedEnslaveDemon,
    ImprovedCurseOfAgony,
    HumanoidSlaying,
    HunterLightningReflexes,
    Entrapment,
    ImprovedWingClip,
    CleverTraps,
    Deterrence,
    ImprovedFeignDeath,
    Surefooted,
    HunterDeflection,
    Counterattack,
    KillerInstinct,
    TrapMastery,
    WyvernSting,
    ImprovedConcussiveShot,
    Efficiency,
    ImprovedHuntersMark,
    LethalShots,
    AimedShot,
    ImprovedArcaneShot,
    Barrage,
    ImprovedSerpentSting,
    MortalShots,
    ImprovedScorpidSting,
    HawkEye,
    ScatterShot,
    TrueshotAura,
    RangedWeaponSpecialization,
    ImprovedAspectOfTheMonkey,
    ImprovedAspectOfTheHawk,
    Pathfinding,
    ImprovedMendPet,
    BestialWrath,
    Intimidation,
    SpiritBond,
    EnduranceTraining,
    BestialDiscipline,
    BestialSwiftness,
    HunterFerocity,
    HunterThickHide,
    UnleashedFury,
    Frenzy,
    ImprovedBlessingOfMight,
    PaladinVengeance,
    PaladinDeflection,
    ImprovedRetributionAura,
    Benediction,
    SanctityAura,
    PaladinTwoHandedWeaponSpecialization,
    Conviction,
    Redoubt,
    ImprovedDevotionAura,
    PaladinToughness,
    PaladinShieldSpecialization,
    GuardiansFavor,
    Reckoning,
    PaladinOneHandedWeaponSpecialization,
    HolyShield,
    BlessingOfSanctuary,
    SpiritualFocus,
    DivineFavor,
    Consecration,
    Repentance,
    BlessingOfKings,
    ImprovedLayOnHands,
    HealingLight,
    ImprovedBlessingOfWisdom,
    DivineIntellect,
    DivineStrength,
    Illumination,
    ImprovedSealOfRighteousness,
    ImprovedSealOfTheCrusader,
    LastingJudgement,
    SealOfCommand,
    ImprovedRighteousFury,
    HolyShock,
    ImprovedHammerOfJustice,
    ImprovedBerserkerRage,
    ImprovedExecute,
    ImprovedIntercept,
    SpiritOfRedemption,
    WarriorDualWieldSpecialization,
    WarriorShieldSpecialization,
    SavageStrikes,
    Survivalist,
    MonsterSlaying,
    ImprovedEyesOfTheBeast,
    ImprovedRevivePet,
    ImprovedConcentrationAura,
    HolyPower,
    UnyieldingFaith,
    PaladinAnticipation,
    PaladinPrecision,
    ImprovedJudgement,
    EyeForAnEye,
    Vindication,
    PursuitOfJustice,
    HolyReach,
    BlessedRecovery,
    Lightwell,
    ImprovedVampiricEmbrace,
    MasterOfElements,
    ElementalWarding,
    StormReach,
    EyeOfTheStorm,
    WeaponMastery,
    ElementalDevastation,
    HealingGrace,
    ImprovedWeaponTotems,
    HealingWay,
    ElementalPrecision,
    MagicAbsorption,
    SleightOfHand,
    HeightenedSenses,
    Deadliness,
    WeaponExpertise,
}

impl Talent {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::BurningSoul => 0x17,
            Self::ImprovedFireWard => 0x18,
            Self::ImprovedScorch => 0x19,
            Self::ImprovedFireball => 0x1a,
            Self::ImprovedFireBlast => 0x1b,
            Self::FlameThrowing => 0x1c,
            Self::Pyroblast => 0x1d,
            Self::Impact => 0x1e,
            Self::ImprovedFlamestrike => 0x1f,
            Self::BlastWave => 0x20,
            Self::CriticalMass => 0x21,
            Self::Ignite => 0x22,
            Self::FirePower => 0x23,
            Self::Combustion => 0x24,
            Self::ImprovedFrostbolt => 0x25,
            Self::Frostbite => 0x26,
            Self::PiercingIce => 0x3d,
            Self::ImprovedFrostNova => 0x3e,
            Self::ImprovedBlizzard => 0x3f,
            Self::ImprovedConeOfCold => 0x40,
            Self::Permafrost => 0x41,
            Self::FrostChanneling => 0x42,
            Self::Shatter => 0x43,
            Self::WintersChill => 0x44,
            Self::ColdSnap => 0x45,
            Self::FrostWarding => 0x46,
            Self::IceBarrier => 0x47,
            Self::IceBlock => 0x48,
            Self::IceShards => 0x49,
            Self::ArcaneSubtlety => 0x4a,
            Self::ArcaneConcentration => 0x4b,
            Self::ArcaneFocus => 0x4c,
            Self::ArcaneMind => 0x4d,
            Self::MageWandSpecialization => 0x4e,
            Self::ImprovedArcaneMissiles => 0x50,
            Self::ImprovedArcaneExplosion => 0x51,
            Self::MagicAttunement => 0x52,
            Self::ImprovedManaShield => 0x53,
            Self::ArcaneResilience => 0x55,
            Self::PresenceOfMind => 0x56,
            Self::ArcanePower => 0x57,
            Self::ImprovedCounterspell => 0x58,
            Self::DeepWounds => 0x79,
            Self::WarriorSwordSpecialization => 0x7b,
            Self::ImprovedHeroicStrike => 0x7c,
            Self::WarriorMaceSpecialization => 0x7d,
            Self::ImprovedCharge => 0x7e,
            Self::ImprovedRend => 0x7f,
            Self::ImprovedThunderClap => 0x80,
            Self::ImprovedHamstring => 0x81,
            Self::WarriorDeflection => 0x82,
            Self::ImprovedOverpower => 0x83,
            Self::AxeSpecialization => 0x84,
            Self::SweepingStrikes => 0x85,
            Self::PolearmSpecialization => 0x86,
            Self::MortalStrike => 0x87,
            Self::WarriorTwoHandedWeaponSpecialization => 0x88,
            Self::AngerManagement => 0x89,
            Self::WarriorAnticipation => 0x8a,
            Self::WarriorToughness => 0x8c,
            Self::IronWill => 0x8d,
            Self::ImprovedBloodrage => 0x8e,
            Self::ImprovedTaunt => 0x8f,
            Self::Defiance => 0x90,
            Self::ImprovedShieldBlock => 0x91,
            Self::ImprovedSunderArmor => 0x92,
            Self::ImprovedRevenge => 0x93,
            Self::ShieldSlam => 0x94,
            Self::ImprovedShieldBash => 0x95,
            Self::ImprovedShieldWall => 0x96,
            Self::ImprovedDisarm => 0x97,
            Self::ConcussionBlow => 0x98,
            Self::LastStand => 0x99,
            Self::ImprovedBattleShout => 0x9a,
            Self::Enrage => 0x9b,
            Self::WarriorFlurry => 0x9c,
            Self::Cruelty => 0x9d,
            Self::BoomingVoice => 0x9e,
            Self::UnbridledWrath => 0x9f,
            Self::PiercingHowl => 0xa0,
            Self::ImprovedDemoralizingShout => 0xa1,
            Self::DeathWish => 0xa5,
            Self::ImprovedCleave => 0xa6,
            Self::Bloodthirst => 0xa7,
            Self::ImprovedSlam => 0xa8,
            Self::RoguePrecision => 0xb5,
            Self::DaggerSpecialization => 0xb6,
            Self::FistWeaponSpecialization => 0xb7,
            Self::RogueMaceSpecialization => 0xb8,
            Self::RogueLightningReflexes => 0xba,
            Self::RogueDeflection => 0xbb,
            Self::ImprovedSinisterStrike => 0xc9,
            Self::ImprovedBackstab => 0xca,
            Self::ImprovedGouge => 0xcb,
            Self::Endurance => 0xcc,
            Self::AdrenalineRush => 0xcd,
            Self::ImprovedKick => 0xce,
            Self::RogueDualWieldSpecialization => 0xdd,
            Self::ImprovedSprint => 0xde,
            Self::BladeFlurry => 0xdf,
            Self::MasterOfDeception => 0xf1,
            Self::RogueSwordSpecialization => 0xf2,
            Self::Camouflage => 0xf4,
            Self::Initiative => 0xf5,
            Self::Setup => 0xf6,
            Self::Elusiveness => 0xf7,
            Self::Opportunity => 0x105,
            Self::ImprovedSap => 0x106,
            Self::ImprovedAmbush => 0x107,
            Self::DirtyDeeds => 0x109,
            Self::ImprovedPoisons => 0x10c,
            Self::Lethality => 0x10d,
            Self::Malice => 0x10e,
            Self::RemorselessAttacks => 0x110,
            Self::Ruthlessness => 0x111,
            Self::Murder => 0x112,
            Self::ImprovedEviscerate => 0x114,
            Self::ImprovedSliceAndDice => 0x115,
            Self::ImprovedExposeArmor => 0x116,
            Self::ImprovedKidneyShot => 0x117,
            Self::ColdBlood => 0x118,
            Self::RelentlessStrikes => 0x119,
            Self::SealFate => 0x11b,
            Self::Preparation => 0x11c,
            Self::Riposte => 0x12d,
            Self::GhostlyStrike => 0x12f,
            Self::Martyrdom => 0x141,
            Self::PowerInfusion => 0x142,
            Self::MentalAgility => 0x155,
            Self::UnbreakableWill => 0x156,
            Self::ImprovedPowerWordShield => 0x157,
            Self::ImprovedPowerWordFortitude => 0x158,
            Self::PriestWandSpecialization => 0x159,
            Self::ImprovedInnerFire => 0x15a,
            Self::Meditation => 0x15b,
            Self::InnerFocus => 0x15c,
            Self::ImprovedManaBurn => 0x15e,
            Self::DivineSpirit => 0x15f,
            Self::SilentResolve => 0x160,
            Self::Inspiration => 0x169,
            Self::Premeditation => 0x17d,
            Self::Vigor => 0x17e,
            Self::HolySpecialization => 0x191,
            Self::SpiritualGuidance => 0x192,
            Self::SearingLight => 0x193,
            Self::SpiritualHealing => 0x194,
            Self::ImprovedRenew => 0x196,
            Self::ImprovedHealing => 0x198,
            Self::PriestHealingFocus => 0x19a,
            Self::SpellWarding => 0x19b,
            Self::ImprovedPrayerOfHealing => 0x19d,
            Self::ArcaneInstability => 0x1a5,
            Self::HolyNova => 0x1ba,
            Self::ShadowWeaving => 0x1cd,
            Self::Darkness => 0x1ce,
            Self::ShadowFocus => 0x1cf,
            Self::Blackout => 0x1d0,
            Self::SpiritTap => 0x1d1,
            Self::ShadowAffinity => 0x1d2,
            Self::ImprovedMindBlast => 0x1e1,
            Self::ImprovedShadowWordPain => 0x1e2,
            Self::ImprovedFade => 0x1e3,
            Self::VampiricEmbrace => 0x1e4,
            Self::MindFlay => 0x1f5,
            Self::Shadowform => 0x209,
            Self::Silence => 0x21d,
            Self::ImprovedPsychicScream => 0x21e,
            Self::CallOfFlame => 0x231,
            Self::CallOfThunder => 0x232,
            Self::Concussion => 0x233,
            Self::Convection => 0x234,
            Self::ElementalFury => 0x235,
            Self::ImprovedFireTotems => 0x237,
            Self::EarthsGrasp => 0x23c,
            Self::ElementalMastery => 0x23d,
            Self::ElementalFocus => 0x23e,
            Self::Reverberation => 0x23f,
            Self::AncestralHealing => 0x245,
            Self::TotemicMastery => 0x246,
            Self::NaturesGuidance => 0x247,
            Self::ImprovedHealingWave => 0x24a,
            Self::ShamanHealingFocus => 0x24b,
            Self::RestorativeTotems => 0x24c,
            Self::ImprovedReincarnation => 0x24d,
            Self::ManaTideTotem => 0x24e,
            Self::ShamanNaturesSwiftness => 0x24f,
            Self::Purification => 0x250,
            Self::TidalFocus => 0x251,
            Self::TidalMastery => 0x252,
            Self::TotemicFocus => 0x253,
            Self::ShamanAnticipation => 0x259,
            Self::ShamanFlurry => 0x25a,
            Self::ImprovedGhostWolf => 0x25d,
            Self::ImprovedLightningShield => 0x25f,
            Self::GuardianTotems => 0x261,
            Self::EnhancingTotems => 0x262,
            Self::ElementalWeapons => 0x263,
            Self::ShamanShieldSpecialization => 0x264,
            Self::ThunderingStrikes => 0x265,
            Self::AncestralKnowledge => 0x266,
            Self::ShamanToughness => 0x267,
            Self::Parry => 0x268,
            Self::TwoHandedAxesAndMaces => 0x269,
            Self::TacticalMastery => 0x281,
            Self::BloodCraze => 0x295,
            Self::Impale => 0x296,
            Self::Hemorrhage => 0x2a9,
            Self::VilePoisons => 0x2aa,
            Self::WarriorOneHandedWeaponSpecialization => 0x2be,
            Self::LightningMastery => 0x2d1,
            Self::ArcticReach => 0x2e5,
            Self::NaturesGrasp => 0x2f9,
            Self::ImprovedWrath => 0x2fa,
            Self::ImprovedMoonfire => 0x2fb,
            Self::NaturesReach => 0x2fc,
            Self::NaturalShapeshifter => 0x30d,
            Self::ImprovedThorns => 0x30e,
            Self::Moonglow => 0x30f,
            Self::ImprovedStarfire => 0x310,
            Self::ImprovedEntanglingRoots => 0x313,
            Self::OmenOfClarity => 0x314,
            Self::NaturesGrace => 0x315,
            Self::Moonfury => 0x316,
            Self::NaturalWeapons => 0x317,
            Self::DruidVengeance => 0x318,
            Self::MoonkinForm => 0x319,
            Self::DruidThickHide => 0x31a,
            Self::FeralAggression => 0x31b,
            Self::DruidFerocity => 0x31c,
            Self::BrutalImpact => 0x31d,
            Self::SharpenedClaws => 0x31e,
            Self::FeralInstinct => 0x31f,
            Self::BloodFrenzy => 0x320,
            Self::PrimalFury => 0x321,
            Self::ImprovedShred => 0x322,
            Self::PredatoryStrikes => 0x323,
            Self::FeralCharge => 0x324,
            Self::SavageFury => 0x325,
            Self::FelineSwiftness => 0x327,
            Self::HeartOfTheWild => 0x328,
            Self::LeaderOfThePack => 0x329,
            Self::ImprovedMarkOfTheWild => 0x335,
            Self::Furor => 0x336,
            Self::NaturesFocus => 0x337,
            Self::ImprovedHealingTouch => 0x338,
            Self::ImprovedRegrowth => 0x339,
            Self::ImprovedEnrage => 0x33a,
            Self::InsectSwarm => 0x33b,
            Self::GiftOfNature => 0x33c,
            Self::Reflection => 0x33d,
            Self::ImprovedRejuvenation => 0x33e,
            Self::DruidNaturesSwiftness => 0x33f,
            Self::Subtlety => 0x349,
            Self::ImprovedTranquility => 0x34a,
            Self::TranquilSpirit => 0x34b,
            Self::Swiftmend => 0x34c,
            Self::ShadowReach => 0x371,
            Self::Stormstrike => 0x385,
            Self::ImprovedNaturesGrasp => 0x399,
            Self::Cataclysm => 0x3ad,
            Self::Bane => 0x3af,
            Self::ImprovedShadowBolt => 0x3b0,
            Self::ImprovedImmolate => 0x3c1,
            Self::Shadowburn => 0x3c3,
            Self::DestructiveReach => 0x3c4,
            Self::ImprovedSearingPain => 0x3c5,
            Self::Emberstorm => 0x3c6,
            Self::Ruin => 0x3c7,
            Self::Conflagrate => 0x3c8,
            Self::Devastation => 0x3d5,
            Self::Aftermath => 0x3d6,
            Self::ImprovedFirebolt => 0x3d7,
            Self::ImprovedLashOfPain => 0x3d8,
            Self::Intensity => 0x3d9,
            Self::Pyroclasm => 0x3da,
            Self::FelConcentration => 0x3e9,
            Self::Nightfall => 0x3ea,
            Self::ImprovedCorruption => 0x3eb,
            Self::ImprovedDrainLife => 0x3ec,
            Self::Suppression => 0x3ed,
            Self::ImprovedCurseOfWeakness => 0x3ee,
            Self::ImprovedLifeTap => 0x3ef,
            Self::GrimReach => 0x3fd,
            Self::DarkPact => 0x3fe,
            Self::SiphonLife => 0x411,
            Self::ShadowMastery => 0x412,
            Self::AmplifyCurse => 0x425,
            Self::CurseOfExhaustion => 0x439,
            Self::ImprovedCurseOfExhaustion => 0x43a,
            Self::ImprovedDrainSoul => 0x44d,
            Self::ImprovedDrainMana => 0x461,
            Self::Aggression => 0x462,
            Self::SerratedBlades => 0x463,
            Self::Incinerate => 0x475,
            Self::ArcaneMeditation => 0x476,
            Self::FaerieFireFeral => 0x48a,
            Self::DivineFury => 0x49d,
            Self::MentalStrength => 0x4b1,
            Self::ForceOfWill => 0x4b2,
            Self::ImprovedHealthstone => 0x4c5,
            Self::ImprovedImp => 0x4c6,
            Self::DemonicEmbrace => 0x4c7,
            Self::ImprovedHealthFunnel => 0x4c8,
            Self::ImprovedVoidwalker => 0x4c9,
            Self::FelDomination => 0x4ca,
            Self::MasterSummoner => 0x4cb,
            Self::FelStamina => 0x4d9,
            Self::FelIntellect => 0x4da,
            Self::ImprovedSuccubus => 0x4db,
            Self::MasterDemonologist => 0x4dc,
            Self::ImprovedFirestone => 0x4ed,
            Self::UnholyPower => 0x4ee,
            Self::ImprovedSpellstone => 0x4ef,
            Self::DemonicSacrifice => 0x501,
            Self::SoulLink => 0x502,
            Self::ImprovedEnslaveDemon => 0x503,
            Self::ImprovedCurseOfAgony => 0x504,
            Self::HumanoidSlaying => 0x515,
            Self::HunterLightningReflexes => 0x517,
            Self::Entrapment => 0x518,
            Self::ImprovedWingClip => 0x519,
            Self::CleverTraps => 0x51a,
            Self::Deterrence => 0x51c,
            Self::ImprovedFeignDeath => 0x51d,
            Self::Surefooted => 0x51e,
            Self::HunterDeflection => 0x51f,
            Self::Counterattack => 0x520,
            Self::KillerInstinct => 0x529,
            Self::TrapMastery => 0x52a,
            Self::WyvernSting => 0x52d,
            Self::ImprovedConcussiveShot => 0x53d,
            Self::Efficiency => 0x53e,
            Self::ImprovedHuntersMark => 0x53f,
            Self::LethalShots => 0x540,
            Self::AimedShot => 0x541,
            Self::ImprovedArcaneShot => 0x542,
            Self::Barrage => 0x543,
            Self::ImprovedSerpentSting => 0x544,
            Self::MortalShots => 0x545,
            Self::ImprovedScorpidSting => 0x547,
            Self::HawkEye => 0x548,
            Self::ScatterShot => 0x549,
            Self::TrueshotAura => 0x551,
            Self::RangedWeaponSpecialization => 0x552,
            Self::ImprovedAspectOfTheMonkey => 0x565,
            Self::ImprovedAspectOfTheHawk => 0x566,
            Self::Pathfinding => 0x568,
            Self::ImprovedMendPet => 0x569,
            Self::BestialWrath => 0x56a,
            Self::Intimidation => 0x56b,
            Self::SpiritBond => 0x56c,
            Self::EnduranceTraining => 0x56d,
            Self::BestialDiscipline => 0x56e,
            Self::BestialSwiftness => 0x56f,
            Self::HunterFerocity => 0x571,
            Self::HunterThickHide => 0x573,
            Self::UnleashedFury => 0x574,
            Self::Frenzy => 0x575,
            Self::ImprovedBlessingOfMight => 0x579,
            Self::PaladinVengeance => 0x57a,
            Self::PaladinDeflection => 0x57b,
            Self::ImprovedRetributionAura => 0x57d,
            Self::Benediction => 0x57f,
            Self::SanctityAura => 0x581,
            Self::PaladinTwoHandedWeaponSpecialization => 0x582,
            Self::Conviction => 0x583,
            Self::Redoubt => 0x58d,
            Self::ImprovedDevotionAura => 0x58e,
            Self::PaladinToughness => 0x58f,
            Self::PaladinShieldSpecialization => 0x590,
            Self::GuardiansFavor => 0x591,
            Self::Reckoning => 0x592,
            Self::PaladinOneHandedWeaponSpecialization => 0x595,
            Self::HolyShield => 0x596,
            Self::BlessingOfSanctuary => 0x597,
            Self::SpiritualFocus => 0x598,
            Self::DivineFavor => 0x599,
            Self::Consecration => 0x59b,
            Self::Repentance => 0x5a1,
            Self::BlessingOfKings => 0x5a2,
            Self::ImprovedLayOnHands => 0x5a3,
            Self::HealingLight => 0x5a4,
            Self::ImprovedBlessingOfWisdom => 0x5a6,
            Self::DivineIntellect => 0x5a9,
            Self::DivineStrength => 0x5aa,
            Self::Illumination => 0x5b5,
            Self::ImprovedSealOfRighteousness => 0x5b7,
            Self::ImprovedSealOfTheCrusader => 0x5b8,
            Self::LastingJudgement => 0x5b9,
            Self::SealOfCommand => 0x5c9,
            Self::ImprovedRighteousFury => 0x5dd,
            Self::HolyShock => 0x5de,
            Self::ImprovedHammerOfJustice => 0x5f1,
            Self::ImprovedBerserkerRage => 0x605,
            Self::ImprovedExecute => 0x606,
            Self::ImprovedIntercept => 0x607,
            Self::SpiritOfRedemption => 0x619,
            Self::WarriorDualWieldSpecialization => 0x62d,
            Self::WarriorShieldSpecialization => 0x641,
            Self::SavageStrikes => 0x655,
            Self::Survivalist => 0x656,
            Self::MonsterSlaying => 0x657,
            Self::ImprovedEyesOfTheBeast => 0x658,
            Self::ImprovedRevivePet => 0x659,
            Self::ImprovedConcentrationAura => 0x65a,
            Self::HolyPower => 0x65b,
            Self::UnyieldingFaith => 0x65c,
            Self::PaladinAnticipation => 0x65d,
            Self::PaladinPrecision => 0x65e,
            Self::ImprovedJudgement => 0x65f,
            Self::EyeForAnEye => 0x660,
            Self::Vindication => 0x661,
            Self::PursuitOfJustice => 0x662,
            Self::HolyReach => 0x663,
            Self::BlessedRecovery => 0x664,
            Self::Lightwell => 0x665,
            Self::ImprovedVampiricEmbrace => 0x666,
            Self::MasterOfElements => 0x667,
            Self::ElementalWarding => 0x668,
            Self::StormReach => 0x669,
            Self::EyeOfTheStorm => 0x66a,
            Self::WeaponMastery => 0x66b,
            Self::ElementalDevastation => 0x66d,
            Self::HealingGrace => 0x66e,
            Self::ImprovedWeaponTotems => 0x66f,
            Self::HealingWay => 0x670,
            Self::ElementalPrecision => 0x671,
            Self::MagicAbsorption => 0x672,
            Self::SleightOfHand => 0x6a4,
            Self::HeightenedSenses => 0x6a5,
            Self::Deadliness => 0x6a6,
            Self::WeaponExpertise => 0x6a7,
        }
    }

}

impl Default for Talent {
    fn default() -> Self {
        Self::BurningSoul
    }
}

impl std::fmt::Display for Talent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BurningSoul => f.write_str("BurningSoul"),
            Self::ImprovedFireWard => f.write_str("ImprovedFireWard"),
            Self::ImprovedScorch => f.write_str("ImprovedScorch"),
            Self::ImprovedFireball => f.write_str("ImprovedFireball"),
            Self::ImprovedFireBlast => f.write_str("ImprovedFireBlast"),
            Self::FlameThrowing => f.write_str("FlameThrowing"),
            Self::Pyroblast => f.write_str("Pyroblast"),
            Self::Impact => f.write_str("Impact"),
            Self::ImprovedFlamestrike => f.write_str("ImprovedFlamestrike"),
            Self::BlastWave => f.write_str("BlastWave"),
            Self::CriticalMass => f.write_str("CriticalMass"),
            Self::Ignite => f.write_str("Ignite"),
            Self::FirePower => f.write_str("FirePower"),
            Self::Combustion => f.write_str("Combustion"),
            Self::ImprovedFrostbolt => f.write_str("ImprovedFrostbolt"),
            Self::Frostbite => f.write_str("Frostbite"),
            Self::PiercingIce => f.write_str("PiercingIce"),
            Self::ImprovedFrostNova => f.write_str("ImprovedFrostNova"),
            Self::ImprovedBlizzard => f.write_str("ImprovedBlizzard"),
            Self::ImprovedConeOfCold => f.write_str("ImprovedConeOfCold"),
            Self::Permafrost => f.write_str("Permafrost"),
            Self::FrostChanneling => f.write_str("FrostChanneling"),
            Self::Shatter => f.write_str("Shatter"),
            Self::WintersChill => f.write_str("Winter's Chill"),
            Self::ColdSnap => f.write_str("ColdSnap"),
            Self::FrostWarding => f.write_str("FrostWarding"),
            Self::IceBarrier => f.write_str("IceBarrier"),
            Self::IceBlock => f.write_str("IceBlock"),
            Self::IceShards => f.write_str("IceShards"),
            Self::ArcaneSubtlety => f.write_str("ArcaneSubtlety"),
            Self::ArcaneConcentration => f.write_str("ArcaneConcentration"),
            Self::ArcaneFocus => f.write_str("ArcaneFocus"),
            Self::ArcaneMind => f.write_str("ArcaneMind"),
            Self::MageWandSpecialization => f.write_str("MageWandSpecialization"),
            Self::ImprovedArcaneMissiles => f.write_str("ImprovedArcaneMissiles"),
            Self::ImprovedArcaneExplosion => f.write_str("ImprovedArcaneExplosion"),
            Self::MagicAttunement => f.write_str("MagicAttunement"),
            Self::ImprovedManaShield => f.write_str("ImprovedManaShield"),
            Self::ArcaneResilience => f.write_str("ArcaneResilience"),
            Self::PresenceOfMind => f.write_str("PresenceOfMind"),
            Self::ArcanePower => f.write_str("ArcanePower"),
            Self::ImprovedCounterspell => f.write_str("ImprovedCounterspell"),
            Self::DeepWounds => f.write_str("DeepWounds"),
            Self::WarriorSwordSpecialization => f.write_str("WarriorSwordSpecialization"),
            Self::ImprovedHeroicStrike => f.write_str("ImprovedHeroicStrike"),
            Self::WarriorMaceSpecialization => f.write_str("WarriorMaceSpecialization"),
            Self::ImprovedCharge => f.write_str("ImprovedCharge"),
            Self::ImprovedRend => f.write_str("ImprovedRend"),
            Self::ImprovedThunderClap => f.write_str("ImprovedThunderClap"),
            Self::ImprovedHamstring => f.write_str("ImprovedHamstring"),
            Self::WarriorDeflection => f.write_str("WarriorDeflection"),
            Self::ImprovedOverpower => f.write_str("ImprovedOverpower"),
            Self::AxeSpecialization => f.write_str("AxeSpecialization"),
            Self::SweepingStrikes => f.write_str("SweepingStrikes"),
            Self::PolearmSpecialization => f.write_str("PolearmSpecialization"),
            Self::MortalStrike => f.write_str("MortalStrike"),
            Self::WarriorTwoHandedWeaponSpecialization => f.write_str("WarriorTwoHandedWeaponSpecialization"),
            Self::AngerManagement => f.write_str("AngerManagement"),
            Self::WarriorAnticipation => f.write_str("WarriorAnticipation"),
            Self::WarriorToughness => f.write_str("WarriorToughness"),
            Self::IronWill => f.write_str("IronWill"),
            Self::ImprovedBloodrage => f.write_str("ImprovedBloodrage"),
            Self::ImprovedTaunt => f.write_str("ImprovedTaunt"),
            Self::Defiance => f.write_str("Defiance"),
            Self::ImprovedShieldBlock => f.write_str("ImprovedShieldBlock"),
            Self::ImprovedSunderArmor => f.write_str("ImprovedSunderArmor"),
            Self::ImprovedRevenge => f.write_str("ImprovedRevenge"),
            Self::ShieldSlam => f.write_str("ShieldSlam"),
            Self::ImprovedShieldBash => f.write_str("ImprovedShieldBash"),
            Self::ImprovedShieldWall => f.write_str("ImprovedShieldWall"),
            Self::ImprovedDisarm => f.write_str("ImprovedDisarm"),
            Self::ConcussionBlow => f.write_str("ConcussionBlow"),
            Self::LastStand => f.write_str("LastStand"),
            Self::ImprovedBattleShout => f.write_str("ImprovedBattleShout"),
            Self::Enrage => f.write_str("Enrage"),
            Self::WarriorFlurry => f.write_str("WarriorFlurry"),
            Self::Cruelty => f.write_str("Cruelty"),
            Self::BoomingVoice => f.write_str("BoomingVoice"),
            Self::UnbridledWrath => f.write_str("UnbridledWrath"),
            Self::PiercingHowl => f.write_str("PiercingHowl"),
            Self::ImprovedDemoralizingShout => f.write_str("ImprovedDemoralizingShout"),
            Self::DeathWish => f.write_str("DeathWish"),
            Self::ImprovedCleave => f.write_str("ImprovedCleave"),
            Self::Bloodthirst => f.write_str("Bloodthirst"),
            Self::ImprovedSlam => f.write_str("ImprovedSlam"),
            Self::RoguePrecision => f.write_str("RoguePrecision"),
            Self::DaggerSpecialization => f.write_str("DaggerSpecialization"),
            Self::FistWeaponSpecialization => f.write_str("FistWeaponSpecialization"),
            Self::RogueMaceSpecialization => f.write_str("RogueMaceSpecialization"),
            Self::RogueLightningReflexes => f.write_str("RogueLightningReflexes"),
            Self::RogueDeflection => f.write_str("RogueDeflection"),
            Self::ImprovedSinisterStrike => f.write_str("ImprovedSinisterStrike"),
            Self::ImprovedBackstab => f.write_str("ImprovedBackstab"),
            Self::ImprovedGouge => f.write_str("ImprovedGouge"),
            Self::Endurance => f.write_str("Endurance"),
            Self::AdrenalineRush => f.write_str("AdrenalineRush"),
            Self::ImprovedKick => f.write_str("ImprovedKick"),
            Self::RogueDualWieldSpecialization => f.write_str("RogueDualWieldSpecialization"),
            Self::ImprovedSprint => f.write_str("ImprovedSprint"),
            Self::BladeFlurry => f.write_str("BladeFlurry"),
            Self::MasterOfDeception => f.write_str("MasterOfDeception"),
            Self::RogueSwordSpecialization => f.write_str("RogueSwordSpecialization"),
            Self::Camouflage => f.write_str("Camouflage"),
            Self::Initiative => f.write_str("Initiative"),
            Self::Setup => f.write_str("Setup"),
            Self::Elusiveness => f.write_str("Elusiveness"),
            Self::Opportunity => f.write_str("Opportunity"),
            Self::ImprovedSap => f.write_str("ImprovedSap"),
            Self::ImprovedAmbush => f.write_str("ImprovedAmbush"),
            Self::DirtyDeeds => f.write_str("DirtyDeeds"),
            Self::ImprovedPoisons => f.write_str("ImprovedPoisons"),
            Self::Lethality => f.write_str("Lethality"),
            Self::Malice => f.write_str("Malice"),
            Self::RemorselessAttacks => f.write_str("RemorselessAttacks"),
            Self::Ruthlessness => f.write_str("Ruthlessness"),
            Self::Murder => f.write_str("Murder"),
            Self::ImprovedEviscerate => f.write_str("ImprovedEviscerate"),
            Self::ImprovedSliceAndDice => f.write_str("ImprovedSliceAndDice"),
            Self::ImprovedExposeArmor => f.write_str("ImprovedExposeArmor"),
            Self::ImprovedKidneyShot => f.write_str("ImprovedKidneyShot"),
            Self::ColdBlood => f.write_str("ColdBlood"),
            Self::RelentlessStrikes => f.write_str("RelentlessStrikes"),
            Self::SealFate => f.write_str("SealFate"),
            Self::Preparation => f.write_str("Preparation"),
            Self::Riposte => f.write_str("Riposte"),
            Self::GhostlyStrike => f.write_str("GhostlyStrike"),
            Self::Martyrdom => f.write_str("Martyrdom"),
            Self::PowerInfusion => f.write_str("PowerInfusion"),
            Self::MentalAgility => f.write_str("MentalAgility"),
            Self::UnbreakableWill => f.write_str("UnbreakableWill"),
            Self::ImprovedPowerWordShield => f.write_str("ImprovedPowerWordShield"),
            Self::ImprovedPowerWordFortitude => f.write_str("ImprovedPowerWordFortitude"),
            Self::PriestWandSpecialization => f.write_str("PriestWandSpecialization"),
            Self::ImprovedInnerFire => f.write_str("ImprovedInnerFire"),
            Self::Meditation => f.write_str("Meditation"),
            Self::InnerFocus => f.write_str("InnerFocus"),
            Self::ImprovedManaBurn => f.write_str("ImprovedManaBurn"),
            Self::DivineSpirit => f.write_str("DivineSpirit"),
            Self::SilentResolve => f.write_str("SilentResolve"),
            Self::Inspiration => f.write_str("Inspiration"),
            Self::Premeditation => f.write_str("Premeditation"),
            Self::Vigor => f.write_str("Vigor"),
            Self::HolySpecialization => f.write_str("HolySpecialization"),
            Self::SpiritualGuidance => f.write_str("SpiritualGuidance"),
            Self::SearingLight => f.write_str("SearingLight"),
            Self::SpiritualHealing => f.write_str("SpiritualHealing"),
            Self::ImprovedRenew => f.write_str("ImprovedRenew"),
            Self::ImprovedHealing => f.write_str("ImprovedHealing"),
            Self::PriestHealingFocus => f.write_str("PriestHealingFocus"),
            Self::SpellWarding => f.write_str("SpellWarding"),
            Self::ImprovedPrayerOfHealing => f.write_str("ImprovedPrayerOfHealing"),
            Self::ArcaneInstability => f.write_str("ArcaneInstability"),
            Self::HolyNova => f.write_str("HolyNova"),
            Self::ShadowWeaving => f.write_str("ShadowWeaving"),
            Self::Darkness => f.write_str("Darkness"),
            Self::ShadowFocus => f.write_str("ShadowFocus"),
            Self::Blackout => f.write_str("Blackout"),
            Self::SpiritTap => f.write_str("SpiritTap"),
            Self::ShadowAffinity => f.write_str("ShadowAffinity"),
            Self::ImprovedMindBlast => f.write_str("ImprovedMindBlast"),
            Self::ImprovedShadowWordPain => f.write_str("ImprovedShadowWordPain"),
            Self::ImprovedFade => f.write_str("ImprovedFade"),
            Self::VampiricEmbrace => f.write_str("VampiricEmbrace"),
            Self::MindFlay => f.write_str("MindFlay"),
            Self::Shadowform => f.write_str("Shadowform"),
            Self::Silence => f.write_str("Silence"),
            Self::ImprovedPsychicScream => f.write_str("ImprovedPsychicScream"),
            Self::CallOfFlame => f.write_str("CallOfFlame"),
            Self::CallOfThunder => f.write_str("CallOfThunder"),
            Self::Concussion => f.write_str("Concussion"),
            Self::Convection => f.write_str("Convection"),
            Self::ElementalFury => f.write_str("ElementalFury"),
            Self::ImprovedFireTotems => f.write_str("ImprovedFireTotems"),
            Self::EarthsGrasp => f.write_str("Earth's Grasp"),
            Self::ElementalMastery => f.write_str("ElementalMastery"),
            Self::ElementalFocus => f.write_str("ElementalFocus"),
            Self::Reverberation => f.write_str("Reverberation"),
            Self::AncestralHealing => f.write_str("AncestralHealing"),
            Self::TotemicMastery => f.write_str("TotemicMastery"),
            Self::NaturesGuidance => f.write_str("Nature's Guidance"),
            Self::ImprovedHealingWave => f.write_str("ImprovedHealingWave"),
            Self::ShamanHealingFocus => f.write_str("ShamanHealingFocus"),
            Self::RestorativeTotems => f.write_str("RestorativeTotems"),
            Self::ImprovedReincarnation => f.write_str("ImprovedReincarnation"),
            Self::ManaTideTotem => f.write_str("ManaTideTotem"),
            Self::ShamanNaturesSwiftness => f.write_str("Nature's Swiftness"),
            Self::Purification => f.write_str("Purification"),
            Self::TidalFocus => f.write_str("TidalFocus"),
            Self::TidalMastery => f.write_str("TidalMastery"),
            Self::TotemicFocus => f.write_str("TotemicFocus"),
            Self::ShamanAnticipation => f.write_str("ShamanAnticipation"),
            Self::ShamanFlurry => f.write_str("ShamanFlurry"),
            Self::ImprovedGhostWolf => f.write_str("ImprovedGhostWolf"),
            Self::ImprovedLightningShield => f.write_str("ImprovedLightningShield"),
            Self::GuardianTotems => f.write_str("GuardianTotems"),
            Self::EnhancingTotems => f.write_str("EnhancingTotems"),
            Self::ElementalWeapons => f.write_str("ElementalWeapons"),
            Self::ShamanShieldSpecialization => f.write_str("ShamanShieldSpecialization"),
            Self::ThunderingStrikes => f.write_str("ThunderingStrikes"),
            Self::AncestralKnowledge => f.write_str("AncestralKnowledge"),
            Self::ShamanToughness => f.write_str("ShamanToughness"),
            Self::Parry => f.write_str("Parry"),
            Self::TwoHandedAxesAndMaces => f.write_str("TwoHandedAxesAndMaces"),
            Self::TacticalMastery => f.write_str("TacticalMastery"),
            Self::BloodCraze => f.write_str("BloodCraze"),
            Self::Impale => f.write_str("Impale"),
            Self::Hemorrhage => f.write_str("Hemorrhage"),
            Self::VilePoisons => f.write_str("VilePoisons"),
            Self::WarriorOneHandedWeaponSpecialization => f.write_str("WarriorOneHandedWeaponSpecialization"),
            Self::LightningMastery => f.write_str("LightningMastery"),
            Self::ArcticReach => f.write_str("ArcticReach"),
            Self::NaturesGrasp => f.write_str("Nature's Grasp"),
            Self::ImprovedWrath => f.write_str("ImprovedWrath"),
            Self::ImprovedMoonfire => f.write_str("ImprovedMoonfire"),
            Self::NaturesReach => f.write_str("Nature's Reach"),
            Self::NaturalShapeshifter => f.write_str("NaturalShapeshifter"),
            Self::ImprovedThorns => f.write_str("ImprovedThorns"),
            Self::Moonglow => f.write_str("Moonglow"),
            Self::ImprovedStarfire => f.write_str("ImprovedStarfire"),
            Self::ImprovedEntanglingRoots => f.write_str("ImprovedEntanglingRoots"),
            Self::OmenOfClarity => f.write_str("OmenOfClarity"),
            Self::NaturesGrace => f.write_str("Nature's Grace"),
            Self::Moonfury => f.write_str("Moonfury"),
            Self::NaturalWeapons => f.write_str("NaturalWeapons"),
            Self::DruidVengeance => f.write_str("DruidVengeance"),
            Self::MoonkinForm => f.write_str("MoonkinForm"),
            Self::DruidThickHide => f.write_str("DruidThickHide"),
            Self::FeralAggression => f.write_str("FeralAggression"),
            Self::DruidFerocity => f.write_str("DruidFerocity"),
            Self::BrutalImpact => f.write_str("BrutalImpact"),
            Self::SharpenedClaws => f.write_str("SharpenedClaws"),
            Self::FeralInstinct => f.write_str("FeralInstinct"),
            Self::BloodFrenzy => f.write_str("BloodFrenzy"),
            Self::PrimalFury => f.write_str("PrimalFury"),
            Self::ImprovedShred => f.write_str("ImprovedShred"),
            Self::PredatoryStrikes => f.write_str("PredatoryStrikes"),
            Self::FeralCharge => f.write_str("FeralCharge"),
            Self::SavageFury => f.write_str("SavageFury"),
            Self::FelineSwiftness => f.write_str("FelineSwiftness"),
            Self::HeartOfTheWild => f.write_str("HeartOfTheWild"),
            Self::LeaderOfThePack => f.write_str("LeaderOfThePack"),
            Self::ImprovedMarkOfTheWild => f.write_str("ImprovedMarkOfTheWild"),
            Self::Furor => f.write_str("Furor"),
            Self::NaturesFocus => f.write_str("Nature's Focus"),
            Self::ImprovedHealingTouch => f.write_str("ImprovedHealingTouch"),
            Self::ImprovedRegrowth => f.write_str("ImprovedRegrowth"),
            Self::ImprovedEnrage => f.write_str("ImprovedEnrage"),
            Self::InsectSwarm => f.write_str("InsectSwarm"),
            Self::GiftOfNature => f.write_str("GiftOfNature"),
            Self::Reflection => f.write_str("Reflection"),
            Self::ImprovedRejuvenation => f.write_str("ImprovedRejuvenation"),
            Self::DruidNaturesSwiftness => f.write_str("Nature's Swiftness"),
            Self::Subtlety => f.write_str("Subtlety"),
            Self::ImprovedTranquility => f.write_str("ImprovedTranquility"),
            Self::TranquilSpirit => f.write_str("TranquilSpirit"),
            Self::Swiftmend => f.write_str("Swiftmend"),
            Self::ShadowReach => f.write_str("ShadowReach"),
            Self::Stormstrike => f.write_str("Stormstrike"),
            Self::ImprovedNaturesGrasp => f.write_str("Improved Nature's Grasp"),
            Self::Cataclysm => f.write_str("Cataclysm"),
            Self::Bane => f.write_str("Bane"),
            Self::ImprovedShadowBolt => f.write_str("ImprovedShadowBolt"),
            Self::ImprovedImmolate => f.write_str("ImprovedImmolate"),
            Self::Shadowburn => f.write_str("Shadowburn"),
            Self::DestructiveReach => f.write_str("DestructiveReach"),
            Self::ImprovedSearingPain => f.write_str("ImprovedSearingPain"),
            Self::Emberstorm => f.write_str("Emberstorm"),
            Self::Ruin => f.write_str("Ruin"),
            Self::Conflagrate => f.write_str("Conflagrate"),
            Self::Devastation => f.write_str("Devastation"),
            Self::Aftermath => f.write_str("Aftermath"),
            Self::ImprovedFirebolt => f.write_str("ImprovedFirebolt"),
            Self::ImprovedLashOfPain => f.write_str("ImprovedLashOfPain"),
            Self::Intensity => f.write_str("Intensity"),
            Self::Pyroclasm => f.write_str("Pyroclasm"),
            Self::FelConcentration => f.write_str("FelConcentration"),
            Self::Nightfall => f.write_str("Nightfall"),
            Self::ImprovedCorruption => f.write_str("ImprovedCorruption"),
            Self::ImprovedDrainLife => f.write_str("ImprovedDrainLife"),
            Self::Suppression => f.write_str("Suppression"),
            Self::ImprovedCurseOfWeakness => f.write_str("ImprovedCurseOfWeakness"),
            Self::ImprovedLifeTap => f.write_str("ImprovedLifeTap"),
            Self::GrimReach => f.write_str("GrimReach"),
            Self::DarkPact => f.write_str("DarkPact"),
            Self::SiphonLife => f.write_str("SiphonLife"),
            Self::ShadowMastery => f.write_str("ShadowMastery"),
            Self::AmplifyCurse => f.write_str("AmplifyCurse"),
            Self::CurseOfExhaustion => f.write_str("CurseOfExhaustion"),
            Self::ImprovedCurseOfExhaustion => f.write_str("ImprovedCurseOfExhaustion"),
            Self::ImprovedDrainSoul => f.write_str("ImprovedDrainSoul"),
            Self::ImprovedDrainMana => f.write_str("ImprovedDrainMana"),
            Self::Aggression => f.write_str("Aggression"),
            Self::SerratedBlades => f.write_str("SerratedBlades"),
            Self::Incinerate => f.write_str("Incinerate"),
            Self::ArcaneMeditation => f.write_str("ArcaneMeditation"),
            Self::FaerieFireFeral => f.write_str("FaerieFireFeral"),
            Self::DivineFury => f.write_str("DivineFury"),
            Self::MentalStrength => f.write_str("MentalStrength"),
            Self::ForceOfWill => f.write_str("ForceOfWill"),
            Self::ImprovedHealthstone => f.write_str("ImprovedHealthstone"),
            Self::ImprovedImp => f.write_str("ImprovedImp"),
            Self::DemonicEmbrace => f.write_str("DemonicEmbrace"),
            Self::ImprovedHealthFunnel => f.write_str("ImprovedHealthFunnel"),
            Self::ImprovedVoidwalker => f.write_str("ImprovedVoidwalker"),
            Self::FelDomination => f.write_str("FelDomination"),
            Self::MasterSummoner => f.write_str("MasterSummoner"),
            Self::FelStamina => f.write_str("FelStamina"),
            Self::FelIntellect => f.write_str("FelIntellect"),
            Self::ImprovedSuccubus => f.write_str("ImprovedSuccubus"),
            Self::MasterDemonologist => f.write_str("MasterDemonologist"),
            Self::ImprovedFirestone => f.write_str("ImprovedFirestone"),
            Self::UnholyPower => f.write_str("UnholyPower"),
            Self::ImprovedSpellstone => f.write_str("ImprovedSpellstone"),
            Self::DemonicSacrifice => f.write_str("DemonicSacrifice"),
            Self::SoulLink => f.write_str("SoulLink"),
            Self::ImprovedEnslaveDemon => f.write_str("ImprovedEnslaveDemon"),
            Self::ImprovedCurseOfAgony => f.write_str("ImprovedCurseOfAgony"),
            Self::HumanoidSlaying => f.write_str("HumanoidSlaying"),
            Self::HunterLightningReflexes => f.write_str("HunterLightningReflexes"),
            Self::Entrapment => f.write_str("Entrapment"),
            Self::ImprovedWingClip => f.write_str("ImprovedWingClip"),
            Self::CleverTraps => f.write_str("CleverTraps"),
            Self::Deterrence => f.write_str("Deterrence"),
            Self::ImprovedFeignDeath => f.write_str("ImprovedFeignDeath"),
            Self::Surefooted => f.write_str("Surefooted"),
            Self::HunterDeflection => f.write_str("HunterDeflection"),
            Self::Counterattack => f.write_str("Counterattack"),
            Self::KillerInstinct => f.write_str("KillerInstinct"),
            Self::TrapMastery => f.write_str("TrapMastery"),
            Self::WyvernSting => f.write_str("WyvernSting"),
            Self::ImprovedConcussiveShot => f.write_str("ImprovedConcussiveShot"),
            Self::Efficiency => f.write_str("Efficiency"),
            Self::ImprovedHuntersMark => f.write_str("Improved Hunter's Mark"),
            Self::LethalShots => f.write_str("LethalShots"),
            Self::AimedShot => f.write_str("AimedShot"),
            Self::ImprovedArcaneShot => f.write_str("ImprovedArcaneShot"),
            Self::Barrage => f.write_str("Barrage"),
            Self::ImprovedSerpentSting => f.write_str("ImprovedSerpentSting"),
            Self::MortalShots => f.write_str("MortalShots"),
            Self::ImprovedScorpidSting => f.write_str("ImprovedScorpidSting"),
            Self::HawkEye => f.write_str("HawkEye"),
            Self::ScatterShot => f.write_str("ScatterShot"),
            Self::TrueshotAura => f.write_str("TrueshotAura"),
            Self::RangedWeaponSpecialization => f.write_str("RangedWeaponSpecialization"),
            Self::ImprovedAspectOfTheMonkey => f.write_str("ImprovedAspectOfTheMonkey"),
            Self::ImprovedAspectOfTheHawk => f.write_str("ImprovedAspectOfTheHawk"),
            Self::Pathfinding => f.write_str("Pathfinding"),
            Self::ImprovedMendPet => f.write_str("ImprovedMendPet"),
            Self::BestialWrath => f.write_str("BestialWrath"),
            Self::Intimidation => f.write_str("Intimidation"),
            Self::SpiritBond => f.write_str("SpiritBond"),
            Self::EnduranceTraining => f.write_str("EnduranceTraining"),
            Self::BestialDiscipline => f.write_str("BestialDiscipline"),
            Self::BestialSwiftness => f.write_str("BestialSwiftness"),
            Self::HunterFerocity => f.write_str("HunterFerocity"),
            Self::HunterThickHide => f.write_str("HunterThickHide"),
            Self::UnleashedFury => f.write_str("UnleashedFury"),
            Self::Frenzy => f.write_str("Frenzy"),
            Self::ImprovedBlessingOfMight => f.write_str("ImprovedBlessingOfMight"),
            Self::PaladinVengeance => f.write_str("PaladinVengeance"),
            Self::PaladinDeflection => f.write_str("PaladinDeflection"),
            Self::ImprovedRetributionAura => f.write_str("ImprovedRetributionAura"),
            Self::Benediction => f.write_str("Benediction"),
            Self::SanctityAura => f.write_str("SanctityAura"),
            Self::PaladinTwoHandedWeaponSpecialization => f.write_str("PaladinTwoHandedWeaponSpecialization"),
            Self::Conviction => f.write_str("Conviction"),
            Self::Redoubt => f.write_str("Redoubt"),
            Self::ImprovedDevotionAura => f.write_str("ImprovedDevotionAura"),
            Self::PaladinToughness => f.write_str("PaladinToughness"),
            Self::PaladinShieldSpecialization => f.write_str("PaladinShieldSpecialization"),
            Self::GuardiansFavor => f.write_str("Guardian's Favor"),
            Self::Reckoning => f.write_str("Reckoning"),
            Self::PaladinOneHandedWeaponSpecialization => f.write_str("PaladinOneHandedWeaponSpecialization"),
            Self::HolyShield => f.write_str("HolyShield"),
            Self::BlessingOfSanctuary => f.write_str("BlessingOfSanctuary"),
            Self::SpiritualFocus => f.write_str("SpiritualFocus"),
            Self::DivineFavor => f.write_str("DivineFavor"),
            Self::Consecration => f.write_str("Consecration"),
            Self::Repentance => f.write_str("Repentance"),
            Self::BlessingOfKings => f.write_str("BlessingOfKings"),
            Self::ImprovedLayOnHands => f.write_str("ImprovedLayOnHands"),
            Self::HealingLight => f.write_str("HealingLight"),
            Self::ImprovedBlessingOfWisdom => f.write_str("ImprovedBlessingOfWisdom"),
            Self::DivineIntellect => f.write_str("DivineIntellect"),
            Self::DivineStrength => f.write_str("DivineStrength"),
            Self::Illumination => f.write_str("Illumination"),
            Self::ImprovedSealOfRighteousness => f.write_str("ImprovedSealOfRighteousness"),
            Self::ImprovedSealOfTheCrusader => f.write_str("ImprovedSealOfTheCrusader"),
            Self::LastingJudgement => f.write_str("LastingJudgement"),
            Self::SealOfCommand => f.write_str("SealOfCommand"),
            Self::ImprovedRighteousFury => f.write_str("ImprovedRighteousFury"),
            Self::HolyShock => f.write_str("HolyShock"),
            Self::ImprovedHammerOfJustice => f.write_str("ImprovedHammerOfJustice"),
            Self::ImprovedBerserkerRage => f.write_str("ImprovedBerserkerRage"),
            Self::ImprovedExecute => f.write_str("ImprovedExecute"),
            Self::ImprovedIntercept => f.write_str("ImprovedIntercept"),
            Self::SpiritOfRedemption => f.write_str("SpiritOfRedemption"),
            Self::WarriorDualWieldSpecialization => f.write_str("WarriorDualWieldSpecialization"),
            Self::WarriorShieldSpecialization => f.write_str("WarriorShieldSpecialization"),
            Self::SavageStrikes => f.write_str("SavageStrikes"),
            Self::Survivalist => f.write_str("Survivalist"),
            Self::MonsterSlaying => f.write_str("MonsterSlaying"),
            Self::ImprovedEyesOfTheBeast => f.write_str("ImprovedEyesOfTheBeast"),
            Self::ImprovedRevivePet => f.write_str("ImprovedRevivePet"),
            Self::ImprovedConcentrationAura => f.write_str("ImprovedConcentrationAura"),
            Self::HolyPower => f.write_str("HolyPower"),
            Self::UnyieldingFaith => f.write_str("UnyieldingFaith"),
            Self::PaladinAnticipation => f.write_str("PaladinAnticipation"),
            Self::PaladinPrecision => f.write_str("PaladinPrecision"),
            Self::ImprovedJudgement => f.write_str("ImprovedJudgement"),
            Self::EyeForAnEye => f.write_str("EyeForAnEye"),
            Self::Vindication => f.write_str("Vindication"),
            Self::PursuitOfJustice => f.write_str("PursuitOfJustice"),
            Self::HolyReach => f.write_str("HolyReach"),
            Self::BlessedRecovery => f.write_str("BlessedRecovery"),
            Self::Lightwell => f.write_str("Lightwell"),
            Self::ImprovedVampiricEmbrace => f.write_str("ImprovedVampiricEmbrace"),
            Self::MasterOfElements => f.write_str("MasterOfElements"),
            Self::ElementalWarding => f.write_str("ElementalWarding"),
            Self::StormReach => f.write_str("StormReach"),
            Self::EyeOfTheStorm => f.write_str("EyeOfTheStorm"),
            Self::WeaponMastery => f.write_str("WeaponMastery"),
            Self::ElementalDevastation => f.write_str("ElementalDevastation"),
            Self::HealingGrace => f.write_str("HealingGrace"),
            Self::ImprovedWeaponTotems => f.write_str("ImprovedWeaponTotems"),
            Self::HealingWay => f.write_str("HealingWay"),
            Self::ElementalPrecision => f.write_str("ElementalPrecision"),
            Self::MagicAbsorption => f.write_str("MagicAbsorption"),
            Self::SleightOfHand => f.write_str("SleightOfHand"),
            Self::HeightenedSenses => f.write_str("HeightenedSenses"),
            Self::Deadliness => f.write_str("Deadliness"),
            Self::WeaponExpertise => f.write_str("WeaponExpertise"),
        }
    }
}

impl TryFrom<u32> for Talent {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            23 => Ok(Self::BurningSoul),
            24 => Ok(Self::ImprovedFireWard),
            25 => Ok(Self::ImprovedScorch),
            26 => Ok(Self::ImprovedFireball),
            27 => Ok(Self::ImprovedFireBlast),
            28 => Ok(Self::FlameThrowing),
            29 => Ok(Self::Pyroblast),
            30 => Ok(Self::Impact),
            31 => Ok(Self::ImprovedFlamestrike),
            32 => Ok(Self::BlastWave),
            33 => Ok(Self::CriticalMass),
            34 => Ok(Self::Ignite),
            35 => Ok(Self::FirePower),
            36 => Ok(Self::Combustion),
            37 => Ok(Self::ImprovedFrostbolt),
            38 => Ok(Self::Frostbite),
            61 => Ok(Self::PiercingIce),
            62 => Ok(Self::ImprovedFrostNova),
            63 => Ok(Self::ImprovedBlizzard),
            64 => Ok(Self::ImprovedConeOfCold),
            65 => Ok(Self::Permafrost),
            66 => Ok(Self::FrostChanneling),
            67 => Ok(Self::Shatter),
            68 => Ok(Self::WintersChill),
            69 => Ok(Self::ColdSnap),
            70 => Ok(Self::FrostWarding),
            71 => Ok(Self::IceBarrier),
            72 => Ok(Self::IceBlock),
            73 => Ok(Self::IceShards),
            74 => Ok(Self::ArcaneSubtlety),
            75 => Ok(Self::ArcaneConcentration),
            76 => Ok(Self::ArcaneFocus),
            77 => Ok(Self::ArcaneMind),
            78 => Ok(Self::MageWandSpecialization),
            80 => Ok(Self::ImprovedArcaneMissiles),
            81 => Ok(Self::ImprovedArcaneExplosion),
            82 => Ok(Self::MagicAttunement),
            83 => Ok(Self::ImprovedManaShield),
            85 => Ok(Self::ArcaneResilience),
            86 => Ok(Self::PresenceOfMind),
            87 => Ok(Self::ArcanePower),
            88 => Ok(Self::ImprovedCounterspell),
            121 => Ok(Self::DeepWounds),
            123 => Ok(Self::WarriorSwordSpecialization),
            124 => Ok(Self::ImprovedHeroicStrike),
            125 => Ok(Self::WarriorMaceSpecialization),
            126 => Ok(Self::ImprovedCharge),
            127 => Ok(Self::ImprovedRend),
            128 => Ok(Self::ImprovedThunderClap),
            129 => Ok(Self::ImprovedHamstring),
            130 => Ok(Self::WarriorDeflection),
            131 => Ok(Self::ImprovedOverpower),
            132 => Ok(Self::AxeSpecialization),
            133 => Ok(Self::SweepingStrikes),
            134 => Ok(Self::PolearmSpecialization),
            135 => Ok(Self::MortalStrike),
            136 => Ok(Self::WarriorTwoHandedWeaponSpecialization),
            137 => Ok(Self::AngerManagement),
            138 => Ok(Self::WarriorAnticipation),
            140 => Ok(Self::WarriorToughness),
            141 => Ok(Self::IronWill),
            142 => Ok(Self::ImprovedBloodrage),
            143 => Ok(Self::ImprovedTaunt),
            144 => Ok(Self::Defiance),
            145 => Ok(Self::ImprovedShieldBlock),
            146 => Ok(Self::ImprovedSunderArmor),
            147 => Ok(Self::ImprovedRevenge),
            148 => Ok(Self::ShieldSlam),
            149 => Ok(Self::ImprovedShieldBash),
            150 => Ok(Self::ImprovedShieldWall),
            151 => Ok(Self::ImprovedDisarm),
            152 => Ok(Self::ConcussionBlow),
            153 => Ok(Self::LastStand),
            154 => Ok(Self::ImprovedBattleShout),
            155 => Ok(Self::Enrage),
            156 => Ok(Self::WarriorFlurry),
            157 => Ok(Self::Cruelty),
            158 => Ok(Self::BoomingVoice),
            159 => Ok(Self::UnbridledWrath),
            160 => Ok(Self::PiercingHowl),
            161 => Ok(Self::ImprovedDemoralizingShout),
            165 => Ok(Self::DeathWish),
            166 => Ok(Self::ImprovedCleave),
            167 => Ok(Self::Bloodthirst),
            168 => Ok(Self::ImprovedSlam),
            181 => Ok(Self::RoguePrecision),
            182 => Ok(Self::DaggerSpecialization),
            183 => Ok(Self::FistWeaponSpecialization),
            184 => Ok(Self::RogueMaceSpecialization),
            186 => Ok(Self::RogueLightningReflexes),
            187 => Ok(Self::RogueDeflection),
            201 => Ok(Self::ImprovedSinisterStrike),
            202 => Ok(Self::ImprovedBackstab),
            203 => Ok(Self::ImprovedGouge),
            204 => Ok(Self::Endurance),
            205 => Ok(Self::AdrenalineRush),
            206 => Ok(Self::ImprovedKick),
            221 => Ok(Self::RogueDualWieldSpecialization),
            222 => Ok(Self::ImprovedSprint),
            223 => Ok(Self::BladeFlurry),
            241 => Ok(Self::MasterOfDeception),
            242 => Ok(Self::RogueSwordSpecialization),
            244 => Ok(Self::Camouflage),
            245 => Ok(Self::Initiative),
            246 => Ok(Self::Setup),
            247 => Ok(Self::Elusiveness),
            261 => Ok(Self::Opportunity),
            262 => Ok(Self::ImprovedSap),
            263 => Ok(Self::ImprovedAmbush),
            265 => Ok(Self::DirtyDeeds),
            268 => Ok(Self::ImprovedPoisons),
            269 => Ok(Self::Lethality),
            270 => Ok(Self::Malice),
            272 => Ok(Self::RemorselessAttacks),
            273 => Ok(Self::Ruthlessness),
            274 => Ok(Self::Murder),
            276 => Ok(Self::ImprovedEviscerate),
            277 => Ok(Self::ImprovedSliceAndDice),
            278 => Ok(Self::ImprovedExposeArmor),
            279 => Ok(Self::ImprovedKidneyShot),
            280 => Ok(Self::ColdBlood),
            281 => Ok(Self::RelentlessStrikes),
            283 => Ok(Self::SealFate),
            284 => Ok(Self::Preparation),
            301 => Ok(Self::Riposte),
            303 => Ok(Self::GhostlyStrike),
            321 => Ok(Self::Martyrdom),
            322 => Ok(Self::PowerInfusion),
            341 => Ok(Self::MentalAgility),
            342 => Ok(Self::UnbreakableWill),
            343 => Ok(Self::ImprovedPowerWordShield),
            344 => Ok(Self::ImprovedPowerWordFortitude),
            345 => Ok(Self::PriestWandSpecialization),
            346 => Ok(Self::ImprovedInnerFire),
            347 => Ok(Self::Meditation),
            348 => Ok(Self::InnerFocus),
            350 => Ok(Self::ImprovedManaBurn),
            351 => Ok(Self::DivineSpirit),
            352 => Ok(Self::SilentResolve),
            361 => Ok(Self::Inspiration),
            381 => Ok(Self::Premeditation),
            382 => Ok(Self::Vigor),
            401 => Ok(Self::HolySpecialization),
            402 => Ok(Self::SpiritualGuidance),
            403 => Ok(Self::SearingLight),
            404 => Ok(Self::SpiritualHealing),
            406 => Ok(Self::ImprovedRenew),
            408 => Ok(Self::ImprovedHealing),
            410 => Ok(Self::PriestHealingFocus),
            411 => Ok(Self::SpellWarding),
            413 => Ok(Self::ImprovedPrayerOfHealing),
            421 => Ok(Self::ArcaneInstability),
            442 => Ok(Self::HolyNova),
            461 => Ok(Self::ShadowWeaving),
            462 => Ok(Self::Darkness),
            463 => Ok(Self::ShadowFocus),
            464 => Ok(Self::Blackout),
            465 => Ok(Self::SpiritTap),
            466 => Ok(Self::ShadowAffinity),
            481 => Ok(Self::ImprovedMindBlast),
            482 => Ok(Self::ImprovedShadowWordPain),
            483 => Ok(Self::ImprovedFade),
            484 => Ok(Self::VampiricEmbrace),
            501 => Ok(Self::MindFlay),
            521 => Ok(Self::Shadowform),
            541 => Ok(Self::Silence),
            542 => Ok(Self::ImprovedPsychicScream),
            561 => Ok(Self::CallOfFlame),
            562 => Ok(Self::CallOfThunder),
            563 => Ok(Self::Concussion),
            564 => Ok(Self::Convection),
            565 => Ok(Self::ElementalFury),
            567 => Ok(Self::ImprovedFireTotems),
            572 => Ok(Self::EarthsGrasp),
            573 => Ok(Self::ElementalMastery),
            574 => Ok(Self::ElementalFocus),
            575 => Ok(Self::Reverberation),
            581 => Ok(Self::AncestralHealing),
            582 => Ok(Self::TotemicMastery),
            583 => Ok(Self::NaturesGuidance),
            586 => Ok(Self::ImprovedHealingWave),
            587 => Ok(Self::ShamanHealingFocus),
            588 => Ok(Self::RestorativeTotems),
            589 => Ok(Self::ImprovedReincarnation),
            590 => Ok(Self::ManaTideTotem),
            591 => Ok(Self::ShamanNaturesSwiftness),
            592 => Ok(Self::Purification),
            593 => Ok(Self::TidalFocus),
            594 => Ok(Self::TidalMastery),
            595 => Ok(Self::TotemicFocus),
            601 => Ok(Self::ShamanAnticipation),
            602 => Ok(Self::ShamanFlurry),
            605 => Ok(Self::ImprovedGhostWolf),
            607 => Ok(Self::ImprovedLightningShield),
            609 => Ok(Self::GuardianTotems),
            610 => Ok(Self::EnhancingTotems),
            611 => Ok(Self::ElementalWeapons),
            612 => Ok(Self::ShamanShieldSpecialization),
            613 => Ok(Self::ThunderingStrikes),
            614 => Ok(Self::AncestralKnowledge),
            615 => Ok(Self::ShamanToughness),
            616 => Ok(Self::Parry),
            617 => Ok(Self::TwoHandedAxesAndMaces),
            641 => Ok(Self::TacticalMastery),
            661 => Ok(Self::BloodCraze),
            662 => Ok(Self::Impale),
            681 => Ok(Self::Hemorrhage),
            682 => Ok(Self::VilePoisons),
            702 => Ok(Self::WarriorOneHandedWeaponSpecialization),
            721 => Ok(Self::LightningMastery),
            741 => Ok(Self::ArcticReach),
            761 => Ok(Self::NaturesGrasp),
            762 => Ok(Self::ImprovedWrath),
            763 => Ok(Self::ImprovedMoonfire),
            764 => Ok(Self::NaturesReach),
            781 => Ok(Self::NaturalShapeshifter),
            782 => Ok(Self::ImprovedThorns),
            783 => Ok(Self::Moonglow),
            784 => Ok(Self::ImprovedStarfire),
            787 => Ok(Self::ImprovedEntanglingRoots),
            788 => Ok(Self::OmenOfClarity),
            789 => Ok(Self::NaturesGrace),
            790 => Ok(Self::Moonfury),
            791 => Ok(Self::NaturalWeapons),
            792 => Ok(Self::DruidVengeance),
            793 => Ok(Self::MoonkinForm),
            794 => Ok(Self::DruidThickHide),
            795 => Ok(Self::FeralAggression),
            796 => Ok(Self::DruidFerocity),
            797 => Ok(Self::BrutalImpact),
            798 => Ok(Self::SharpenedClaws),
            799 => Ok(Self::FeralInstinct),
            800 => Ok(Self::BloodFrenzy),
            801 => Ok(Self::PrimalFury),
            802 => Ok(Self::ImprovedShred),
            803 => Ok(Self::PredatoryStrikes),
            804 => Ok(Self::FeralCharge),
            805 => Ok(Self::SavageFury),
            807 => Ok(Self::FelineSwiftness),
            808 => Ok(Self::HeartOfTheWild),
            809 => Ok(Self::LeaderOfThePack),
            821 => Ok(Self::ImprovedMarkOfTheWild),
            822 => Ok(Self::Furor),
            823 => Ok(Self::NaturesFocus),
            824 => Ok(Self::ImprovedHealingTouch),
            825 => Ok(Self::ImprovedRegrowth),
            826 => Ok(Self::ImprovedEnrage),
            827 => Ok(Self::InsectSwarm),
            828 => Ok(Self::GiftOfNature),
            829 => Ok(Self::Reflection),
            830 => Ok(Self::ImprovedRejuvenation),
            831 => Ok(Self::DruidNaturesSwiftness),
            841 => Ok(Self::Subtlety),
            842 => Ok(Self::ImprovedTranquility),
            843 => Ok(Self::TranquilSpirit),
            844 => Ok(Self::Swiftmend),
            881 => Ok(Self::ShadowReach),
            901 => Ok(Self::Stormstrike),
            921 => Ok(Self::ImprovedNaturesGrasp),
            941 => Ok(Self::Cataclysm),
            943 => Ok(Self::Bane),
            944 => Ok(Self::ImprovedShadowBolt),
            961 => Ok(Self::ImprovedImmolate),
            963 => Ok(Self::Shadowburn),
            964 => Ok(Self::DestructiveReach),
            965 => Ok(Self::ImprovedSearingPain),
            966 => Ok(Self::Emberstorm),
            967 => Ok(Self::Ruin),
            968 => Ok(Self::Conflagrate),
            981 => Ok(Self::Devastation),
            982 => Ok(Self::Aftermath),
            983 => Ok(Self::ImprovedFirebolt),
            984 => Ok(Self::ImprovedLashOfPain),
            985 => Ok(Self::Intensity),
            986 => Ok(Self::Pyroclasm),
            1001 => Ok(Self::FelConcentration),
            1002 => Ok(Self::Nightfall),
            1003 => Ok(Self::ImprovedCorruption),
            1004 => Ok(Self::ImprovedDrainLife),
            1005 => Ok(Self::Suppression),
            1006 => Ok(Self::ImprovedCurseOfWeakness),
            1007 => Ok(Self::ImprovedLifeTap),
            1021 => Ok(Self::GrimReach),
            1022 => Ok(Self::DarkPact),
            1041 => Ok(Self::SiphonLife),
            1042 => Ok(Self::ShadowMastery),
            1061 => Ok(Self::AmplifyCurse),
            1081 => Ok(Self::CurseOfExhaustion),
            1082 => Ok(Self::ImprovedCurseOfExhaustion),
            1101 => Ok(Self::ImprovedDrainSoul),
            1121 => Ok(Self::ImprovedDrainMana),
            1122 => Ok(Self::Aggression),
            1123 => Ok(Self::SerratedBlades),
            1141 => Ok(Self::Incinerate),
            1142 => Ok(Self::ArcaneMeditation),
            1162 => Ok(Self::FaerieFireFeral),
            1181 => Ok(Self::DivineFury),
            1201 => Ok(Self::MentalStrength),
            1202 => Ok(Self::ForceOfWill),
            1221 => Ok(Self::ImprovedHealthstone),
            1222 => Ok(Self::ImprovedImp),
            1223 => Ok(Self::DemonicEmbrace),
            1224 => Ok(Self::ImprovedHealthFunnel),
            1225 => Ok(Self::ImprovedVoidwalker),
            1226 => Ok(Self::FelDomination),
            1227 => Ok(Self::MasterSummoner),
            1241 => Ok(Self::FelStamina),
            1242 => Ok(Self::FelIntellect),
            1243 => Ok(Self::ImprovedSuccubus),
            1244 => Ok(Self::MasterDemonologist),
            1261 => Ok(Self::ImprovedFirestone),
            1262 => Ok(Self::UnholyPower),
            1263 => Ok(Self::ImprovedSpellstone),
            1281 => Ok(Self::DemonicSacrifice),
            1282 => Ok(Self::SoulLink),
            1283 => Ok(Self::ImprovedEnslaveDemon),
            1284 => Ok(Self::ImprovedCurseOfAgony),
            1301 => Ok(Self::HumanoidSlaying),
            1303 => Ok(Self::HunterLightningReflexes),
            1304 => Ok(Self::Entrapment),
            1305 => Ok(Self::ImprovedWingClip),
            1306 => Ok(Self::CleverTraps),
            1308 => Ok(Self::Deterrence),
            1309 => Ok(Self::ImprovedFeignDeath),
            1310 => Ok(Self::Surefooted),
            1311 => Ok(Self::HunterDeflection),
            1312 => Ok(Self::Counterattack),
            1321 => Ok(Self::KillerInstinct),
            1322 => Ok(Self::TrapMastery),
            1325 => Ok(Self::WyvernSting),
            1341 => Ok(Self::ImprovedConcussiveShot),
            1342 => Ok(Self::Efficiency),
            1343 => Ok(Self::ImprovedHuntersMark),
            1344 => Ok(Self::LethalShots),
            1345 => Ok(Self::AimedShot),
            1346 => Ok(Self::ImprovedArcaneShot),
            1347 => Ok(Self::Barrage),
            1348 => Ok(Self::ImprovedSerpentSting),
            1349 => Ok(Self::MortalShots),
            1351 => Ok(Self::ImprovedScorpidSting),
            1352 => Ok(Self::HawkEye),
            1353 => Ok(Self::ScatterShot),
            1361 => Ok(Self::TrueshotAura),
            1362 => Ok(Self::RangedWeaponSpecialization),
            1381 => Ok(Self::ImprovedAspectOfTheMonkey),
            1382 => Ok(Self::ImprovedAspectOfTheHawk),
            1384 => Ok(Self::Pathfinding),
            1385 => Ok(Self::ImprovedMendPet),
            1386 => Ok(Self::BestialWrath),
            1387 => Ok(Self::Intimidation),
            1388 => Ok(Self::SpiritBond),
            1389 => Ok(Self::EnduranceTraining),
            1390 => Ok(Self::BestialDiscipline),
            1391 => Ok(Self::BestialSwiftness),
            1393 => Ok(Self::HunterFerocity),
            1395 => Ok(Self::HunterThickHide),
            1396 => Ok(Self::UnleashedFury),
            1397 => Ok(Self::Frenzy),
            1401 => Ok(Self::ImprovedBlessingOfMight),
            1402 => Ok(Self::PaladinVengeance),
            1403 => Ok(Self::PaladinDeflection),
            1405 => Ok(Self::ImprovedRetributionAura),
            1407 => Ok(Self::Benediction),
            1409 => Ok(Self::SanctityAura),
            1410 => Ok(Self::PaladinTwoHandedWeaponSpecialization),
            1411 => Ok(Self::Conviction),
            1421 => Ok(Self::Redoubt),
            1422 => Ok(Self::ImprovedDevotionAura),
            1423 => Ok(Self::PaladinToughness),
            1424 => Ok(Self::PaladinShieldSpecialization),
            1425 => Ok(Self::GuardiansFavor),
            1426 => Ok(Self::Reckoning),
            1429 => Ok(Self::PaladinOneHandedWeaponSpecialization),
            1430 => Ok(Self::HolyShield),
            1431 => Ok(Self::BlessingOfSanctuary),
            1432 => Ok(Self::SpiritualFocus),
            1433 => Ok(Self::DivineFavor),
            1435 => Ok(Self::Consecration),
            1441 => Ok(Self::Repentance),
            1442 => Ok(Self::BlessingOfKings),
            1443 => Ok(Self::ImprovedLayOnHands),
            1444 => Ok(Self::HealingLight),
            1446 => Ok(Self::ImprovedBlessingOfWisdom),
            1449 => Ok(Self::DivineIntellect),
            1450 => Ok(Self::DivineStrength),
            1461 => Ok(Self::Illumination),
            1463 => Ok(Self::ImprovedSealOfRighteousness),
            1464 => Ok(Self::ImprovedSealOfTheCrusader),
            1465 => Ok(Self::LastingJudgement),
            1481 => Ok(Self::SealOfCommand),
            1501 => Ok(Self::ImprovedRighteousFury),
            1502 => Ok(Self::HolyShock),
            1521 => Ok(Self::ImprovedHammerOfJustice),
            1541 => Ok(Self::ImprovedBerserkerRage),
            1542 => Ok(Self::ImprovedExecute),
            1543 => Ok(Self::ImprovedIntercept),
            1561 => Ok(Self::SpiritOfRedemption),
            1581 => Ok(Self::WarriorDualWieldSpecialization),
            1601 => Ok(Self::WarriorShieldSpecialization),
            1621 => Ok(Self::SavageStrikes),
            1622 => Ok(Self::Survivalist),
            1623 => Ok(Self::MonsterSlaying),
            1624 => Ok(Self::ImprovedEyesOfTheBeast),
            1625 => Ok(Self::ImprovedRevivePet),
            1626 => Ok(Self::ImprovedConcentrationAura),
            1627 => Ok(Self::HolyPower),
            1628 => Ok(Self::UnyieldingFaith),
            1629 => Ok(Self::PaladinAnticipation),
            1630 => Ok(Self::PaladinPrecision),
            1631 => Ok(Self::ImprovedJudgement),
            1632 => Ok(Self::EyeForAnEye),
            1633 => Ok(Self::Vindication),
            1634 => Ok(Self::PursuitOfJustice),
            1635 => Ok(Self::HolyReach),
            1636 => Ok(Self::BlessedRecovery),
            1637 => Ok(Self::Lightwell),
            1638 => Ok(Self::ImprovedVampiricEmbrace),
            1639 => Ok(Self::MasterOfElements),
            1640 => Ok(Self::ElementalWarding),
            1641 => Ok(Self::StormReach),
            1642 => Ok(Self::EyeOfTheStorm),
            1643 => Ok(Self::WeaponMastery),
            1645 => Ok(Self::ElementalDevastation),
            1646 => Ok(Self::HealingGrace),
            1647 => Ok(Self::ImprovedWeaponTotems),
            1648 => Ok(Self::HealingWay),
            1649 => Ok(Self::ElementalPrecision),
            1650 => Ok(Self::MagicAbsorption),
            1700 => Ok(Self::SleightOfHand),
            1701 => Ok(Self::HeightenedSenses),
            1702 => Ok(Self::Deadliness),
            1703 => Ok(Self::WeaponExpertise),
            v => Err(crate::errors::EnumError::new("Talent", v as u64),)
        }
    }
}
