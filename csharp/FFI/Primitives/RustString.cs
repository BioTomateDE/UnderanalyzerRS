using System.Runtime.InteropServices;
using Underanalyzer;
using static System.Text.Encoding;

namespace FFI;

[StructLayout(LayoutKind.Sequential)]
public readonly struct RustString : IGMString
{
    private readonly IntPtr Ptr;
    private readonly nuint Len;

    public bool Exists => Len != 0;

    public unsafe string ToManagedString()
    {
        byte* ptr = (byte*)Ptr.ToPointer();
        int length = checked((int)Len);
        return UTF8.GetString(ptr, length);
    }

    // this can be cached probably
    // if anyone wants to optimise this, here's a start ig
    public string Content => ToManagedString();
}
