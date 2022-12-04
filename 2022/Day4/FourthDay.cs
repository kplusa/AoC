using System.Reflection;
using System.Text.RegularExpressions;

namespace AoC._2022.Day4;

public static class FourthDay
{
    public static void Execute()
    {
        Console.WriteLine($"Puzzle of the: {MethodBase.GetCurrentMethod()!.DeclaringType?.Name}");
        var pattern = @"\d+";
        var total = 0;
        var numbersLimit = 4;
        var path = "./2022/Day4/pairs.txt";
        List<int> numbers = new();
        foreach (var line in File.ReadLines(path))
        foreach (Match match in Regex.Matches(line, pattern))
        {
            var digit = Convert.ToInt32(match.Value);
            numbers.Add(digit);
            if (--numbersLimit <= 0)
            {
                if (numbers[0] <= numbers[3] && numbers[2] <= numbers[1])
                    //if ((numbers[0] >= numbers[2] && numbers[1] <= numbers[3]) || (numbers[2] >= numbers[0] && numbers[3] <= numbers[1]) )
                    total++;

                numbers.Clear();
                numbersLimit = 4;
            }
        }

        Console.WriteLine(total);
    }
}