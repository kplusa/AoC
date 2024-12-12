using System.Reflection;

namespace AoC._2022.Day2;

public static class SecondDay
{
    public static void Execute()
    {
        Console.WriteLine($"Puzzle of the: {MethodBase.GetCurrentMethod()!.DeclaringType?.Name}");
        /*
            A - Rock
            B- Paper
            C- Scissors
            
            X - Rock
            Y- Paper
            Z- Scissors
            
            Rock 1
            Paper 2
            Scissors 3
            
            win 6
            draw 3
            lose 0
        */
        var path = "./Day2/strategy_guide.txt";
        Dictionary<string, int> paperRockScissorsDictionary = new();
        paperRockScissorsDictionary.Add("X", 1);
        paperRockScissorsDictionary.Add("Y", 2);
        paperRockScissorsDictionary.Add("Z", 3);
        var draw = 3;
        var win = 6;
        var lose = 0;
        List<int> listOfScoreOfRound = new();
        var score = 0;
        foreach (var line in File.ReadLines(path))
        {
            var letters = line.Split(' ');
            if (letters[0] == "A")
            {
                if (letters[1] == "X")
                {
                    score += paperRockScissorsDictionary["Z"];
                    score += lose;
                }
                else if (letters[1] == "Y")
                {
                    score += paperRockScissorsDictionary["X"];
                    score += draw;
                }
                else if (letters[1] == "Z")
                {
                    score += paperRockScissorsDictionary["Y"];
                    score += win;
                }
            }
            else if (letters[0] == "B")
            {
                if (letters[1] == "X")
                {
                    score += paperRockScissorsDictionary["X"];
                    score += lose;
                }
                else if (letters[1] == "Y")
                {
                    score += paperRockScissorsDictionary["Y"];
                    score += draw;
                }
                else if (letters[1] == "Z")
                {
                    score += paperRockScissorsDictionary["Z"];
                    score += win;
                }
            }
            else if (letters[0] == "C")
            {
                if (letters[1] == "X")
                {
                    score += paperRockScissorsDictionary["Y"];
                    score += lose;
                }
                else if (letters[1] == "Y")
                {
                    score += paperRockScissorsDictionary["Z"];
                    score += draw;
                }
                else if (letters[1] == "Z")
                {
                    score += paperRockScissorsDictionary["X"];
                    score += win;
                }
            }

            listOfScoreOfRound.Add(score);
            score = 0;
        }

        var totalScore = listOfScoreOfRound.Sum();
        Console.WriteLine(totalScore);
    }
}