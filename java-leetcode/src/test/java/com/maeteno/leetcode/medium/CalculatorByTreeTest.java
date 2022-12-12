package com.maeteno.leetcode.medium;

import org.junit.Test;

import static org.junit.Assert.assertEquals;

public class CalculatorByTreeTest {

    @Test
    public void test_demo_1() {
        CalculatorByTree calc = new CalculatorByTree();

        int result = calc.calculate("1+25+3+4*7");

        assertEquals(10, result);
    }
}