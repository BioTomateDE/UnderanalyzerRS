using System.Runtime.InteropServices;
using Underanalyzer;
using static Underanalyzer.IGMInstruction;

namespace FFI;

// A variable, function or string.
[StructLayout(LayoutKind.Sequential)]
public readonly struct GMObject : IGMVariable, IGMFunction, IGMString
{
    public readonly RustString Name;
    // Only used if it's a variable:
    public readonly int VariableID;
    public readonly int InstType;

    public bool Exists => Name.Exists;

    // Not checked to allow game object references
    InstanceType IGMVariable.InstanceType => (InstanceType)InstType;
    IGMString IGMVariable.Name => Name;
    int IGMVariable.VariableID => VariableID;

    string IGMString.Content => Name.Content;

    IGMString IGMFunction.Name => Name;
}
