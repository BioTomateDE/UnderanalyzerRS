using System.Runtime.InteropServices;
using Underanalyzer.Decompiler;

namespace FFI;

[StructLayout(LayoutKind.Sequential)]
struct ReturnValue
{
    public CsString str;
    public byte error;
}

static class Exports
{
    [UnmanagedCallersOnly(EntryPoint = "decompile_to_string")]
    static unsafe ReturnValue DecompileToString(GameContext* gameContext, GMCode* code)
    {
        try
        {
            DecompileSettings settings = new();
            DecompileContext decompileContext = new(*gameContext, *code, settings);
            string output = decompileContext.DecompileToString();
            CsString outputRaw = CsString.FromManagedString(output);
            return new ReturnValue { str = outputRaw, error = 0 };
        }
        catch (ThreadAbortException e)
        {
            // This exception would get rethrown after the catch block.
            // Therefore, it needs to kill the program for safety.
            Console.WriteLine($"[FATAL] ThreadAbortException occurred: {e.Message}");
            System.Environment.Exit(999);
            while (true) { }
            // (apparently this exception got removed? so remove this block later)
        }
        catch (Exception e)
        {
            CsString message = CsString.FromManagedString(e.ToString());
            return new ReturnValue { str = message, error = 1 };
        }
    }

    [UnmanagedCallersOnly(EntryPoint = "free_cs_string")]
    public static void FreeRawString(IntPtr ptr)
    {
        CsString.Deallocate(ptr);
    }
}
