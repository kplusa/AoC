using System.Collections;
using System.ComponentModel;
using System.Reflection;
using System.Text.RegularExpressions;

namespace AoC._2022.Day5;

public static class FifthDay
{
    public static void Execute()
    {
        Console.WriteLine($"Puzzle of the: {MethodBase.GetCurrentMethod()!.DeclaringType?.Name}");
        var path = "./2022/Day5/instructions.txt";
        var stack1 = new Stack<char>(new[] { 'Z', 'P', 'M', 'H', 'R' });
        var stack2 = new Stack<char>(new[] { 'P', 'C', 'J', 'B' });
        var stack3 = new Stack<char>(new[] { 'S', 'N', 'H', 'G', 'L', 'C', 'D' });
        var stack4 = new Stack<char>(new[] { 'F', 'T', 'M', 'D', 'Q', 'S', 'R', 'L' });
        var stack5 = new Stack<char>(new[] { 'F', 'S', 'P', 'Q', 'B', 'T', 'Z', 'M' });
        var stack6 = new Stack<char>(new[] { 'T', 'F', 'S', 'Z', 'B', 'G' });
        var stack7 = new Stack<char>(new[] { 'N', 'R', 'V' });
        var stack8 = new Stack<char>(new[] { 'P', 'G', 'L', 'T', 'D', 'V', 'C', 'M' });
        var stack9 = new Stack<char>(new[] { 'W', 'Q', 'N', 'J', 'F', 'M', 'L' });
        var pattern = @"\d+";
        List<char> copies = new();
        List<int> digits = new();
        foreach (var line in File.ReadLines(path))
        {
            copies.Clear();
            digits.Clear();
            foreach (Match match in Regex.Matches(line, pattern))
            {
                var digit = Convert.ToInt32(match.Value);
                digits.Add(digit);
            }

            for (int i = 0; i < digits[0]; i++)
            {
                switch (digits[1])
                {
                    case 1:
                        copies.Add(stack1.First());
                        stack1.Pop();
                        break;
                    case 2:
                        copies.Add(stack2.First());
                        stack2.Pop();
                        break;
                    case 3:
                        copies.Add(stack3.First());
                        stack3.Pop();
                        break;
                    case 4:
                        copies.Add(stack4.First());
                        stack4.Pop();
                        break;
                    case 5:
                        copies.Add(stack5.First());
                        stack5.Pop();
                        break;
                    case 6:
                        copies.Add(stack6.First());
                        stack6.Pop();
                        break;
                    case 7:
                        copies.Add(stack7.First());
                        stack7.Pop();
                        break;
                    case 8:
                        copies.Add(stack8.First());
                        stack8.Pop();
                        break;
                    case 9:
                        copies.Add(stack9.First());
                        stack9.Pop();
                        break;
                }
            }

            //copies.Reverse(); Part two
            foreach (var item in copies)
            {
                switch (digits[2])
                {
                    case 1:
                        stack1.Push(item);
                        break;
                    case 2:
                        stack2.Push(item);
                        break;
                    case 3:
                        stack3.Push(item);
                        break;
                    case 4:
                        stack4.Push(item);
                        break;
                    case 5:
                        stack5.Push(item);
                        break;
                    case 6:
                        stack6.Push(item);
                        break;
                    case 7:
                        stack7.Push(item);
                        break;
                    case 8:
                        stack8.Push(item);
                        break;
                    case 9:
                        stack9.Push(item);
                        break;
                }
            }
        }
        Console.WriteLine($"{stack1.First()}" +
                          $"{stack2.First()}" +
                          $"{stack3.First()}" +
                          $"{stack4.First()}" +
                          $"{stack5.First()}" +
                          $"{stack6.First()}" +
                          $"{stack7.First()}" +
                          $"{stack8.First()}" +
                          $"{stack9.First()}");
    }
}