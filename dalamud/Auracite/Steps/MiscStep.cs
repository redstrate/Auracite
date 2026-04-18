using FFXIVClientStructs.FFXIV.Client.Game.UI;
using FFXIVClientStructs.FFXIV.Client.Game.Character;
using Lumina.Excel.Sheets;
using System.Collections.Generic;
using FFXIVClientStructs.FFXIV.Client.Game;

namespace Auracite;

public class MiscStep : IStep
{
    public event IStep.CompletedDelegate? Completed;

    public void Run()
    {
        unsafe
        {
            var playerState = PlayerState.Instance();
            var uiState = UIState.Instance();
            var questManager = QuestManager.Instance();

            Plugin.package!.name = playerState->CharacterNameString;
            Plugin.package!.world = IStep.SaveNameValue<World>(Plugin.ObjectTable.LocalPlayer!.HomeWorld.RowId, world => world.Name);
            Plugin.package!.data_center = IStep.SaveNameValue<WorldDCGroupType>(Plugin.ObjectTable.LocalPlayer!.HomeWorld.Value.DataCenter.RowId, data_center => data_center.Name);
            Plugin.package!.city_state = IStep.SaveNameValue<Town>(playerState->StartTown, town => town.Name);
            Plugin.package!.nameday = new DayMonthValue();
            Plugin.package!.nameday.day = playerState->BirthDay;
            Plugin.package!.nameday.month = playerState->BirthMonth;
            Plugin.package!.guardian = IStep.SaveNameValue<GuardianDeity>(playerState->GuardianDeity, guardian => guardian.Name);
            Plugin.package!.gender = new NameValue();
            Plugin.package!.gender.value = playerState->Sex;
            Plugin.package!.gender.name = playerState->Sex == 1 ? "Female" : "Male";
            Plugin.package!.tribe = IStep.SaveNameValue<Tribe>(playerState->Tribe, tribe => playerState->Sex == 1 ? tribe.Feminine : tribe.Masculine);
            Plugin.package!.race = IStep.SaveNameValue<Race>(playerState->Race, race => playerState->Sex == 1 ? race.Feminine : race.Masculine);
            var classJobSheet = Plugin.DataManager.GetExcelSheet<ClassJob>()!;
            for (int i = 0; i < playerState->ClassJobLevels.Length; i++) {
                var classLevel = new ClassJobLevel();
                foreach (var row in classJobSheet) {
                    if (row.ExpArrayIndex == i) {
                        classLevel.name = row.NameEnglish.ToString()!;
                        classLevel.value = row.RowId;
                        break;
                    }
                }
                classLevel.level = playerState->ClassJobLevels[i];
                classLevel.exp = playerState->ClassJobExperience[i];

                // Exclude currently unavailable jobs
                if (classLevel.name != null) {
                    Plugin.package!.classjob_levels.Add(classLevel);
                }
            }
            Plugin.package!.grand_company = IStep.SaveNameValue<GrandCompany>(playerState->GrandCompany, company => company.Name);
            Plugin.package!.grand_company_ranks.Add(playerState->GCRankMaelstrom);
            Plugin.package!.grand_company_ranks.Add(playerState->GCRankTwinAdders);
            Plugin.package!.grand_company_ranks.Add(playerState->GCRankImmortalFlames);

            Plugin.package.is_battle_mentor = PlayerState.Instance()->IsBattleMentor();
            Plugin.package.is_trade_mentor = PlayerState.Instance()->IsTradeMentor();
            Plugin.package.is_novice = PlayerState.Instance()->IsNovice();
            Plugin.package.is_returner = PlayerState.Instance()->IsReturner();
            Plugin.package.player_commendations = PlayerState.Instance()->PlayerCommendations;

            var localPlayer = Plugin.ObjectTable.LocalPlayer;
            if (localPlayer != null)
            {
                var gameObject = (Character*)localPlayer.Address;
                Plugin.package.voice = gameObject->Vfx.VoiceId;
            }

            // unlocks
            Plugin.package.unlocks = IStep.ConsumeBitArray(uiState->UnlockLinksBitArray);
            Plugin.package.seen_active_help = IStep.ConsumeBitArray(uiState->UnlockedHowTosBitArray);
            Plugin.package.minions = IStep.ConsumeBitArray(uiState->UnlockedCompanionsBitArray);
            Plugin.package.mounts = IStep.ConsumeBitArray(playerState->UnlockedMountsBitArray);
            Plugin.package.orchestrion_rolls = IStep.ConsumeBitArray(playerState->UnlockedOrchestrionRollsBitArray);
            Plugin.package.cutscene_seen = IStep.ConsumeBitArray(uiState->SeenCutscenesBitArray);
            Plugin.package.ornaments = IStep.ConsumeBitArray(playerState->UnlockedOrnamentsBitArray);
            Plugin.package.caught_fish = IStep.ConsumeBitArray(playerState->CaughtFishBitArray);
            Plugin.package.caught_spearfish = IStep.ConsumeBitArray(playerState->CaughtSpearfishBitArray);
            Plugin.package.adventures = IStep.ConsumeBitArray(playerState->CompletedAdventuresBitArray);
            Plugin.package.triple_triad_cards = IStep.ConsumeBitArray(uiState->UnlockedTripleTriadCardsBitArray);
            Plugin.package.glasses_styles = IStep.ConsumeBitArray(playerState->UnlockedGlassesStylesBitArray);
            Plugin.package.chocobo_taxi_stands = IStep.ConsumeBitArray(uiState->UnlockedChocoboTaxiStandsBitArray);
            Plugin.package.unlocked_companion_equip = new List<byte>(uiState->Buddy.CompanionInfo.BuddyEquipUnlockBitmask.ToArray());

            // aether currents
            Plugin.package.comp_flg_set = IStep.ConsumeBitArray(playerState->UnlockedAetherCurrentCompFlgSetsBitArray);
            Plugin.package.unlocked_aether_currents = IStep.ConsumeBitArray(playerState->UnlockedAetherCurrentsBitArray);

            // aetherytes
            Plugin.package.unlocked_aetherytes = IStep.ConsumeBitArray(uiState->UnlockedAetherytesBitArray);
            Plugin.package.homepoint = playerState->HomeAetheryteId;
            Plugin.package.favorite_aetherytes = new List<ushort>(playerState->FavouriteAetherytes.ToArray());
            Plugin.package.free_aetheryte = playerState->FreeAetheryteId;

            // classjob
            Plugin.package.current_class = playerState->CurrentClassJobId;
            Plugin.package.first_class = playerState->FirstClass;
            Plugin.package.rested_exp = (int)playerState->BaseRestedExperience;

            // content
            Plugin.package.unlocked_special_content = IStep.ConsumeBitArray(playerState->UnlockedSpecialContentBitArray);
            Plugin.package.unlocked_raids = IStep.ConsumeBitArray(playerState->UnlockedRaidsBitArray);
            Plugin.package.unlocked_dungeons = IStep.ConsumeBitArray(playerState->UnlockedDungeonsBitArray);
            Plugin.package.unlocked_guildhests = IStep.ConsumeBitArray(playerState->UnlockedGuildOrdersBitArray);
            Plugin.package.unlocked_trials = IStep.ConsumeBitArray(playerState->UnlockedTrialsBitArray);
            Plugin.package.unlocked_crystalline_conflicts = IStep.ConsumeBitArray(playerState->UnlockedCrystallineConflictsBitArray);
            Plugin.package.unlocked_frontlines = IStep.ConsumeBitArray(playerState->UnlockedFrontlinesBitArray);
            Plugin.package.cleared_raids = IStep.ConsumeBitArray(playerState->CompletedRaidsBitArray);
            Plugin.package.cleared_dungeons = IStep.ConsumeBitArray(playerState->CompletedDungeonsBitArray);
            Plugin.package.cleared_guildhests = IStep.ConsumeBitArray(playerState->CompletedGuildOrdersBitArray);
            Plugin.package.cleared_trials = IStep.ConsumeBitArray(playerState->CompletedTrialsBitArray);
            Plugin.package.cleared_crystalline_conflicts = IStep.ConsumeBitArray(playerState->CompletedCrystallineConflictsBitArray);
            Plugin.package.cleared_frontlines = IStep.ConsumeBitArray(playerState->CompletedFrontlinesBitArray);
            Plugin.package.cleared_masked_carnivale = IStep.ConsumeBitArray(playerState->CompletedMaskedCarnivaleBitArray);
            Plugin.package.unlocked_misc_content = IStep.ConsumeBitArray(playerState->UnlockedMiscContentBitArray);
            Plugin.package.cleared_misc_content = IStep.ConsumeBitArray(playerState->CompletedMiscContentBitArray);

            // quest
            Plugin.package.completed_quests = IStep.ConsumeBitArray(questManager->CompletedQuestsBitArray);

            // volatile
            Plugin.package.position_x = Plugin.ObjectTable.LocalPlayer.Position.X;
            Plugin.package.position_y = Plugin.ObjectTable.LocalPlayer.Position.Y;
            Plugin.package.position_z = Plugin.ObjectTable.LocalPlayer.Position.Z;
            Plugin.package.rotation = Plugin.ObjectTable.LocalPlayer.Rotation;
            Plugin.package.zone_id = Plugin.ClientState.TerritoryType;
        }

        Completed?.Invoke();
    }

    public string StepName()
    {
        return "Misc Data";
    }

    public string StepDescription()
    {
        return "No user action required.";
    }

    public void Dispose()
    {
    }
}
