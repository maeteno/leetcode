package com.maeteno.leetcode.medium;

import lombok.extern.slf4j.Slf4j;
import org.junit.Assert;
import org.junit.Test;

@Slf4j
public class CalculatorByQueueTest {
    private final CalculatorByQueue calc = new CalculatorByQueue();

    @Test
    public void test_demo_1() {
        // when
        int result = calc.calculate("5 + 1 * 2 - 6 / 2");

        // then
        Assert.assertEquals(4, result);
    }

    @Test
    public void test_demo_2() {
        // when
        int result = calc.calculate("3*5/3");

        // then
        Assert.assertEquals(5, result);
    }

    @Test
    public void test_demo_3() {
        // when
        int result = calc.calculate("3*6/3");

        // then
        Assert.assertEquals(6, result);
    }

    @Test
    public void test_demo_4() {
        // when
        int result = calc.calculate("3*6/3-0+4");

        // then
        Assert.assertEquals(10, result);
    }
}