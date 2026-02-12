using System.Runtime.InteropServices;
using Underanalyzer;
using static Underanalyzer.IGMInstruction;

namespace FFI;

[StructLayout(LayoutKind.Sequential)]
public readonly struct GMInstruction : IGMInstruction
{
    public readonly GMVariable ResolvedVariable;
    public readonly GMFunction ResolvedFunction;
    public readonly RustString ValueString;
    public readonly double ValueDouble;
    public readonly long ValueLong;
    public readonly int ValueInt;
    public readonly int BranchOffset;
    public readonly int ArgumentCount;
    public readonly int AssetReference;
    public readonly short ValueShort;
    public readonly short ExtKind;
    public readonly short InstType;
    public readonly byte Kind;
    public readonly byte Type1;
    public readonly byte Type2;
    public readonly byte ComparisonKind;
    public readonly byte DuplicationSize;
    public readonly byte DuplicationSize2;
    public readonly byte ReferenceVarType;
    public readonly byte PopSwapSize;
    public readonly byte PopWithContextExit; // bool

    Opcode IGMInstruction.Kind => checked((Opcode)Kind);
    ExtendedOpcode IGMInstruction.ExtKind => checked((ExtendedOpcode)ExtKind);
    ComparisonType IGMInstruction.ComparisonKind => checked((ComparisonType)ComparisonKind);
    DataType IGMInstruction.Type1 => checked((DataType)Type1);
    DataType IGMInstruction.Type2 => checked((DataType)Type2);
    InstanceType IGMInstruction.InstType => checked((InstanceType)InstType);
    IGMVariable? IGMInstruction.ResolvedVariable => ResolvedVariable;
    IGMFunction? IGMInstruction.ResolvedFunction => ResolvedFunction;
    VariableType IGMInstruction.ReferenceVarType => checked((VariableType)ReferenceVarType);
    double IGMInstruction.ValueDouble => ValueDouble;
    short IGMInstruction.ValueShort => ValueShort;
    int IGMInstruction.ValueInt => ValueInt;
    long IGMInstruction.ValueLong => ValueLong;
    IGMString? IGMInstruction.ValueString => ValueString;
    int IGMInstruction.BranchOffset => BranchOffset;
    bool IGMInstruction.PopWithContextExit => BoolFromByte(PopWithContextExit);
    byte IGMInstruction.DuplicationSize => DuplicationSize;
    byte IGMInstruction.DuplicationSize2 => DuplicationSize2;
    int IGMInstruction.ArgumentCount => ArgumentCount;
    int IGMInstruction.PopSwapSize => PopSwapSize;
    int IGMInstruction.AssetReferenceId => AssetReference & 0xFF_FF_FF;

    public AssetType GetAssetReferenceType(IGameContext context)
    {
        var ctx = (GameContext)context;
        int type = AssetReference >> 24;

        return type switch
        {
            0 => AssetType.Object,
            1 => AssetType.Sprite,
            2 => AssetType.Sound,
            3 => AssetType.Room,
            4 => AssetType.Path,
            5 => AssetType.Script,
            6 => AssetType.Font,
            7 => AssetType.Timeline,
            8 => AssetType.Shader,
            9 => AssetType.Sequence,
            10 => AssetType.AnimCurve,
            11 => AssetType.ParticleSystem,
            13 => AssetType.Background,
            14 => AssetType.RoomInstance,
            _ => throw new InvalidOperationException($"Unknown asset type {type}"),
        };
    }

    IGMFunction? IGMInstruction.TryFindFunction(IGameContext? context)
    {
        if (ResolvedFunction.Exists)
            return ResolvedFunction;
        return null; //TODO
    }

    IGMVariable? IGMInstruction.TryFindVariable(IGameContext? context)
    {
        if (ResolvedVariable.Exists)
            return ResolvedVariable;
        return null; //TODO
    }

    private bool BoolFromByte(byte number)
    {
        if (number == 0)
            return false;
        if (number == 1)
            return true;
        throw new ArgumentOutOfRangeException("Expected boolean byte to be 0 or 1, got {number}");
    }
}
