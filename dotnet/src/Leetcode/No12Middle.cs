namespace dotnet.Leetcode;

public class No12Middle
{
    /// <summary>
    ///  <seealso cref="https://leetcode.cn/problems/integer-to-roman/description/"/>
    /// </summary>
    /// <param name="num">十进制数字</param>
    /// <returns>罗马数字</returns>
    public string IntToRoman(int num)
    {
        int[] values = { 1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1 };
        string[] reps = { "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I" };

        var res = "";
        
        for (var i = 0; i < 13; i++)
        {
            while (num >= values[i])
            {
                num -= values[i];
                res += reps[i];
            }
        }

        return res;
    }
}