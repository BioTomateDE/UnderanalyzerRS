using System.Diagnostics.CodeAnalysis;
using System.Runtime.InteropServices;
using Underanalyzer.Decompiler;

namespace FFI;

[StructLayout(LayoutKind.Sequential)]
public struct DecompileSettings : IDecompileSettings
{
    // TODO: allow settings to be controlled from rust later

    // empty constructor to avoid CS8983
    public DecompileSettings() { }

    string IDecompileSettings.IndentString => "    ";
    bool IDecompileSettings.UseSemicolon => true;
    bool IDecompileSettings.UseCSSColors => false;
    bool IDecompileSettings.PrintWarnings => true;
    bool IDecompileSettings.MacroDeclarationsAtTop => true;
    bool IDecompileSettings.EmptyLineAfterBlockLocals => true;
    bool IDecompileSettings.EmptyLineAroundEnums => true;
    bool IDecompileSettings.EmptyLineAroundBranchStatements => true;
    bool IDecompileSettings.EmptyLineBeforeSwitchCases => false;
    bool IDecompileSettings.EmptyLineAfterSwitchCases => false;
    bool IDecompileSettings.EmptyLineAroundFunctionDeclarations => true;
    bool IDecompileSettings.EmptyLineAroundStaticInitialization => true;
    bool IDecompileSettings.OpenBlockBraceOnSameLine => true;
    bool IDecompileSettings.RemoveSingleLineBlockBraces => false;
    bool IDecompileSettings.CleanupTry => false;
    bool IDecompileSettings.CleanupElseToContinue => true;
    bool IDecompileSettings.CleanupDefaultArgumentValues => true;
    bool IDecompileSettings.CleanupBuiltinArrayVariables => true;
    bool IDecompileSettings.CleanupLocalVarDeclarations => true;
    bool IDecompileSettings.CreateEnumDeclarations => true;
    string IDecompileSettings.UnknownEnumName => "UnknownEnum";
    string IDecompileSettings.UnknownEnumValuePattern => "Variant{0}";
    string IDecompileSettings.UnknownArgumentNamePattern => "arg{0}";
    bool IDecompileSettings.AllowLeftoverDataOnStack => true;

    bool IDecompileSettings.TryGetPredefinedDouble(
        double value,
        [MaybeNullWhen(false)] out string result,
        out bool isMultiPart
    )
    {
        Dictionary<double, string> singlePartDoubles = new() { { 3.141592653589793, "pi" } };
        Dictionary<double, string> multiPartDoubles = new()
        {
            { 6.283185307179586, "2 * pi" },
            { 12.566370614359172, "4 * pi" },
            { 31.41592653589793, "10 * pi" },
            { 0.3333333333333333, "1/3" },
            { 0.6666666666666666, "2/3" },
            { 1.3333333333333333, "4/3" },
            { 23.333333333333332, "70/3" },
            { 73.33333333333333, "220/3" },
            { 206.66666666666666, "620/3" },
            { 51.42857142857143, "360/7" },
            { 1.0909090909090908, "12/11" },
            { 0.06666666666666667, "1/15" },
            { 0.9523809523809523, "20/21" },
            { 0.03333333333333333, "1/30" },
            { 0.008333333333333333, "1/120" },
        };

        if (singlePartDoubles.TryGetValue(value, out result))
        {
            isMultiPart = false;
            return true;
        }

        if (multiPartDoubles.TryGetValue(value, out result))
        {
            isMultiPart = true;
            return true;
        }

        result = null;
        isMultiPart = false;
        return false;
    }
}
