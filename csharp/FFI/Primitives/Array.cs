using System.Runtime.InteropServices;

namespace FFI;

[StructLayout(LayoutKind.Sequential)]
public readonly struct RawArray<T>
    where T : unmanaged
{
    private readonly IntPtr Ptr;
    public readonly nuint Len;
    private readonly nuint Cap;

    public ref readonly T Get(int index)
    {
        if (index < 0)
        {
            throw new InvalidOperationException($"Index {index} is negative");
        }
        if ((nuint)index >= Len)
        {
            throw new InvalidOperationException(
                $"Index {index} is out of range in array with length {Len}"
            );
        }

        unsafe
        {
            return ref ((T*)Ptr)[index];
        }
    }
}
