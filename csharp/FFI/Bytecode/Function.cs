using System.Runtime.InteropServices;
using Underanalyzer;

namespace FFI;

[StructLayout(LayoutKind.Sequential)]
public readonly struct GMFunction : IGMFunction
{
    public readonly RustString Name;

    public bool Exists => Name.Exists;

    IGMString IGMFunction.Name => Name;
}
