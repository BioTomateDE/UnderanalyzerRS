using System.Runtime.InteropServices;
using Underanalyzer;
using static Underanalyzer.IGMInstruction;

namespace FFI;

[StructLayout(LayoutKind.Sequential)]
public readonly struct GMInstruction : IGMInstruction
{
    /// Push value (int/long/double)
    readonly long Primitive;
    /// Reference to variable OR function OR string
    readonly GMObject Object;
    /// Some extra value(s)
    readonly int Immediate;
    /// Data types
    readonly byte Types;
    readonly byte OpcodeKind;
    readonly byte ObjectKind;

    Opcode IGMInstruction.Kind => checked((Opcode)OpcodeKind);
    ExtendedOpcode IGMInstruction.ExtKind => (ExtendedOpcode)Immediate;
    ComparisonType IGMInstruction.ComparisonKind => (ComparisonType)Immediate;
    DataType IGMInstruction.Type1 => checked((DataType)(Types & 0xF));
    DataType IGMInstruction.Type2 => checked((DataType)(Types >> 4));
    InstanceType IGMInstruction.InstType => (InstanceType)(Immediate & 0xFF_FF);
    IGMString? IGMInstruction.ValueString => ObjectKind == 1 ? Object : null;
    IGMVariable? IGMInstruction.ResolvedVariable => ObjectKind == 2 ? Object : null;
    IGMFunction? IGMInstruction.ResolvedFunction => ObjectKind == 3 ? Object : null;
    VariableType IGMInstruction.ReferenceVarType => (VariableType)((uint)Immediate >> 16);
    double IGMInstruction.ValueDouble => BitConverter.Int64BitsToDouble(Primitive);
    short IGMInstruction.ValueShort => (short)Immediate;
    int IGMInstruction.ValueInt => (int)Primitive;
    long IGMInstruction.ValueLong => Primitive;
    int IGMInstruction.BranchOffset => Immediate;
    bool IGMInstruction.PopWithContextExit => Immediate == (0xF << 20);
    byte IGMInstruction.DuplicationSize => (byte)Immediate;
    byte IGMInstruction.DuplicationSize2 => (byte)((uint)Immediate >> 8);
    int IGMInstruction.ArgumentCount => Immediate;
    int IGMInstruction.PopSwapSize => Immediate; // if array mode then 6 else 5
    int IGMInstruction.AssetReferenceId => Immediate & 0xFF_FF_FF;

    public AssetType GetAssetReferenceType(IGameContext context)
    {
        return checked((AssetType)(Immediate >> 24));
    }

    IGMFunction? IGMInstruction.TryFindFunction(IGameContext? context)
    {
        if (ObjectKind == 3) return Object;
        return null; //TODO
    }

    IGMVariable? IGMInstruction.TryFindVariable(IGameContext? context)
    {
        if (ObjectKind == 2) return Object;
        return null; //TODO
    }
}
