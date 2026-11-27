using System.Runtime.InteropServices;
using static System.Text.Encoding;

namespace FFI;

[StructLayout(LayoutKind.Sequential)]
public struct CsString
{
    private IntPtr Ptr;
    private nuint Len;

    public static CsString FromManagedString(string str)
    {
        if (str is null)
        {
            Console.WriteLine("[WARN] Managed String is null; converting to empty string");
            str = "";
            //return new RawString { Ptr = IntPtr.Zero, Len = 0 };
        }

        int byteCount = UTF8.GetByteCount(str);
        IntPtr ptr = Marshal.AllocHGlobal(byteCount);
        unsafe
        {
            fixed (char* chars = str)
            {
                UTF8.GetBytes(chars, str.Length, (byte*)ptr, byteCount);
            }
        }
        return new CsString { Ptr = ptr, Len = (nuint)byteCount };
    }

    public static void Deallocate(IntPtr ptr)
    {
        Marshal.FreeHGlobal(ptr);
    }

    public void Deallocate()
    {
        Deallocate(Ptr);
    }
}
