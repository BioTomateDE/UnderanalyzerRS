using System.Runtime.InteropServices;
using Underanalyzer;

namespace FFI;

[StructLayout(LayoutKind.Sequential)]
public readonly struct GMCode : IGMCode
{
    public readonly RustString Name;
    public readonly RawArray<GMInstruction> Instructions;
    public readonly RawArray<GMCode> Children;
    public readonly int Length;
    public readonly int StartOffset;
    public readonly short ArgumentCount;
    public readonly short LocalCount;

    public int InstructionCount => (int)Instructions.Len;
    public IGMCode? Parent => null; // TODO
    public int ChildCount => (int)Children.Len;
    IGMString IGMCode.Name => Name;
    int IGMCode.Length => Length;
    int IGMCode.StartOffset => StartOffset;
    int IGMCode.ArgumentCount => ArgumentCount;
    int IGMCode.LocalCount => LocalCount;

    public IGMCode GetChild(int index) => Children.Get(index);

    public IGMInstruction GetInstruction(int index) => Instructions.Get(index);
}
