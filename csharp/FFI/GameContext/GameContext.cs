using System.Runtime.InteropServices;
using Underanalyzer;
using Underanalyzer.Compiler;
using Underanalyzer.Decompiler;
using Underanalyzer.Decompiler.GameSpecific;

namespace FFI;

public enum LTSBranch : byte
{
    Pre2022 = 1,
    LTS2022 = 2,
    Post2022 = 3,
}

[StructLayout(LayoutKind.Sequential)]
public struct GMVersion
{
    public readonly uint Major;
    public readonly uint Minor;
    public readonly uint Release;
    public readonly uint Build;

    public bool AtLeast(uint major, uint minor = 0, uint release = 0, uint build = 0)
    {
        if (Major != major)
            return (Major > major);
        if (Minor != minor)
            return (Minor > minor);
        if (Release != release)
            return (Release > release);
        if (Build != build)
            return (Build > build);
        return true;
    }
}

[StructLayout(LayoutKind.Sequential)]
public readonly struct GameContext : IGameContext
{
    readonly GMVersion Ver;
    readonly byte WadVersion;
    readonly LTSBranch Branch;

    readonly byte UsesShortCurcuit;
    readonly byte UsesArrayCow;

    readonly RawArray<RustString> AssetObjectNames;
    readonly RawArray<RustString> AssetSpriteNames;
    readonly RawArray<RustString> AssetSoundNames;
    readonly RawArray<RustString> AssetRoomNames;
    readonly RawArray<RustString> AssetBackgroundNames;
    readonly RawArray<RustString> AssetPathNames;
    readonly RawArray<RustString> AssetScriptNames;
    readonly RawArray<RustString> AssetFontNames;
    readonly RawArray<RustString> AssetTimelineNames;
    readonly RawArray<RustString> AssetShaderNames;
    readonly RawArray<RustString> AssetSequenceNames;
    readonly RawArray<RustString> AssetAnimCurveNames;
    readonly RawArray<RustString> AssetParticleSystemNames;

    bool IsVer(uint major, uint minor = 0, uint release = 0, uint build = 0)
    {
        return Ver.AtLeast(major, minor, release, build);
    }

    bool IsVerNonLTS(uint major, uint minor = 0, uint release = 0, uint build = 0)
    {
        switch (Branch)
        {
            case LTSBranch.Post2022:
                return IsVer(major, minor, release, build);
            case LTSBranch.Pre2022:
            case LTSBranch.LTS2022:
                return false;
            default:
                throw new ArgumentOutOfRangeException(
                    "Expeed LTS Branch to be 1, 2 or 3; got {Branch}"
                );
        }
    }

    public bool UsingGMS2OrLater => IsVer(2);
    public bool UsingGMLv2 => IsVer(2, 3);
    public bool UsingStringRealOptimizations => IsVer(2) || Ver.Build == 1539 || Ver.Build >= 1763;
    public bool UsingTypedBooleans => IsVer(2, 3, 7);
    public bool UsingNullishOperator => IsVer(2, 3, 7);
    public bool UsingAssetReferences => IsVer(2023, 8);
    public bool UsingRoomInstanceReferences => IsVer(2024, 2);
    public bool UsingFunctionScriptReferences => IsVer(2024, 2);
    public bool UsingNewFunctionResolution => IsVer(2023, 13);
    public bool Bytecode14OrLower => WadVersion <= 14;
    public bool UsingLogicalShortCircuit => UsesShortCurcuit != 0;
    public bool UsingLongCompoundBitwise => IsVer(2, 3, 2);
    public bool UsingExtraRepeatInstruction => !IsVerNonLTS(2022, 11);
    public bool UsingFinallyBeforeThrow => !IsVer(2024, 6);
    public bool UsingConstructorSetStatic => IsVer(2024, 11);
    public bool UsingArrayCopyOnWrite => UsesArrayCow != 0;
    public bool UsingNewArrayOwners => IsVer(2, 3, 2);
    public bool UsingReentrantStatic => !IsVer(2024, 11);
    public bool UsingNewFunctionVariables => IsVer(2024, 2);
    public bool UsingSelfToBuiltin => IsVer(2024, 2);
    public bool UsingGlobalConstantFunction => IsVer(2023, 11);
    public bool UsingObjectFunctionForesight => IsVer(2024, 11);
    public bool UsingBetterTryBreakContinue => IsVer(2024, 11);
    public bool UsingBuiltinDefaultArguments => IsVer(2024, 11);
    public bool UsingOptimizedFunctionDeclarations => IsVer(2024, 14);

    public IGlobalFunctions GlobalFunctions => new GlobalFunctions(); // empty cache for now

    public GameSpecificRegistry GameSpecificRegistry => new();

    public IBuiltins Builtins => new BuiltinList(Ver);
    public ICodeBuilder CodeBuilder => throw new NotImplementedException("CodeBuilder");

    public bool GetAssetId(string assetName, out int assetId)
    {
        throw new NotImplementedException("GetAssetId");
    }

    public string? GetAssetName(AssetType assetType, int assetIndex)
    {
        if (assetIndex < 0)
            return null;
        switch (assetType)
        {
            case AssetType.Object:
                return _GetAssetNameFor(in AssetObjectNames, assetIndex);
            case AssetType.Sprite:
                return _GetAssetNameFor(in AssetSpriteNames, assetIndex);
            case AssetType.Sound:
                return _GetAssetNameFor(in AssetSoundNames, assetIndex);
            case AssetType.Room:
                return _GetAssetNameFor(in AssetRoomNames, assetIndex);
            case AssetType.Background:
                return _GetAssetNameFor(in AssetBackgroundNames, assetIndex);
            case AssetType.Path:
                return _GetAssetNameFor(in AssetPathNames, assetIndex);
            case AssetType.Script:
                return _GetAssetNameFor(in AssetScriptNames, assetIndex);
            case AssetType.Font:
                return _GetAssetNameFor(in AssetFontNames, assetIndex);
            case AssetType.Timeline:
                return _GetAssetNameFor(in AssetTimelineNames, assetIndex);
            case AssetType.Shader:
                return _GetAssetNameFor(in AssetShaderNames, assetIndex);
            case AssetType.Sequence:
                return _GetAssetNameFor(in AssetSequenceNames, assetIndex);
            case AssetType.AnimCurve:
                return _GetAssetNameFor(in AssetAnimCurveNames, assetIndex);
            case AssetType.ParticleSystem:
                return _GetAssetNameFor(in AssetParticleSystemNames, assetIndex);
            case AssetType.RoomInstance:
                if (assetIndex < 100_000)
                {
                    return null;
                }
                return $"inst_{assetIndex}";
        }
        return null;
    }

    public bool GetRoomInstanceId(string roomInstanceName, out int assetId)
    {
        throw new NotImplementedException("GetRoomInstanceId");
    }

    public bool GetScriptId(string scriptName, out int assetId)
    {
        throw new NotImplementedException("GetScriptId");
    }

    public bool GetScriptIdByFunctionName(string functionName, out int assetId)
    {
        throw new NotImplementedException("GetScriptIdByFunctionName");
    }

    private string? _GetAssetNameFor(in RawArray<RustString> array, int index)
    {
        if (index > 0 && (nuint)index < array.Len)
        {
            return array.Get(index).Content;
        }
        return null;
    }
}
