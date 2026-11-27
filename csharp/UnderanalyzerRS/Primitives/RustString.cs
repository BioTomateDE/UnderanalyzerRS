using System.Runtime.InteropServices;
using Underanalyzer;
using static System.Text.Encoding;

namespace FFI;

[StructLayout(LayoutKind.Sequential)]
public readonly struct RustString : IGMString
{
    private readonly IntPtr Ptr;
    private readonly int Len;

    public bool Exists => Len != 0;

    public unsafe string ToManagedString()
    {
        byte* ptr = (byte*)Ptr.ToPointer();
        return UTF8.GetString(ptr, Len);
    }

    // this can be cached probably
    // TODO: if anyone wants to optimise this, here's a start ig
    public string Content => ToManagedString();
}
