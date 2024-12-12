using System.Reflection;

namespace AoC._2022.Day6;

public static class SixthDay
{
    public static void Execute()
    {
        Console.WriteLine($"Puzzle of the: {MethodBase.GetCurrentMethod()!.DeclaringType?.Name}");
        var path = "./Day6/datastream.txt";
        var line = File.ReadLines(path).First();
        var i = 0;
        var sequence = "";
        var messageLength = 14;
        foreach (var character in line)
        {
            i++;
            if (i < messageLength)
                sequence += character;
            else
                sequence = line.Substring(i - messageLength, messageLength);

            if (sequence.Length != messageLength) continue;
            var containsDuplicates = sequence.GroupBy(x => x).Any(x => x.Count() > 1);
            if (!containsDuplicates)
                break;
        }

        Console.WriteLine(i);
    }
}