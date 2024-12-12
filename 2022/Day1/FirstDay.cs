using System.Reflection;

namespace AoC._2022.Day1;

public class FirstDay
{
    public static void Execute()
    {
        Console.WriteLine($"Puzzle of the: {MethodBase.GetCurrentMethod()!.DeclaringType?.Name}");
        var path = "./Day1/data.txt";
        var sum = 0;
        List<int> listOfSums = new();
        foreach (var line in File.ReadLines(path))
        {
            if (string.IsNullOrEmpty(line))
            {
                listOfSums.Add(sum);
                sum = 0;
                continue;
            }

            sum += Convert.ToInt32(line);
        }

        var maxSummator = listOfSums.Max().ToString();
        var topThreeSummators = listOfSums.OrderDescending().Take(3).Sum();
        Console.WriteLine(maxSummator);
        Console.WriteLine(topThreeSummators);
    }
}