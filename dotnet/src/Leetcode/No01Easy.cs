namespace dotnet.Leetcode;

public class No01Easy
{
    public int[] TwoSum(int[] nums, int target)
    {
        for (var x = 0; x < nums.Length; x++)
        {
            var tmp = target - nums[x];
            for (var y = x + 1; y < nums.Length; y++)
            {
                if (nums[y] == tmp)
                {
                    return new[] { x, y };
                }
            }
        }

        return Array.Empty<int>();
    }
}

