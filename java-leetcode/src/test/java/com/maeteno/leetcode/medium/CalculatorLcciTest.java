package com.maeteno.leetcode.medium;

import org.junit.Assert;
import org.junit.Test;

public class CalculatorLcciTest {

    @Test
    public void calculate() {
        CalculatorLcci lcci = new CalculatorLcci();

        int result = lcci.calculate("5 + 1 * 2 - 6 / 2");

        Assert.assertEquals(4, result);
    }
}