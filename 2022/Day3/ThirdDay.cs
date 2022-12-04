using System.Reflection;

namespace AoC._2022.Day3;

public static class ThirdDay
{
    public static void Execute()
    {
        Console.WriteLine($"Puzzle of the: {MethodBase.GetCurrentMethod()!.DeclaringType?.Name}");
        var priorityOfLetter = new Dictionary<char, int>();
        var path = "./2022/Day3/rucksacks.txt";
        List<int> listOfPrioritiesOfItems = new();
        var value = 0;
        var i = 0;
        List<string> lineOfThreeElves = new();
        for (var c = 'A'; c <= 'z'; c++)
            if (char.IsLetter(c))
                priorityOfLetter.Add(char.IsLower(c) ? char.ToUpper(c) : char.ToLower(c), ++value);
        foreach (var line in File.ReadLines(path))
        {
            lineOfThreeElves.Add(line);
            if (i == 2)
            {
                var count = 1;
                var first = lineOfThreeElves[0];
                var second = lineOfThreeElves[1];
                var third = lineOfThreeElves[2];
                foreach (var character in first)
                    if (second.Contains(character) && third.Contains(character) && count == 1)
                    {
                        count = 0;
                        listOfPrioritiesOfItems.Add(priorityOfLetter[character]);
                    }

                lineOfThreeElves.Clear();
                i = -1;
            }

            i++;
        }

        var total = listOfPrioritiesOfItems.Sum();
        Console.WriteLine(total);
    }
}