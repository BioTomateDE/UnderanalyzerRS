// Basic disassembler for debugging purposes.
using System.Reflection;
using Underanalyzer;
using static Underanalyzer.IGMInstruction;

namespace Assembly;

internal static class Disassembler
{
    static readonly bool VERBOSE = false;
    static readonly bool LOG = false;

    public static string Disassemble(IGMCode code)
    {
        MyStringBuilder sb = new();
        sb.AppendLine($"\x1B[2;3m# Disassembly for {code.Name.Content} ({code.InstructionCount} instructions)\x1B[0m");
        for (int i = 0; i < code.InstructionCount; i++)
        {
            IGMInstruction instr = code.GetInstruction(i);
            if (VERBOSE) DoVerbose(sb, instr);
            else DoNormal(sb, instr);
            sb.AppendLine();
        }
        return sb.ToString();
    }


    static void DoNormal(MyStringBuilder sb, IGMInstruction instr)
    {
        sb.Append("\x1B[97m");
        sb.Append(GetMnemonic(instr.Kind, instr.ExtKind));
        sb.Append("\x1B[0m");
        switch (instr.Kind)
        {
            case Opcode.Convert:
            case Opcode.Multiply:
            case Opcode.Divide:
            case Opcode.GMLDivRemainder:
            case Opcode.GMLModulo:
            case Opcode.Add:
            case Opcode.Subtract:
            case Opcode.And:
            case Opcode.Or:
            case Opcode.Xor:
            case Opcode.ShiftLeft:
            case Opcode.ShiftRight:
                WriteTypes(sb, instr.Type1, instr.Type2);
                break;
            case Opcode.Compare:
                WriteTypes(sb, instr.Type1, instr.Type2);
                sb.Append(" ");
                sb.Append(instr.ComparisonKind);
                break;
            case Opcode.Negate:
            case Opcode.Not:
            case Opcode.Exit:
            case Opcode.Return:
            case Opcode.PopDelete:
                WriteTypes(sb, instr.Type1);
                break;
            case Opcode.Pop:
                WriteTypes(sb, instr.Type1, instr.Type2);
                sb.Append(' ');
                WriteVar(sb, instr.ResolvedVariable, instr.ReferenceVarType);
                break;
            case Opcode.Duplicate:
                sb.Append(" ");
                sb.Append(instr.DuplicationSize);
                if (instr.DuplicationSize2 != 0)
                {
                    sb.Append(" ");
                    sb.Append(instr.DuplicationSize2);
                }
                break;
            case Opcode.Branch:
            case Opcode.BranchTrue:
            case Opcode.BranchFalse:
            case Opcode.PushWithContext:
            case Opcode.PopWithContext:
                if (instr.PopWithContextExit)
                    sb.Append(" <drop>");
                else
                {
                    sb.Append(" ");
                    sb.Append(instr.BranchOffset);
                }
                break;
            case Opcode.Push:
                WriteTypes(sb, instr.Type1);
                sb.Append(" ");
                switch (instr.Type1)
                {
                    case DataType.Boolean:
                    case DataType.Int32:
                        if (instr.ResolvedFunction is { } f)
                        {
                            sb.Append("[function]");
                            sb.Append("\x1B[32m");
                            sb.Append(f.Name.Content);
                            sb.Append("\x1B[0m");
                        }
                        else
                        {
                            sb.Append(instr.ValueInt);
                        }
                        break;
                    case DataType.Double:
                        sb.Append(instr.ValueDouble);
                        break;
                    case DataType.Int16:
                        sb.Append(instr.ValueShort);
                        break;
                    case DataType.Int64:
                        sb.Append(instr.ValueLong);
                        break;
                    case DataType.String:
                        sb.Append("\x1B[33m");
                        sb.Append(Escape(instr.ValueString.Content));
                        sb.Append("\x1B[0m");
                        break;
                    case DataType.Variable:
                        WriteVar(sb, instr.ResolvedVariable, instr.ReferenceVarType);
                        break;
                    default:
                        sb.Append("UNKNOWN DATA TYPE");
                        break;
                }
                break;
            case Opcode.PushLocal:
            case Opcode.PushGlobal:
            case Opcode.PushBuiltin:
                sb.Append(" ");
                WriteVar(sb, instr.ResolvedVariable, instr.ReferenceVarType);
                break;
            case Opcode.PushImmediate:
                sb.Append(" ");
                sb.Append(instr.ValueShort);
                break;
            case Opcode.Call:
                sb.Append(" ");
                sb.Append("\x1B[32m");
                sb.Append(instr.ResolvedFunction.Name.Content);
                sb.Append("\x1B[0m");
                sb.Append("(argc=\x1B[1m");
                sb.Append(instr.ArgumentCount);
                sb.Append("\x1B[0m)");
                break;
            case Opcode.CallVariable:
                sb.Append(" argc=");
                sb.Append(instr.ArgumentCount);
                break;
            case Opcode.Extended:
                sb.Append("TODO: extended");
                break;

            default:
                sb.Append("UNKNOWN OPCODE???");
                break;
        }
    }

