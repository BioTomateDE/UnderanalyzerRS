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
            //Console.WriteLine(Assembly.Disassembler.Disassemble(*code));
            Underanalyzer.Decompiler.DecompileSettings settings = new();
            // Underanalyzer.Decompiler.GameSpecific.GameSpecificRegistry registry = new();
            // using StreamReader reader = new("/home/bio/fork/utmtpr/UndertaleModLib/GameSpecificData/Underanalyzer/undertale.json");
            // string json = reader.ReadToEnd();
            // registry.DeserializeFromJson(json);
            DecompileContext decompileContext = new(*gameContext, *code, settings);
            string output = decompileContext.DecompileToString();
            CsString outputRaw = CsString.FromManagedString(output);
            return new ReturnValue { str = outputRaw, error = 0 };
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
