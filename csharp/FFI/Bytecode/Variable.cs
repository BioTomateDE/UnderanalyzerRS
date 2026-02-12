using System.Runtime.InteropServices;
using Underanalyzer;
using static Underanalyzer.IGMInstruction;

namespace FFI;

[StructLayout(LayoutKind.Sequential)]
public readonly struct GMVariable : IGMVariable
{
    public readonly RustString Name;
    public readonly int VariableID;
    public readonly short InstType;

    public bool Exists => Name.Exists;

    IGMString IGMVariable.Name => Name;

    // Not checked to allow game object references
    InstanceType IGMVariable.InstanceType => (InstanceType)InstType;

    int IGMVariable.VariableID => VariableID;
}
