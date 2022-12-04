using System.Text.RegularExpressions;

namespace AoC.Day4;

public static class DayFour
{
    public static void Execute()
    {
        var pattern = @"\d+";
        int total = 0;
        int numbersLimit = 4;
        var path = "./Day4/pairs.txt";
        List<int> numbers = new();
        foreach (var line in File.ReadLines(path))
        {
            foreach (Match match in Regex.Matches(line, pattern))
            {
                var digit = Convert.ToInt32((match.Value));
                numbers.Add(digit);
                if (--numbersLimit <= 0)
                {
                    if (numbers[0] <= numbers[3] && numbers[2] <= numbers[1])
                    {
                        //if ((numbers[0] >= numbers[2] && numbers[1] <= numbers[3]) || (numbers[2] >= numbers[0] && numbers[3] <= numbers[1]) )
                            total++;
                    }

                    numbers.Clear();
                    numbersLimit = 4;
                }
            }
        }
        Console.WriteLine(total);
    }
}