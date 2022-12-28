using Microsoft.VisualStudio.TestTools.UnitTesting;
using Microsoft.VisualStudio.TestTools.UnitTesting.Logging;

namespace dotnet.Leetcode;

[TestClass]
public class No02MiddleTest
{
    [TestMethod]
    public void test_no_12()
    {
        var no12Middle = new No12Middle();

        var roman = no12Middle.IntToRoman(24);

        Logger.LogMessage($"{roman}");
    }
}