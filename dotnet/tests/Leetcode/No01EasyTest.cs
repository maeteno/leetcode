using Microsoft.VisualStudio.TestTools.UnitTesting;
using Microsoft.VisualStudio.TestTools.UnitTesting.Logging;

namespace dotnet.Leetcode;

[TestClass]
public class No01EasyTest
{
    [TestMethod]
    public void test_no_01()
    {
        var no01Easy = new No01Easy();
        var twoSum = no01Easy.TwoSum(new[] { 1, 2, 6, 7 }, 3);
        Logger.LogMessage($"{twoSum[0]},{twoSum[1]}");
    }
}