    static void DoVerbose(MyStringBuilder sb, IGMInstruction instr)
    {
        sb.Append(GetMnemonic(instr.Kind, instr.ExtKind));
        sb.Append(" ArgC=");
        sb.Append(instr.ArgumentCount);
        sb.Append(" AssetRefID=");
        sb.Append(instr.AssetReferenceId);
        sb.Append(" BranchOffset=");
        sb.Append(instr.BranchOffset);
        sb.Append(" CmpKind=");
        sb.Append(instr.ComparisonKind);
        sb.Append(" DupSize=");
        sb.Append(instr.DuplicationSize);
        sb.Append(" DupSize2=");
        sb.Append(instr.DuplicationSize2);
        sb.Append(" InstType=");
        sb.Append(instr.InstType);
        sb.Append(" PopSwapSize=");
        sb.Append(instr.PopSwapSize);
        sb.Append(" PopEnvX=");
        sb.Append(instr.PopWithContextExit);
        sb.Append(" RefVarType=");
        sb.Append(instr.ReferenceVarType);
        sb.Append(" Func=");
        if (instr.ResolvedFunction is { } f)
            sb.Append(f.Name.Content);
        else sb.Append("null");
        sb.Append(" Var=");
        if (instr.ResolvedVariable is { } v)
        {
            sb.Append(v.Name.Content);
            sb.Append("(ID=");
            sb.Append(v.VariableID);
            sb.Append(" Inst=");
            sb.Append(v.InstanceType);
            sb.Append(")");
        }
        else sb.Append("null");
        sb.Append(" Type1=");
        sb.Append(instr.Type1);
        sb.Append(" Type2=");
        sb.Append(instr.Type2);
        sb.Append(" Double=");
        sb.Append(instr.ValueDouble);
        sb.Append(" Int=");
        sb.Append(instr.ValueInt);
        sb.Append(" Long=");
        sb.Append(instr.ValueLong);
        sb.Append(" Short=");
        sb.Append(instr.ValueShort);
        sb.Append(" String=");
        if (instr.ValueString is { } s)
            sb.Append(Escape(s.Content));
        else sb.Append("null");
    }

    static string GetMnemonic(Opcode opcode, ExtendedOpcode extended)
    {
        if (opcode == Opcode.Extended)
        {
            var field = typeof(ExtendedOpcode).GetField(Enum.GetName(typeof(ExtendedOpcode), extended)!)!;
            var info = field.GetCustomAttribute<OpcodeInfo>()!;
            return info.Mnemonic;
        }
        else
        {
            var field = typeof(Opcode).GetField(Enum.GetName(typeof(Opcode), opcode)!)!;
            var info = field.GetCustomAttribute<OpcodeInfo>()!;
            return info.Mnemonic;
        }
    }

    class MyStringBuilder
    {
        System.Text.StringBuilder Sb;
        public MyStringBuilder()
        {
            Sb = new();
        }

        public void Append(object thingy)
        {
            if (LOG)
            {
                Console.Write(thingy);
            }
            Sb.Append(thingy);
        }

        public void AppendLine()
        {
            if (LOG)
            {
                Console.WriteLine();
            }
            Sb.AppendLine();
        }

        public void AppendLine(string stringy)
        {
            if (LOG)
            {
                Console.WriteLine(stringy);
            }
            Sb.AppendLine(stringy);
        }


        public override string ToString() => Sb.ToString();
    }

    static void WriteVar(MyStringBuilder sb, IGMVariable var, VariableType refType)
    {
        sb.Append("\x1B[31m");
        if (refType != VariableType.Normal)
        {
            sb.Append("\x1B[93;2m[");
            sb.Append(refType.ToString().ToLowerInvariant());
            sb.Append("]\x1B[0m");
        }
        sb.Append(InstKeyword(var.InstanceType));
        sb.Append("\x1B[0m");
        sb.Append(".");
        sb.Append("\x1B[36m");
        sb.Append(var.Name.Content);
        sb.Append("\x1B[0;2m  # VarID: ");
        sb.Append(var.VariableID);
        sb.Append("\x1B[0m");
    }

    static void WriteTypes(MyStringBuilder sb, DataType type1, DataType? type2 = null)
    {
        sb.Append("\x1B[2m");
        sb.Append(".");
        sb.Append(DataTypeChar(type1));
        if (type2 is DataType ty2)
        {
            sb.Append(".");
            sb.Append(DataTypeChar(ty2));
        }
        sb.Append("\x1B[0m");
    }

    static char DataTypeChar(DataType ty) => ty switch
    {
        DataType.Double => 'd',
        DataType.Int32 => 'i',
        DataType.Int64 => 'l',
        DataType.Boolean => 'b',
        DataType.Variable => 'v',
        DataType.String => 's',
        DataType.Int16 => 'e',
        _ => '?',
    };

    static string InstKeyword(InstanceType inst) => inst switch
    {
        InstanceType.Self => "self",
        InstanceType.Other => "other",
        InstanceType.All => "all",
        InstanceType.Noone => "noone",
        InstanceType.Global => "global",
        InstanceType.Builtin => "builtin",
        InstanceType.Local => "local",
        InstanceType.StackTop => "stacktop",
        InstanceType.Argument => "arg",
        InstanceType.Static => "static",
        _ => inst.ToString(),
    };

    // https://stackoverflow.com/a/14087738
    static string Escape(string input)
    {
        System.Text.StringBuilder literal = new(input.Length + 2);
        literal.Append("\"");
        foreach (var c in input)
        {
            switch (c)
            {
                case '\"': literal.Append("\\\""); break;
                case '\\': literal.Append(@"\\"); break;
                case '\0': literal.Append(@"\0"); break;
                case '\a': literal.Append(@"\a"); break;
                case '\b': literal.Append(@"\b"); break;
                case '\f': literal.Append(@"\f"); break;
                case '\n': literal.Append(@"\n"); break;
                case '\r': literal.Append(@"\r"); break;
                case '\t': literal.Append(@"\t"); break;
                case '\v': literal.Append(@"\v"); break;
                default:
                    // ASCII printable character
                    if (c >= 0x20 && c <= 0x7e)
                    {
                        literal.Append(c);
                        // As UTF16 escaped character
                    }
                    else
                    {
                        literal.Append(@"\u");
                        literal.Append(((int)c).ToString("x4"));
                    }
                    break;
            }
        }
        literal.Append("\"");
        return literal.ToString();
    }

}
